#[derive(Debug, PartialEq)]
pub struct Tag<'a> {
    _raw: &'a str,
    start_pos: usize,
    end_pos: usize,
}

#[derive(Debug, PartialEq)]
pub struct Text<'a> {
    _raw: &'a str,
    start_pos: usize,
    end_pos: usize,
}

#[derive(Debug, PartialEq)]
pub struct Comment<'a> {
    _raw: &'a str,
    start_pos: usize,
    end_pos: usize,
}

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    // DocTypeTag currently not implemented
    OpeningTag(Tag<'a>),
    ClosingTag(Tag<'a>),
    Text(Text<'a>),
    Comment(Comment<'a>),
}

#[derive(Debug, PartialEq)]
pub struct DocumentTokens<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
}

pub enum Symbol {
    Open(usize),
    Close(usize),
}

pub fn tokenize(input: &str) -> DocumentTokens {
    // 1. Find < and >
    // 2. Create token candidates:
    //   - Between < and >
    //   - Between > and <
    // 3. Create Tokens
    let mut symbols: Vec<Symbol> = Vec::with_capacity(100);

    input.char_indices().for_each(|(i, c)| match c {
        '<' => symbols.push(Symbol::Open(i)),
        '>' => symbols.push(Symbol::Close(i)),
        _ => (),
    });

    let mut tokens: Vec<Token> = Vec::with_capacity(symbols.len() * 2);
    symbols.windows(2).for_each(|token_pair| match token_pair {
        &[Symbol::Open(start_pos), Symbol::Close(end_pos)] => match input.chars().nth(start_pos + 1) {
            Some('/') => tokens.push(Token::ClosingTag(Tag { _raw: &input[start_pos..=end_pos], start_pos, end_pos })),
            Some('!') => {
                println!("{}", &input[start_pos..start_pos+3]);
                if &input[start_pos..=start_pos+3] == "<!--" && &input[end_pos-2..=end_pos] == "-->" {
                    tokens.push(Token::Comment(Comment { _raw: &input[start_pos..=end_pos], start_pos, end_pos }))
                }
            },
            _ => tokens.push(Token::OpeningTag(Tag { _raw: &input[start_pos..=end_pos], start_pos, end_pos })),
        },
        &[Symbol::Close(start_pos), Symbol::Open(end_pos)] if start_pos + 1 != end_pos
            => tokens.push(Token::Text(Text { _raw: &input[start_pos+1..=end_pos-1], start_pos: start_pos + 1, end_pos: end_pos - 1 })),
        _ => (),
    });

    DocumentTokens {
        source: input,
        tokens,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opening_tag() {
        let input = r#"<html>"#;
        let result = tokenize(input);
        assert_eq!(result, DocumentTokens {
            source: input,
            tokens: vec![ Token::OpeningTag(Tag { _raw: "<html>", start_pos: 0, end_pos: 5 }) ],
        });
    }

    #[test]
    fn closing_tag() {
        let input = r#"</html>"#;
        let result = tokenize(input);
        assert_eq!(result, DocumentTokens {
            source: input,
            tokens: vec![ Token::ClosingTag(Tag { _raw: "</html>", start_pos: 0, end_pos: 6 }) ],
        });
    }

    #[test]
    fn comment() {
        let input = r#"<!-- comment -->"#;
        let result = tokenize(input);
        assert_eq!(result, DocumentTokens {
            source: input,
            tokens: vec![ Token::Comment(Comment { _raw: "<!-- comment -->", start_pos: 0, end_pos: 15 }) ],
        });
    }

    #[test]
    fn text_between_tags() {
        let input = r#"<p>text</p>"#;
        let result = tokenize(input);
        assert_eq!(result, DocumentTokens {
            source: input,
            tokens: vec![
                Token::OpeningTag(Tag { _raw: "<p>", start_pos: 0, end_pos: 2 }),
                Token::Text(Text { _raw: "text", start_pos: 3, end_pos: 6 }),
                Token::ClosingTag(Tag { _raw: "</p>", start_pos: 7, end_pos: 10 }),
            ],
        });
    }
}
