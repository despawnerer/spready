use chumsky::prelude::*;

pub type Span = std::ops::Range<usize>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Token {
    Integer(String),
    Float(String),
    Op(char),
    Ctrl(char),
    Reference(String),
}

pub fn lexer() -> impl Parser<char, Vec<(Token, Span)>, Error = Simple<char>> {
    // Floats
    let float = text::int(10)
        .chain(just('.'))
        .chain::<char, _, _>(text::digits(10))
        .collect::<String>()
        .map(Token::Float);

    // Integers
    let int = text::int(10).map(Token::Integer);

    // Operators
    let op = one_of("+-*/").map(Token::Op);

    // Control characters (delimiters, semicolons, etc.)
    let ctrl = one_of("()").map(Token::Ctrl);

    // References
    let reference = filter(char::is_ascii_alphabetic) // one letter only
        .chain::<char, _, _>(text::digits(10))
        .collect::<String>()
        .map(Token::Reference);

    // A single token can be one of the above
    let token = float
        .or(int)
        .or(op)
        .or(ctrl)
        .or(reference)
        .recover_with(skip_then_retry_until([]));

    let tokens = token
        .map_with_span(|tok, span| (tok, span))
        .padded()
        .repeated();

    just('=').ignore_then(tokens)
}
