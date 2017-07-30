//extern crate num;

use std::env;
use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::cmp::max;
//use std::collections::HashMap;
//use num::pow;

fn boyer_moore(text: &[u8], patt_list: &[&[u8]]) -> Result<Vec<Vec<i32>>, String> {
    let m = patt_list.into_iter().map(|arr| arr.len()).max().unwrap();
    let n = text.len();
    let l = patt_list.len();

    if m != patt_list.into_iter().map(|arr| arr.len()).min().unwrap() {
        return Err("The lengths of patterns have to be the same. ".to_string());
    }

    if n < m {
        return Err("The length of input text have to be longer than that of pattern. ".to_string());
    }

    // prepare a result container
    let mut result: Vec<Vec<i32>> = Vec::new();
    for _ in 0..l { // O(l)
        result.push(Vec::new());
    }

    // generate skip table
    let mut skip_table: Vec<[usize; 256]> = Vec::new();
    for patt in patt_list {
        let mut tab: [usize; 256] = [m; 256];
        for i in 0..m-1 {
            tab[patt[i] as usize] = m-i-1;
        }
        skip_table.push(tab);
    }

    for (k, patt) in patt_list.into_iter().enumerate() {
        let mut pos = m-1;
        'search: while pos < n {
            for i in 0..m {
                if text[pos-i] != patt[m-1-i] {
                    pos = pos + max(skip_table[k][text[pos-i] as usize], i+1);
                    continue 'search;
                }
            }
            result[k].push((pos-m+1) as i32);
            pos = pos + m;
        }
    }

    Ok(result)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Usage : cargo run text_filename pattern1 pattern2 ...");
    }

    // read text file
    let path = Path::new(&args[1]);
    let mut reader = BufReader::new(
        match File::open(&path) {
            Err(why)    => panic!("Could not open a file. :{}", Error::description(&why)),
            Ok(file)    => file,
        });
    let mut text = String::new();
    let _ = reader.read_line(&mut text);
    let text = text;


    let patterns = &args[2..args.len()];
    let patt_list = patterns.into_iter().map(|s| s.as_bytes()).collect::<Vec<&[u8]>>();

    match boyer_moore(text.as_bytes(), &patt_list) {
        Ok(v) => {
            for (i, arr) in v.into_iter().enumerate() {
                print!("{} : ", patterns[i as usize]);
                for x in &arr {
                    print!("{}, ", x);
                }
                println!("");
            }
        }
        Err(msg)    => println!("{}", msg),
    }
}
