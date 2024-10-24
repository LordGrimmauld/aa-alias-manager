// SPDX-License-Identifier: BSD-3-Clause

mod args;

use crate::args::Cli;
use clap::Parser;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

#[derive(Deserialize, Hash, Eq, PartialEq, Debug)]
struct Pattern {
    target: String,
    name: String,
    pattern: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    let pattern_file = File::open(cli.patterns.clone())
        .expect(format!("Failed to open pattern file {}", cli.patterns.display()).as_str());
    let patterns: Vec<Pattern> = serde_json::from_reader(pattern_file)
        .expect(format!("Failed to parse pattern file {}", cli.patterns.display()).as_str());

    let store_items = Command::new("nix-store")
        .arg("-q")
        .arg("/run/current-system")
        .arg("-R")
        .output()
        .expect("failed to query store dependencies of current system");

    fs::create_dir_all(cli.output.clone())
        .expect(format!("failed to create alias folder {}", cli.output.display()).as_str());

    let alias_files: HashMap<&Pattern, File> = patterns
        .iter()
        .map(|p| {
            let mut fp = cli.output.clone();
            fp.push(&p.name);
            (
                p,
                File::options()
                    .append(cli.append)
                    .write(!cli.append)
                    .create(true)
                    .open(fp.clone())
                    .expect(format!("failed to create alias file {}", fp.display()).as_str()),
            )
        })
        .collect();

    String::from_utf8_lossy(&store_items.stdout)
        .split_whitespace()
        .map(PathBuf::from)
        .filter(|p| p.is_dir())
        .for_each(|path| {
            alias_files.iter().for_each(|(pattern, mut file)| {
                pattern.pattern.iter().for_each(|store_suffix| {
                    let mut path_part = path.clone();
                    path_part.push(store_suffix);
                    if path_part.is_dir() {
                        file.write(
                            format!("alias {} -> {},\n", pattern.target, path_part.display())
                                .as_ref(),
                        )
                            .expect("Error writing alias to file");
                    }
                });
            })
        });
}
