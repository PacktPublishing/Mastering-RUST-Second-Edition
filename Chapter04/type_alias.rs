// type_alias.rs

pub struct ParsedPayload<T> {
    inner: T
}

pub struct ParseError<E> {
    inner: E
}

type ParserResult<T, E> = Result<ParsedPayload<T>, ParseError<E>>;

pub fn parse_payload<T, E>(stream: &[u8]) -> ParserResult<T, E> {
    unimplemented!();
}

fn main() {
    // todo
}
