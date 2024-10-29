// SPDX-License-Identifier: BSD-3-Clause

use std::fmt::Write;
use is_executable::IsExecutable;
use serde::Deserialize;
use std::fs::File;
use std::path::PathBuf;

#[derive(Deserialize, Hash, Eq, PartialEq, Debug)]
pub(crate) struct Pattern {
    pub(crate) target: String,
    pub(crate) name: String,
    pub(crate) store_suffixes: Vec<String>,

    #[serde(default)]
    pub(crate) individual: bool,

    #[serde(default)]
    only_exe: bool,

    #[serde(default)]
    disallowed_strings: Vec<String>,

    #[serde(default)]
    only_include: Vec<String>,
}

impl Pattern {
    pub(crate) fn matches_individual(&self, file: &PathBuf) -> bool {
        let filename: String = file.file_name().unwrap_or_default().to_str().unwrap_or_default().to_string();
        (file.is_file() || file.is_symlink()) // todo: should symlink match??
            && (file.is_executable() || !self.only_exe)
            && self.disallowed_strings.iter().all(|s| !filename.contains(s))
            && (self.only_include.len() == 0 || self.only_include.iter().any(|s| filename.eq(s)))
    }

    pub(crate) fn find_matches(&self, store_entry: &PathBuf, mut consumer: impl FnMut(&str)) {
        if !store_entry.is_dir() { return; } // Nothing worth aliasing except directory contents

        // instead of making a new store_path for each suffix, we have this path
        //   between runs it is equal to store_entry (but we have mutable access to it)
        //   each run uses it for their own operations but cleans up after itself
        let mut store_path = store_entry.clone();

        // we write strings to this buffer and then call `consume` with a reference to it.
        // Fewer allocations that calling format!, thus faster.
        let mut str_buf = String::new();

        // reused for the `self.target` clones that add onto the path
        let mut target = PathBuf::from(&self.target);

        for store_suffix in &self.store_suffixes {
            debug_assert_eq!(&store_path, store_entry);
            debug_assert!(str_buf.is_empty());

            store_path.push(store_suffix);
            if !store_path.is_dir() {
                // do nothing. This is relevant if the test path /nix/store/*/<store_suffix> does not exist.
            } else if self.individual {
                // yield multiple elements. This is useful because apparmor parser becomes slow with too many overlaps.
                for child in store_path.read_dir().unwrap_or_else(|err| panic!("Error traversing Path: {path:?}, {err}", path=store_path)) {
                    let child = child.unwrap_or_else(|err| panic!("Error while reading directory contents: {path:?}, {err}", path=store_path));
                    let child_path = child.path();
                    if !self.matches_individual(&child_path) {
                        continue;
                    }
                    let child_name = child_path.file_name().unwrap_or_default();
                    store_path.push(child_name);
                    target.push(child_name);
                    writeln!(&mut str_buf, "alias {} -> {},", self.target, store_path.display()).expect("could not write match to string");
                    consumer(&str_buf);

                    // clean up
                    str_buf.clear();
                    target.pop();
                    store_path.pop();
                }
            } else {
                // yield a single element
                writeln!(&mut str_buf, "alias {} -> {},", self.target, store_path.display()).expect("could not write match to string");
                consumer(&str_buf);
                str_buf.clear();
            }

            // cleanup
            store_path.clear();
            store_path.extend(store_entry);
        }
    }
}

pub(crate) fn get_patterns(path: PathBuf) -> Vec<Pattern> {
    let pattern_file = File::open(path.clone())
        .expect(format!("Failed to open pattern file {}", path.display()).as_str());
    serde_json::from_reader(pattern_file)
        .expect(format!("Failed to parse pattern file {}", path.display()).as_str())
}
