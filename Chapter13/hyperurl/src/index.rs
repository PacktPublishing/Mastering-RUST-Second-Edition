// hyperurl/src/index.rs

pub static INDEX_PAGE: &str = r##"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta http-equiv="X-UA-Compatible" content="ie=edge">
    <title>hyperurl - A url shortener in Rust</title>
    <style> body { text-align: center;} </style>
</head>
<body>
<h3>hyperurl - A url shortening service</h3>
    <p>To shorten a url, make a post request using curl as:</p>
    <p><code>curl --data "https://creativcoder.github.io" http://localhost:3002/shorten</code></p>
    <p>You will get a reply as:</p>
    <p>{
    "127.0.0.1:3002/992a7": "https://creativcoder.github.io"
    }</p>
    <p>Put the shortened url (127.0.0.1:3002/992a7) in the browser to get redirected to the original website.</p>
    <p>Made with ❤ with hyper in Rust</p>
</body>
</html>
"##;
