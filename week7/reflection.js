/*globals parse_input:true */
var fs = require("fs");
const N = 25;

read_stop_file = function() {
    var data = fs.readFileSync("../stop_words.txt");
    data = data.toString();
    data = data.toLowerCase();
    data = data.replace(/\n|\r/g, "")
    return new Set(data.split(","));
}

if (process.argv.length == 3) {
    extract_words = 'function(name){data = fs.readFileSync(name);data = data.toString();data = data.replace("/\\n|\\r/g", " ");data = data.toLowerCase();var items = [];var re = /[a-z][a-z]+/g; var matches = []; while (match = re.exec(data)) matches.push(match[0]); for(var entry of matches) {if (!stop_words.has(entry))items.push(entry);}return items;}';
    calc_freq = 'function(word_list){word_freqs = {};for (let word of word_list){if (word in word_freqs) word_freqs[word] += 1; else word_freqs[word] = 1;} return word_freqs;}';
    sort_freq = 'function(word_freqs){var items = Object.entries(word_freqs).map(function(entry) {return [entry[0], entry[1]];});items.sort(function(first, second) {return second[1] - first[1] ;});return items;}';
    file_name = process.argv[2];
} else {
    extract_words ='function(_){return [];}';
    calc_freq = 'function(_){return {};}';
    sort_freq = 'function(_){return {};}';
    file_name = '';
}

// Add functions dynamically to the program
eval('extract_words = ' + extract_words);
eval('calc_freq = ' + calc_freq);
eval('sort_freq = ' + sort_freq);

stop_words = read_stop_file();
var word_freqs = sort_freq(calc_freq(extract_words(file_name)));
word_freqs.slice(0, N).forEach((item) => {
    console.log(`${item[0]} - ${item[1]}`);
});
