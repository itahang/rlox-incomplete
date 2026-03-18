use std::fmt;

pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Number(n) => write!(f, "{}", n),
            Literal::String(s) => write!(f, "{}", s),
            Literal::Bool(b) => write!(f, "{}", b),
            Literal::Nil => write!(f, "nil"),
        }
    }
}


#[cfg(test)]
mod tests{
    use crate::rlox_libs::Literal::Literal;

    #[test]
    fn test_literal_to_string(){
        let n = Literal::Number(2.2);
        let s = Literal::String(String::from("hello"));
        let b = Literal::Bool(true);
        let nil = Literal::Nil;

        let result = format!("{n},{s},{b},{nil}");

        assert_eq!(result,"2.2,hello,true,nil")

    }
}