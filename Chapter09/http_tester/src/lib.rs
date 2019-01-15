
#[macro_export]
macro_rules! http_test { 
    ($url:tt GET => $code:expr) => { 
        let request = reqwest::get($url).unwrap(); 
        println!("Testing GET {} => {}", $url, $code);
        assert_eq!(request.status().as_u16(), $code); 
    }; 
    ($url:tt POST => $code:expr, $($k:expr => $v:expr),*) => {
        let params = [$(($k, $v),)*];
        let client = reqwest::Client::new();
        let res = client.post($url)
            .form(&params)
            .send().unwrap();
        println!("Testing POST {} => {}", $url, $code); 
        assert_eq!(res.status().as_u16(), $code);
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_http_verbs() {
        http_test!("http://duckduckgo.com" GET => 200);
        http_test!("http://httpbin.org/post" POST => 200, "hello" => "world", "foo" => "bar");
    }
}
