extern crate regex;
#[macro_use] extern crate lazy_static;
extern crate lexical;
use std::fs::*;
use std::env;
use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use regex::Regex;

pub struct Claim {
	id: u32,
	top: u32,
	left: u32,
	width: u32,
	height: u32,
}

pub fn parse(arg: &str) -> Claim {
	// #7 @ 756,392: 27x19
	lazy_static! {
		static ref re: Regex = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
	}
	let caps = re.captures(arg).unwrap();
	Claim {
		id: lexical::parse(caps.get(1).unwrap().as_str()), 
		left: lexical::parse(caps.get(2).unwrap().as_str()), 
		top: lexical::parse(caps.get(3).unwrap().as_str()), 
		width: lexical::parse(caps.get(4).unwrap().as_str()), 
		height: lexical::parse(caps.get(5).unwrap().as_str()),
	}

}

pub fn overlap(c1: &Claim, c2: &Claim) -> u32 {
	let bottom = c1.top + c1.height;
	let right = c1.left + c1.width;
	let mut area = 0;
	let mut x1 = 0;
	let mut x2 = 0;
	let mut y1 = 0;
	let mut y2 = 0;

	if bottom >= c2.top || c2.top+c2.height >= c1.top {
		return 0;
	}

	if right >= c2.left || c2.left + c2.width >= c1.left {
		return 0;
	}

	if bottom >= c2.top { y2 = c2.top ; }
	if bottom >= c2.top+c2.height {y2=c2.top}
	
   	let x = x2-x1;
   	let y = y2-y1;
   	area = x * y;

   /*	println!("{} {} {} {} -> {} {} {} {} : {}", 
   		bottom, c2.top,
   		right, c2.left,
   		c2.left + c2.width, right,
   		c2.top + c2.height, bottom,
   		area);
   		*/
   	return area;

	return 0;
}


fn main() -> io::Result<()>{
    let mut v = Vec::new();
    let mut totarea = 0;

    if env::args().count() < 2 {
        println!("Missing argument");
    } else {
        let args: Vec<String> = env::args().collect();
        let arg = &args[1];
        let infile = File::open(arg)?;
        let _reader = BufReader::new(infile);
        for line in _reader.lines() {
        	let l = line.unwrap();
        	let c = parse(&l.clone());
        	//println!("{},{},{},{},{}", c.id,c.top,c.left,c.width,c.height);
        	v.push(c);
        }
	}

	for i in v.iter() {
		for j in v.iter() {
			if i.id == j.id {
				continue;
			} else {
				let area = overlap(i,j);
				if area > 0 {
//        			println!("{}: {},{} {},{}", i.id,i.top,i.left,i.width,i.height);
//        			println!("{}: {},{} {},{}", j.id,j.top,j.left,j.width,j.height);
					println! ("{} overlaps {}",i.id, j.id );
					totarea += area;
				}
			}

		}
	}
	println!("Total area : {}", totarea);
	Ok(())
}
