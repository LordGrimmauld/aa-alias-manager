// SPDX-License-Identifier: BSD-3-Clause

mod args;
use crate::args::Cli;
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    println!("output is {}", cli.output.display());
}
