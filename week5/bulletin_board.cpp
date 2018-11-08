#include "common.h"
#include <iostream>
#include <sstream>
#include <fstream>
#include <vector>
#include <set>
#include <map>
#include <unordered_map>
#include <algorithm>
#include <iterator>
#include <regex>

using namespace std;
const int N = 25;

class Event {
public:
  enum EventType { 
    LOAD, START, WORD, VALID_WORD, PRINT, RUN, END_FILE
  };
  Event(EventType type, const std::string& data): type(type), data(data) {}
  friend ostream& operator<<(ostream& os, const Event& event);  
  EventType type;
  // String, 
  std::string data;
};
ostream& operator<<(ostream& os, const Event& event)  
{  
    os << event.type <<", "<< event.data;  
    return os;  
}

// Interfaces
class Handler {
public:
  //virtual ~Handler() {}
  virtual void callback(const Event&) = 0;
};

class EventManager {
public:
  void subscribe(Event::EventType event, Handler* handler) {
    auto it = subscriptions.find(event);
		if (it != subscriptions.end()) {
      it->second.push_back(handler);
    } else {
      std::vector<Handler*> vect;
      vect.push_back(handler);
      subscriptions.insert(std::make_pair(event, vect));
    }	
  }
  void publish(const Event& event) {
    auto it = subscriptions.find(event.type);
    if (it != subscriptions.end())
      for(auto sub: it->second)
        sub->callback(event);
  }
private:
  std::unordered_map<Event::EventType, vector<Handler*>, std::hash<int> > subscriptions;
};

class DataStorage: public Handler {
public:
  DataStorage(EventManager* manager): manager(manager) {
    manager->subscribe(Event::LOAD, this);
    manager->subscribe(Event::START, this);
  }
  void callback(const Event& event) {
    if (event.type == Event::LOAD) {
      // Read file and normalize
      ifstream cin(event.data, ios::in);    
      std::stringstream buffer;
      buffer << cin.rdbuf();
      data = buffer.str();
      std::transform(data.begin(), data.end(), data.begin(), ::tolower);
    } else if (event.type == Event::START) {
      produce_words();
      manager->publish(Event(Event::END_FILE, std::string()));
    }
  }
  
  void produce_words() {
    std::regex e ("[a-z]{1}[a-z]+");
    std::smatch m;
    string tok;

    auto words_begin = std::sregex_iterator(data.begin(), data.end(), e);
    auto words_end = std::sregex_iterator();
 
    for (std::sregex_iterator i = words_begin; i != words_end; ++i) {
        std::smatch match = *i;                                                 
        std::string match_str = match.str(); 
        if ( match_str.size() <= 1)
          continue;
        manager->publish(Event(Event::WORD, match_str));
    }  
  }
private:
  EventManager* manager;
  std::string data;
};

class StopWordFilter : public Handler {
public:
  StopWordFilter(EventManager* manager): manager(manager) {
    manager->subscribe(Event::LOAD, this);
    manager->subscribe(Event::WORD, this);
  }
  void callback(const Event& event) {
    if (event.type == Event::LOAD) {
      ifstream cin("../stop_words.txt");
      std::stringstream buffer;
      buffer << cin.rdbuf();
      stop_words = split_to_set(buffer.str(), ',');
    } else if (event.type == Event::WORD) {
      if (!is_stop_word(event.data)) {
        manager->publish(Event(Event::VALID_WORD, event.data));
      }
    }
  }
  bool is_stop_word(const std::string& word) {
    return stop_words.find(word) != stop_words.end();
  }
private:
  EventManager* manager;
  std::set<std::string> stop_words;
};

class WordFrequencyCounter : public Handler {
public:
  WordFrequencyCounter(EventManager* manager): manager(manager) {
    manager->subscribe(Event::VALID_WORD, this);
    manager->subscribe(Event::PRINT, this);
  }
  void callback(const Event& event) {
    if (event.type == Event::VALID_WORD) {
      auto it = words_freq.find(event.data);
      if (it != words_freq.end())
        it->second++;
      else
	  	words_freq.insert(std::make_pair(event.data, 1));
    } else if (event.type == Event::PRINT) {
     
      std::map<int, std::string> sorted_words_freq = flip_map(words_freq);
      int counter = N;
      for(auto it = sorted_words_freq.rbegin(); it != sorted_words_freq.rend() && counter > 0; ++it) {
        std::cout << it->second << " - " << it->first  << "\n";
        counter--;
        
      }
    }
  }
private:
  EventManager* manager;
  std::map<std::string, int> words_freq;
};

class WordWithZCounter : public Handler {
public:
  WordWithZCounter(EventManager* manager): manager(manager), word_counter(0) {
    manager->subscribe(Event::VALID_WORD, this);
    manager->subscribe(Event::PRINT, this);
  }
  void callback(const Event& event) {

    if (event.type == Event::VALID_WORD) {
      if (event.data[0] == 'z')
        word_counter++;
    } else if (event.type == Event::PRINT) {
      std::cout<<"Valid words with letter z - " << word_counter << std::endl; 
    }
  }
private:
  EventManager* manager;
  int word_counter;
};

class WordFrequencyApplicaion : public Handler {
public:
  WordFrequencyApplicaion(EventManager* manager): manager(manager) {
    manager->subscribe(Event::RUN, this);
    manager->subscribe(Event::END_FILE, this);
  }
  void callback(const Event& event) {
    if (event.type == Event::RUN) {
      manager->publish(Event(Event::LOAD, event.data));
      manager->publish(Event(Event::START, std::string()));
    } else if (event.type == Event::END_FILE) {
      manager->publish(Event(Event::PRINT, std::string()));
    }
  }
private:
  EventManager* manager;
};

int main(int argc, char* argv[]) {
  auto event_manager = new EventManager();
  auto ds = new DataStorage(event_manager);
  auto sw = new StopWordFilter(event_manager);
  auto wfc = new WordFrequencyCounter(event_manager);
  auto wz = new WordWithZCounter(event_manager);

  auto wfa = new WordFrequencyApplicaion(event_manager);
  event_manager->publish(Event(Event::RUN, argv[1]));
  return 0;
}
