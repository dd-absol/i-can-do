use crate::ucs::{State, search};

#[derive(Clone, Eq, PartialEq)]
struct LightState {
    lights_on: Vec<bool>,
    lights_goal: Vec<bool>,
    switch_number: usize,
    lights_from_switch: Vec<Vec<usize>>
}

struct LightParameter {
    n: usize,
    lights_from_switch: Vec<Vec<usize>>,
    lights: Vec<usize>
}

impl State for LightState {
    type Action = usize;
    type Parameter = LightParameter;

    fn is_goal(&self) -> bool {
        self.lights_goal == self.lights_on 
    }

    fn get_starting_state(parameter: LightParameter) -> Self {
        let mut lights_goal = vec![false; parameter.n];

        for l in parameter.lights {
            lights_goal[l] = true;
        }

        LightState { 
            lights_on: vec![false; parameter.n], 
            lights_goal,
            switch_number: parameter.lights_from_switch.len(), 
            lights_from_switch: parameter.lights_from_switch 
        }
    }

    fn get_next_state(&self, action: &Self::Action) -> Self {
        let mut lights_on = self.lights_on.clone();

        for l in self.lights_from_switch[*action].iter() {
            lights_on[*l] = ! lights_on[*l];
        }

        LightState { lights_on, ..self.clone() }
    }

    fn get_legal_actions(&self) -> Vec<Self::Action> {
        (0..self.switch_number).collect()
    }

    fn get_cost(_action: &Self::Action) -> usize {
        1
    }

    fn heuristic(&self) -> usize {
        self.lights_goal.iter().zip(self.lights_on.iter()).filter(|l| l.0 != l.1).count()
    }
}

struct LightController {
    n: usize,
    light_list: Vec<Vec<usize>>
}

impl LightController {
    fn new(n: usize, corresponding_lights_list: &[Vec<usize>]) -> Self {
    
        LightController {
            n,
            light_list: corresponding_lights_list.to_vec()
        }
    }

    fn solve(&self, lights: &Vec<usize>) -> Option<Vec<usize>> {
        search::<LightState>(LightParameter {
            n: self.n,
            lights_from_switch: self.light_list.clone(),
            lights: lights.clone()
        })
    }
}

#[cfg(test)]
mod fixed_tests {
    use super::LightController;

    #[test]
    fn exhaustive_small_tests() {
        let tests = vec![
            (2, vec![vec![0], vec![1]], vec![true, true, true, true]),
            (2, vec![vec![1, 0], vec![1]], vec![true, true, true, true]),
            (
                2,
                vec![vec![0, 1], vec![0, 1]],
                vec![   true, false, false, true],
            ),
            (
                3,
                vec![vec![], vec![2], vec![0], vec![1, 0, 2]],
                vec![true, true, true, true, true, true, true, true],
            ),
            (
                3,
                vec![vec![], vec![], vec![], vec![2, 1]],
                vec![true, false, false, false, false, false, true, false],
            ),
            (0, vec![vec![]], vec![true]),
            (1, vec![], vec![true, false]),
            (0, vec![], vec![true]),
            (1, vec![vec![0]], vec![true, true]),
            (1, vec![vec![]], vec![true, false]),
            (4, vec![vec![0, 2, 3], vec![1, 2, 3], vec![1], vec![2], ], vec![true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true]),
            (4, vec![vec![3], vec![1], vec![0, 1, 2], vec![0, 1, 2, 3], ], vec![true, false, true, false, false, true, false, true, true, false, true, false, false, true, false, true]),
            (4, vec![vec![1, 2, 3], vec![0, 1, 3], vec![1, 2, 3], vec![], vec![3], ], vec![true, false, false, true, false, true, true, false, true, false, false, true, false, true, true, false]),
            (4, vec![vec![0, 1], vec![], vec![0, 1, 2, 3], ], vec![true, false, false, true, false, false, false, false, false, false, false, false, true, false, false, true]),
        ];

        for (n, corresponding_lights_list, are_possible) in tests {
            let controller = LightController::new(n, &corresponding_lights_list);
            for (lights_choice, is_possible) in
                powerset(n).into_iter().zip(are_possible.into_iter())
            {
                test_controller(
                    n,
                    &corresponding_lights_list,
                    &controller,
                    lights_choice,
                    is_possible,
                );
            }
        }
    }

    fn powerset(n: usize) -> Vec<Vec<usize>> {
        if n == 0 {
            vec![vec![]]
        } else {
            let mut s = powerset(n - 1);
            let mut s2 = s.clone();
            s2.iter_mut().for_each(|ss| ss.push(n - 1));
            s.append(&mut s2);
            s
        }
    }

    fn test_controller(
        n: usize,
        corresponding_lights_list: &Vec<Vec<usize>>,
        controller: &LightController,
        lights_choice: Vec<usize>,
        is_possible: bool,
    ) {
        if let Some(switches_indices) = controller.solve(&lights_choice) {
            assert_eq!(
                toggle_switches(corresponding_lights_list, n, &switches_indices),
                lights_choice,
                "controller for corresponding_lights_list {:?} failed to provide a right set of switches: tried to turn on the lights {:?} with the switches {:?}",
                corresponding_lights_list,
                lights_choice,
                switches_indices
            )
        } else {
            assert!(!is_possible, "controller for corresponding_lights_list {:?} returned None for the lights set {:?}", corresponding_lights_list, lights_choice);
        }
    }

    fn toggle_switches(
        corresponding_lights_list: &Vec<Vec<usize>>,
        n: usize,
        switches_indices: &Vec<usize>,
    ) -> Vec<usize> {
        let mut lights_states = vec![false; n];
        for switch in switches_indices {
            for light in &corresponding_lights_list[*switch] {
                lights_states[*light] = !lights_states[*light]
            }
        }
        (0..n).filter(|i| lights_states[*i]).collect()
    }

}
