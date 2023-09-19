use std::{rc::Rc, collections::BinaryHeap, cmp::Reverse};

pub trait State {
    type Action: PartialEq + Eq + Clone;
    type Parameter;

    fn is_goal(&self) -> bool;

    fn get_starting_state(parameter: Self::Parameter) -> Self;

    fn get_next_state(&self, action: &Self::Action) -> Self;

    fn get_legal_actions(&self) -> Vec<Self::Action>;

    fn get_cost(_action: &Self::Action) -> usize { 0 }

    fn heuristic(&self) -> usize { 0 }
}

#[derive(PartialEq, Eq)]
struct Node<S: State> {
    parent: Option<Parent<S>>,
    cost: usize,
    state: Rc<S>
}

#[derive(PartialEq, Eq)]
struct Parent<S: State> { node: Rc<Node<S>>, action: S::Action }

impl<S: State + PartialEq + Eq> PartialOrd for Node<S> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<S: State + Eq> Ord for Node<S> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Reverse(self.cost + self.state.heuristic()).cmp(&Reverse(other.cost + other.state.heuristic()))
    }
}

impl<S: State> Node<S> {
    fn build_solution(mut node: Rc<Self>) -> Vec<S::Action> {
        let mut res = Vec::new();

        while let Some(parent) = &node.parent {
            let temp = Rc::clone(&parent.node);
            res.push(parent.action.clone());
            node = temp
        }

        res
    }
    
    fn expand(node: Rc<Self>) -> Vec<Rc<Self>> {
        node.state.get_legal_actions().into_iter().map(|action| {
            let state = Rc::new(node.state.get_next_state(&action));
            Rc::new( Node {
                cost: node.cost + S::get_cost(&action),
                parent: Some(Parent {
                    action: action,
                    node: Rc::clone(&node)
                }),
                state
            })
        }).collect()
    }
}

pub fn search<S: State + Eq + PartialEq>(parameter: S::Parameter) -> Option<Vec<<S as State>::Action>> {
    let mut frontier = BinaryHeap::new();
    let mut explored = Vec::new();
    frontier.push(Rc::new(Node {
        parent: None,
        cost: 0,
        state: Rc::new(S::get_starting_state(parameter))
    }));

    while let Some(node) = frontier.pop() {
        if node.state.is_goal() {
            return Some(Node::build_solution(node))
        }

        if !explored.contains(&node.state) {
            explored.push(Rc::clone(&node.state));

            for n in Node::expand(node) {
                frontier.push(n);
            }                
        }
    }

    None
}