use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

extern crate regex;
use regex::Regex;
use std::process;

const N: u32 = 25;

/// Parse User Comand line input (input file and stop words files)
fn parse_input() -> (String, String) {
  let args: Vec<_> = env::args().collect();
  if args.len() != 3 {
      print!("Usage: cargo run --bin pipeline [input_file] [stop_words_file]\n");
      process::exit(0);
  }
  (args[1].to_string(), args[2].to_string())
}

// Read both files
fn read_files(in_files: (String, String)) -> (String, String) {
  let mut fs = File::open(in_files.0).expect("input file not found");
  
  let mut input_contents : String =  String::new();
  fs.read_to_string(&mut input_contents)
     .expect("something went wrong reading the input file");
    
  fs = File::open(in_files.1).expect("stop words file not found");
  let mut contents = String::new();
  fs.read_to_string(&mut contents)
      .expect("something went wrong reading the stop words file");
  
  contents.make_ascii_lowercase();
     
  (input_contents, contents)
}

/// Normalizes the input files by lowercasing them
fn normalize(mut data: (String, String)) -> (String,String) {
  data.0.make_ascii_lowercase();
  data.1.make_ascii_lowercase();
  data
}

/// Scan the input file
/// Do not modifies stop words yet
fn scan(data: (String,String)) -> (HashMap<String, u32>, String) {
  let re = Regex::new(r"[a-z]{2}[a-z]*");
  let mut frequency: HashMap<String, u32> = HashMap::new();
  
  for word in re.unwrap().find_iter(&data.0) {
    *frequency.entry(word.as_str().to_string()).or_insert(0) += 1;
  }
  (frequency, data.1)
}

/// Removes stop words from the input file histogram
fn remove_stop_words(mut frequency: (HashMap<String, u32>, String)) -> HashMap<String, u32> {
  let v: HashSet<&str> = frequency.1.split(',').collect();  
  for word in v {
    frequency.0.remove(word);
  }
  frequency.0
}

fn sort(frequency: HashMap<String, u32>) -> Vec<(String, u32)> {
  let mut sorted_words = Vec::from_iter(frequency);
  sorted_words.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
  sorted_words.truncate(N as usize);
  sorted_words
}

fn print_all(frequency: Vec<(String, u32)>) {
  for (word, count) in frequency {
    println!("{}  -  {}", word, count);
  }
}


fn main() {
  print_all(sort(remove_stop_words(scan(normalize(read_files(parse_input()))))));  
}

