extern crate queues;
use queues::*;
use crate::search;
use crate::block_world::*;
use std::rc::Rc;

pub struct BFS
{
    fringe: Queue<Rc<Blockworld>>,
    expanded_nodes: i32,
}

impl BFS
{

    pub fn new() -> Self
    {
        BFS
        {
            fringe: queue![],
            expanded_nodes: 0,
        }
    }

    pub fn run(&mut self, root: Rc<Blockworld>)
    {
        self.fringe.add(root);

        let mut depth: i32 = 0;

        while self.fringe.size() > 0
        {
            let current = self.fringe.remove().unwrap();
            if search::check_for_solution(&current)
            {
                let stack = Vec::new();
                search::print_solution(current, stack);
                return;
            }

            if current.depth > depth
            {
                depth = current.depth;
                println!("Current depth: {}", depth);
            }


            self.expand(current);
        }
    }

    fn expand(& mut self, node: Rc<Blockworld>)
    {
        for direction in &node.possible_moves
        {
            self.fringe.add(Rc::new(Blockworld::new(node.clone(), direction.clone())));
        }
    }
}