var readlineSync = require('readline-sync');
var fs = require("fs");
const N = 25;

// Set up the database 
extract_stop_words = function() {
  var data = fs.readFileSync("../stop_words.txt");
  data = data.toString();
  data = data.toLowerCase();
  data = data.replace(/\n|\r/g, "");
  var stop_words = new Set(data.split(","));
  return stop_words;
}
var stop_words = extract_stop_words();
var data = {};


// The "server"-side application handlers
quit_handler = function(args) {
  process.exit();
}

default_get_handler = function(args) {
  rep = "What would you like to do?";
  rep += "\n1 - Quit" + "\n2 - Upload file";
  links = {"1" : ["post", "execution", null], "2" : ["get", "file_form", null]};
  return {rep:rep, links:links};
};

upload_get_handler = function(args) {
  return {rep: "Name of file to upload?", links: ["post", "file"]}; // last item will be appended from user input
}

upload_post_handler = function(args) {
  create_data = function(filename) {
    if (filename in data) // If already cached
      return;
    word_freqs = {};

    contents = fs.readFileSync(filename);
    
    contents = contents.toString();
    contents = contents.replace("/\\n|\\r/g", " ");
    contents = contents.toLowerCase();
    
    var items = [];
    var re = /[a-z][a-z]+/g;
    var matches = [];
    while (match = re.exec(contents))
      matches.push(match[0]);
    
    for(var entry of matches) {
      if (!stop_words.has(entry)) {  
        if (entry in word_freqs) 
          word_freqs[entry] += 1;
        else 
          word_freqs[entry] = 1;
      }
    }

    // Also sort
    var items = Object.entries(word_freqs).map(function(entry) {
      return [entry[0], entry[1]];
    });
    items.sort(function(first, second) {
      return second[1] - first[1] ;
    });

    data[filename] = items;
    
  }

  if (args == null)
    return error_state();

  filename = args;
  try {
    create_data(filename);
    return word_get_handler([filename, 0]);
  } catch(err) {
    return error_state();
  }

  
}

word_get_handler = function(args){

  get_word = function(filename, word_index) {
    if (filename in data && word_index < data[filename].length)
      return data[filename][word_index];
    else
      return ["no more words", 0];
  }

  filename = args[0]; 
  word_index = args[1];

  word_info = get_word(filename, word_index)
  rep = "\n#" + (word_index+1) + ": " + word_info[0] + " - " + word_info[1];
  rep += "\n\nWhat would you like to do next?"
  rep += "\n1 - Quit" + "\n2 - Upload file"
  rep += "\n3 - See next most-frequently occurring word"
  links = {"1" : ["post", "execution", null], 
          "2" : ["get", "file_form", null], 
          "3" : ["get", "word", [filename, word_index+1]]}
  return {rep: rep, links: links};

}

var handlers = {
  "post_execution": quit_handler,
  "get_default": default_get_handler, 
  "get_file_form": upload_get_handler, 
  "post_file": upload_post_handler, 
  "get_word": word_get_handler 
};

// Returns {state and links}
handle_request = function(verb, uri, args) {
  var handler_key = function(verb, uri) {
    return verb + "_" + uri;
  }

  if (handler_key(verb, uri) in handlers) {
    return handlers[handler_key(verb, uri)](args);
  } else {
    return handlers[handler_key("get", "default")](args);
  }
}

// Return new request
render_and_get_input = function(rep, links) {
  console.log(rep);

  is_dict = function(obj) {
    return obj.constructor == Object;
  }

  is_array = function(obj) {
    return obj instanceof Array;
  }

  if (is_dict(links)) {  // many possible next states
    var input = readlineSync.question("Your choice? ");

    // TODO: Log the answer in a database
    if (input in links)
      return links[input];
    else
      return ["get", "default", null];
    
  } else if (is_array(links)){ // only one possible next state
    if (links[0] == "post") {// get "form" data
      var input = readlineSync.question("File name? ");
      links.push(input); // add the data at the end
      return links;
    }
    else { // get action, don't get user input
      return links;
    }
  } else {
    return ["get", "default", null];
  }
}

var request = ["get", "default", null];
while (true) {
  response = handle_request(...request); // Using spread syntax
  request = render_and_get_input(response.rep, response.links);
}
