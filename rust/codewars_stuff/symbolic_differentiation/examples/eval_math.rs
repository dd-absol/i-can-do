
fn main() {
    println!("Hellow");
}

#[derive(Debug)]
enum Part {
    Nb(f64),
    Op(Operator)
}

#[derive(Debug)]
enum Operator {
    HP(Mult),
    LP(Add)
}

#[derive(Debug)]
enum Mult {
    Prod,
    Div
}

#[derive(Debug)]
enum Add {
    Sum,
    Diff
}

struct ParserState {
    parsing_nb: bool,
    parsing_par: bool,
    state: String,
    buffer: Vec<char>,
}

fn calc(expr: &str) -> f64 {

    let mut parser = ParserState {
        parsing_nb: false,
        parsing_par: false,
        state: String::new(),
        buffer: Vec::new()
    };
    
    let instructions = expr.chars().filter_map(|c| parser.read_expr(c)).rev().collect::<Vec<Part>>();
    
    dbg!(&instructions);

    operate(instructions, 0.0, Operator::LP(Add::Sum))
}

fn operate(mut instructions: Vec<Part>, mut left: f64, op: Operator) -> f64 {
    use Part::*;
    use Operator::*;

    match op {
        HP(mult) => {
            let right = match instructions.pop() {
                Some(Nb(x)) => x,
                _ => return left
            };
    
            left = match mult {
                Mult::Prod => left * right,
                Mult::Div => left / right
            };
    
            match instructions.pop() {
                Some(Op(op)) => operate(instructions, left, op),
                _ => left
            }
        }
        LP(add) => {
            let new_left = match instructions.pop() {
                Some(Nb(x)) => x,
                _ => left
            };

            match add {
                Add::Sum => left + match instructions.pop() {
                    Some(Op(op)) => operate(instructions, new_left, op),
                    _ => new_left
                },
                Add::Diff => left - match instructions.pop() {
                    Some(Op(op)) => operate(instructions, new_left, op),
                    _ => new_left
                }
            }
        }
    }
}

impl ParserState {
    fn read_expr(&mut self, real_c: char) -> Option<Part> {
        use Part::*;
        use Operator::*;
        use Add::*;
        use Mult::*;
    
        let c = match self.buffer.pop() {
            None => real_c,
            Some(other_c) => {
                if real_c != ' ' {
                    self.buffer.push(real_c);
                }
    
                other_c
            }
        };
    
        if c == ')' {
            self.parsing_par = false;
            return Some(Nb(calc(&self.state)))
        }
    
        if self.parsing_par {
            self.state.push(c);
            return None
        }
    
        if c == '(' {
            self.state.clear();
            self.parsing_par = true;
            return None
        }
    
        if c.is_digit(10) || c == '.'  {
            if !self.parsing_nb {
                self.state.clear();
                self.parsing_nb = true;
            }
            
            self.state.push(c);
    
            return None
        }
    
        //plutôt vers le bas (n'est pas synchronisé avec la fin du nombre ce qui est un pb)
        if self.parsing_nb {
            self.parsing_nb = false;
            if c != ' ' {
                self.buffer.push(c);
            }
    
            return Some(Nb(self.state.parse::<f64>().unwrap()))
        }
    
        match c {
            '+' => Some(Op(LP(Sum))),
            '-' => Some(Op(LP(Diff))),
            '*' => Some(Op(HP(Prod))),
            '/' => Some(Op(HP(Div))),
            ' ' => match self.buffer.pop() {
                None => None,
                Some(other_c) => self.read_expr(other_c)
            }
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::calc;

    // Wrap custom message to reduce repitition
    macro_rules! assert_expr_eq {
        ($expr: expr, $expect: expr) => {
            assert_eq!(
                calc($expr),
                $expect,
                "\nexpected expression \"{}\" to equal \"{:?}\", but got \"{:?}\"",
                $expr,
                $expect,
                calc($expr),
            );
        }
    }
    
    #[test]
    fn single_values() {
        assert_expr_eq!("0", 0.0);
        assert_expr_eq!("1", 1.0);
        assert_expr_eq!("42", 42.0);
        assert_expr_eq!("350", 350.0);
    }

    #[test]
    fn basic_operations() {
        assert_expr_eq!("1 + 1", 2.0);
        assert_expr_eq!("1 - 1", 0.0);
        assert_expr_eq!("1 * 1", 1.0);
        assert_expr_eq!("1 / 1", 1.0);
        assert_expr_eq!("12 * 123", 1476.0);
    }

    #[test]
    fn whitespace_between_operators_and_operands() {
        assert_expr_eq!("1-1", 0.0);
        assert_expr_eq!("1 -1", 0.0);
        assert_expr_eq!("1- 1", 0.0);
        assert_expr_eq!("1* 1", 1.0);
    }

    #[test]
    fn unary_minuses() {
        assert_expr_eq!("1- -1", 2.0);
        assert_expr_eq!("1--1", 2.0);
        assert_expr_eq!("1 - -1", 2.0);
        assert_expr_eq!("-42", -42.0);
    }

    #[test]
    fn parentheses() {
        assert_expr_eq!("(1)", 1.0);
        assert_expr_eq!("((1))", 1.0);
        assert_expr_eq!("((80 - (19)))", 61.0);
    }

    #[test]
    fn multiple_operators() {
        assert_expr_eq!("12* 123/(-5 + 2)", -492.0);
        assert_expr_eq!("1 - -(-(-(-4)))", -3.0);
        assert_expr_eq!("2 /2+3 * 4.75- -6", 21.25);
        assert_expr_eq!("2 / (2 + 3) * 4.33 - -6", 7.732);
        assert_expr_eq!("(1 - 2) + -(-(-(-4)))", 3.0);
        assert_expr_eq!("((2.33 / (2.9+3.5)*4) - -6)", 7.45625);
    }
}