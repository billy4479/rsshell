#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("Invalid character: '{0}'")]
    InvalidCharacter(char),

    #[error("Invalid escape sequence: '{0}'")]
    InvalidEscapeSequence(char),

    #[error("Invalid escape sequence: no character after '\\'")]
    EmptyEscapeSequence(),
}

#[derive(Debug, Clone)]
pub enum QuoteType {
    Single,
    Double,
    Backtick,
}

#[derive(Debug, Clone)]
pub enum Token {
    Word(String),
    NewLine(u32),
    Parenthesis(char),
    Quote(QuoteType),
    DollarSign,
    Pipe,
    And,
    Or,
    Semicolon,
    Greater,
    Less,
    Whitespace(u32),
    Tab(u32),
    SingleAmpersand,
}

pub type Tokens = Vec<Token>;

pub fn parse(source: String) -> Result<Tokens, ParseError> {
    let mut iter = source.chars().peekable();
    let mut result = Vec::<Token>::new();

    while let Some(c) = iter.peek() {
        match c {
            '\n' => {
                if let Some(last) = result.last_mut() {
                    if let Token::NewLine(newlines) = *last {
                        *last = Token::NewLine(newlines + 1);
                        iter.next();
                        continue;
                    }
                }

                result.push(Token::NewLine(1));
                iter.next();
            }
            '$' => {
                result.push(Token::DollarSign);
                iter.next();
            }
            '(' | ')' | '[' | ']' | '{' | '}' => {
                result.push(Token::Parenthesis(*c));
                iter.next();
            }
            '\'' => {
                result.push(Token::Quote(QuoteType::Single));
                iter.next();
            }
            '"' => {
                result.push(Token::Quote(QuoteType::Double));
                iter.next();
            }
            '`' => {
                result.push(Token::Quote(QuoteType::Backtick));
                iter.next();
            }
            ' ' => {
                if let Some(last) = result.last_mut() {
                    if let Token::Whitespace(whitespaces) = *last {
                        *last = Token::Whitespace(whitespaces + 1);
                        iter.next();
                        continue;
                    }
                }

                result.push(Token::Whitespace(1));
                iter.next();
            }
            '\t' => {
                if let Some(last) = result.last_mut() {
                    if let Token::Tab(tabs) = *last {
                        *last = Token::Tab(tabs + 1);
                        iter.next();
                        continue;
                    }
                }

                result.push(Token::Tab(1));
                iter.next();
            }
            '#' => {
                for c in iter.by_ref() {
                    if c == '\n' {
                        break;
                    }
                }
            }
            '|' => {
                if let Some(next) = iter.next() {
                    if next == '|' {
                        result.push(Token::Or);
                        iter.next();
                        continue;
                    }
                }
                result.push(Token::Pipe);
                iter.next();
            }
            '&' => {
                if let Some(next) = iter.next() {
                    if next == '&' {
                        result.push(Token::And);
                        iter.next();
                        continue;
                    }
                }
                result.push(Token::SingleAmpersand);
                iter.next();
            }
            '>' => {
                result.push(Token::Greater);
                iter.next();
            }
            '<' => {
                result.push(Token::Less);
                iter.next();
            }
            ';' => {
                result.push(Token::Semicolon);
                iter.next();
            }

            _ => {
                let mut word = "".to_owned();

                while let Some(c) = iter.peek() {
                    let separators = [' ', '"', '\'', '`', '\n', '<', '>', ';'];

                    match c {
                        'a'..='z' | 'A'..='Z' | '_' => {
                            word.push(*c);
                            iter.next();
                        }
                        '\\' => {
                            iter.next();
                            if let Some(c) = iter.next() {
                                match c {
                                    '\\' => word.push('\\'),
                                    ' ' => word.push(' '),
                                    _ => {
                                        if separators.contains(&c) {
                                            word.push(c);
                                        }
                                        return Err(ParseError::InvalidEscapeSequence(c));
                                    }
                                }
                            } else {
                                return Err(ParseError::EmptyEscapeSequence());
                            }
                        }
                        _ => {
                            if separators.contains(c) {
                                break;
                            }
                            // TODO: Split variable name
                            word.push(*c);
                            iter.next();
                        }
                    }
                }

                result.push(Token::Word(word))
            }
        }
    }

    Ok(result)
}
