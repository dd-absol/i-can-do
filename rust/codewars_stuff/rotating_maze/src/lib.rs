#![allow(dead_code)]

mod flux_puzzle {
    use std::{fmt::Display, collections::{HashSet, BinaryHeap}, rc::Rc, cmp::Reverse};
    
    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    enum Direction {
        North,
        South, 
        East,
        West
    }
    
    use Direction::*;
    
    impl Direction {
        fn next_clockwise(self) -> Direction {
            match self {
                North => East,
                East => South,
                South => West,
                West => North
            }
        }
    
        fn list() -> Vec<Direction> {
            Vec::from([East, South, West, North])
        }
    }
    
    impl Display for Direction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                North => write!(f,"N"),
                South => write!(f, "S"),
                East => write!(f, "E"),
                West => write!(f, "W")
            }    
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    enum Action {
        Move(Direction),
        Rotation
    }
    
    type Cell = HashSet<Direction>;

    #[derive(Debug, PartialEq, Eq, Clone)]
    struct MazeState {
        ball: (usize, usize),
        objective: (usize, usize),
        maze: Vec<Vec<Cell>>,
    }
    
    impl MazeState {
        fn check_neighbor(&self, dir: &Direction) -> bool {
            match dir {
                North => self.maze[self.ball.0 - 1][self.ball.1].contains(&South),
                South => self.maze[self.ball.0 + 1][self.ball.1].contains(&North),
                East => self.maze[self.ball.0][self.ball.1 + 1].contains(&West),
                West => self.maze[self.ball.0][self.ball.1 - 1].contains(&East),
            }
        }

        fn get_starting_state(imaze: &Vec<Vec<i8>>) -> Self {
            let maze: Vec<Vec<Cell>> = imaze.iter().map(|row|
                row.iter().map(|i| new_cell(*i)).collect()
            ).collect();

            let mut ball = (0, 0);
            let mut objective = (0, 0);
            
            let x1 = imaze.iter().enumerate().find(|(_, row)| 
                match row.iter().enumerate().find(|(_, i)| **i == -1) {
                    Some((index, _)) => { ball.1 = index; return true },
                    None => false
                }
            ).unwrap().0;

            let x2 = imaze.iter().enumerate().find(|(_, row)| 
                match row.iter().enumerate().find(|(_, i)| **i == -2) {
                    Some((index, _)) => { objective.1 = index; return true },
                    None => false
                }
            ).unwrap().0;
            
            ball.0 = x1;
            objective.0 = x2;
            
            MazeState { ball, maze, objective }
        }
    
        fn rotate(&self) -> Self {
            let maze = self.maze.iter().map(|row|
                row.iter().map(|cell|
                    HashSet::from_iter(cell.iter().cloned().map(|dir| dir.next_clockwise()))
                    ).collect()
                ).collect();
                
                MazeState { maze, ..*self }
            }
        
        fn get_cell(&self) -> &Cell {
            &self.maze[self.ball.0][self.ball.1]
        }
        
        fn get_legal_actions(&self) -> Vec<Action> {
            let mut res = vec![Action::Rotation];

            for dir in self.get_cell().intersection(&self.direction_list()) {
                if self.check_neighbor(&dir) { res.push(Action::Move(dir.clone())); }
            }
            
            return res
        }
        
        fn direction_list(&self) -> Cell {
            HashSet::from_iter(Direction::list().into_iter().filter(|dir| match dir {
                North if self.ball.0 == 0 => false,
                South if self.ball.0 + 1 == self.maze.len() => false,
                West if self.ball.1 == 0 => false,
                East if self.ball.1 + 1 == self.maze[self.ball.0].len() => false,
                _ => true
            }))
        }
        
        fn get_next_state(&self, action: &Action) -> Self {
            match action {
                Action::Rotation => self.rotate(),
                Action::Move(dir) => {
                    let ball = match dir {
                        North => (self.ball.0 - 1, self.ball.1),
                        South => (self.ball.0 + 1, self.ball.1),
                        East => (self.ball.0, self.ball.1 + 1),
                        West => (self.ball.0, self.ball.1 - 1)
                    };
                    
                    MazeState { ball, maze: self.maze.clone(), ..*self }
                }
            }
        }

        fn is_goal(&self) -> bool {
            return self.ball == self.objective
        }
    }

    fn new_cell(mut i: i8) -> Cell {
        let dir_list = Direction::list();
        if i < 0 {
            return HashSet::from_iter(dir_list.iter().cloned())
        }

        let mut res = HashSet::new();
        for n in 0..4 {
            if i % 2 == 0 {
                res.insert(dir_list[n].clone());
            }

            i = i / 2;
        }
        
        return res
    }

    #[derive(Debug, PartialEq, Eq)]
    struct Node {
        parent: Option<Parent>,
        cost: u32,
        state: Rc<MazeState>
    }

    #[derive(Debug, PartialEq, Eq)]
    struct Parent { node: Rc<Node>, action: Action }

    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Node {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            Reverse(self.cost).cmp(&Reverse(other.cost))
        }
    }

    fn expand(node: Rc<Node>) -> Vec<Rc<Node>> {
        node.state.get_legal_actions().into_iter().map(|action|
            Rc::new( Node {
                state: Rc::new(node.state.get_next_state(&action)),
                cost: if action == Action::Rotation { node.cost + 1 } else { node.cost },
                parent: Some(Parent {
                    action: action,
                    node: Rc::clone(&node)
                })
            })
        ).collect()
    }

    fn build_solution(goal: Rc<Node>) -> Vec<String> {
        let mut node = goal;
        let mut res = Vec::new();

        while let Some(parent) = &node.parent {
            let temp = Rc::clone(&parent.node);
            res.push(parent.action.clone());
            node = temp
        }

        
        let mut temp = String::new();
        let mut final_res = res.into_iter().rev().filter_map(|action| match action {
            Action::Rotation => { let r = temp.clone(); temp.clear(); Some(r) },
            Action::Move(dir) => { temp.push_str(&dir.to_string()); None }
        }).collect::<Vec<String>>();

        final_res.push(temp);
        final_res
    }
    
    pub fn maze_solver(maze: &Vec<Vec<i8>>) -> Option<Vec<String>> {
        let mut frontier = BinaryHeap::new();
        let mut explored = Vec::new();
        frontier.push(Rc::new(Node {
            parent: None,
            cost: 0,
            state: Rc::new(MazeState::get_starting_state(maze))
        }));

        while let Some(node) = frontier.pop() {
            if node.state.is_goal() {
                return Some(build_solution(node))
            }

            if !explored.contains(&node.state) {
                explored.push(Rc::clone(&node.state));

                for n in expand(node) {

                    frontier.push(n);
                }                
            }

        }

        None
    }
}


#[cfg(test)]
mod example_tests {
    use super::*;
    
    #[test]
    fn example_tests() {
        let example_tests = vec![
            (
                vec![
                    vec![4,2,5,4],
                    vec![4,15,11,1],
                    vec![-1,9,6,8],
                    vec![12,7,7,-2],
                ],
                Some(vec!["NNE", "EE", "S", "SS"])
            ),
            (
                vec![
                    vec![6,3,10,4,11],
                    vec![8,10,4,8,5],
                    vec![-1,14,11,3,-2],
                    vec![15,3,4,14,15],
                    vec![14,7,15,5,5],
                ],
                Some(vec!["", "", "E", "", "E", "NESE"])
            ),
            (
                vec![
                    vec![9,1,9,0,13,0],
                    vec![14,1,11,2,11,4],
                    vec![-1,2,11,0,0,15],
                    vec![4,3,9,6,3,-2],
                ],
                Some(vec!["E", "SE", "", "E", "E", "E"])
            ),
            (
                vec![
                    vec![-1,6,12,15,11],
                    vec![8,7,15,7,10],
                    vec![13,7,13,15,-2],
                    vec![11,10,8,1,3],
                    vec![12,6,9,14,7],
                ],
                None
            ),
            (
                vec![
                    vec![6,3,0,9,14,13,14],
                    vec![-1,14,9,11,15,14,15],
                    vec![2,15,0,12,6,15,-2],
                    vec![4,10,7,6,15,5,3],
                    vec![7,3,13,13,14,7,0],
                ],
                None
            ),
        ];
        
        example_tests.iter().for_each(|(maze,sol)| {
            let refsol = sol.as_ref().map(|r| r.iter().map(|&s| String::from(s)).collect());
            test_helper::run_test(maze,flux_puzzle::maze_solver(maze),refsol)
        });
    }
}

#[cfg(test)]
mod test_helper {
	use std::collections::HashSet;
	use std::collections::HashMap;
	
	pub fn run_test(r: &Vec<Vec<i8>>,_user: Option<Vec<String>>,_refsol: Option<Vec<String>>) {
		if let Some(user) = _user {
            let refsol = _refsol.unwrap();
            
    		if user.join("").chars().any(|ch| !"WENS".contains(ch)) {
    			return assert!(false,"Solution elements must only consist of the following characters: \"NWSE\"");}
    		let ref_str = format!("Here is a valid solution:\n{}",sol_str(&refsol));
    		let user_str = format!("Here is your solution:\n{}",sol_str(&user));
    		if user.len() > refsol.len() {
    			return assert!(false,"Your solution completes the task in {} iterations.\nThis test can be completed in {} iterations.\n{}\n{}",user.len(),refsol.len(),ref_str,user_str);}
    		let dir_map: HashMap<u8,(i8,i8)> = vec![(78,(-1,0)),(87,(0,-1)),(83,(1,0)),(69,(0,1))].into_iter().collect();
    		let dnum: HashMap<u8,usize> = vec![(78,0),(87,1),(83,2),(69,3)].into_iter().collect();
    		let dword = ["north","west","south","east"];
    		let grid: Vec<Vec<u8>> = r.iter().map(|row| row.iter().map(|&n| n.max(0) as u8).collect()).collect();
    		let xl = r.len();
    		let yl = r[0].len();
    		let (mut px,mut py): (usize,usize) = (0,0);
    		let mut dst: (usize,usize) = (0,0);
    		for (i,row) in r.iter().enumerate() {
    			for (j,cel) in row.iter().enumerate() {
    				if *cel < 0 {
    					if *cel == -1 { px = i; py = j; } else { dst = (i,j); }}}}
    		let bad_move = |s: String| assert!(false,"Invalid move: {}\n{}",s,user_str);
    		
    		for (i,s) in user.iter().enumerate() {
    			let mut visited: HashSet<(usize,usize)> = HashSet::new();
    			for (j,b) in s.bytes().enumerate() {
    				let dq: usize = dnum[&b];
    				let (nx,ny): (i8,i8) = dir_map[&b];
    				let pos_str = format!("during move {} at iteration {}.\nLast valid position was [{}, {}].",j,i,px,py);
    				let _qx = nx + px as i8;
    				let _qy = ny + py as i8;
    				if _qx <  0 || _qx >= xl as i8 || _qy < 0 || _qy >= yl as i8 {
    					return bad_move(format!("Out of bounds {}",pos_str));}
    				let qx = _qx as usize;
    				let qy = _qy as usize;
    				let obstructs: Vec<(usize,u8)> = wall_check(grid[px][py],grid[qx][qy],dq,i%4);
    				if !obstructs.is_empty() {
    					let (celln,d) = obstructs[0];
    					let (xx,yy) = if d == 0 { (px,py) } else { (qx,qy) };
    					return bad_move(format!("Path obstructed by a wall on the {} side of [{}, {}] {}",dword[celln],xx,yy,pos_str));}
    				if visited.contains(&(qx,qy)) {
    					return bad_move(format!("Entered cell [{}, {}] a second time",qx,qy));}
    				px = qx;
    				py = qy;
    				visited.insert((qx,qy));}
    		}
    		
    		if dst != (px,py) {
    			return assert!(false,"The ball did not reach the destination. Its last position was [{}, {}]\n{}",px,py,user_str);}
    		
    		return assert!(true);
        } else {
			return assert!(_refsol.is_none(),"This puzzle has no solution");
        }
	}
	
	fn sol_str(r: &Vec<String>) -> String { format!("[ \"{}\" ]",r.join("\", \"")) }
	fn celrot(n: u8) -> u8 { let x = n << 1; x/16 + x%16 }
	fn get_celval(n: u8,c: usize) -> usize { (0..c).into_iter().fold(n, |z,_| celrot(z)) as usize }
	
	fn wall_check(fro: u8,too: u8,d: usize,c: usize) -> Vec<(usize,u8)> {
		let wall_fro = get_celval(fro,c) & (8 >> d) == 0;
		let wall_too = get_celval(too,c) & (8 >> (d+2)%4) == 0;
		if wall_fro && wall_too { vec![] } else if !wall_fro { vec![(d,0)] } else { vec![((d+2)%4,1)] }
	}
}
