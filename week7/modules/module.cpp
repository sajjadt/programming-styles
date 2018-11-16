#include "module.h"
#include <iostream>
#include <sstream>
#include <fstream>
#include <vector>
#include <set>
#include <map>
#include <algorithm>
#include <iterator>
#include <regex>

using namespace std;
std::vector<std::string> extract_words(const std::string& input_path) {

  std::vector<std::string> words_list;
  std::stringstream buffer;
    
  // Load Stop words
  ifstream sin("../stop_words.txt");
  
  buffer << sin.rdbuf();
  std::set<std::string> stop_words = split_to_set(buffer.str(), ',');
  
  struct Local {
    static bool is_stop_word(const std::set<std::string>& stop_words, const std::string& word) {
      return stop_words.find(word) != stop_words.end();
    }
  };
  
  ifstream cin(input_path);
  buffer << cin.rdbuf();
  std::string data = buffer.str();
  // Lower
  std::transform(data.begin(), data.end(), data.begin(), ::tolower);    

  // Split
  std::regex e ("[a-z]{1}[a-z]+");
  std::smatch m;
  string tok;

  auto words_begin = std::sregex_iterator(data.begin(), data.end(), e);
  auto words_end = std::sregex_iterator();
 
  for (std::sregex_iterator i = words_begin; i != words_end; ++i) {
    std::smatch match = *i;                                                 
    std::string match_str = match.str(); 
    if ( Local::is_stop_word(stop_words, match_str) || match_str.size() <= 1)
      continue;
    words_list.push_back(match_str); 
  }  

  return words_list;
}

std::vector<std::pair<std::string, int> > top(const std::vector<std::string>& word_list, const int N) {
  std::vector<std::pair<std::string, int> > freqs;
  std::map<std::string, int> words_freq;
    
  for (auto word: word_list) {
    auto it = words_freq.find(word);
    if (it != words_freq.end())
      it->second++;
    else
	    words_freq.insert(std::make_pair(word, 1));   
  }
   
  std::map<int, std::string> sorted_words_freq = flip_map(words_freq);
  int counter = N;
  for(auto it = sorted_words_freq.rbegin(); it != sorted_words_freq.rend() && counter > 0; ++it) {
    freqs.push_back(std::make_pair(it->second, it->first)); //it->second << " - " << it->first  << "\n";
    counter--;  
  }
  return freqs;
}
