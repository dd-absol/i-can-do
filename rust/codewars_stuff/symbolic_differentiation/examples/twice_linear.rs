fn main() {
    println!("hi mom!");
}

fn dbl_linear(n: u32) -> u32{
    0
}

#[cfg(test)]
mod tests {
    use super::dbl_linear;
    fn testing(n: u32, exp: u32) -> () {
        assert_eq!(dbl_linear(n), exp)
    }
    
    #[test]
    fn basics_dbl_linear() {
        testing(10, 22);
        testing(20, 57);
        testing(30, 91);
        testing(50, 175);
        testing(100, 447);
    }
}