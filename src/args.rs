// SPDX-License-Identifier: BSD-3-Clause

use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(name="aa-alias-manager", version=env!("CARGO_PKG_VERSION"),about=env!("CARGO_PKG_DESCRIPTION"), author=env!("CARGO_PKG_AUTHORS"))]
pub struct Cli {
    /// target folder to write into. WARNING: Will be overridden if --append is not set.
    #[clap(short, long, default_value = "aliases.d")]
    pub output: PathBuf,

    /// Pattern file to read and use
    #[clap(short, long, default_value = "patterns.json")]
    pub patterns: PathBuf,

    /// Append to an existing set of aliases instead of overriding. Might be useful if working with multiple generations.
    #[clap(short, long, default_value_t = false)]
    pub append: bool,
}
