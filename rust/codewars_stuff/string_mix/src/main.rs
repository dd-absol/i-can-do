fn mix(s1: &str, s2: &str) -> String {
    let mut res = ('a'..='z').into_iter().rev().filter_map(|letter| {
        let count1 = s1.chars().filter(|c| *c == letter).count();
        let count2 = s2.chars().filter(|c| *c == letter).count();

        if count1 < 2 && count2 < 2 { return None }

        if count1 > count2 {
            return Some((count1, "1".to_string(), letter))
        } else if count1 == count2 {
            return Some((count1, "=".to_string(), letter))
        } else {
            return Some((count2, "2".to_string(), letter))
        }
    }).collect::<Vec<(usize, String, char)>>();

    res.sort_by(|a, b| (-(a.0 as i32), a.1.clone(), a.2).cmp(&(-(b.0 as i32), b.1.clone(), b.2)));
    
    res.into_iter().map(|(count, is1, c)| format!(
        "{}:{}", is1,
        (0..count).map(|_| c).collect::<String>()
    )).collect::<Vec<String>>().join("/")
}

fn main() {
    println!("{}", mix("aaa", "bbbbb"));
}

#[cfg(test)]
mod tests {
    use super::mix;
    
    #[test]
    fn basics_mix() {
        testing("Are they here", "yes, they are here", 
            "2:eeeee/2:yy/=:hh/=:rr");
        testing("looping is fun but dangerous", "less dangerous than coding", 
            "1:ooo/1:uuu/2:sss/=:nnn/1:ii/2:aa/2:dd/2:ee/=:gg");
        testing(" In many languages", " there's a pair of functions", 
            "1:aaa/1:nnn/1:gg/2:ee/2:ff/2:ii/2:oo/2:rr/2:ss/2:tt");
        testing("Lords of the Fallen", "gamekult", "1:ee/1:ll/1:oo");
        testing("codewars", "codewars", "");
        testing("A generation must confront the looming ", "codewarrs", 
            "1:nnnnn/1:ooooo/1:tttt/1:eee/1:gg/1:ii/1:mm/=:rr");
    }
    
    fn testing(s1: &str, s2: &str, exp: &str) -> () {
        assert_eq!(&mix(s1, s2), exp)
    }
}