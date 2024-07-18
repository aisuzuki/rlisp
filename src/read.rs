use crate::lisp::SExpr;

impl From<String> for SExpr {
    fn from(str: String) -> Self {
        // number

        // TODO: avoid nested pattern matching
        match str.parse::<f64>() {
            Ok(num) => Self::Num(num),
            Err(_) => {
                // bool
                match str.parse::<bool>() {
                    Ok(b) => SExpr::Bool(b),
                    Err(_) => {
                        // string (todo: support double quote escape)
                        let first = str.chars().nth(0).unwrap(); // todo error
                        let last = str.chars().last().unwrap(); // todo
                        if first == '"' && last == '"' {
                            let mut str_value = str.chars();
                            str_value.next();
                            str_value.next_back();
                            return Self::Str(str_value.collect());
                        } else if str != "" && str != "nil" {
                            return Self::Symbol(str.to_string());
                        } else {
                            return Self::Nil(); // todo precise check
                        }
                    }
                }
            }
        }
    }
}

pub fn read(input: &mut String) -> SExpr {
    let mut tokens = tokenise(input);
    println!("tokens {:#?}", tokens);
    // TODO: if tokens.len() == 0 {  }

    // debug
    let exprs = parse(&mut tokens);

    return exprs;
}

fn tokenise(input: &String) -> Vec<String> {
    input
        .replace("(", " ( ")
        .replace(")", " ) ")
        .replace("'", " ' ")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

fn parse<'a>(tokens: &'a mut Vec<String>) -> SExpr {
    let token = tokens.remove(0);
    // todo support nil "()""
    if token == "(" {
        // list
        let mut stack = Vec::new();
        while tokens[0] != ")" {
            stack.push(parse(tokens));
        }
        tokens.remove(0);

        println!("list process: {:#?}", tokens);

        SExpr::List(stack)
    } else if token == ")" {
        panic!(">>> token error"); // todo error handling
    } else {
        SExpr::from(token.to_string())
    }
}
