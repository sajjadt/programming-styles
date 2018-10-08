use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

extern crate regex;
use regex::Regex;

const STOP_FILE_NAME : &'static str = "../stop_words.txt";
const N: u32 = 25;

fn parse_input() -> String {
  let args: Vec<_> = env::args().collect();
  args[1].to_string()
}

fn read_file(in_file: String) -> String {
  let mut fs = File::open(in_file).expect("input file not found");
  
  let mut input_contents : String =  String::new();
  fs.read_to_string(&mut input_contents)
     .expect("something went wrong reading the input file");
  input_contents
}

fn normalize(mut data: String) -> String {
  data.make_ascii_lowercase();
  data
}

fn scan(data: String) -> HashMap<String, u32> {
  let re = Regex::new(r"[a-z]{2}[a-z]*");
  let mut frequency: HashMap<String, u32> = HashMap::new();
  
  for word in re.unwrap().find_iter(&data) {
    *frequency.entry(word.as_str().to_string()).or_insert(0) += 1;
  }
  frequency
}

fn remove_stop_words(mut frequency: HashMap<String, u32>) -> HashMap<String, u32> {
    
  let mut fs = File::open(STOP_FILE_NAME).expect("stop words file not found");
  let mut contents = String::new();
  fs.read_to_string(&mut contents)
      .expect("something went wrong reading the stop words file");
  
  contents.make_ascii_lowercase();
  let v: HashSet<&str> = contents.split(',').collect();  
  for word in v {
    frequency.remove(word);
  }
  frequency
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
  print_all(sort(remove_stop_words(scan(normalize(read_file(parse_input()))))));  
}

