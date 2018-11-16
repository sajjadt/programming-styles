#include "modules/module.h"
#include <iostream>
#include <dlfcn.h>

typedef std::vector<std::string> (*PFN_EXTRACT_WORDS) (const std::string&);
typedef std::vector<std::pair<std::string, int> > (*PFN_FREQUENCIES) (const std::vector<std::string>&, int);

int main(int argc, char** argv) {
  void* handle = dlopen("./libModule.so", RTLD_LAZY);
  if (!handle) {    
    std::cout << "Could not open the library" << std::endl;
    return 1;
  }

  PFN_EXTRACT_WORDS extract_words = reinterpret_cast<PFN_EXTRACT_WORDS>(dlsym(handle, "extract_words"));
  if (!extract_words) {
    std::cout << "Could not find symbol extract_words" << std::endl;
    dlclose(handle);
    return 1;
  }

  PFN_FREQUENCIES top = reinterpret_cast<PFN_FREQUENCIES>(dlsym(handle, "top"));
  if (!top) {
    std::cout << "Could not find symbol top" << std::endl;
    dlclose(handle);
    return 1;
  }

  std::vector<std::pair<std::string, int> > pairs = top(extract_words(argv[1]), 25);
  for(auto pair: pairs) {
    std::cout << pair.first << " - " << pair.second  << "\n";
  }

  dlclose(handle);
  return 0;
}