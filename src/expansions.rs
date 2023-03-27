use std::{iter::Peekable, ops::Deref, slice::Iter};

use crate::parser::{QuoteType, Token, Tokens};

pub fn do_expansions(tokens: Tokens) {
    let mut iter = tokens.iter().peekable();

    let mut is_within_singlequote = false;

    while let Some(token) = iter.peek() {
        match token {
            Token::Quote(quote) => match quote {
                QuoteType::Single => {
                    is_within_singlequote = !is_within_singlequote;
                    iter.next();
                }
                QuoteType::Backtick => {
                    if is_within_singlequote {
                        iter.next();
                        continue;
                    }
                    iter.next();
                    let mut to_expand = Vec::<Token>::new();
                    let mut backtick_found = false;

                    while let Some(token) = iter.next() {
                        match token {
                            Token::Quote(quote) => {
                                if matches!(quote, QuoteType::Backtick) {
                                    backtick_found = true;
                                    break;
                                }
                                to_expand.push(token.deref().clone());
                            }
                            _ => {
                                to_expand.push(token.deref().clone());
                            }
                        }
                    }

                    if !backtick_found {
                        eprintln!("Backtick not found!")
                    }

                    println!("Command expansion: {:?}", to_expand);
                }
                _ => {
                    iter.next();
                }
            },
            Token::DollarSign => {
                if is_within_singlequote {
                    iter.next();
                    continue;
                }
                iter.next();
                let first = match iter.next() {
                    None => continue,
                    Some(x) => x,
                };

                match first {
                    Token::Parenthesis(parenthesis) => {
                        todo!();
                    }
                    Token::Word(word) => {
                        let variable_name = word
                            .split(&[' ', '/', '-'])
                            .next()
                            .expect("expected at least one split");
                        println!("Variable expansion: {}", variable_name);
                    }
                    _ => {
                        eprintln!("[!] found {:?} after $", first);
                    }
                }
                iter.next();
            }
            _ => {
                iter.next();
                continue;
            }
        };
    }
}

fn arithmetic_substitution(iter: &mut Peekable<Iter<Token>>) -> String {
    todo!();
}

fn parameter_substitution(iter: &mut Peekable<Iter<Token>>) -> String {
    todo!();
}

fn command_substitution(command: &str) -> String {
    todo!();
}
