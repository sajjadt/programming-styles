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
extern crate futures;
use futures::{future, Future};
const N: u32 = 25;

use std::{thread, time};

extern crate actix;
use actix::*;

// Messagess
struct ReadFileCmd(String);
struct WordCmd(String);
struct Top25Cmd();
struct SendWordsCmd(Recipient<Top25Cmd>);
struct SetDownstreamCmd(Recipient<WordCmd>); // to be sent to stop word maanager

// we have to define the response type for `Sum` message
impl Message for ReadFileCmd {
  type Result = usize;
}
impl Message for WordCmd {
  type Result = usize;
}
impl Message for Top25Cmd {
  type Result = usize;
}
impl Message for SendWordsCmd {
  type Result = usize;
}
impl Message for SetDownstreamCmd {
  type Result = usize;
}

// Actor definitions
struct DataStorageManager {
  words: Vec<String>,
  downstream: Option<Recipient<WordCmd>>
}
impl Actor for DataStorageManager {
  type Context = Context<Self>;
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
}
impl Handler<ReadFileCmd> for DataStorageManager {
  type Result = usize;
  fn handle(&mut self, msg: ReadFileCmd, _ctx: &mut Context<Self>) -> Self::Result {
    self.init(&msg.0);
    0
  }
}

impl Handler<SendWordsCmd> for DataStorageManager {
  type Result = usize;

  fn handle(&mut self, msg: SendWordsCmd, _ctx: &mut Context<Self>) -> Self::Result {
    let five_us = time::Duration::from_micros(5);
    for word in &self.words {
      match &self.downstream {
        Some(x) => x.do_send(WordCmd(word.to_string())),
        None    => panic!("Invalid downstream"),
      };
      // Wait for downstram to catch up
      thread::sleep(five_us);
    }
    msg.0.do_send(Top25Cmd());
    0
  }
}
impl Handler<SetDownstreamCmd> for DataStorageManager {
  type Result = usize;
  fn handle(&mut self, msg: SetDownstreamCmd, _ctx: &mut Context<Self>) -> Self::Result {
    self.downstream = Some(msg.0);
    0
  }
} 
#[derive(Default)]
struct StopWordManager {
  stop_words: HashSet<String>,
  downstream: Option<Recipient<WordCmd>>
}
impl Actor for StopWordManager {
  type Context = Context<Self>;
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
}
impl Handler<ReadFileCmd> for StopWordManager {
  type Result = usize;
  fn handle(&mut self, msg: ReadFileCmd, _ctx: &mut Context<Self>) -> Self::Result {
    self.init(&msg.0);
    0
  }
}
impl Handler<WordCmd> for StopWordManager {
  type Result = usize;
  fn handle(&mut self, msg: WordCmd, _ctx: &mut Context<Self>) -> Self::Result {
    if ! self.is_stop_word(&msg.0) {
      match &self.downstream {
        Some(x) => x.do_send(WordCmd(msg.0.to_string())),
        None    => panic!("Invalid downstream")
      };
    }
    0
  }
}
impl Handler<SetDownstreamCmd> for StopWordManager {
  type Result = usize;
  fn handle(&mut self, msg: SetDownstreamCmd, _ctx: &mut Context<Self>) -> Self::Result {
    self.downstream = Some(msg.0);
    0
  }
} 

#[derive(Default)]
struct WordFrequencyManager {
  word_freq: HashMap<String, u32>,
}
impl Actor for WordFrequencyManager {
  type Context = Context<Self>;
}
impl WordFrequencyManager {
  fn increament_count(&mut self, s:&str) {
    if self.word_freq.contains_key(&s.to_string()) {
      let count = self.word_freq[s];
      self.word_freq.insert(s.to_string(), count + 1);
    } else {
      self.word_freq.insert(s.to_string(), 1);
    }
  }
}
impl Handler<WordCmd> for WordFrequencyManager {
  type Result = usize;
  fn handle(&mut self, msg: WordCmd, _ctx: &mut Context<Self>) -> Self::Result {
    self.increament_count(&msg.0);
    0
  }
}
impl Handler<Top25Cmd> for WordFrequencyManager {
  type Result = usize;
  fn handle(&mut self, _msg: Top25Cmd, _ctx: &mut Context<Self>) -> Self::Result {
    let mut sorted_words = Vec::from_iter(self.word_freq.clone());
    sorted_words.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    sorted_words.truncate(N as usize);    
    for (word, count) in sorted_words {
      println!("{} - {}", word, count);
    }
    System::current().stop();
    0
  }
}

fn main() {
 
  let args: Vec<_> = env::args().collect();
  if args.len() < 2 {
    println!("Usage: cargo run [input_file]");
    process::exit(0);
  }

  let sys = System::new("Term Frequency");

  // Start actors in separate threads. Each arbiter implements event loop its own thread.
  let wfm_addr = Arbiter::start(|_| WordFrequencyManager{word_freq: Default::default()} );
  let dsm_addr = Arbiter::start(|_| DataStorageManager{words:Default::default(), downstream: None} );
  let swm_addr = Arbiter::start(|_| StopWordManager{stop_words: Default::default(), downstream: None} );
  
  dsm_addr.do_send(ReadFileCmd(args[1].as_str().to_string())); 
  swm_addr.do_send(ReadFileCmd("../stop_words.txt".to_string()));
  
  dsm_addr.do_send(SetDownstreamCmd(swm_addr.clone().recipient()));
  swm_addr.do_send(SetDownstreamCmd(wfm_addr.clone().recipient()));
  dsm_addr.do_send(SendWordsCmd(wfm_addr.clone().recipient()));
  
  sys.run();
}