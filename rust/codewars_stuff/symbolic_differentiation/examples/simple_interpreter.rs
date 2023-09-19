use std::collections::HashMap;
use std::vec::IntoIter;

#[allow(unused)]
fn main() {
    println!("{:?}", Interpreter::new().input("1 + 1"))
}

#[derive(Debug, Clone)]
enum Part {
    FnName(String),
    Op(Operator),
    HV(HasValue),
    Assignement
}

#[derive(Debug, Clone)]
enum HasValue {
    Id(String),
    Nb(f32),
    Parentheses(Box<Expression>),
}

type Expression = Vec<Part>;

//ascending priority order
// TODO refactor Assignement and arrow into main parts
#[derive(PartialEq, Debug, Clone)]
enum Operator {
    Add(Additive),
    Mult(Multiplicative),
}

#[derive(PartialEq, Debug, Clone)]
enum Additive {
    Sum,
    Difference,
}

#[derive(PartialEq, Debug, Clone)]
enum Multiplicative {
    Product,
    Division,
    Mod
}

// TODO use &str instead of String pretty much everywhere
struct Interpreter {
    variables: HashMap<String, f32>,
    functions: HashMap<String, Function>
}

#[derive(Clone)]
struct Function {
    parameters: Vec<String>,
    expression: Expression
}

impl Interpreter {
    fn new() -> Interpreter {
        Interpreter {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }
    
    fn input(&mut self, input: &str) -> Result<Option<f32>, String> {
        if input.chars().all(|c| c == ' ') {
            return Ok(None)
        }
    
    
        match input.get(0..3) {
            Some("fn ") => { self.parse_fn(&input[3..])?; Ok(None) },
            _ => {
                let (res, mut trail) = self.evaluate_expression(dbg!(Self::parse_expression(&self.functions, input)?).into_iter())?;

                if let Some(_) = trail.next() {
                    return Err("two much stuff".to_string())
                }

                Ok(Some(res))
            }
        }

    }

    fn evaluate_expression(&mut self, mut it: IntoIter<Part>) -> Result<(f32, IntoIter<Part>), String> {
        use Part::*;

        match it.next() {
            Some(FnName(fname)) => self.call_function(fname, it),
            Some(HV(HasValue::Id(id))) => match it.clone().peekable().peek() {
                Some(Assignement) => {
                    it.next();

                    let (res, trail) = self.evaluate_expression(it)?;
                    
                    Ok((self.assign(id, res)?, trail))
                }
                _ => { 
                    if let Some(left) = self.read_var(&id) {
                        return self.do_math(left, it) 
                    };
                    
                    Err(format!("use of unassigned var {}", id))
                }
            },
            Some(HV(hv)) => { let left = self.evaluate_hv(hv)?; self.do_math(left, it) }
            _ => Err(format!("operator at beginning of expr"))
        }

    }

    // returns the trail of instructions
    fn call_function(&mut self, name: String, mut it: IntoIter<Part>) -> Result<(f32, IntoIter<Part>), String> {

        let f = match self.functions.get(&name) {
            Some(fun) => fun.clone(),
            None => return Err("calling undefined function".to_string())
        };

        let mut scope = Interpreter::new();

        for param in f.parameters.into_iter() {
            let (value, trail) = self.evaluate_expression(it)?;

            scope.assign(param, value)?;

            it = trail;
        }

        Ok((scope.evaluate_expression(f.expression.into_iter())?.0, it))
    }



    // math stuff
    fn do_math(&mut self, left: f32, mut it: IntoIter<Part>) -> Result<(f32, IntoIter<Part>), String> {
        use Operator::*;
        use Part::*;

        let mut peek_it = it.clone().peekable();
        
        match peek_it.peek() {
            Some(Op(Add(add))) => { it.next(); self.add_math(add.clone(), left, it) }
            Some(Op(Mult(mult))) => { it.next(); self.mult_math(mult.clone(), left, it) }
            _ => Ok((left, it)),
        }
    }
    
    fn assign(&mut self, variable: String, value: f32) -> Result<f32, String> {
        // we check the name isn't a function
        if self.functions.keys().any(|key| *key == variable) {
            return Err("cannot overwrite function with variable".to_string())
        }

        self.variables.insert(variable, value);

        Ok(value)
    }

    fn add_math(&mut self, op: Additive, left: f32, mut it: IntoIter<Part>) -> Result<(f32, IntoIter<Part>), String> {
        let (next_left, trail) = match it.next() {
            Some(Part::HV(hv)) => (self.evaluate_hv(hv)?, it),
            Some(Part::FnName(f)) => self.call_function(f, it)?,
            _ => return Err("no right operand in add".to_string())
        };

        let (right, trail) = match op {
            Additive::Sum => self.do_math(next_left, trail)?,
            Additive::Difference => self.do_math(-next_left, trail)?
        };

        Ok((left + right, trail))
    }

    fn mult_math(&mut self, op: Multiplicative, left: f32, mut it: IntoIter<Part>) -> Result<(f32, IntoIter<Part>), String> {
        let (right, trail) = match it.next() {
            Some(Part::HV(hv)) => (self.evaluate_hv(hv)?, it),
            Some(Part::FnName(f)) => self.call_function(f, it)?,
            _ => return Err("no right operand in mult".to_string())
        };

        match op {
            Multiplicative::Product => self.do_math(left * right, trail),
            Multiplicative::Division => self.do_math(left / right, trail),
            Multiplicative::Mod => self.do_math(left % right, trail),
        }
    }



    // parsing stuff
    fn parse_expression(functions: &HashMap<String, Function>, expression: &str) -> Result<Expression, String> {
        // handles parentheses
        let mut layer_depth = 0_i32;
        let mut layer_evolution = vec![0];
        layer_evolution.append(&mut expression.chars().filter_map(|c| match c {
            '(' => { layer_depth += 1; Some(layer_depth) },
            ')' => { layer_depth -= 1; Some(layer_depth) },
            _ => None
        }).collect::<Vec<i32>>());

        if layer_depth != 0 || layer_evolution.iter().any(|depth| *depth < 0) {
            return Err("bad parentheses".to_string())
        }

        let mut parser_state = Vec::new();
        let mut par_split = Vec::new();
        let mut layer = 0;
        
        //actual parsing starts here
        for c in expression.chars() {
            if c == '(' {
                if layer == 0 {
                    par_split.push(parser_state.iter().collect::<String>());
                    parser_state.clear();
                } else {
                    parser_state.push(c);
                }
                layer += 1;
            } else if c == ')' {
                layer -= 1;
                if layer == 0 {
                    par_split.push(parser_state.iter().collect::<String>());
                    parser_state.clear();
                } else {
                    parser_state.push(c);
                }
            } else {
                parser_state.push(c)
            }
        }

        par_split.push(parser_state.iter().collect::<String>());

        dbg!(par_split).into_iter().scan(true, |state, factor| {
            *state = !*state;

            if *state {
                match Self::parse_expression(functions, factor.as_str()) {
                    Ok(expr) => return Some(vec![Ok(Part::HV(HasValue::Parentheses(Box::new(expr))))]),
                    Err(e) => return Some(vec![Err(e)])
                }
            }

            Some(Self::parse_out_of_par(functions, factor.as_str()))
        }).flatten().collect()        
        
    }

    fn parse_out_of_par(functions: &HashMap<String, Function>, factor: &str) -> Vec<Result<Part, String>> {
        use Part::*;

        factor.split_whitespace().map(|part| match part.chars().next() {
            Some('+' | '-' | '*' | '/' | '%') => Self::parse_op(part).map(|op| Op(op)),
            Some('=') => Self::parse_ass(part),
            Some(letter) if letter.is_ascii_alphabetic() => Self::parse_id(part).map(|id| match functions.get(id) {
                Some(_) => FnName(id.to_string()),
                None => HV(HasValue::Id(id.to_string()))
            }),
            Some(digit) if digit.is_digit(10) => Self::parse_nb(part).map(|nb| HV(HasValue::Nb(nb))),
            _ => return Err("wrong first character for anything".to_string())
        }).collect()
    }

    // TODO make functions with no parameteres into variables
    // TODO check for variable name (cannot overwrite a var)
    fn parse_fn(&mut self, input: &str) -> Result<(), String> {
        let mut it = input.split_whitespace();
        
        let name = match it.next() {
            Some(part) => Self::parse_id(part),
            None => return Err("only fn".to_string())
        }?;

        // mandatory check
        if self.variables.keys().any(|key| *key == name) {
            return Err("cannot overwrite function with variable".to_string())
        }
        
        let parameters = it.take_while(|part| *part != "=>").map(|part|
            Self::parse_id(part).map(|id| id.to_string())
        ).collect::<Result<Vec<String>, String>>()?;

        for i in 0..(parameters.len()) {
            if parameters.iter().skip(i + 1).any(|p| parameters[i] == *p) {
                return Err(format!("twice same parameter {:?}", parameters[i]))
            }
        }

        let expression = Self::parse_expression(&self.functions, input.split_whitespace().skip_while(|part| *part != "=>").skip(1).collect::<Vec<&str>>().join(" ").as_str())?;

        if expression.iter().any(|part| match part {
                Part::HV(HasValue::Id(id)) => parameters.iter().all(|par| id != par),
                _ => false
            }
        ) { return Err("invalid parameter in function declaration".to_string()) }

        self.functions.insert(name.to_string(), Function {
            parameters,
            expression
        });

        Ok(())
    }
    
    fn parse_op(part: &str) -> Result<Operator, String> {
        use Operator::*;
        use Additive::*;
        use Multiplicative::*;

        let mut it = part.chars();

        let res = match it.next() {
            Some('+') => Add(Sum),
            Some('-') => Add(Difference),
            Some('/') => Mult(Division),
            Some('*') => Mult(Product),
            Some('%') => Mult(Mod),
            _ => return Err("unreachable".to_string())
        };

        match it.next() {
            None => Ok(res),
            _ => Err("too long for op".to_string())
        }
    }

    fn parse_ass(part: &str) -> Result<Part, String> {
        match part {
            "=" => Ok(Part::Assignement),
            _ => Err("whitespace after = sign problem".to_string())
        }
    }

    fn parse_id(part: &str) -> Result<&str, String> {
        if part == "fn" {
            return Err("trying to use fn as an identifier".to_string())
        }

        match part.chars().all(|c| c.is_ascii_alphabetic() || c == '_' || c.is_digit(10)) {
            true => Ok(part),
            false => Err("bad characters".to_string())
        }
    }

    fn parse_nb(part: &str) -> Result<f32, String> {
        match part.parse::<f32>() {
            Ok(nb) => Ok(nb),
            Err(_) => Err("failed to parse".to_string())
        }
    }
    // end of parsing stuff


    fn evaluate_hv(&mut self, value: HasValue) -> Result<f32, String> {
        match value {
            HasValue::Nb(nb) => Ok(nb),
            HasValue::Id(id) => match self.read_var(&id) {
                Some(value) => Ok(value),
                None => Err(format!("{:?} not affected", id))
            },
            HasValue::Parentheses(expr) => match self.evaluate_expression(expr.into_iter()) {
                Ok((res, mut trail)) => { if matches!(trail.next(), None) { Ok(res) } else { Err("parentheses does not finish evaluating".to_string()) } },
                Err(e) => Err(e)
            }
        }
    }

    fn read_var(&self, id: &String) -> Option<f32> {
        self.variables.get(id).copied()
    }

}
