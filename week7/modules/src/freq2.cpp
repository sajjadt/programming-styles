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

std::vector<std::pair<std::string, int> > top(const std::vector<std::string>& word_list, const int N) {
  std::cout<<"Using freq2"<<std::endl;
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
