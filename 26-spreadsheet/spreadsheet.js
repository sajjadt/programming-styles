var fs = require("fs");
const N = 25;

table = {
   // Setup table
  setup: function() {
    this.all_words = {data: [], function: null};
    this.stop_words = {data: [], function: null};
    this.non_stop_words = {
      data: [], 
      function: _ => {
        var set = new Set(this.stop_words.data);
        this.non_stop_words.data = this.all_words.data.filter(item => !set.has(item));
      }
    }, // push a word from stop words if not in set(stop_words)
    this.unique_words = {
      data: [], 
      function: _ => {
        var set = new Set(this.non_stop_words.data);
        this.unique_words.data = Array.from(set.keys());
      }
    };
    this.counts = {
      data: [], 
      function: _ => { // Count unique words occurance on words list
        var word_freqs = {};
        for (let word of this.non_stop_words.data)
          if (word in word_freqs)
            word_freqs[word] += 1;
          else
            word_freqs[word] = 1;
        this.counts.data = this.unique_words.data.map(item => word_freqs[item]);
      }
    };
    this.sorted_counts = {
      data: [], 
      function: _ => {
        // Zip unique words and count columns
        parent = this;
        zipped = this.unique_words.data.map(function(e, i) {
          return [e, parent.counts.data[i]];
        })

        // Sort zipped based on frequency
        zipped.sort(function(first, second) {
          return second[1] - first[1] ;
        });
        
        this.sorted_counts.data = zipped.map(e => e[0] + " - " + e[1])
      }
    };

    this.cells = [this.all_words, this.stop_words, this.non_stop_words, this.unique_words, this.counts, this.sorted_counts];
  },
  set_input: function(input_file_path, stop_words_file_path) {
    // Input words
    data = fs.readFileSync(input_file_path);
    data = data.toString();
    data = data.replace(/\n|\r/g, " ");
    data = data.toLowerCase();
    var re = /[a-z][a-z]+/g;
    while (match = re.exec(data))
      this.all_words.data.push(match[0]);

    // Stop words
    data = fs.readFileSync(stop_words_file_path);
    data = data.toString();
    data = data.toLowerCase();
    data = data.replace(/\n|\r/g, "");
    this.stop_words.data = data.split(",");
  },
  // Update
  update: function() {
    for (var cell of this.cells) {
      if (cell.function != null)
        cell.function();
    }
  }
};

table.setup();
table.set_input(process.argv[2], "../stop_words.txt");
table.update();

table.sorted_counts.data.slice(0, N).map(e => console.log(e));
