use std::process;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Duration;
use std::thread;
use std::iter::FromIterator;


extern crate regex;
use regex::Regex;

extern crate may;
use may::sync::mpmc::{Sender, Receiver};
use std::sync::mpsc::RecvTimeoutError;

static NTHREADS: u32 = 5;
const N: u32 = 25;

fn read_file_and_lower(in_file: String) -> String {
  let mut fs = File::open(in_file).expect("input file not found");
  
  let mut input_contents : String =  String::new();
  fs.read_to_string(&mut input_contents)
     .expect("something went wrong reading the input file");
     
  input_contents.make_ascii_lowercase();
  input_contents
}

fn main() {

  let args: Vec<_> = env::args().collect();
  if args.len() != 2 {
    print!("Usage: cargo run --bin data_spaces [input_file]\n");
    process::exit(0);
  }

  // Word space
  let (word_space_tx, word_space_rx): (Sender<String>, Receiver<String>) = may::sync::mpmc::channel();
  
  // N freq spaces (partitioning based on the alphabet space)
  let mut freq_spaces : Vec<(Sender<HashMap<String, u32>>, Receiver<HashMap<String, u32>>)> = vec![];
  for _i in 0..NTHREADS {
    freq_spaces.push(may::sync::mpmc::channel());
  }

  
  // Make a vector to hold the children which are spawned.
  let mut threads = vec![];

  // Thread 1: read input file, tokenize and store into word space
  threads.push(thread::spawn(move || {
    let mut num_words = 0;
    let content : String = read_file_and_lower(args[1].to_string());
    let re = Regex::new(r"[a-z]{2}[a-z]*");  
    for word in re.unwrap().find_iter(content.as_str()) {
      word_space_tx.send(word.as_str().to_string()).unwrap();
      num_words += 1;
    }
    println!("Collect word thread finished. Sent {} words.", num_words);
  }));

  // N threads: read from word space and fills N frequency spaces divided by alphabet
  for i in 0..NTHREADS {
    let word_space_rx = word_space_rx.clone();
    
    let mut freq_space_tx: Vec<Sender<HashMap<String, u32>>> = vec![];
    for j in 0..NTHREADS {
      freq_space_tx.push(freq_spaces[j as usize].0.clone());
    }

    threads.push(thread::spawn(  move || {
      // Read stop file
      let contents = read_file_and_lower("../stop_words.txt".to_string());

      let mut stop_words: HashSet<String> = HashSet::new();
      for word in contents.split(',') {
        stop_words.insert(word.to_string());
      }
     
      // Local frequency results
      let mut word_freq_v: Vec<HashMap<String, u32>> = vec![];
      for _i in 0..NTHREADS {
        word_freq_v.push(HashMap::new());
      }
      let word_freq: HashMap<String, u32> = HashMap::new();

      let mut done = false;
      let mut num_nonstop_words = 0;
      while ! done {
        match word_space_rx.recv_timeout(Duration::from_millis(100))  {
          Ok(message) =>  {
            if !stop_words.contains(&message.to_string()) {

              // First char determines the target freq space
              let ch_group = (message.chars().next().unwrap() as u32 - 'a' as u32) % NTHREADS;
              
              if word_freq_v[ch_group as usize].contains_key(&message) {
                let c = word_freq_v[ch_group as usize][&message.to_string()];
                word_freq_v[ch_group as usize].insert(message, c + 1);
              } else {
                word_freq_v[ch_group as usize].insert(message, 1);
              }

              num_nonstop_words += 1;
            }
          },
          Err(RecvTimeoutError::Timeout) => {done = true; println!("Timed-out");},
          Err(RecvTimeoutError::Disconnected) => {done = true; println!("Sender thread disconnected");}
        }
      }

      // Write the data into freq spaces
      for i in 0..NTHREADS {
        freq_space_tx[i as usize].send(word_freq_v.remove(0)).unwrap();
      }

      println!("Process word thread {} finished. handled {} non_stop words ...", i, num_nonstop_words);
    }));
  }

  for child in threads {
    child.join(); 
  }

  let (final_freq_space_tx, final_freq_space_rx): (Sender<Vec<(String, u32)>>, Receiver<Vec<(String, u32)>>) = may::sync::mpmc::channel();
  let mut gather_threads = vec![];

  // Gather and combine
  for i in 0..NTHREADS {

    let freq_space_rx = freq_spaces[i as usize].1.clone();
    let final_freq_space_tx = final_freq_space_tx.clone();

    gather_threads.push(thread::spawn(  move || {
      
      let mut done = false;
      let mut total_word_freq: HashMap<String, u32> = HashMap::new();
      while ! done {
        match freq_space_rx.recv_timeout(Duration::from_millis(0))  {
          Ok(message) =>  {
            for (word, freq) in &message {
              if total_word_freq.contains_key(word) {
                let total_freq = total_word_freq[&word.to_string()];
                total_word_freq.insert((&word).to_string(),  total_freq + *freq);
              } else {
                total_word_freq.insert(word.to_string(), *freq);
              }
            }
          },
          Err(RecvTimeoutError::Timeout) => {done = true; println!("Timed-out");},
          Err(RecvTimeoutError::Disconnected) => {done = true; println!("Sender thread disconnected");}
        }
      }
      let mut words = Vec::from_iter(total_word_freq);
      final_freq_space_tx.send(words).unwrap();
      println!("thread gathter {} done..", i);
    }));
  }

  for child in gather_threads {
    child.join(); 
  }

  let mut done = false;
  let mut total_word_freq: Vec<(String, u32)> = Vec::new();
  while ! done {
    let mut message_recv = final_freq_space_rx.recv_timeout(Duration::from_millis(0));
    match message_recv  {
      Ok(mut message) =>  {
        total_word_freq.append(&mut message);
      },
      Err(RecvTimeoutError::Timeout) => {done = true; println!("Timed-out");},
      Err(RecvTimeoutError::Disconnected) => {done = true; println!("Sender thread disconnected");}
    }
  }

  // Sort and truncate
  total_word_freq.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
  total_word_freq.truncate(N as usize);

  // Print local freq table
  for (word, freq) in &total_word_freq {
    println!("{} - {}", word, freq);
  }

}
