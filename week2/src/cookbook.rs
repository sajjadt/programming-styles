use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

#[macro_use]
extern crate lazy_static;

extern crate regex;
use regex::Regex;
use std::sync::RwLock;


const STOP_FILE_NAME : &'static str = "../stop_words.txt";
const N: u32 = 25;
lazy_static! {
  static ref IN_FILE: RwLock<String> = RwLock::new("".to_string());
  static ref INPUT_CONTENTS: RwLock<String> = RwLock::new("".to_string());
  static ref FREQ_TABLE: RwLock<HashMap<String, u32>> = RwLock::new(HashMap::new());
  static ref SORTED_WORDS: RwLock<Vec<(String, u32)>> = RwLock::new(Vec::new());
}

fn parse_input() {
  let args: Vec<_> = env::args().collect();
  let mut in_file = IN_FILE.write().unwrap();
  *in_file = args[1].to_string();
}

fn read_input_file() {
  let in_file = IN_FILE.read().unwrap();
  let mut fs = File::open(&*in_file).expect("input file not found");
  
  let mut input_contents = INPUT_CONTENTS.write().unwrap();
  fs.read_to_string(&mut input_contents)
     .expect("something went wrong reading the input file");
}


fn normalize() {
  // collect input words
  let mut input_contents = INPUT_CONTENTS.write().unwrap();
  input_contents.make_ascii_lowercase();
}

fn scan() {
  // Ignore some of the words
  let re = Regex::new(r"[a-z]{2}[a-z]*");
  
  let input_contents = INPUT_CONTENTS.read().unwrap();
  let mut freq_table = FREQ_TABLE.write().unwrap(); 
  
  for word in re.unwrap().find_iter(&*input_contents) {  // word is a &str
    *freq_table.entry(word.as_str().to_string()).or_insert(0) += 1; // word does not live long enough
  }
}

fn remove_stop_words() {
  let mut frequency = FREQ_TABLE.write().unwrap();   
  let mut fs = File::open(STOP_FILE_NAME).expect("stop words file not found");
  let mut contents = String::new();
  fs.read_to_string(&mut contents)
      .expect("something went wrong reading the stop words file");
  
  contents.make_ascii_lowercase();
  let v: HashSet<&str> = contents.split(',').collect();  
  for word in v {
    frequency.remove(word);
  }
}

fn sort() {
  let frequency = FREQ_TABLE.read().unwrap();
  let mut sorted_words = SORTED_WORDS.write().unwrap();
  let mut vv = Vec::from_iter(&*frequency);
  vv.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
  vv.truncate(N as usize);
  
  for (word, count) in vv {
    sorted_words.push( (word.to_string(), *count));
  }
  
}

fn print_all() {
  let sorted_words = SORTED_WORDS.read().unwrap(); 
  for (word, count) in &*sorted_words {
    println!("{}  -  {}", word, count);
  }
}


fn main() { 
  parse_input();
  read_input_file();
  normalize();
  scan();
  remove_stop_words();
  sort();
  print_all();
}

