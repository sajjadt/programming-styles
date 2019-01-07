/*globals parse_input:true */
var fs = require("fs");
const N = 25;

extract_words = function(name){
  var data = fs.readFileSync("../stop_words.txt");
  data = data.toString();
  data = data.toLowerCase();
  data = data.replace(/\n|\r/g, "")
  var stop_words = new Set(data.split(","));

  data = fs.readFileSync(name);
  data = data.toString();
  data = data.replace("/\\n|\\r/g", " ");
  data = data.toLowerCase();
  
  var items = [];
  var re = /[a-z][a-z]+/g;
  var matches = [];
  while (match = re.exec(data))
    matches.push(match[0]);
  
  for(var entry of matches) {
    if (!stop_words.has(entry))
      items.push(entry);
  }
  return items;
}

frequencies = function (word_list) {
  word_freqs = {};
  for (let word of word_list){
    if (word in word_freqs) 
      word_freqs[word] += 1;
    else 
      word_freqs[word] = 1;
  } 
  return word_freqs;
}

sort = function(frequency) {
  var items = Object.entries(frequency).map(function(entry) {
    return [entry[0], entry[1]];
  });
  items.sort(function(first, second) {
    return second[1] - first[1] ;
  });
  return items.slice(0, N);
}

print_all = function(frequency, next) {      
  frequency.forEach((item) => {
    console.log(`${item[0]} - ${item[1]}`);
  });
}

profile = function(f) {
  return function() {
    start_time = Date.now();
    ret_value = f.apply(null, arguments);
    elapsed = Date.now() - start_time;
    console.log(f.name +" took " + elapsed + " ms.");
    return ret_value;
  }
}

profiled_funcs = ["extract_words", "frequencies", "sort"];
for (var func of profiled_funcs) {
  global[func] = profile(global[func]);
}

print_all(sort(frequencies(extract_words(process.argv[2]))));

