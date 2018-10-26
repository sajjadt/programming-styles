use std::env;
use std::process;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;
use std::default::Default;
use std::iter::FromIterator;

extern crate regex;
use regex::Regex;

const N: u32 = 25;

#[derive(Default)]
struct DataStorageManager {
  words: Vec<String>
}
impl DataStorageManager {
  fn init(&mut self, input_file:&str) {
    let mut fs = File::open(input_file).expect("input file not found");
    
    let mut content : String = String::new();
    fs.read_to_string(&mut content)
          .expect("something went wrong reading the input file");
    content.make_ascii_lowercase();
    let re = Regex::new(r"[a-z]{2}[a-z]*");  
    for word in re.unwrap().find_iter(content.as_str()) {
      self.words.push(word.as_str().to_string());
    }
  }
  fn words(&self) -> &Vec<String>  {
    &self.words
  }
  fn dispatch(&mut self, message: &[&str]) -> Option<&Vec<String>> {
    if message[0] == "init" {
      self.init(message[1]);
      return None;
    } else if message[0] == "words" {
      return Some(&self.words());
    } else {
      panic!("Invalid message: {}", message[0]);
    }
  }
}

#[derive(Default)]
struct StopWordManager {
  stop_words: HashSet<String>
}

impl StopWordManager {
  fn init(&mut self, stop_file:&str) {
    let mut fs = File::open(stop_file).expect("input file not found");
    let mut input_contents : String =  String::new();
    fs.read_to_string(&mut input_contents)
     .expect("something went wrong reading the input file");
     
    input_contents.make_ascii_lowercase();
    for word in input_contents.split(',') {
      self.stop_words.insert(word.to_string());
    }
  }
  fn is_stop_word(&self, word:&str) -> bool {
    self.stop_words.contains(word)
  }
  fn dispatch(&mut self, message:&[&str]) -> Option<bool> {
    if message[0] == "init" {
      self.init(message[1]);
      return None;
    } else if message[0] == "is_stop_word" {
      return Some(self.is_stop_word(message[1]));
    } else {
      panic!("Invalid message: {}", message[0]);  
    } 
  }
}

#[derive(Default)]
struct WordFrequencyManager {
  word_freq: HashMap<String, u32>,
}
impl WordFrequencyManager {
  fn dispatch(&mut self, message:&[&str]) -> Option<Vec<(String, u32)>> {
    if message[0] == "increment_count" {
      self.increament_count(message[1]);
      return None;
    } else if message[0] == "sorted" {
      let mut sorted_words = Vec::from_iter(self.word_freq.clone());
      sorted_words.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
      sorted_words.truncate(N as usize);
      return Some(sorted_words);
    } else {
      panic!("Invalid message: {}", message[0]);
    }
  }

  fn increament_count(&mut self, s:&str) {
    if self.word_freq.contains_key(&s.to_string()) {
      let count = self.word_freq[s];
      self.word_freq.insert(s.to_string(), count + 1);
    } else {
      self.word_freq.insert(s.to_string(), 1);
    }
  }
}

struct WordFrequencyController {
  storage_manager: DataStorageManager,
  stop_word_manager: StopWordManager,
  word_freq_manager: WordFrequencyManager
}

impl WordFrequencyController {
  fn init(&mut self, input_file:&str, stop_file:&str) {
    self.storage_manager.dispatch(&["init", input_file]);
    self.stop_word_manager.dispatch(&["init", stop_file]);
  }
  fn run(&mut self) {
    let words = self.storage_manager.dispatch(&["words"]);
    for word in words.unwrap() {
      if ! self.stop_word_manager.dispatch(&["is_stop_word", word.as_str()]).unwrap() {
        self.word_freq_manager.dispatch(&["increment_count", word]);

      }
    }
    let word_freq = self.word_freq_manager.dispatch(&["sorted"]).unwrap();
    for (word, count) in word_freq {
      println!("{} - {}", word, count);
    }
  }

  fn dispatch(&mut self, message:&[&str]) {
    if message[0] == "init" {
      self.init(message[1], message[2]);
    } else if message[0] == "run" {
      self.run();
    } else {
      panic!("Invalid message: {}", message[0]);
    }
  }
}

fn main() {
  let args: Vec<_> = env::args().collect();
  if args.len() != 3 {
    print!("Usage: cargo run --bin pipeline [input_file] [stop_words_file]\n");
    process::exit(0);
  }
  let mut controller = WordFrequencyController{
    storage_manager: DataStorageManager{words: Default::default()}, 
    stop_word_manager: StopWordManager{stop_words: Default::default()}, 
    word_freq_manager: WordFrequencyManager{word_freq: Default::default()}
  };
  controller.dispatch(&["init", args[1].as_str(), args[2].as_str()]);
  controller.dispatch(&["run"]);
}