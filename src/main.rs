// SPDX-License-Identifier: BSD-3-Clause

mod args;

use std::collections::HashMap;
use crate::args::Cli;
use clap::Parser;
use std::process::Command;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{PathBuf};
use std::str::FromStr;
use serde::{Deserialize};

#[derive(Deserialize,Hash, Eq, PartialEq, Debug)]
struct Pattern {
    target: String,
    name: String,
    pattern: Vec<String>,
}

fn main() -> std::io::Result<()>{
    let cli = Cli::parse();
    
    let pattern_file = File::open(cli.patterns)?;
    let patterns: Vec<Pattern> = serde_json::from_reader(pattern_file)?;

    let store_items = Command::new("nix-store")
        .arg("-q")
        .arg("/run/current-system")
        .arg("-R")
        .output()
        .expect("failed to execute process");

    fs::create_dir_all(cli.output.clone())?;

    let alias_files: HashMap<&Pattern, File> = patterns.iter().map(|p| {
        let mut fp = cli.output.clone();
        fp.push(&p.name);
        let mut file = File::options().append(true).create(true).open(fp).unwrap();
        (p, file)
    }).collect();
    
    String::from_utf8_lossy(&store_items.stdout)
        .split_whitespace()
        .map(PathBuf::from)
        .filter(|p| p.is_dir())
        .for_each(|path| {
            alias_files.iter().for_each(|(pattern, mut file)|{
                pattern.pattern.iter().for_each(|store_suffix| {
                    let mut path_part = path.clone();
                    path_part.push(store_suffix);
                    if path_part.is_dir() {
                        file.write(format!("alias {} -> {},\n", pattern.target, path_part.display()).as_ref()).expect("Error writing alias to file");
                    }
                });
            })
        });

    // println!("output is {}", store_items_str.get(0));
    Ok(())
}
