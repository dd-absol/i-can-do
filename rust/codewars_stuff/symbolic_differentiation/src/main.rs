use std::{iter::Product, fmt::format};

fn diff(expr: &str) -> String {
    parse_expr(expr).diff().simplify().parse_back()
}

#[derive(Debug, Clone, PartialEq)]
enum Expression {
    Par(Parenthese),
    Nb(f32),
    X
}

#[derive(Debug, Clone, PartialEq)]
struct FnCall {
    func: Function,
    arg: Box<Expression>
}

#[derive(Debug, Clone, PartialEq)]
struct Operation {
    operator: Operator,
    left_arg: Box<Expression>,
    right_arg: Box<Expression>
}

#[derive(Debug, Clone, PartialEq)]
enum Parenthese {
    Op(Operation),
    Fn(FnCall),
}

#[derive(Debug, Clone, PartialEq)]
//+ - * / ^ cos sin tan exp ln
enum Operator {
    Sum,
    Difference,
    Product,
    Division,
    Exponentiation
}

#[derive(Debug, Clone, PartialEq)]
enum Function {
    Cos,
    Sin,
    Tan,
    Exp,
    Ln
}

fn parse_expr(expr: &str) -> Expression {
    match expr.chars().next() {
        Some('x') => Expression::X,
        Some('(') => Expression::Par(parse_par(&expr[1..(expr.len() - 1)])),
        _ => Expression::Nb(expr.parse::<f32>().unwrap())
    }
}

fn parse_par(expr: &str) -> Parenthese {
    match expr.split_whitespace().next() {
        Some("+"|"-"|"*"|"/"|"^") => Parenthese::Op(parse_operation(expr)),
        _ => Parenthese::Fn(parse_fncall(expr))
    }
}

fn parse_operation(expr: &str) -> Operation {
    use Operator::*;

    let mut it = expr.chars();

    let operator = match it.next().unwrap() {
        '+' => Sum,
        '-' => Difference,
        '*' => Product,
        '/' => Division,
        '^' => Exponentiation,
        _ => panic!("wrong operator")
    };

    let mut depth = 0;

    it.next();
    let left_arg = Box::new(parse_expr(it.by_ref().take_while(|c| match c {
        '(' => { depth += 1; true }
        ')' => { depth -= 1; true }
        ' ' => depth > 0,
        _ => true
    }).collect::<String>().as_str()));

    let right_arg = Box::new(parse_expr(it.by_ref().take_while(|c| match c {
        '(' => { depth += 1; true }
        ')' => { depth -= 1; true }
        ' ' => depth > 0,
        _ => true
    }).collect::<String>().as_str()));
    
    Operation { operator, left_arg, right_arg }
}

fn parse_fncall(expr: &str) -> FnCall {
    let mut it = expr.split_whitespace();

    let func = match it.next() {
        Some("cos") => Function::Cos,
        Some("sin") => Function::Sin,
        Some("tan") => Function::Tan,
        Some("exp") => Function::Exp,
        Some("ln")=> Function::Ln,
        _ => panic!()
    };

    let arg = Box::new(parse_expr(it.collect::<Vec<&str>>().join(" ").as_str()));

    FnCall { func, arg }
}

trait Differentiable {
    fn diff(&self) -> Box<Expression>;
}

impl Differentiable for Expression {
    fn diff(&self) -> Box<Expression> {
        use Expression::*;
    
        match self {
            Nb(_) => Box::new(Nb(0.0)),
            X => Box::new(Nb(1.0)),
            Par(p) => p.diff()
        }        
    }

}

impl Differentiable for Parenthese {
    fn diff(&self) -> Box<Expression> {
        match self {
            Parenthese::Fn(f) => f.diff(),
            Parenthese::Op(op) => op.diff()
        }
    }
}

impl Differentiable for FnCall {
    fn diff(&self) -> Box<Expression> {
        use Parenthese::*;
        use Expression::{Par, Nb};
        use Function::*;
        use Operator::*;
        
        let arg = self.arg.clone();
        
        let left_arg = arg.clone().diff();
    
        let right_arg = Box::new(Par(match self.func {
            Sin => Fn(FnCall { func: Cos, arg }),
            Cos => Op(Operation {
                operator: Product,
                left_arg: Box::new(Par(Fn(FnCall { func: Sin, arg }))),
                right_arg: Box::new(Nb(-1.0))
            }),
            Exp => Fn(FnCall { func: Exp, arg }),
            Tan => Op(Operation { 
                operator: Sum, 
                left_arg: Box::new(Nb(1.0)), 
                right_arg: Box::new(Par(Op(Operation { 
                    operator: Exponentiation, 
                    left_arg: Box::new(Par(Fn(FnCall { func: Tan, arg }))), 
                    right_arg: Box::new(Nb(2.0)) 
                }))) 
            }),
            Ln => Op(Operation { operator: Division, left_arg: Box::new(Nb(1.0)), right_arg: arg })
        }));

        Box::new(Par(Op(Operation { operator: Product, left_arg, right_arg })))
    }

}

impl Differentiable for Operation {
    fn diff(&self) -> Box<Expression> {
        use Operator::*;
        use Expression::*;
        use Parenthese::Op;

        let u = &self.left_arg;
        let v = &self.right_arg;

        Box::new(Par(Op(match self.operator {
            Sum => Operation { operator: Sum, left_arg: u.diff(), right_arg: v.diff() },
            Difference => Operation { operator: Difference, left_arg: u.diff(), right_arg: v.diff() },
            Product => Operation { 
                operator: Sum, 
                left_arg: Box::new(Par(Op(Operation { operator: Product, left_arg: u.diff(), right_arg: v.clone() }))), 
                right_arg: Box::new(Par(Op(Operation { operator: Product, left_arg: v.diff(), right_arg: u.clone() }))) 
            },
            Division => Operation { 
                operator: Division,
                left_arg: Box::new(Par(Op(Operation {
                    operator: Difference, 
                    left_arg: Box::new(Par(Op(Operation { operator: Product, left_arg: u.diff(), right_arg: v.clone() }))), 
                    right_arg: Box::new(Par(Op(Operation { operator: Product, left_arg: v.diff(), right_arg: u.clone() }))) 
                }))),
                right_arg: Box::new(Par(Op(Operation { operator: Exponentiation, left_arg: v.clone(), right_arg: Box::new(Nb(2.0)) }))) 
            },
            Exponentiation => diff_pow(self.left_arg.clone(), self.right_arg.clone())
        })))
    }

}

// this is where all the weird way of differentiating exponents go
fn diff_pow(u: Box<Expression>, v: Box<Expression>) -> Operation {
    use Operator::*;
    use Expression::{ Nb, Par };
    use Parenthese::Op;

    if let Nb(n) = *v {
        return Operation { operator: Product, left_arg: Box::new(Nb(n)), right_arg: Box::new(Par(Op(Operation { 
            operator: Product, 
            left_arg: u.diff(),
            right_arg: Box::new(Par(Op(Operation { 
                operator: Exponentiation, 
                left_arg: u.clone(), 
                right_arg: Box::new(Nb(n - 1.0)) 
            }))) 
        })))}
    }

    if let Nb(n) = *u {
        return Operation {
            operator: Product,
            left_arg: v.diff(),
            right_arg: Box::new(Par(Op(Operation { 
                operator: Exponentiation, 
                left_arg: u, 
                right_arg: v.clone()
            }))) 
        }
    }

    panic!()
}


trait Simplifiable {
    fn simplify(self) -> Box<Expression>;
}

impl Simplifiable for Expression {
    fn simplify(self) -> Box<Expression> {
        use Expression::Par;

        match self {
            Par(p) => p.simplify(),
            _ => Box::new(self)
        }
    }
}

impl Simplifiable for Parenthese {
    fn simplify(self) -> Box<Expression> {
        use Parenthese::*;
        use Expression::Par;

        match self {
            Fn(FnCall { func, arg }) => Box::new(Par(Fn(FnCall { func , arg: arg.simplify() }))),
            Op(op) => op.simplify()
        }
    }
}

impl Simplifiable for Operation {
    fn simplify(self) -> Box<Expression> {
        use Operator::*;
        use Expression::{ Par, Nb };

        let left_arg = self.left_arg.simplify();
        let right_arg = self.right_arg.simplify();

        if let (Nb(a), Nb(b)) = (*left_arg.clone(), *right_arg.clone()) {
            return match self.operator {
                Product => Box::new(Nb(a * b)),
                Sum => Box::new(Nb(a + b)),
                Division => Box::new(Nb(a / b)),
                Difference => Box::new(Nb(a - b)),
                Exponentiation => Box::new(Nb(a.powf(b)))
            }
        }

        match self.operator {
            Product if *left_arg == Nb(0.0) => left_arg,
            Product if *right_arg == Nb(0.0) => right_arg,
            Product if *left_arg == Nb(1.0) => right_arg,
            Product if *right_arg == Nb(1.0) => left_arg,
            // Sum if *right_arg == Nb(0.0) => left_arg,
            // Sum if *left_arg == Nb(0.0) => right_arg,
            Exponentiation if *right_arg == Nb(0.0) => Box::new(Nb(1.0)),
            Exponentiation if *right_arg == Nb(1.0) => left_arg,
            operator => Box::new(Par(Parenthese::Op(Operation { 
                operator, 
                left_arg, 
                right_arg 
            })))
        }
    }
}


trait ParseBack {
    fn parse_back(self) -> String;
}

impl ParseBack for Expression {
    fn parse_back(self) -> String {
        use Expression::*;

        match self {
            Nb(n) => n.to_string(),
            X => "x".to_string(),
            Par(p) => p.parse_back()
        }
    }
}

impl ParseBack for Parenthese {
    fn parse_back(self) -> String {
        use Parenthese::*;

        match self {
            Fn(fncall) => fncall.parse_back(), 
            Op(op) => op.parse_back()
        }
    }
}

impl ParseBack for FnCall {
    fn parse_back(self) -> String {
        use Function::*;

        match self.func {
            Cos => format!("(cos {})", self.arg.parse_back()),
            Sin => format!("(sin {})", self.arg.parse_back()),
            Exp => format!("(exp {})", self.arg.parse_back()),
            Tan => format!("(tan {})", self.arg.parse_back()),
            Ln  => format!("(ln {})", self.arg.parse_back())
        }
    }
}

impl ParseBack for Operation {
    fn parse_back(self) -> String {
        use Operator::*;

        match self.operator {
            Sum => format!("(+ {} {})", self.left_arg.parse_back(), self.right_arg.parse_back()),
            Difference => format!("(- {} {})", self.left_arg.parse_back(), self.right_arg.parse_back()),
            Product => format!("(* {} {})", self.left_arg.parse_back(), self.right_arg.parse_back()),
            Division => format!("(/ {} {})", self.left_arg.parse_back(), self.right_arg.parse_back()),
            Exponentiation => format!("(^ {} {})", self.left_arg.parse_back(), self.right_arg.parse_back())
        }
    }
}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
fn main() {
    println!("{:?}", parse_expr("(sin x)").diff())
}

#[cfg(test)]
mod tests {
    use super::*;
    
#[test]
fn test_fixed() {
    assert_eq!(diff("5"), "0");
    assert_eq!(diff("x"), "1");
    assert_eq!(diff("5"), "0");
    assert_eq!(diff("(+ x x)"), "2");
    assert_eq!(diff("(- x x)"), "0");
    assert_eq!(diff("(* x 2)"), "2");
    assert_eq!(diff("(/ x 2)"), "0.5");
    assert_eq!(diff("(^ x 2)"), "(* 2 x)");
    assert_eq!(diff("(cos x)"), "(* -1 (sin x))");
    assert_eq!(diff("(sin x)"), "(cos x)");
    
    // assert_eq!(diff("(tan x)"), "(+ 1 (^ (tan x) 2))");
    
    let result = diff("(tan x)");
    assert!(
        result == "(+ 1 (^ (tan x) 2))" || result == "(/ 1 (^ (cos x) 2))",
        "expected (+ 1 (^ (tan x) 2)) or (/ 1 (^ (cos x) 2))"
    );
    
    assert_eq!(diff("(exp x)"), "(exp x)");
    assert_eq!(diff("(ln x)"), "(/ 1 x)");
    assert_eq!(diff("(+ x (+ x x))"), "3");
    assert_eq!(diff("(- (+ x x) x)"), "1");
    assert_eq!(diff("(* 2 (+ x 2))"), "2");
    assert_eq!(diff("(/ 2 (+ 1 x))"), "(/ -2 (^ (+ 1 x) 2))");
    assert_eq!(diff("(cos (+ x 1))"), "(* -1 (sin (+ x 1)))");

    let result = diff("(cos (* 2 x))");
    assert!(
        result == "(* 2 (* -1 (sin (* 2 x))))"
            || result == "(* -2 (sin (* 2 x)))"
            || result == "(* (* -1 (sin (* 2 x))) 2)",
        "expected (* 2 (* -1 (sin (* 2 x)))) or (* -2 (sin (* 2 x))) or (* (* -1 (sin (* 2 x))) 2)"
    );

    assert_eq!(diff("(sin (+ x 1))"), "(cos (+ x 1))");
    assert_eq!(diff("(sin (* 2 x))"), "(* 2 (cos (* 2 x)))");
    
    // assert_eq!(diff("(tan (* 2 x))"), "(* 2 (+ 1 (^ (tan (* 2 x)) 2)))");
    
    let result = diff("(tan (* 2 x))");
    assert!(
        result == "(* 2 (+ 1 (^ (tan (* 2 x)) 2)))"
            || result == "(* 2 (/ 1 (^ (cos (* 2 x)) 2)))",
        "expected (* 2 (+ 1 (^ (tan (* 2 x)) 2))) or (* 2 (/ 1 (^ (cos (* 2 x)) 2)))"
    );
    
    assert_eq!(diff("(exp (* 2 x))"), "(* 2 (exp (* 2 x)))");
    assert_eq!(diff(&diff("(sin x)")), "(* -1 (sin x))");
    assert_eq!(diff(&diff("(exp x)")), "(exp x)");

    let result = diff(&diff("(^ x 3)"));
    assert!(result == "(* 3 (* 2 x))" || result == "(* 6 x)",
        "expected (* 3 (* 2 x)) or (* 6 x)");
}
}
