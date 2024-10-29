// SPDX-License-Identifier: BSD-3-Clause

mod args;
use crate::args::Cli;

mod pattern;
use crate::pattern::{get_patterns, Pattern};

use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::{exit, Command};

fn main() {
    let cli = Cli::parse();

    let patterns = get_patterns(cli.patterns);

    let store_items = Command::new("nix-store")
        .arg("-q")
        .arg("/run/current-system")
        .arg("-R")
        .output()
        .expect("failed to query store dependencies of current system");

    // courtesy: don't delete target folders if they don't match the rough pattern
    if cli.output.is_dir() && !cli.append {
        if fs::read_dir(cli.output.clone())
            .expect("Error while reading target directory contents")
            .map(|f| f.expect("Error while confirming target directory contents"))
            .any(|f| !f.path().is_file()) {
            eprintln!("Found irregular file in output. Refusing to wipe output directory.");
            exit(1)
        }

        fs::remove_dir_all(cli.output.clone())
            .expect(format!("Failed to clean old output {}", cli.output.display()).as_str());
    }

    fs::create_dir_all(cli.output.clone())
        .expect(format!("Failed to create alias folder {}", cli.output.display()).as_str());

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
                pattern.find_matches(&path, |a| {
                    file.write_all(a.as_ref()).expect("Error writing alias to file");
                });
            })
        });
}
