var fs = require("fs");
const N = 25;

function* words (input_file_path) {
  data = fs.readFileSync(input_file_path);
  data = data.toString();
  data = data.replace(/\n|\r/g, " ");
  data = data.toLowerCase();
  var re = /[a-z][a-z]+/g;
  while (match = re.exec(data)) {
    yield match[0]; 
  }       
}

function* non_stop_words (input_file_path) {
  
  data = fs.readFileSync("../stop_words.txt");
  data = data.toString();
  data = data.toLowerCase();
  data = data.replace(/\n|\r/g, "");
  var stop_words = new Set (data.split(","));   
  const itertor = words(input_file_path);  
  for(i = itertor.next(); !i.done;i = itertor.next()) {
    word = i.value;
    if (! stop_words.has(word)) {
      yield word;
    } 
  }
}

function* count_and_sort (input_file_path) {
  var word_freqs = {};
  
  const itertor = non_stop_words(input_file_path); 
  for(i = itertor.next(); !i.done;i = itertor.next()) {
    word = i.value;
    if (word in word_freqs)
      word_freqs[word] += 1;
    else
      word_freqs[word] = 1;
  }

  var items = Object.entries(word_freqs).map(function(entry) {
    return [entry[0], entry[1]];
  });
  items.sort(function(first, second) {
    return second[1] - first[1] ;
  });
  yield items;
}

var iterator = count_and_sort(process.argv[2]);
var sorted = iterator.next().value;
for(var i =0 ;i < N; i++) {
  console.log(sorted[i][0] + " - " + sorted[i][1]);
}
