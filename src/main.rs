use std::io;
use std::fs;
use std::env::args;

fn main() {
    for arg in args().skip(1){
        fs::remove_file(arg);
    }
}
