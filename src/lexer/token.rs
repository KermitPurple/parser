#[derive(Debug, PartialEq)]
pub enum Token {
    Num(i64),
    Op(OpType),
    LParen,
    RParen,
}

impl Token {
    pub fn parse(string: &str) -> Result<Self, TokenError> {
        use Token::*;
        match string {
            "(" => Ok(LParen),
            ")" => Ok(RParen),
            op if OpType::is_op(op) => Ok(Op(OpType::parse(op)?)),
            num if num.chars().all(|ch| ch.is_ascii_digit()) && num.len() > 0 => {
                Ok(Num(str::parse(num).unwrap()))
            }
            _ => Err(TokenError),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum OpType {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

impl OpType {
    pub fn parse(string: &str) -> Result<Self, TokenError> {
        use OpType::*;
        match string {
            "+" => Ok(Add),
            "-" => Ok(Sub),
            "*" => Ok(Mul),
            "/" => Ok(Div),
            "%" => Ok(Rem),
            _ => Err(TokenError),
        }
    }

    pub fn is_op(string: &str) -> bool {
        match string {
            "+" | "-" | "*" | "/" | "%" => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct TokenError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_parse() {
        use OpType::*;
        use Token::*;
        assert_eq!(Token::parse("100").unwrap(), Num(100));
        assert_eq!(Token::parse("22").unwrap(), Num(22));
        assert_eq!(Token::parse("19").unwrap(), Num(19));
        assert_eq!(Token::parse("9999").unwrap(), Num(9999));
        assert_eq!(Token::parse("+").unwrap(), Op(Add));
        assert_eq!(Token::parse("-").unwrap(), Op(Sub));
        assert_eq!(Token::parse("*").unwrap(), Op(Mul));
        assert_eq!(Token::parse("/").unwrap(), Op(Div));
        assert_eq!(Token::parse("%").unwrap(), Op(Rem));
        assert_eq!(Token::parse("(").unwrap(), LParen);
        assert_eq!(Token::parse(")").unwrap(), RParen);
    }

    #[test]
    fn test_op_parse() {
        use OpType::*;
        assert_eq!(OpType::parse("+").unwrap(), Add);
        assert_eq!(OpType::parse("-").unwrap(), Sub);
        assert_eq!(OpType::parse("*").unwrap(), Mul);
        assert_eq!(OpType::parse("/").unwrap(), Div);
        assert_eq!(OpType::parse("%").unwrap(), Rem);
        assert!(OpType::parse("wrong").is_err());
    }

    #[test]
    fn test_is_op() {
        assert!(OpType::is_op("+"));
        assert!(OpType::is_op("-"));
        assert!(OpType::is_op("*"));
        assert!(OpType::is_op("/"));
        assert!(OpType::is_op("%"));
        assert!(!OpType::is_op("wrong"));
    }
}
