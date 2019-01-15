// livemd/app/index.js

import('./livemd').then((livemd) => {
    var editor = document.getElementById("editor");
    var preview = document.getElementById("preview");

    var markdownToHtml = function() {
        var markdownText = editor.value;
        html = livemd.parse(markdownText);
        preview.innerHTML = html;
    };

    editor.addEventListener('input', markdownToHtml);
    // Kick off parsing of initial text
    markdownToHtml();
}).catch(console.error);
