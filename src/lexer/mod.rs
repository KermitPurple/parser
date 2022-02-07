mod token;
use token::{Token, OpType, TokenError};

pub fn parse(string: &str) -> Result<Vec<Token>, TokenError> {
    let mut result = vec![];
    let mut index = 0;
    let mut prev_numeric = false;
    for (i, ch) in string.chars().enumerate() {
        if !ch.is_numeric() || !prev_numeric{
            if let Ok(token) = Token::parse(string[index..i].trim()) {
                result.push(token);
            }
            index = i;
        }
        prev_numeric = ch.is_numeric();
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
        assert_eq!(parse("10+12").unwrap(), vec![Num(10), Op(Add), Num(12)]);
        assert_eq!(parse("(10+12)*2").unwrap(), vec![LParen, Num(10), Op(Add), Num(12), RParen, Op(Mul), Num(2)]);
    }
}
