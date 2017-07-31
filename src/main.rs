//extern crate num;

use std::env;
use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::cmp::max;
use std::iter;
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

    // generate skip table and next table
    let mut skip_table: Vec<[usize; 256]> = Vec::new();
    let mut next_table: Vec<Vec<usize>> = Vec::new(); 
    for patt in patt_list {
        // skip table
        let mut tab: [usize; 256] = [m; 256];
        for i in 0..m-1 {
            tab[patt[i] as usize] = m-i-1;
        }
        skip_table.push(tab);

        // next table
        let mut tab = iter::repeat(0).take(m).collect::<Vec<usize>>();
        for s in (0..m).rev() {
            let mut disp = m;
            let mut reached = true;
            for j in (s+1..m).rev() {
                disp = j;
//println!("j-s : {}, j : {}", j-s, j);
                if patt[j-s-1] != patt[j] {
                    reached = false;
                    break;
                }
            }
            if !reached {
                tab[disp] = m-disp+s;
            } else {
                for k in 0..disp {
                    tab[k] = m-k+s;
                }
            }

//print!("{} : ", s);
//for i in 0..m {
//    print!("{:>2}, ", tab[i]);
//}
//println!("");
        }
        next_table.push(tab);
    }

//for next in next_table.iter() {
//    for val in next {
//        print!("{}, ", val);
//    }
//    println!("");
//}


    for (k, patt) in patt_list.into_iter().enumerate() {
        let mut pos = m-1;
        let skip = skip_table[k];
        let next = &next_table[k];
        while pos < n {
            let mut disp = 0;
            let mut sub = 0;
            let mut found = true;
            for j in 0..m {
                disp = m-j-1;
                // if matching failed
                if text[pos-sub] != patt[disp] {
                    found = false;
                    break;
                }
                sub = sub+1;
            }
            if found {
                result[k].push((pos+1-sub) as i32);
                pos = pos+1;
            }
            pos = pos + max(skip[text[pos] as usize], next[disp]) - sub;
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
