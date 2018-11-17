var fs = require("fs");
const N = 25;

if (process.argv.length == 3) {
    scan = 'function(name){data = fs.readFileSync(name);data = data.toString();data = data.replace("/\\n|\\r/g", " ");data = data.toLowerCase();var items = [];var re = /[a-z][a-z]+/g; var matches = []; while (match = re.exec(data)) matches.push(match[0]); return matches;}';
    remove_stop_words = 'function(words){valid_words = []; var data = fs.readFileSync("../stop_words.txt"); data = data.toString();var stop_words = new Set(data.split(",")); for(var entry of words) {if (!stop_words.has(entry))valid_words.push(entry);}return valid_words;}';
    calc_freq = 'function(word_list){word_freqs = {};for (let word of word_list){if (word in word_freqs) word_freqs[word] += 1; else word_freqs[word] = 1;} return word_freqs;}';
    sort_freq = 'function(word_freqs){var items = Object.entries(word_freqs).map(function(entry) {return [entry[0], entry[1]];});items.sort(function(first, second) {return second[1] - first[1] ;});return items;}';
    print = 'function(word_freqs){word_freqs.slice(0, N).forEach((item) => {console.log(`${item[0]} - ${item[1]}`);});}';
    file_name = process.argv[2];
} else {
    scan ='function(_){return [];}';
    remove_stop_words = 'function(_){return [];}';
    calc_freq = 'function(_){return {};}';
    sort_freq = 'function(_){return {};}';
    print = 'function(_){}';
    file_name = '';
}

// Add functions dynamically to the program
eval('scan = ' + scan);
eval('remove_stop_words = ' + remove_stop_words);
eval('calc_freq = ' + calc_freq);
eval('sort_freq = ' + sort_freq);
eval('print = ' + print)

print(sort_freq(calc_freq(remove_stop_words(scan(file_name)))));

