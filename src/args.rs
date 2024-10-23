// SPDX-License-Identifier: BSD-3-Clause

use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(name="aa-alias-manager", version=env!("CARGO_PKG_VERSION"),about=env!("CARGO_PKG_DESCRIPTION"), author=env!("CARGO_PKG_AUTHORS"))]
pub struct Cli {
    #[clap(short, long, default_value = "aliases.d")]
    pub output: PathBuf,

    #[clap(short, long, default_value = "patterns.json")]
    pub patterns: PathBuf,

}
