use crate::lisp::SExpr;
use std::error::Error;

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

pub fn read(input: &mut String) -> Result<SExpr, Box<dyn Error>> {
    let mut tokens = tokenise(input);
    println!("tokens {:#?}", tokens);
    if tokens.len() == 0 {
        return Err("to be fixed: empty input - ignored".into());
    }

    // parse(&mut tokens)
    // debug
    if let Ok(parsed) = parse(&mut tokens) {
        println!("parsed {:#?}", parsed);
        return Ok(parsed);
    } else {
        return Err("parse error".into());
    }
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

// Create direct cons cell
fn parse<'a>(tokens: &'a mut Vec<String>) -> Result<SExpr, Box<dyn Error>> {
    let token = tokens.remove(0);
    // todo support nil "()""
    if token == ")" {
        return Err("wrong close \")\"".into()); // use fmt
    }

    if token == "(" {
        // list
        let mut stack = Vec::new();
        while tokens[0] != ")" {
            match parse(tokens) {
                Ok(t) => {
                    stack.push(t);
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }
        tokens.remove(0);

        println!("list process: {:#?}", tokens);

        Ok(SExpr::List(stack))
    } else {
        Ok(SExpr::from(token.to_string()))
    }
}

// it's kinda funny to write test code within a program file

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_string() {
        let mut input = "\"stringdata\"".to_string();
        let result = read(&mut input);
        assert!(result.is_ok());

        if let Ok(s) = result {
            match s {
                SExpr::Str(str) => {
                    assert_eq!(str, input.replace("\"", ""));
                }
                _ => panic!("result should be string"),
            }
        }
    }

    #[test]
    fn read_symbol() {
        let mut input = "symbol".to_string();
        let result = read(&mut input);
        assert!(result.is_ok());

        if let Ok(s) = result {
            match s {
                SExpr::Symbol(symbol) => {
                    assert_eq!(symbol, input);
                }
                _ => panic!("result should be symbol"),
            }
        }
    }

    #[test]
    fn read_integer() {
        let mut input = "100".to_string();
        let result = read(&mut input);
        assert!(result.is_ok());

        if let Ok(s) = result {
            match s {
                SExpr::Num(n) => {
                    assert_eq!(n, 100.0);
                }
                _ => panic!("result should be number"),
            }
        }
    }

    #[test]
    fn read_bool() {
        // test true and false
        for input in ["true".to_string(), "false".to_string()].iter() {
            let result = read(&mut input.clone());
            assert!(result.is_ok());

            if let Ok(s) = result {
                match s {
                    SExpr::Bool(b) => {
                        assert_eq!(b, input.parse::<bool>().unwrap());
                    }
                    _ => panic!("result should be bool"),
                }
            }
        }
    }

    #[test]
    fn read_nil() {
        let mut input = "nil".to_string();
        let result = read(&mut input);
        assert!(result.is_ok());

        if let Ok(s) = result {
            match s {
                SExpr::Nil() => {
                    assert!(true);
                }
                _ => panic!("result should be nil"),
            }
        }
    }

    #[test]
    fn read_list() {
        let mut input = "(+ 1 2)".to_string();
        let result = read(&mut input);
        assert!(result.is_ok());

        if let Ok(s) = result {
            match s {
                SExpr::List(l) => {
                    assert_eq!(l.len(), 3);
                    assert!(matches!(l[0], SExpr::Symbol(ref sym) if sym == "+"));
                    assert!(matches!(l[1], SExpr::Num(n) if n == 1.0));
                    assert!(matches!(l[2], SExpr::Num(n) if n == 2.0));
                }
                _ => panic!("result should be list"),
            }
        }
    }
}
