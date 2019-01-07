use std::env;
use std::process;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
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
 

extern crate actix;
use actix::*;
 
// Messagess
struct StartCmd(String);
struct GotWordCmd(Option<String>);
impl Message for StartCmd {
  type Result = bool;
}
impl Message for GotWordCmd {
  type Result = bool;
}

// Actors: AllWords -> NonStopWords -> CountAndSort
// Actor definitions
struct AllWords {
  downstream: Recipient<GotWordCmd>
}
impl Actor for AllWords {
  type Context = Context<Self>; 
}
impl Handler<StartCmd> for AllWords {
  type Result = bool;
  
  fn handle(&mut self, msg: StartCmd, _ctx: &mut Context<Self>) -> Self::Result {
    
    let fs = File::open(&msg.0).expect("input file not found");
    for line in BufReader::new(fs).lines() {
      let mut rline = line.unwrap();
      rline.make_ascii_lowercase();
      let re = Regex::new(r"[a-z]{2}[a-z]*");
      for word in re.unwrap().find_iter(rline.as_str()) {
        // Yield a new word
        self.downstream.send(GotWordCmd(Some(word.as_str().to_string()))).wait();
      }
    }
    self.downstream.send(GotWordCmd(None)).wait();
    true
  }
}

struct NonStopWords {
  stop_words: HashSet<String>,
  downstream: Recipient<GotWordCmd>
}
impl NonStopWords {
  fn is_stop_word(&self, word:&str) -> bool {
    self.stop_words.contains(word)
  }
}
impl Actor for NonStopWords {
  type Context = Context<Self>;
}
impl Handler<StartCmd> for NonStopWords {
  type Result = bool;
  fn handle(&mut self, msg: StartCmd, _ctx: &mut Context<Self>) -> Self::Result {

    let mut fs = File::open(&msg.0).expect("input file not found");
    let mut input_contents : String =  String::new();
    fs.read_to_string(&mut input_contents)
     .expect("something went wrong reading the input file");
     
    input_contents.make_ascii_lowercase();
    for word in input_contents.split(',') {
      self.stop_words.insert(word.to_string());
    }

    println!("StopWordManager: read words [Done]");
    true
  }
}
impl Handler<GotWordCmd> for NonStopWords {
  type Result = bool;
  fn handle(&mut self, msg: GotWordCmd, _ctx: &mut Context<Self>) -> Self::Result {
    
    match &msg.0 {
      Some(x) => {
        // Yield
        if !self.is_stop_word(x) {
          self.downstream.send(GotWordCmd(msg.0.clone())).wait();
        }
      },
      // End of Seq
      None => {
        self.downstream.send(GotWordCmd(None)).wait();
      }
    };
    true
  }
}

struct CountAndSort {
  word_freq: HashMap<String, u32>
}
impl Actor for CountAndSort {
  type Context = Context<Self>;
}
impl CountAndSort {
  fn increament_count(&mut self, s:&str) {
    if self.word_freq.contains_key(&s.to_string()) {
      let count = self.word_freq[s];
      self.word_freq.insert(s.to_string(), count + 1);
    } else {
      self.word_freq.insert(s.to_string(), 1);
    }
  }
  fn print_top(&mut self) {
    let mut sorted_words = Vec::from_iter(self.word_freq.clone());
    sorted_words.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    sorted_words.truncate(N as usize);    
    for (word, count) in sorted_words {
      println!("{} - {}", word, count);
    }
  }
}
impl Handler<GotWordCmd> for CountAndSort {
  type Result = bool;
  fn handle(&mut self, msg: GotWordCmd, _ctx: &mut Context<Self>) -> Self::Result {
    match &msg.0 {
      Some(x) => {
        self.increament_count(x);
      },
      // End of seq
      None => {
        self.print_top();
        System::current().stop();
        },
    }
    true
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
  let count_and_sort = Arbiter::start(|_| CountAndSort {word_freq: Default::default()} );
  let cs = count_and_sort.clone().recipient();
  let nonstop_words = Arbiter::start(|_| NonStopWords {downstream: cs, stop_words: Default::default()});
  let ns = nonstop_words.clone().recipient();
  let all_words = Arbiter::start(|_| AllWords {downstream: ns} );  

  nonstop_words.do_send(StartCmd("../stop_words.txt".to_string()));
  all_words.do_send(StartCmd(args[1].as_str().to_string()));

  sys.run();
}
