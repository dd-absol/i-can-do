use std::cmp;

fn main() {
    println!("hello world");
}

fn volume(heightmap: &Vec<Vec<i32>>) -> i32 {
    let height = heightmap.len();
    let width = heightmap[0].len();

    if height < 3 || height < 3 {
        return 0;
    }
    
    let mut lowest_point = heightmap[1..(height - 1)]
        .iter()
        .map(|row| [row[0], row[width - 1]])
        .flatten()
        .min()
        .unwrap();
    
    lowest_point = cmp::min(lowest_point, cmp::min(
        *heightmap[0][1..(width - 1)].iter().min().unwrap(), 
        *heightmap[height - 1][1..(width - 1)].iter().min().unwrap()
    ));

    let mut res = 0;

    let filled_hmap: Vec<Vec<i32>> = heightmap[1..(height - 1)].iter()
        .map(|row| row[1..(width - 1)].iter().map(
            |&height| if height < lowest_point {
                res += lowest_point - height;
                lowest_point
            } else {
                height
            }
        ).collect::<Vec<i32>>())
        .collect();
    
    return res + volume(&filled_hmap);
}

#[cfg(test)]
mod tests {
    use super::*;

    // this just helps with the test output on failure.
    fn pretty_test(map: &Vec<Vec<i32>>, expected: i32) {
        let result = volume(&map);
        let mut printy = String::new();
        for row in map {
            printy.push_str(format!("{:?}\n", row).as_str());
        }
        assert_eq!(result, expected, "\nYour result (left) did not match expected result (right) for map:\n{}", printy);
    }

    #[test]
    fn small_maps_test() {

        let tests = [
            (vec![vec![0]], 0),
            
            (vec![vec![22]], 0),
            
            (vec![vec![2, 1, 2],
                  vec![1, 0, 1],
                  vec![2, 1, 2]], 1),
            
            (vec![vec![1, 1, 1],
                  vec![1, 8, 1],
                  vec![1, 1, 1]], 0),
            
            (vec![vec![9, 9, 9, 9],
                  vec![9, 0, 0, 9],
                  vec![9, 0, 0, 9],
                  vec![9, 9, 9, 9]], 36),
            
            (vec![vec![9, 9, 9, 9, 9],
                  vec![9, 0, 1, 2, 9],
                  vec![9, 7, 8, 3, 9],
                  vec![9, 6, 5, 4, 9],
                  vec![9, 9, 9, 9, 9]], 45),
            
            (vec![vec![8, 8, 8, 8, 6, 6, 6, 6],
                  vec![8, 0, 0, 8, 6, 0, 0, 6],
                  vec![8, 0, 0, 8, 6, 0, 0, 6],
                  vec![8, 8, 8, 8, 6, 6, 6, 0]], 56),
            
            (vec![vec![ 0, 10,  0, 20,  0],
                  vec![20,  0, 30,  0, 40],
                  vec![ 0, 40,  0, 50,  0],
                  vec![50,  0, 60,  0, 70],
                  vec![ 0, 60,  0, 70,  0]], 150),
            
            (vec![vec![3, 3, 3, 3, 3],
                  vec![3, 0, 0, 0, 3],
                  vec![3, 3, 3, 0, 3],
                  vec![3, 0, 0, 0, 3],
                  vec![3, 0, 3, 3, 3],
                  vec![3, 0, 0, 0, 3],
                  vec![3, 3, 3, 0, 3]], 0),
            
            (vec![vec![3, 3, 3, 3, 3],
                  vec![3, 2, 2, 2, 3],
                  vec![3, 3, 3, 2, 3],
                  vec![3, 1, 1, 1, 3],
                  vec![3, 1, 3, 3, 3],
                  vec![3, 0, 0, 0, 3],
                  vec![3, 3, 3, 0, 3]], 0),
            
            (vec![vec![3, 3, 3, 3, 3],
                  vec![3, 0, 0, 0, 3],
                  vec![3, 3, 3, 0, 3],
                  vec![3, 0, 0, 0, 3],
                  vec![3, 0, 3, 3, 3],
                  vec![3, 0, 0, 0, 3],
                  vec![3, 3, 3, 1, 3]], 11),
            
            (vec![vec![3, 3, 3, 1, 3],
                  vec![3, 0, 0, 0, 3],
                  vec![3, 0, 3, 3, 3],
                  vec![3, 0, 0, 0, 3],
                  vec![3, 3, 3, 0, 3],
                  vec![3, 0, 0, 0, 3],
                  vec![3, 3, 3, 3, 3]], 11),
        ];
        
        for (map, expected) in tests.iter() {
            pretty_test(map, *expected);
        }
    }

    #[test]
    fn negative_heights_tests() {
        let tests = [
            (vec![vec![-1]], 0),
            
            (vec![vec![3, 3, 3, 3, 3],
                  vec![3, 0, 0, 0, 3],
                  vec![3, 3, 3, 0, 3],
                  vec![3, 0, -2, 0, 3],
                  vec![3, 0, 3, 3, 3],
                  vec![3, 0, 0, 0, 3],
                  vec![3, 3, 3, 1, -3]], 13),
            
            (vec![vec![8192, 8192, 8192, 8192],
                  vec![8192,-8192,-8192, 8192],
                  vec![8192,-8192,-8192, 8192],
                  vec![8192, 8192, 8192, 8192]], 65536)
        ];

        for (map, expected) in tests.iter() {
            pretty_test(map, *expected);
        }
    }

    #[test]
    fn large_map_test() {
        // 50x50 map without leaks; 100 around the border, 0 inside
        let mut map = vec![vec![100; 50]; 50];
        for y in 1..49 {
            for x in 1..49 {
                map[y][x] = 0;
            }
        }
        // volume = 100 * (48 * 48)
        pretty_test(&map, 230_400);
    }
}
