mod token;
use token::{Token, TokenError};

pub fn parse(string: &str) -> Result<Vec<Token>, TokenError> {
    let mut result = vec![];
    let mut index = 0;
    for (i, ch) in string.chars().enumerate() {
        if ch.is_whitespace() {
            result.push(Token::parse(string[index..i].trim())?);
            index = i;
        }
    }
    if let Ok(token) = Token::parse(string[index..].trim()) {
        result.push(token);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        use Token::*;
        use token::OpType::*;
        assert_eq!(parse("+").unwrap(), vec![Op(Add)]);
        assert_eq!(parse("10 + 12").unwrap(), vec![Num(10), Op(Add), Num(12)]);
    }
}
