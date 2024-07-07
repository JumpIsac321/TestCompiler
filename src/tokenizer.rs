use crate::CodeError;
use crate::Token;

pub fn tokenize(program_string: String) -> Result<Vec<Token>, CodeError> {
    let chars: Vec<char> = program_string.chars().collect();
    let mut i = 0usize;
    let mut tokens = Vec::new();
    loop {
        match get_token(&mut i, &chars) {
            Ok(o) => tokens.push(o),
            Err(e) => return Err(e),
        }
        if let Token::Eof = tokens[tokens.len() - 1] {
            break;
        }
    }
    Ok(tokens)
}
fn get_token(i: &mut usize, chars: &[char]) -> Result<Token, CodeError> {
    while *i < chars.len() && chars[*i].is_whitespace() {
        *i += 1;
    }
    if *i >= chars.len() {
        return Ok(Token::Eof);
    }
    if chars[*i].is_ascii_alphabetic() {
        return Ok(get_alphanum(i, chars));
    } else if chars[*i].is_ascii_digit() {
        return get_num(i, chars);
    } else if chars[*i].is_ascii_punctuation() {
        return get_symbol(i, chars);
    } else {
        return Err(CodeError::FailureError);
    }
}

fn decode_buf(buf: &str) -> Result<Token, CodeError> {
    match buf {
        "exit" => Ok(Token::Exit),
        "1" => Ok(Token::IntLit(1)),
        ";" => Ok(Token::Semi),
        "(" => Ok(Token::OP),
        ")" => Ok(Token::CP),
        _ => Err(CodeError::SyntaxError),
    }
}

fn decode_num(buf: &str) -> Result<Token, CodeError> {
    let num: i32 = match buf.parse() {
        Ok(n) => n,
        Err(_) => return Err(CodeError::SyntaxError),
    };
    Ok(Token::IntLit(num))
}

fn get_alphanum(i: &mut usize, chars: &[char]) -> Token {
    let mut buf = String::new();
    while *i < chars.len() && chars[*i].is_ascii_alphanumeric() {
        buf.push(chars[*i]);
        *i += 1;
    }
    decode_alphanum(&buf)
}
fn decode_alphanum(buf: &str) -> Token {
    match buf {
        "exit" => Token::Exit,
        _ => Token::Ident(buf.to_string()),
    }
}
fn get_num(i: &mut usize, chars: &[char]) -> Result<Token, CodeError> {
    let mut buf = String::new();
    while *i < chars.len() && chars[*i].is_ascii_digit() {
        buf.push(chars[*i]);
        *i += 1;
    }
    decode_num(&buf)
}
fn get_symbol(i: &mut usize, chars: &[char]) -> Result<Token, CodeError> {
    let mut buf = String::new();
    buf.push(chars[*i]);
    *i += 1;
    decode_buf(&buf)
}

#[cfg(test)]
mod tests {
    use crate::Token;

    use super::decode_alphanum;

    #[test]
    fn works() {
        let token = decode_alphanum("var");
        match token {
            Token::Ident(s) => assert_eq!(s, "var"),
            _ => panic!("Not int literal"),
        }
    }
}
