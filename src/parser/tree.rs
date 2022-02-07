pub enum Tree {
    Int(i64),
    Add(Box<Tree>, Box<Tree>),
    Sub(Box<Tree>, Box<Tree>),
    Mul(Box<Tree>, Box<Tree>),
    Div(Box<Tree>, Box<Tree>),
    Rem(Box<Tree>, Box<Tree>),
}

impl Tree {
    fn eval(&self) -> i64 {
        use Tree::*;
        match self {
            Int(num) => *num,
            Add(a, b) => a.eval() + b.eval(),
            Sub(a, b) => a.eval() - b.eval(),
            Mul(a, b) => a.eval() * b.eval(),
            Div(a, b) => a.eval() / b.eval(),
            Rem(a, b) => a.eval() % b.eval(),
        }
    }
}

impl ToString for Tree {
    fn to_string(&self) -> String {
        use Tree::*;
        match self {
            Int(num) => num.to_string(),
            Add(a, b) => format!("({} + {})", a.to_string(), b.to_string()),
            Sub(a, b) => format!("({} - {})", a.to_string(), b.to_string()),
            Mul(a, b) => format!("({} * {})", a.to_string(), b.to_string()),
            Div(a, b) => format!("({} / {})", a.to_string(), b.to_string()),
            Rem(a, b) => format!("({} % {})", a.to_string(), b.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval(){
        assert_eq!(3, Tree::Add(Box::new(Tree::Int(1)), Box::new(Tree::Int(2))).eval());
        assert_eq!(
            7,
            Tree::Add(
                Box::new(Tree::Int(1)),
                Box::new(Tree::Mul(
                    Box::new(Tree::Int(2)),
                    Box::new(Tree::Int(3))
                ))
            ).eval(),
        );
    }

    #[test]
    fn test_to_string(){
        assert_eq!("(1 + 2)", Tree::Add(Box::new(Tree::Int(1)), Box::new(Tree::Int(2))).to_string());
        assert_eq!(
            "(1 + (2 * 3))",
            Tree::Add(
                Box::new(Tree::Int(1)),
                Box::new(Tree::Mul(
                    Box::new(Tree::Int(2)),
                    Box::new(Tree::Int(3))
                ))
            ).to_string(),
        );
    }
}
