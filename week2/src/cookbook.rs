use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

extern crate regex;
use regex::Regex;
const N: u32 = 25;
use std::process;

fn read_file_and_lower(in_file: String) -> String {
  let mut fs = File::open(in_file).expect("input file not found");
  
  let mut input_contents : String =  String::new();
  fs.read_to_string(&mut input_contents)
     .expect("something went wrong reading the input file");
     
  input_contents.make_ascii_lowercase();
  input_contents
}

fn calc_freq(input: String, v: HashSet<&str>) -> HashMap<String, u32>{
  let re = Regex::new(r"[a-z]{2}[a-z]*");
  let mut frequency: HashMap<String, u32> = HashMap::new();
  
  for word in re.unwrap().find_iter(&input) {  // word is a &str
    if ! v.contains(word.as_str()) {
      *frequency.entry(word.as_str().to_string()).or_insert(0) += 1; // word does not live long enough
    }
  }
  frequency
}

fn sort(frequency: HashMap<String, u32>) -> Vec<(String, u32)> {
  let mut sorted_words = Vec::from_iter(frequency);
  sorted_words.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
  sorted_words.truncate(N as usize);
  sorted_words
}

fn main() {
  
  let args: Vec<_> = env::args().collect();
  
  if args.len() != 2 {
      print!("Usage: cargo run --bin pipeline [input_file]\n");
      process::exit(0);
  }
  
  let stop_file_name = "../stop_words.txt";
  let input_file_name = &args[1];

  let mut contents = read_file_and_lower(stop_file_name.to_string());
  let v: HashSet<&str> = contents.split(',').collect();

  let mut input_contents = read_file_and_lower(input_file_name.to_string());
 
  let mut frequency = calc_freq(input_contents.to_string(), v); 
  let mut vv = sort(frequency);
  
  for (word, count) in vv {
    println!("{}  -  {}", word, count);
  }
  
}

