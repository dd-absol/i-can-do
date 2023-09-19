use crate::ucs::{State, search};

#[derive(PartialEq, Eq)]
struct BoardState {
    board: Vec<Vec<bool>>,
    queens_placed: usize
}

impl BoardState {
    fn place_queen(&self, action: &(usize, usize)) -> Self {
        let n = self.board.len();

        let mut board: Vec<Vec<bool>> = self.board.iter().enumerate().map(|(x, row)| {
            if x == action.0 {
                return vec![false; n]
            }

            row.iter().enumerate().map(|(y, tile)| {
                if y == action.1 {
                    return false
                }

                *tile
            }).collect()
        }).collect();

        let (mut x, mut y) = action;
        while x > 0 && y > 0 { x -= 1; y -= 1; board[x][y] = false; }
        let (mut x, mut y) = action;
        while x > 0 && y < n - 1 { x -= 1; y += 1; board[x][y] = false; }
        let (mut x, mut y) = action;
        while x < n - 1 && y > 0 { x += 1; y -= 1; board[x][y] = false; }
        let (mut x, mut y) = action;
        while x < n - 1 && y < n - 1 { x += 1; y += 1; board[x][y] = false; }

        BoardState { board, queens_placed: self.queens_placed + 1 }
    }
}

impl State for BoardState {
    type Action = (usize, usize);

    type Parameter = (usize, (usize, usize));

    fn is_goal(&self) -> bool {
        self.queens_placed == self.board.len()
    }

    fn get_starting_state((n, mandatory_queen): Self::Parameter) -> Self {
        BoardState { board: vec![vec![true; n]; n], queens_placed: 0 }.place_queen(&mandatory_queen)
    }

    fn get_next_state(&self, action: &Self::Action) -> Self {
        self.place_queen(action)
    }

    fn get_legal_actions(&self) -> Vec<Self::Action> {
        self.board.iter().enumerate().map(|(x, row)| row.iter().enumerate().filter(|(_, tile)| **tile).map(move |(y, _)| (x, y))).flatten().collect()
    }

    fn get_cost(_action: &Self::Action) -> usize { 0 }

    fn heuristic(&self) -> usize {  
        self.board.iter().flatten().filter(|tile| **tile).count()
    }
}

pub fn solve_n_queens(n: usize, mandatory_coords: (usize, usize)) -> Option<String> {
    let mut res = vec![vec!['.'; n]; n];

    res[mandatory_coords.1][mandatory_coords.0] = 'Q';
    search::<BoardState>((n, mandatory_coords))?.into_iter().for_each(|(x, y)| { res[y][x] = 'Q'; });

    let fin = res.into_iter().map(|row| format!("{}\n", row.into_iter().collect::<String>())).collect::<String>();
    // println!("{}", &fin);
    Some(fin)
}



#[cfg(test)]
mod tests {
    use super::solve_n_queens;

    #[test]
    fn basic_tests() {
        let basic_tests = vec![(8, (3, 0)), (4, (2, 0)), (1, (0, 0))];
        for (n, fixed) in basic_tests.into_iter() {
            test_solution(n, fixed);
        }
    }

    #[test]
    fn no_solution_tests() {
        let no_solutions = vec![(2, (0, 0)), (3, (2, 0)), (6, (0, 0))];
        for (n, fixed) in no_solutions.into_iter() {
            test_no_solution(n, fixed);
        }
    }

    fn check_board(board: &[u8], n: usize, fixed: (usize, usize)) {
        let mut offset = 0;
        let mut num_queens = 0;
        let mut queens: Vec<Option<usize>> = vec![None; n];
        #[allow(clippy::needless_range_loop)] // should be more clear to keep the `y` indexing
        for y in 0..n {
            for x in 0..n {
                match board[offset] {
                    b'Q' => {
                        assert!(
                            queens[y].is_none(),
                            "The board should not have horizontal attacks between Queens"
                        );
                        num_queens += 1;
                        queens[y] = Some(x);
                    }
                    b'.' => {}
                    _ => panic!("The board has invalid character"),
                }
                offset += 1;
            }

            assert_eq!(
                board[offset], b'\n',
                "The board has missing/incorrect characters"
            );
            offset += 1;
        }

        assert_eq!(num_queens, n, "The number of queens should be equal to size");

        let queens = queens.into_iter().map(Option::unwrap).collect::<Vec<_>>();
        assert!(
            queens[fixed.1] == fixed.0,
            "The mandatory queen is not in the required position"
        );

        // Check no attacks
        let mut taken_cols = vec![false; n];
        let mut taken_diag1 = vec![false; 2 * n];
        let mut taken_diag2 = vec![false; 2 * n];
        for row in 0..n {
            let col = queens[row];
            assert!(
                !taken_cols[col],
                "The board has vertical attacks between Queens"
            );
            assert!(
                !taken_diag1[col + row],
                "The board has diag1 attacks between Queens"
            );
            assert!(
                !taken_diag2[n + col - row - 1],
                "The board has diag2 attacks between Queens"
            );
            taken_cols[col] = true;
            taken_diag1[col + row] = true;
            taken_diag2[n + col - row - 1] = true;
        }
    }

    fn test_solution(n: usize, fixed: (usize, usize)) {
        if let Some(board) = solve_n_queens(n, fixed) {
            check_board(&board.as_bytes(), n, fixed);
        } else {
            panic!("Returned None when there's a solution");
        } 
    }

    fn test_no_solution(n: usize, fixed: (usize, usize)) {
        assert_eq!(
            solve_n_queens(n, fixed),
            None,
            "Expected None when no solution is possible"
        );
    }
}