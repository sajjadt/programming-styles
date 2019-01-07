#pragma once
#include <string>
#include <set>
#include <map>
#include <sstream>
#include <algorithm>

std::set<std::string> split_to_set(std::string str, char delimiter) {
  std::set<std::string> internal;
  std::stringstream ss(str); // Turn the string into a stream.
  std::string tok;

  while(getline(ss, tok, delimiter)) {
    std::transform(tok.begin(), tok.end(), tok.begin(), ::tolower);
    internal.insert(tok);
  }

  return internal;
}

// flips an associative container of A,B pairs to B,A pairs
template<typename A, typename B>
std::pair<B,A> flip_pair(const std::pair<A,B> &p) {
    return std::pair<B,A>(p.second, p.first);
}
template<typename A, typename B, template<class,class,class...> class M, class... Args>
std::map<B,A> flip_map(const M<A,B,Args...> &src) {
    std::map<B,A> dst;
    std::transform(src.begin(), src.end(),
                   std::inserter(dst, dst.begin()),
                   flip_pair<A,B>);
    return dst;
}