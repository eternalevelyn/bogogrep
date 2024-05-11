use std::fs;
use std::io::Error;
use rand::prelude::*;

pub fn read_file(path: &String) -> Result<String, Error> {        
    match fs::read_to_string(path) {
        Ok(string) => return Ok(string),
        Err(e) => return Err(e),
    };
}
pub fn search_str_rand<'a>(data: String, matcher: &String)-> Vec<String> {
    let mut matches: Vec<String> = vec![];
    let datalines = data.lines();
    let datalineno = data.lines().count();
    let mut match_found = false;
    let mut runs: u64 = 0;
    let mut rng = thread_rng();
    let mut checked_lines: Vec<usize> = vec![];
    let checkable_lines: Vec<usize> = {
        let mut checkables: Vec<usize> = vec![];
        for i in 0..datalineno-1 {
            checkables.push(i)
        };
        checkables
    };
    while !match_found {
        runs += 1;
        let lineno: usize = rng.gen_range(0..datalineno-1);
        let selected_ln = match data.lines().nth(lineno)  {
            Some(ln) => ln,
            None => "",           
        };
        checked_lines.sort();
        checked_lines.dedup();
        if checked_lines != checkable_lines {
            if !checked_lines.contains(&lineno) {
                checked_lines.push(lineno);
                if selected_ln.contains(&matcher[..]) {
                    let fmted = format!("{lineno}: '{selected_ln}'; found at try #{runs}");
                    matches.push(fmted);
                } else { continue; }
            } else { continue; }
        } else { match_found = true; } 
    };
    return matches
}