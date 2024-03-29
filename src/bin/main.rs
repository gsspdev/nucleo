#![allow(unused_imports)]
#![allow(dead_code)]

extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate atty;
extern crate shlex;
extern crate nucleo;
extern crate time;

use derive_builder::Builder;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

use clap::{crate_version, App, Arg, ArgMatches};
// use nucleo::prelude::*;

fn main() {
    println!("Hello, Nucleo!")
}
