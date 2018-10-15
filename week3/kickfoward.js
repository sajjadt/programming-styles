/*globals parse_input:true */
var fs = require("fs");
const N = 25;

// Function call chain order
// print_all(sort(remove_stop_words(scan(normalize(read_files(parse_input()))))));  

parse_input = function(next) {
    if (process.argv.length != 4){
        console.log("Bad input...")
        return;
    }
    // next = read_files
    next(process.argv[2], process.argv[3], normalize);
}


read_files = function(input_file, stop_words_file, next) {
    var input = "";
    var stop_words = "";
    fs.readFile(input_file, function (err, data) {
    if (err) throw err;
        input = data.toString();
        fs.readFile(stop_words_file, function (err, data) {
        if (err) throw err;
            stop_words = data.toString();
            // next = normalize
            next(input, stop_words, scan);
        });
    });
}

normalize = function(input, stop_words, next) {
    input = input.replace(/\n|\r/g, " ");
    input = input.toLowerCase();
    stop_words = stop_words.replace(/\n|\r/g, "")
    stop_words = stop_words.toLowerCase();
    // next = scan
    next(input, stop_words, remove_stop_words);
}

scan = function(input, stop_words, next) {
    
    var re = /[a-z][a-z]+/g;
    var matches = []
    while (match = re.exec(input))
        matches.push(match)
    
    var frequency = {};
    for (var term of matches) {
        if (term in frequency)
            frequency[term] += 1;
        else
            frequency[term] = 1;
    }
    // next = remove_stop_words
    next(frequency, new Set(stop_words.split(",")), sort);
}

no_op = function() {
    return;
}

remove_stop_words = function(frequency, stop_words, next) {
    stop_words.forEach(function (value1, set) {
        if (value1 in frequency) {
            delete frequency[value1];
        }
    }); 
    // next = sort
    next(frequency, print_all);
}

// Sort and truncate
sort = function(frequency, next) {
    
    var items = Object.keys(frequency).map(function(key) {
      return [key, frequency[key]];
    });
    
    // Sort the array based on the second element
    items.sort(function(first, second) {
      return second[1] - first[1] ;
    });
    
    // next = print all
    next(items.slice(0, N), no_op);
}

print_all = function(frequency, next) {      
    frequency.forEach((item) => {
        console.log(`${item[0]} - ${item[1]}`);
    });
    
    // next = no-op
    next();
}

parse_input(read_files);

