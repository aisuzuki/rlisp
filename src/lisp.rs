#[derive(Debug)]
pub enum SExpr {
    Num(f64),
    Bool(bool),
    Symbol(String),
    Str(String),
    List(Vec<SExpr>),
    Nil(),
}

pub enum LispExpr {
    // todo
}

pub struct ConsCell<T> {
    pub car: T,
    pub cdr: Option<Box<ConsCell<T>>>,
}

impl<T> ConsCell<T> {
    pub fn new(car: T, cdr: Option<Box<ConsCell<T>>>) -> Self {
        ConsCell { car, cdr }
    }
}

// todo: implement macro for car/cdr
