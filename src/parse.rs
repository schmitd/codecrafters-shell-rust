use strum::{Display, EnumString};

type WordPart = String;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Atom(char),
    Op(char),
    Eof,
}

// XXX Can this be salvaged, is it worth it, is it cool?
// #[derive(EnumString, Debug, Clone, PartialEq, Eq)]
// enum Op {
//     #[strum(serialize = "|")]
//     Pipe,
//     // And,
//     // Or,
//     Redirect(Redirect),
//     #[strum(serialize = "(")]
//     LParen,
//     #[strum(serialize = ")")]
//     RParen,
// }

// #[derive(EnumString, Debug, Clone, PartialEq, Eq)]
// enum Redirect {
//     #[strum(serialize = "<")]
//     In,
//     #[strum(serialize = ">")]
//     Out,
//     #[strum(serialize = ">>")]
//     Append,
// }

struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    fn new(input: &str) -> Self {
        let mut tokens = input
            .chars()
            .filter(|it| !it.is_ascii_whitespace())
            .map(|c| match c {
                '0'..='9' | 'a'..='z' | 'A'..='Z' => Token::Atom(c),
                _ => Token::Op(c),
            })
            .collect::<Vec<_>>();
        tokens.reverse();
        Self { tokens }
    }

    fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Eof)
    }

    fn peek(&self) -> Token {
        self.tokens.last().cloned().unwrap_or(Token::Eof)
    }
}

use std::fmt;

enum S {
    Atom(char),
    Cons(char, Vec<S>),
}
impl fmt::Display for S {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            S::Atom(i) => write!(f, "{}", i),
            S::Cons(head, rest) => {
                write!(f, "({}", head)?;
                for s in rest {
                    write!(f, " {}", s)?
                }
                write!(f, ")")
            }
        }
    }
}

fn expr(input: &str) -> S {
    let mut lexer = Lexer::new(input);
    expr_bp(&mut lexer)
}
fn expr_bp(lexer: &mut Lexer) -> S {
    todo!()
}
#[test]
fn tests() {
    let s = expr("1 + 2 * 3");
    assert_eq!(s.to_string(), "(+ 1 (* 2 3))")
}
