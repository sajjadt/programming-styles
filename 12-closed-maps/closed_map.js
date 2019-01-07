var fs = require("fs");

const N = 25;
var data_storage_obj = {
    'data' : [],
    'init': function(self, input_file_path) {
      data = fs.readFileSync(input_file_path);
      data = data.toString();
      data = data.replace(/\n|\r/g, " ");
      data = data.toLowerCase();

      var re = /[a-z][a-z]+/g;
      var matches = []
      while (match = re.exec(data))
        matches.push(match[0])

      self['data'] = matches;
    },
    'words': function(self) {
        return self['data'];
    }
};

var stop_words_obj = {
  'stop_words' : [],
  'init' : function(self, stop_file_path) {
    data = fs.readFileSync(stop_file_path);
    data = data.toString();
    data = data.toLowerCase();
    data = data.replace(/\n|\r/g, "")
    self['stop_words'] = new Set(data.split(","));
  },
  'is_stop_word' : function(self, word) {
    return self['stop_words'].has(word);
  }
}

var word_freqs_obj = {
  'freqs' : {},
  'increment_count' : function(self, w) { 
      if (w in self['freqs'])
        self['freqs'][w] += 1;
      else
        self['freqs'][w] = 1;
     
  },
  'sorted' : function(self) {
    var items = Object.entries(self['freqs']).map(function(entry) {
      return [entry[0], entry[1]];
    });

    items.sort(function(first, second) {
      return second[1] - first[1] ;
    });
    
    return items;
  }
}

if (process.argv.length != 4){
 console.log("Bad input...")
 return;
}

data_storage_obj['init'](data_storage_obj, process.argv[2]);
stop_words_obj['init'](stop_words_obj, process.argv[3]);

words = data_storage_obj['words'](data_storage_obj);
for (let word of words) {
  if (! stop_words_obj["is_stop_word"](stop_words_obj, word) ) {
    word_freqs_obj["increment_count"](word_freqs_obj, word);
  }
}

word_freqs_obj['top25'] = function(self) {
  var items = self['sorted'](self);
  return items.slice(0, N);
}
var word_freq = word_freqs_obj["top25"](word_freqs_obj);

word_freq.forEach((item) => {
  console.log(`${item[0]} - ${item[1]}`);
});
