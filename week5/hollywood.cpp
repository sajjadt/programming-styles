#include "common.h"
#include <iostream>
#include <sstream>
#include <fstream>
#include <vector>
#include <set>
#include <map>
#include <algorithm>
#include <iterator>
#include <regex>


const int N = 25;
using namespace std;
/// Interfaces
class Loadable {
public:
  virtual ~Loadable() {}
  virtual void load_call(const std::string&) = 0;
};
class Workable {
public:
  virtual ~Workable() {}
  virtual void work_call() = 0;
};
class Endable {
public:
  virtual ~Endable() {}
  virtual void end_call() = 0;
};
class Wordable {
public:
  virtual ~Wordable() {}
  virtual void word_call(const std::string&) = 0;
};

class WordFrequencyFramework {
public:
  void register_for_load_event( Loadable* item) {
    load_event_handlers.push_back(item);
  }
  void register_for_dowork_event (Workable* item) {
    dowork_event_handlers.push_back(item);
  }
  void register_for_end_event( Endable* item) { 
    end_event_handlers.push_back(item);
  }
  void run(const std::string& input_file) {
    for (auto handler: load_event_handlers) 
      handler->load_call(input_file);
    for (auto handler: dowork_event_handlers) 
      handler->work_call();
    for (auto handler: end_event_handlers) 
      handler->end_call();
  }
private:  
  std::vector<Loadable*> load_event_handlers;
  std::vector<Workable*> dowork_event_handlers;
  std::vector<Endable*> end_event_handlers;
}; 
 
class StopWordFilter: public Loadable {
public:  
  StopWordFilter(WordFrequencyFramework* _wfapp): wfapp(_wfapp) {
    _wfapp->register_for_load_event(this);
  }
  void load_call(const std::string& ignore) {
    // load stop words
    ifstream cin("../stop_words.txt");
    std::stringstream buffer;
    buffer << cin.rdbuf();
    stop_words = split_to_set(buffer.str(), ',');
  }
  bool is_stop_word(const std::string& word) {
    return stop_words.find(word) != stop_words.end();
  }
private:
  WordFrequencyFramework *wfapp;
  std::set<std::string> stop_words;
};

class DataStorage: public Loadable, public Workable {
public:
  DataStorage(WordFrequencyFramework* _wfapp, StopWordFilter* _sw): sw(_sw) {
    _wfapp->register_for_load_event(this);
    _wfapp->register_for_dowork_event(this);
  }
  void register_for_word_event( Wordable* item) {
    word_event_handlers.push_back(item);
  }
  void load_call(const std::string& input_path) {
    // Read file and normalize
    ifstream cin(input_path);    
    std::stringstream buffer;
    buffer << cin.rdbuf();
    data = buffer.str();
    std::transform(data.begin(), data.end(), data.begin(), ::tolower);
  }
  void work_call() {
    // split
    std::regex e ("[a-z]{1}[a-z]+");
    std::smatch m;
    string tok;

    auto words_begin = std::sregex_iterator(data.begin(), data.end(), e);
    auto words_end = std::sregex_iterator();
 
    for (std::sregex_iterator i = words_begin; i != words_end; ++i) {
        std::smatch match = *i;                                                 
        std::string match_str = match.str(); 
        if ( sw->is_stop_word(match_str) || match_str.size() <= 1)
          continue;

        for (auto handler: word_event_handlers) 
          handler->word_call(match_str); 
    }  
  }
private:
  StopWordFilter* sw;
  std::vector<Wordable*> word_event_handlers;
  std::string data;
};

class WordFrequencyCounter: public Wordable, public Endable {
public:  
  WordFrequencyCounter(WordFrequencyFramework* wfapp, DataStorage* ds) {
    ds->register_for_word_event(this);
    wfapp->register_for_end_event(this);
  }
  void word_call(const std::string& word) {
    auto it = words_freq.find(word);
		if (it != words_freq.end())
			it->second++;
		else
			words_freq.insert(std::make_pair(word, 1));
  }
  void end_call() {
    std::map<int, std::string> sorted_words_freq = flip_map(words_freq);
    int counter = N;
    for(auto it = sorted_words_freq.rbegin(); it != sorted_words_freq.rend() && counter > 0; ++it) {
      std::cout << it->second << " - " << it->first  << "\n";
      counter--;
    }
  }
private:
  std::map<std::string, int> words_freq;
};

class WordWithZCounter: public Wordable, public Endable {
public:  
  WordWithZCounter(WordFrequencyFramework * wfapp, DataStorage* ds) {
    ds->register_for_word_event(this);
    wfapp->register_for_end_event(this);
  }
  void word_call(const std::string& word) {
    if (word[0] == 'z') {
      word_count++;
    }
  }
  void end_call() {
    std::cout<<"Valid words with letter z - " << word_count << std::endl; 
  }
private:
  int word_count;
};

using namespace std;
int main(int argc, char* argv[]) {

  auto wfapp = new WordFrequencyFramework;
  auto sw = new StopWordFilter(wfapp);
  auto ds = new DataStorage(wfapp, sw);
  auto wfc = new WordFrequencyCounter(wfapp, ds);
  auto wz = new WordWithZCounter(wfapp, ds);

  wfapp->run(argv[1]);
  
  return 0;
}
