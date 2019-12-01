use std::fs::*;
use std::env;
use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::cmp;
use std::string::ToString;

fn count(input: String) -> (bool, bool) {
	let mut twocount = false;
    let mut threecount = false;
    let mut seen = HashMap::new(); 
    for c in input.chars() {
    	let count = seen.entry(c).or_insert(0);
    	*count += 1;
    }
    for (k,v) in &seen {
    	if *v == 2 {
    		twocount = true;
    	}
    	if *v == 3 {
    		threecount = true;
    	}
    }
    (twocount, threecount)
}

fn levenshtein_d1stance<T>(s1: &T, s2: &T) -> usize where T: ToString {
    let v1: Vec<char> = s1.to_string().chars().collect();
    let v2: Vec<char> = s2.to_string().chars().collect();
    let v1len = v1.len();
    let v2len = v2.len();
    
    // Early exit if one of the strings is empty
    if v1len == 0 { return v2len; }
    if v2len == 0 { return v1len; }

    fn min3<T: Ord>(v1: T, v2: T, v3: T) -> T{
        cmp::min(v1, cmp::min(v2, v3))
    }
    fn delta(x: char, y: char) -> usize {
        if x == y { 0 } else { 1 }
    }
    
    let mut column: Vec<usize> = (0..v1len+1).collect();
    for x in 1..v2len+1 {
        column[0] = x;
        let mut lastdiag = x-1;
        for y in 1..v1len+1 {
            let olddiag = column[y];
            column[y] = min3(column[y] + 1,
                             column[y-1] + 1,
                             lastdiag + delta(v1[y-1], v2[x-1]));
            lastdiag = olddiag;
        }
    }
    column[v1len]
}

fn main() -> io::Result<()> {
	let mut two = 0;
	let mut three = 0;
	let mut boxes = Vec::new();

    if env::args().count() < 2 {
        println!("Missing argument");
    } else {
        let args: Vec<String> = env::args().collect();
        let arg = &args[1];
        let infile = File::open(arg)?;
        let _reader = BufReader::new(infile);
        for line in _reader.lines() {
        	let l = line.unwrap();
        	let _x = count(l.clone());
        	if _x.0 == true { two += 1; }
        	if _x.1 == true { three += 1; }
        	if _x.0 == true || _x.1 == true {
        		boxes.push(l.clone());
        	}
        }
	}

   	println!("Checksum: {}", two * three);

 	for i in boxes.iter() {
 		for j in boxes.iter() {
 			if levenshtein_d1stance(i,j) == 1 {
 				let mut idx = 0;
 				while idx < i.len() {
 					if i.chars().nth(idx) == j.chars().nth(idx) {
 						print!("{}",i.chars().nth(idx).unwrap());
 					}
 					idx += 1;
 				}
                std::process::exit(0);
 			}
 		}
 	}
	Ok(())
}
