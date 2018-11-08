var fs = require("fs");

const N = 25;

var Quarantine = {
  funcs: [],
  'bind': function (func) {
    this.funcs.push(func);
    return this;
  },
  'is_function': function(foo) {
    return foo && {}.toString.call(foo) === '[object Function]';
  },
  'guard_callable': function(v) {
    return this.is_function(v) ? v() : v;
  },
  'execute': function() {
    // First order or second order
    value = null;
    for(var func of this.funcs) {
      value = func(this.guard_callable(value));
    }
    this.guard_callable(value);
  }
};

// Pure functions
var frequencies = function(word_list) {
  word_freqs = {};
  console.log("F");
  for (let word of word_list)
    if (word in word_freqs)
      word_freqs[word] += 1;
    else
      word_freqs[word] = 1;
  return word_freqs;
}
var sort = function(word_freqs) {
  console.log("S");
  var items = Object.entries(word_freqs).map(function(entry) {
    return [entry[0], entry[1]];
  });
  items.sort(function(first, second) {
    return second[1] - first[1] ;
  });
  return items;
}


// Second order functions
var get_input = function(ignore) {
  return function() {
    console.log("GI");
    return process.argv[2];
  }
}
var extract_words = function(path) {
  return function() {
    console.log("EW");
    data = fs.readFileSync(path);
    data = data.toString();
    data = data.replace(/\n|\r/g, " ");
    data = data.toLowerCase();

    var re = /[a-z][a-z]+/g;
    var matches = []
    while (match = re.exec(data))
      matches.push(match[0])

    return matches;
  }
}
var remove_stop_words = function(words_list) {
  return function() {
    console.log("RS");
    data = fs.readFileSync("../stop_words.txt");
    data = data.toString();
    data = data.toLowerCase();
    data = data.replace(/\n|\r/g, "")
    _set = new Set(data.split(","));
    items = [];
    for(var entry of words_list) {
      if (!_set.has(entry))
        items.push(entry);
    }
    return items;
  }
}
var top25_freqs = function(word_freqs) {
  console.log("T");
  return function() {
    console.log("T");
    top25 = [];
    var slice =  word_freqs.slice(0, N);
    for (var m of slice) {
     console.log(m[0] + " - " + m[1]);
    }
  }
}

Quarantine.
bind(get_input).
bind(extract_words).
bind(remove_stop_words).
bind(frequencies).
bind(sort).
bind(top25_freqs).
execute();
