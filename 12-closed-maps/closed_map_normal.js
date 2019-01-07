var fs = require("fs");

const N = 25;
var data_storage_obj = {
    'data' : [],
    'init': function(input_file_path) {
      data = fs.readFileSync(input_file_path);
      data = data.toString();
      data = data.replace(/\n|\r/g, " ");
      data = data.toLowerCase();

      var re = /[a-z][a-z]+/g;
      var matches = []
      while (match = re.exec(data))
        matches.push(match[0])

      this['data'] = matches;
    },
    'words': function() {
        return this['data'];
    }
};

var stop_words_obj = {
  'stop_words' : [],
  'init' : function(stop_file_path) {
    data = fs.readFileSync(stop_file_path);
    data = data.toString();
    data = data.toLowerCase();
    data = data.replace(/\n|\r/g, "")
    this['stop_words'] = new Set(data.split(","));
  },
  'is_stop_word' : function(word) {
    return this['stop_words'].has(word);
  }
}

var word_freqs_obj = {
  'freqs' : {},
  'increment_count' : function(w) { 
      if (w in this['freqs'])
        this['freqs'][w] += 1;
      else
        this['freqs'][w] = 1;
     
  },
  'sorted' : function() {
    var items = Object.entries(this['freqs']).map(function(entry) {
      return [entry[0], entry[1]];
    });
    
    // Sort the array based on the second element
    items.sort(function(first, second) {
      return second[1] - first[1] ;
    });
    
    return items.slice(0, N);
  }
}

if (process.argv.length != 4){
 console.log("Bad input...")
 return;
}

data_storage_obj['init'](process.argv[2]);
stop_words_obj['init'](process.argv[3]);

words = data_storage_obj['words']();
for (let word of words) {
  if (! stop_words_obj["is_stop_word"](word) ) {
    word_freqs_obj["increment_count"](word);
  }
}

var word_freq = word_freqs_obj["sorted"]();
word_freq.forEach((item) => {
  console.log(`${item[0]} - ${item[1]}`);
});