#!/usr/bin/env python
import re, sys, operator

# Mileage may vary. If this crashes, make it lower
RECURSION_LIMIT = 300
# We add a few more, because, contrary to the name,
# this doesn't just rule recursion: it rules the 
# depth of the call stack
#sys.setrecursionlimit(2*RECURSION_LIMIT+10)

def simple_count(word, stopwords, wordfreqs):
    # Process the head word
    if word not in stopwords:
        if word in wordfreqs:
            wordfreqs[word] += 1
        else:
            wordfreqs[word] = 1
    return True
  
def simple_print(wordfreq):
    (w, c) = wordfreq
    print w,.. '-', c
    return True
        
Y = lambda f: (lambda x: x(x))(lambda y: f(lambda *args: y(y)(*args)))
print_l = lambda f: lambda W: (None if W == [] else simple_print(W[0]) and f(W[1:]))
count_l = lambda f: lambda WL,SL,FL: (None if WL == [] else simple_count(WL[0],SL,FL) and f(WL[1:],SL,FL))

stop_words = set(open('../stop_words.txt').read().split(','))
words = re.findall('[a-z]{2,}', open(sys.argv[1]).read().lower())
word_freqs = {}

for i in range(0, len(words), RECURSION_LIMIT):
    Y(count_l)(words[i:i+RECURSION_LIMIT], stop_words, word_freqs)
Y(print_l)(sorted(word_freqs.iteritems(), key=operator.itemgetter(1), reverse=True)[:25])

