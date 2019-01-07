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

using namespace std;
void print(std::vector<std::pair<std::string, int> > pairs) {
  std::cout<<"Using print1"<<std::endl;
  for(auto pair: pairs) {
    std::cout << pair.first << " - " << pair.second  << "\n";
  }
}