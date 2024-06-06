#[derive(Debug)]
pub enum Literal {
    Num(f64),
    Str(String),
    Bool(bool),
    Nil,
}