use std::process;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::thread;

extern crate regex;
use regex::Regex;

extern crate itertools;
use itertools::Itertools;

// Read file
fn read_file_and_lower(in_file: String) -> String {
  let mut fs = File::open(in_file).expect("input file not found");
  
  let mut input_contents : String =  String::new();
  fs.read_to_string(&mut input_contents)
     .expect("something went wrong reading the input file");
     
  input_contents.make_ascii_lowercase();
  input_contents
}

// Regroup
fn regroup(pairs_list: Vec<Vec<(String, u32)>>) -> HashMap<String, Vec<(String, u32)>> {
  let mut results : HashMap<String, Vec<(String, u32)>> = HashMap::new();
  
  results.insert("a-e".to_string(), Vec::new());
  results.insert("f-j".to_string(), Vec::new());
  results.insert("k-o".to_string(), Vec::new());
  results.insert("p-t".to_string(), Vec::new());
  results.insert("u-z".to_string(), Vec::new());
  
  for pairs in pairs_list {
    for pair in pairs {
      match pair.0.chars().next().unwrap() {
        'a' ... 'e' => results.get_mut("a-e").unwrap().push(pair),
        'f' ... 'j' => results.get_mut("f-j").unwrap().push(pair),
        'k' ... 'o' => results.get_mut("k-o").unwrap().push(pair),
        'p' ... 't' => results.get_mut("p-t").unwrap().push(pair),
        'u' ... 'z' => results.get_mut("u-z").unwrap().push(pair),
        _ => println!("unexpted character"),
      }
    }
  }
  results
}

// This is the `main` thread
fn main() {

  const N: u32 = 25;

  let args: Vec<_> = env::args().collect();
  if args.len() != 2 {
    print!("Usage: cargo run --bin map_reduce [input_file]\n");
    process::exit(0);
  }

  let mut content : String = read_file_and_lower(args[1].to_string());
  let mut children = vec![];

  // Map (split words) to chunks of input text
  // Vec [ (word, 1), (word, 1), ... ]
  for mut lines_iter in &content.lines().into_iter().chunks(200) {
    let mut lines = lines_iter.join("\n");
    let mut freqs : Vec<(String, u32)> = Vec::new();
    children.push(thread::spawn(move || -> Vec<(String, u32)> {
        
      let mut contents = read_file_and_lower("../stop_words.txt".to_string());
      let mut stop_words: HashSet<String> = HashSet::new();
      for word in contents.split(',') {
        stop_words.insert(word.to_string());
      }

      let re = Regex::new(r"[a-z]{2}[a-z]*"); 

      for word in re.unwrap().find_iter(&lines) {
        if ! stop_words.contains(&word.as_str().to_string()) {
          freqs.push((word.as_str().to_string(), 1));
        }
      }
      println!("Thraed split words finished...");
      freqs
    }));
  }

  // Reduce
  // n * Vec<(String, u32)> -> Vec<Vec<(String, u32)>>
  let mut intermediate_results : Vec<Vec<(String, u32)>> = Vec::new();
  for child in children {
    intermediate_results.push(child.join().unwrap());
  }

  // Regroup
  // Vec<Vec<(String, u32)>>) -> HashMap<String, Vec<(String, u32)>>
  let mut regroupped_results = regroup(intermediate_results);

  // Map (count words)
  // HashMap<group, Vec<(String, u32)>> -> 
  let mut children2 = vec![];
  for (key, value) in regroupped_results {
    children2.push(thread::spawn(move || -> HashMap<String, u32> {
      println!("Count word thread map");
      let mut freqs : HashMap<String, u32> = HashMap::new();

      for (word, freq) in &value {
        if freqs.contains_key(word) {
          let c = freqs[word];
          freqs.insert(word.to_string(), c + *freq);
        } else {
          freqs.insert(word.to_string(), *freq);
        }
      }

      freqs
    }));
  }

  let mut calculated : Vec<(String, u32)> = Vec::new();
  for child in children2 {
    let mut result = Vec::from_iter(child.join().unwrap());
    calculated.append(&mut result);
  }

  // Sort and truncate
  let mut sorted_words = Vec::from_iter(calculated);
  sorted_words.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
  sorted_words.truncate(N as usize);

  // Print local freq table
  for (word, freq) in &sorted_words {
    println!("{} - {}", word, freq);
  }

}
