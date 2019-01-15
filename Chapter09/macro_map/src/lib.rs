
#[macro_export]
macro_rules! map {
    ( $( $k:expr => $v:expr ),* ) => (
        {
            let mut map = ::std::collections::HashMap::new();
            $(
                map.insert($k, $v);
            )*
            map
        }
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_map_macro() {
        let a = map! {
            "1" => 1,
            "2" => 2
        };

        assert_eq!(a["1"], 1);
        assert_eq!(a["2"], 2);
    }
}
