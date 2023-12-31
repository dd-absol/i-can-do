use std::collections::{VecDeque, HashMap};

fn boolfuck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let mut pointer = BoolfuckPointer::new(input);
    
    pointer.read(code);

    pointer.return_output()
}

struct BoolfuckPointer {
    storage: VecDeque<bool>,
    tape_pointer: usize,
    instr_position: usize,
    output: Vec<bool>,
    matching_par_stack: HashMap<usize, usize>,
    input_stack: Vec<bool>
}

impl BoolfuckPointer {
    fn new(input: Vec<u8>) -> Self {
        let bit_input = input.into_iter().map(|byte|
            (0..8).scan(byte, |acc, _| {
                let res = *acc % 2;
                *acc = *acc / 2;
                return Some(res == 1)
            })
        ).flatten().collect::<Vec<bool>>();

        BoolfuckPointer { 
            storage: VecDeque::from([false; u8::MAX as usize]), 
            tape_pointer: (u8::MAX/2)as usize, 
            instr_position: 0,
            output: Vec::new(), 
            input_stack: bit_input.into_iter().rev().collect(),
            matching_par_stack: HashMap::new()
        }
    }

    fn return_output(self) -> Vec<u8> {
        let mut it = self.output.into_iter().peekable();
        let mut res = Vec::new();
        let mut not_done = true;

        while not_done && matches!(it.peek(), Some(_)) {
            res.push((0..8).fold(0, |acc, i| match it.next() {
                Some(true) => acc + 2u8.pow(i),
                Some(false) => acc,
                None => { not_done = false; acc }
            }))
        }

        res
    }

    //+,;<>[]
    fn read(&mut self, code: &str) -> () {
        let mut max_iter = 0;

        while let Some(instr) = code.chars().nth(self.instr_position) {
            //if max_iter == 300000 { break }
            match instr {
                '+' => { self.storage[self.tape_pointer] = !self.storage[self.tape_pointer]; },
                ',' => { self.storage[self.tape_pointer] = match self.input_stack.pop() {
                    Some(bit) => bit,
                    None => false,
                }; },
                ';' => { self.output.push(self.storage[self.tape_pointer]); },
                '<' => match self.tape_pointer {
                    0 => { self.storage.push_front(false); },
                    _ => { self.tape_pointer -= 1 }
                },
                '>' => { 
                    self.tape_pointer += 1;
                    match self.storage.get(self.tape_pointer) {
                        Some(_) => (),
                        None => { self.storage.push_back(false); }
                    }
                },
                '[' => { if !self.storage[self.tape_pointer] { 
                    self.instr_position = *self.matching_par_stack.entry(self.instr_position).or_insert(find_matching_forward(code, self.instr_position))
                } },
                ']' => { if self.storage[self.tape_pointer] { 
                    self.instr_position = *self.matching_par_stack.entry(self.instr_position).or_insert(find_matching_backward(code, self.instr_position)) 
                } },
                _ => ()
            }
            max_iter += 1;
            self.instr_position += 1;
        }
        
        dbg!(max_iter);

    }

}

fn find_matching_forward(code: &str, position: usize) -> usize {
    let mut depth = 0;

    code.chars().enumerate().skip(position + 1).find(|(_, c)| match c {
        '[' => { depth += 1; false }
        ']' if depth == 0 => true,
        ']' => { depth -= 1; false },
        _ => false
    }).unwrap().0
}

fn find_matching_backward(code: &str, position: usize) -> usize {
    let mut depth = 0;

    code.chars().enumerate().collect::<Vec<(usize, char)>>().into_iter().rev().skip(code.len() - position).find(|(_, c)| match c {
        ']' => { depth += 1; false }
        '[' if depth == 0 => true,
        '[' => { depth -= 1; false },
        _ => false
    }).unwrap().0
}

fn main() {
    println!("{:?}", boolfuck(">,>,>,>,>,>,>,>,<<<<<<<<>>>>>>>>>>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>>>>>>>>>>>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]<<<<<<<<<<<<<<<<<<<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]<<<<<<<<<<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>>>>>>>>>>>>>>>>>>>>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]<<<<<<<<<<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>>>>>>>>>>>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]<<<<<<<<<<<<<<<<<<<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]<<<<<<<<<<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>;>;>;>;>;>;>;>;<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>;>;>;>;>;>;>;>;<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>>>>>>>>>>>>>>>>>>>>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>>>>>>>>>>>;>;>;>;>;>;>;>;<<<<<<<<>>>>>>>>>>;>;>;>;>;>;>;>;<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]<<<<<<<<<<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>>>>>>>>>>>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]<<<<<<<<<<<<<<<<<<<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]<<<<<<<<<<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]<<<<<<<<<>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]<<<<<<<<<<<<<<<<<<<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]", vec![10]));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_test_cases() {
        // Hello World Program taken from the official website
        assert_eq!(boolfuck(";;;+;+;;+;+;+;+;+;+;;+;;+;;;+;;+;+;;+;;;+;;+;+;;+;+;;;;+;+;;+;;;+;;+;+;+;;;;;;;+;+;;+;;;+;+;;;+;+;;;;+;+;;+;;+;+;;+;;;+;;;+;;+;+;;+;;;+;+;;+;;+;+;+;;;;+;+;;;+;+;+;", Vec::new()), b"Hello, world!\n", "Your interpreter did not work with the code example provided on the official website");
        // Echo until byte(0) encountered
        assert_eq!(boolfuck(">,>,>,>,>,>,>,>,>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>;>;>;>;>;>;>;>;>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>,>,>,>,>,>,>,>,>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]", b"Codewars\x00".to_vec()), b"Codewars");
        // Two numbers multiplier
        assert_eq!(boolfuck(">,>,>,>,>,>,>,>,>>,>,>,>,>,>,>,>,<<<<<<<<+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]>[>]+<[+<]>>>>>>>>>[+]>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<<<<<<<<<<+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<<<<<<<<<<<<<<<<<<[>]+<[+<]>>>>>>>>>[+]>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]<<<<<<<<<<<<<<<<<<+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>>>>>>>>>>>;>;>;>;>;>;>;>;<<<<<<<<", vec![8, 9]), vec![72]);
    }
}