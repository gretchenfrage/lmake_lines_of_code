// region: lmake_readme include "README.md" //! A
//! # lmake_lines_of_code  
//!
//! ***version: 2020.613.1337  date: 2020-06-13 authors: Luciano Bestia***  
//! **Lines of code for Rust projects**
//!
//!
//! | src code | doc comments | comments | examples | tests |
//! | :------: | :----------: | :------: | :------: | :---: |
//! |  lines   |     lines    |   lines  |   lines  | lines |
//! |   233    |      88      |    50    |    0     |   0   |
//!
//! [comment]: # (lmake_lines_of_code start)
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-282-green.svg)](https://github.com/LucianoBestia/lmake_lines_of_code/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-106-blue.svg)](https://github.com/LucianoBestia/lmake_lines_of_code/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-51-purple.svg)](https://github.com/LucianoBestia/lmake_lines_of_code/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/lmake_lines_of_code/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/LucianoBestia/lmake_lines_of_code/)
//!
//! [comment]: # (lmake_lines_of_code end)
//!
//! ## Lines of code for Rust projects
//!
//! Lines of code are not a "perfect" measurement of anything.\
//! Anybody can write a very big number of lines of useless code and comments.\
//! But for 95% of the cases they are good enough.\
//! Most of the developers use some "standard" coding practices and that is quantifiable and comparable.\
//! \
//! The `src_code_lines` is the most important count.\
//! That is actual code written for that project without  doc comments, comments, unit tests, integration tests and examples.\
//! Sometimes is great to see a big number here. It means there was a lot of work invested. But other times we want to see a small number. It means the developer understands the problem very well and don't try to solve anything outside that scope.\
//! \
//! The `src_doc_comment_lines` counts doc comments. They will eventually become docs. The count of lines shows how many documentation is written.\
//! \
//! The `src_comment_lines` counts code comments. Code comments are important to understand the code. The count of lines shows how understandable is the code.\
//! \
//! The `tests_lines` counts lines in tests and shows how good is the code tested. Here are the unit tests and integration test combined.\
//! \
//! The `examples_lines` counts lines in examples and shows how good is explained how to use the code.  
//!
//! ## Folder and file structure
//!
//! The folder structure of a single Rust project is simple.\
//! The project starts in the folder that contains cargo.toml.\
//! The /src/ folder contains all the rust \*.rs files.\
//! The /tests/ folder contains integration tests.\
//! The /examples/ folder contains examples.\
//! Inside a rs file the doc comment line start with /// or //!.\
//! The normal comments start with // or /!.\
//! I will ignore the block comments. They are usually NOT used for comments, but to temporarily disable a piece of code. So I count this as code and not comments.\
//! \
//! The src/\*.rs file can contain unit tests that start with #[cfg(test)]. I assume that these are always at the end of the file. There should not be any normal code after #[cfg(test)], only tests.\
//! \
//! All other files: md, toml, html, js, ... are not counted.  
//!
//! ### Workspace
//!
//! Workspaces have member projects, that are written in cargo.toml.\
//! The program counts lines of every project and sums them together.  
//!
//! ## Output
//!
//! The output is markdown text for a table and markdown text for shield badges.\
//! Just copy/paste it into README.md.  
//!
//! ## Install and run
//!
//! `cargo install lmake_lines_of_code`  
//! In the Rust project folder, where cargo.toml is:  
//! `lmake_lines_of_code`
//!
//! ## Development
//!
//! Documentation:\
//! <https://lucianobestia.github.io/lmake_lines_of_code>\
//! List of prepared make tasks for development: build, run, doc, publish,...\
//! `clear; cargo make`  
//!
//! ## CREV - Rust code reviews - Raise awareness
//!
//! Please, spread this info.\
//! Open source code needs a community effort to express trustworthiness.\
//! Start with reading the reviews of the crates you use.Example: [web.crev.dev/rust-reviews/crate/num-traits/](https://web.crev.dev/rust-reviews/crate/num-traits/)  
//! Than install the CLI [cargo-crev](https://github.com/crev-dev/cargo-crev)\. Read the [Getting Started guide](https://github.com/crev-dev/cargo-crev/blob/master/cargo-crev/src/doc/getting_started.md).
//! On your Rust project, verify the trustworthiness of all dependencies, including transient dependencies with `cargo crev verify`\
//! Write a new review for the crates you trust. Or for the crate versions you think are dangerous.\
//! Help other developers, inform them and share your opinion.\
//! Use this webpage to help you: [web.crev.dev/rust-reviews/review_new](https::/web.crev.dev/rust-reviews/review_new)  
// endregion: lmake_readme include "README.md" //! A

// region: Clippy
#![deny(unused_must_use)]

// CONS: Unnecessary code.
// PROS: more readable without knowing that the type is bool.
#[allow(clippy::bool_comparison)]
// endregion: Clippy

// region: mod, extern and use statements
mod count_lines_mod;
mod readme_include_mod;
mod utilsmod;

use count_lines_mod::*;
use readme_include_mod::*;

//use clap::*;
//use unwrap::unwrap;

#[allow(unused_imports)]
use ansi_term::Colour::{Green, Red, Yellow};
//use ansi_term::Style;
use clap::App;
use std::env;
// endregion

#[allow(clippy::print_stdout, clippy::integer_arithmetic)]
/// The program starts here.
fn main() {
    // this function is different for Windows and for Linux.
    // Look at the code of this function (2 variations).
    enable_ansi_support();

    // define the CLI input line parameters using the clap library
    let _arguments = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();
    //let v = one_project_count_lines();
    let v = workspace_or_project_count_lines();
    println!("{}", as_md_table(&v));
    let text_to_include = as_shield_badges(&v);
    println!("{}", &text_to_include);
    readme_include(&text_to_include);
}

// region: different function code for Linux and Windows
#[cfg(target_family = "windows")]
/// only on windows "enable ansi support" must be called
pub fn enable_ansi_support() {
    let _enabled = ansi_term::enable_ansi_support();
}

#[cfg(target_family = "unix")]
//on Linux "enable ansi support" must not be called
pub fn enable_ansi_support() {
    // do nothing
}
// endregion
