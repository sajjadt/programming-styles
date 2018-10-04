use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

extern crate regex;
use regex::Regex;


fn main() {
  
  let N = 25;
  let args: Vec<_> = env::args().collect();
  let stop_file_name = "../stop_words.txt";
  let input_file_name = &args[1];

  let mut fs = File::open(stop_file_name).expect("stop words file not found");
  let mut contents = String::new();
  fs.read_to_string(&mut contents)
      .expect("something went wrong reading the stop words file");
  
  contents.make_ascii_lowercase();
  let v: HashSet<&str> = contents.split(',').collect();

  let mut input_contents = String::new();
  fs = File::open(input_file_name).expect("input file not found");
  fs.read_to_string(&mut input_contents)
      .expect("something went wrong reading the input file");
  
  input_contents.make_ascii_lowercase();
  let re = Regex::new(r"[a-z]{2}[a-z]*");
  let mut frequency: HashMap<&str, u32> = HashMap::new();

  for word in re.unwrap().find_iter(&input_contents) {  // word is a &str
    if ! v.contains(word.as_str()) {
      *frequency.entry(word.as_str()).or_insert(0) += 1; // word does not live long enough
    }
  }

  let mut vv = Vec::from_iter(frequency);
  vv.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
  vv.truncate(N);
  for (word, count) in vv {
    println!("{}  -  {}", word, count);
  }
  
}

