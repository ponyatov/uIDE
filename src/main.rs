#![allow(unused_imports)]

mod config;
mod gui;
mod vm;

use memmap2::Mmap;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let _argc = argv.len();
    arg(0, &argv[0]);
    for (argc, argv) in argv.iter().enumerate().skip(1) {
        arg(argc, argv);
        let file = File::open(Path::new(argv)).unwrap();
        let src = unsafe { Mmap::map(&file).unwrap() };
        eprintln!("\tsize: {} bytes", src.len());
    }
    gui::run(&String::from("/dev/null")).expect("GUI fault");
}

fn arg(argc: usize, argv: &str) {
    eprintln!("argv[{argc}] = {argv:?}");
}
