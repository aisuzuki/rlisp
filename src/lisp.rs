#[derive(Debug)]
pub enum SExpr {
    // TODO float
    Num(f64),
    Bool(bool),         // delete -> nil or t
    Symbol(String),
    Str(String),
    List(Vec<SExpr>),
    Nil(),
}

// parse string into cons cell directry in read
pub enum Type {
    // TODO float
    Num(f64),
    Str(String),
    Symbol(String),     // symbol table address
    Cons(),
    Nil(),
    T(),
}

// TODO make Nil as single instance

pub struct LispExpr {
}

pub struct ConsCell {
    pub t: Type,
    pub car: Box<ConsCell>,     // allocate car in memory table
    pub cdr: Box<ConsCell>,     // allocate cdr in memory table
}

impl ConsCell {
    pub fn new(t: Type, car: ConsCell, cdr: ConsCell) -> Self {
        ConsCell {t, car: Box::new(car), cdr: Box::new(cdr) }       // allocate car/cdr in memory table
    }
}

// todo: implement macro for car/cdr
