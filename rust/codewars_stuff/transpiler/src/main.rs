// * source syntax
// function ::= expression "(" [parameters] ")" [lambda]
//            | expression lambda

// expression ::= nameOrNumber
//              | lambda

// parameters ::= expression ["," parameters]

// lambdaparam ::= nameOrNumber ["," lambdaparam]
// lambdastmt  ::= nameOrNumber [lambdastmt]

// lambda ::= "{" [lambdaparam "->"] [lambdastmt] "}"

// * target syntax
// function ::= expression "(" [parameters] ")"

// expression ::= nameOrNumber
//              | lambda

// parameters ::= expression ["," parameters]

// lambdaparam ::= nameOrNumber ["," lambdaparam]
// lambdastmt  ::= nameOrNumber ";" [lambdastmt]

// lambda ::= "(" [lambdaparam] "){" [lambdastmt] "}"

#[derive(Debug)]
struct FnCall {
    expression: Expression,
    parameters: Vec<Expression>,
}

#[derive(Debug)]
enum Expression {
    NameOrNumber(String),
    Lambda(Lambda)
}

#[derive(Debug)]
struct Lambda {
    arguments: Vec<String>,
    core: Vec<String>
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum Token {
    NameOrNumber(String),
    SpecialChar(char),
    Arrow
}

fn tokenize<'a>(program : &'a str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    
    let mut iter = program.chars().peekable();
    loop {
        match iter.peek() {
            Some(&c) => match c {
                'a'..='z'|'A'..='Z'|'_' => {
                    let mut tmp = String::new();
                    while iter.peek().is_some() && (iter.peek().unwrap().is_ascii_alphanumeric() || *iter.peek().unwrap() == '_') {
                        tmp.push(iter.next().unwrap());
                    }
                    tokens.push(Token::NameOrNumber(tmp));
                },
                '0'..='9' => {
                    let mut tmp = String::new();
                    while iter.peek().is_some() && iter.peek().unwrap().is_digit(10)  {
                        tmp.push(iter.next().unwrap());
                    }
                    tokens.push(Token::NameOrNumber(tmp));
                    if let Some(true) = iter.peek().map(|c| c.is_ascii_alphabetic()) {
                        return Err("Hugh?".to_string())
                    }
                },
                ' '|'\n' => { iter.next(); },
                '-' => {
                    iter.next();
                    match iter.peek() {
                        Some('>') => { tokens.push(Token::Arrow); iter.next(); },
                        _ => { tokens.push(Token::SpecialChar('-')); }
                    }
                }
                _ => {
                    tokens.push(Token::SpecialChar(iter.next().unwrap()));
                },
            },
            None => break
        }
    }

    Ok(tokens)
}

fn parse_lambda(tokens: Vec<Token>) -> Result<Lambda, String> {
    let mut it = tokens.iter();
    
    let mut arguments = Vec::new();
    if tokens.iter().any(|token| Token::Arrow == *token) {
        arguments = parse_args(
            it.by_ref()
            .take_while(|token| **token != Token::Arrow)
            .cloned()
            .collect()
        )?;
        if arguments.is_empty() {
            return Err("Hugh".to_string())
        }
    }

    let core = it.map(|token| match token {
        Token::NameOrNumber(n) => Ok(n.to_string()),
        _ => Err("Hugh?".to_string()) 
    }).collect::<Result<Vec<String>, String>>()?;
    
    Ok(Lambda { arguments, core })
}

fn parse_params(tokens: Vec<Token>) -> Result<Vec<Expression>, String> {
    let mut it = tokens.iter().peekable();
    let mut parameters = Vec::new();

    if !tokens.is_empty() {
        loop {
            let next_param = match it.next() {
                Some(Token::SpecialChar('{')) => {
                    let res = Expression::Lambda(
                        parse_lambda(
                            it.by_ref()
                                .take_while(|token| **token != Token::SpecialChar('}'))
                                .cloned()
                                .collect()
                    )?);
                    if it.peek().is_none() && tokens[tokens.len() - 1] != Token::SpecialChar('}') {
                        return Err("Hugh?".to_string())
                    }
                    res
                },
                Some(Token::NameOrNumber(n)) => Expression::NameOrNumber(n.to_string()),
                _ => return Err("Hugh?".to_string())
            };
            
            parameters.push(next_param);
            
            match it.next() {
                Some(Token::SpecialChar(',')) => (),
                None => break,
                Some(_) => return Err("Hugh?".to_string())
            }
        }
    }

    Ok(parameters)
}

fn parse_args(tokens: Vec<Token>) -> Result<Vec<String>, String> {
    let mut it = tokens.iter();
    let mut args = Vec::new();

    if !tokens.is_empty() {
        loop {
            args.push(match it.next() {
                Some(Token::NameOrNumber(n)) => n.to_string(),
                _ => return Err("Hugh?".to_string())
            });
            
            match it.next() {
                Some(Token::SpecialChar(',')) => (),
                None => break,
                Some(_) => return Err("Hugh?".to_string())
            }
        }
    }

    Ok(args)
}

fn source_to_ast(expr: &str) -> Result<FnCall, String> {
    let tokens = tokenize(expr)?;
    let mut it = tokens.iter().peekable();

    let expression = match it.next() {
        Some(Token::SpecialChar('{')) => Expression::Lambda(
            parse_lambda(
                it.by_ref()
                    .take_while(|token| **token != Token::SpecialChar('}'))
                    .cloned()
                    .collect()
            )?
        ),
        Some(Token::NameOrNumber(n)) => Expression::NameOrNumber(n.to_string()),
        _ => return Err("Hugh?".to_string())
    };

    let mut parameters = Vec::new(); 
    
    match it.next() {
        Some(Token::SpecialChar('(')) => {
            parameters = parse_params(
                it.by_ref()
                    .take_while(|token| **token != Token::SpecialChar(')'))
                    .cloned()
                    .collect()
            )?;
            
            if it.peek().is_none() && tokens[tokens.len() - 1] != Token::SpecialChar(')') {
                return Err("Hugh?".to_string())
            }
            

            match it.next() {
                Some(Token::SpecialChar('{')) => { 
                    parameters.push(Expression::Lambda(
                        parse_lambda(
                            it.by_ref()
                                .take_while(|token| **token != Token::SpecialChar('}'))
                                .cloned()
                                .collect()
                        )?
                    ));
                    if it.peek().is_none() && tokens[tokens.len() - 1] != Token::SpecialChar('}') {
                        return Err("Hugh?".to_string())
                    }
                },
                Some(_) => return Err("Hugh?".to_string()),
                None => ()
            }
        },
        Some(Token::SpecialChar('{')) => { 
            parameters.push(Expression::Lambda(
                parse_lambda(
                    it.by_ref()
                        .take_while(|token| **token != Token::SpecialChar('}'))
                        .cloned()
                        .collect()
                )?
            ));
            if it.peek().is_none() && tokens[tokens.len() - 1] != Token::SpecialChar('}') {
                return Err("Hugh?".to_string())
            }
                    
        },
        _ => return Err("Hugh?".to_string()) 
    }

    if it.next().is_some() {
        return Err("Hugh?".to_string())
    }    

    Ok(FnCall { expression, parameters })
}

trait ToTarget {
    fn to_target(self) -> Result<String, String>;
}

impl ToTarget for FnCall {
    fn to_target(self) -> Result<String, String> {
        Ok(format!(
            "{}({})",
            self.expression.to_target()?,
            self.parameters.into_iter().map(|param| param.to_target()).collect::<Result<Vec<String>, String>>()?.join(",")
        ))
    }
}

impl ToTarget for Expression {
    fn to_target(self) -> Result<String, String> {
        match self {
            Expression::NameOrNumber(n) => Ok(n.to_string()),
            Expression::Lambda(l) => l.to_target(),
        }
    }
}

impl ToTarget for Lambda {
    fn to_target(self) -> Result<String, String> {
        Ok(format!(
            "({}){{{}}}",
            self.arguments.join(","),
            self.core.into_iter().map(|n| format!("{n};")).collect::<String>()
        ))
        
    }
}

fn transpile(expr: &str) -> Result<String, String> {
    source_to_ast(expr)?.to_target()
}

fn main() {
    println!("{:?}", source_to_ast("{x, y -> x y}()"));
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod sample_tests {
    use super::transpile;

    fn accepts(expr: &str, expected: &str) {
        do_test(expr, Ok(expected.to_string()));
    }

    fn rejects(expr: &str) {
        do_test(expr, Err("Hugh?".to_string()));
    }

    fn do_test(expr: &str, expected: Result<String,String>) {
        assert_eq!(transpile(expr), expected, "\nYour result (left) did not match expected output (right) for the expression:\n{expr:?}");
    }
    
    #[test]
    fn transpiles_very_simple_expressions() {
        accepts("call()", "call()");
        accepts("_call(a,b)", "_call(a,b)");
        accepts("042()", "042()");
    }
    
    #[test]
    fn rejects_trivial_mistakes() {
        rejects("name%1&*");
        rejects("abc9_(");
        rejects("f(42a)");
        rejects("call");
        rejects("f()()");
    }
    
    #[test]
    fn handles_whitespace() {
        accepts("call   (    jim ,      my )", "call(jim,my)");
        accepts("\n \n  1(  \n )\n", "1()");
    }
    
    #[test]
    fn handles_lambda_in_calls() {
        accepts("call({\n})", "call((){})");
        accepts("call(a, b, {})", "call(a,b,(){})");
        rejects("f({)");
        rejects("f(})");
    }
    
    #[test]
    fn handles_lambda_outside_calls() {
        accepts("{}(x)", "(){}(x)");
        accepts("call(a,b){}", "call(a,b,(){})");
        accepts("{}{}", "(){}((){})");
        rejects("{}");
        rejects("f(){");
    }
    
    #[test]
    fn handles_lambda_variations() {
        accepts("call{666}", "call((){666;})");
        accepts("{p ->}()", "(p){}()");
        accepts("{p,  5 ->}()", "(p,5){}()");
        accepts("call({a,b,c->d e}){\n}", "call((a,b,c){d;e;},(){})");
        accepts("{a b c}{d->e\nf}", "(){a;b;c;}((d){e;f;})");
        accepts("{x\n->y}(666){4,2->0}", "(x){y;}(666,(4,2){0;})");
        rejects("{a,b->c}");
        rejects("f({p,t,})");
        rejects("f({a b -> c})");
        rejects("{u,w,v -> x,y}{}");
        rejects("{}{1a 2}")
    }
}
