use std::fs::*;
use std::env;
use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
extern crate lexical;

fn main() -> io::Result<()> {
    let mut _x = 0;
    let mut _rd = 0;
    let mut hm = HashMap::new();
    let mut v = Vec::new();

    if env::args().count() < 2 {
        println!("Missing argument");
    } else {
        let args: Vec<String> = env::args().collect();
        let arg = &args[1];
        let infile = File::open(arg)?;
        let _reader = BufReader::new(infile);
        for line in _reader.lines() {
            _rd = lexical::parse(line.unwrap());
            v.push(_rd);
        }

        loop {
            let mut _iter = v.iter();
            print!("Reading {}, map size {}\r",arg,hm.len());
            for _i in _iter {
                _x += _i;
                if hm.contains_key(&_x) {
                    println!("\n{} was seen before!", _x);
                    std::process::exit(_x);
                }
                hm.insert(_x,_x);
            }
        }
    }
    println!("Result {}",_x);
    Ok(())

}
