// SPDX-License-Identifier: BSD-3-Clause

use is_executable::IsExecutable;
use serde::Deserialize;
use std::fs::File;
use std::iter;
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

    pub(crate) fn find_matches<'a>(&'a self, store_entry: &'a PathBuf) -> Box<dyn Iterator<Item=String> + '_> {
        Box::new(self.store_suffixes.iter().flat_map(move |store_suffix| -> Box<dyn Iterator<Item=String>> {
            let mut store_path = store_entry.clone();
            store_path.push(store_suffix);

            let finalized_store_path = store_path.clone();

            if !finalized_store_path.is_dir() {
                Box::new(iter::empty())
            } else if !self.individual {
                Box::new(iter::once(format!("alias {} -> {},\n", self.target, finalized_store_path.display())))
            } else {
                Box::new(finalized_store_path.read_dir()
                    .expect(format!("Error traversing Path: {}", finalized_store_path.display()).as_str())
                    .map(move |f| f.expect(format!("Error while reading directory contents: {}", store_path.display()).as_str()))
                    .map(|f| f.path())
                    .filter(|f| self.matches_individual(f))
                    .map(move |f| {
                        let mut path_part_specific = finalized_store_path.clone();
                        path_part_specific.push(f.file_name().unwrap_or_default());

                        let mut target = PathBuf::from(&self.target);
                        target.push(f.file_name().unwrap_or_default());

                        return format!("alias {} -> {},\n", target.display(), path_part_specific.display());
                    }))
            }
        }))
    }
}

pub(crate) fn get_patterns(path: PathBuf) -> Vec<Pattern> {
    let pattern_file = File::open(path.clone())
        .expect(format!("Failed to open pattern file {}", path.display()).as_str());
    serde_json::from_reader(pattern_file)
        .expect(format!("Failed to parse pattern file {}", path.display()).as_str())
}