// native_counter/main.js

var count_words_func = require('.');
var wc = count_words_func("A test text to test native module", "test");
console.log(wc);
