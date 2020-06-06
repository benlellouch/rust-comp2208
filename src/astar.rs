use std::collections::BinaryHeap;
use crate::block_world::Blockworld;
use std::rc::Rc;
use crate::search;

pub struct AStar
{
    fringe: BinaryHeap<Rc<Blockworld>>,    
}

impl AStar
{
    pub fn new() -> Self
    {
        AStar
        {
            fringe: BinaryHeap::new()
        }
    }

    pub fn run(&mut self, root: Rc<Blockworld>)
    {
        self.fringe.push(root);

        let mut depth: i32 = 0;

        while self.fringe.len() > 0
        {
            let current = self.fringe.pop().unwrap();
            if search::check_for_solution(&current)
            {
                println!("Solution found at depth {}", current.depth);
                let stack = Vec::new();
                search::print_solution(current, stack);
                return;
            }

            // if current.depth < 20
            // {
            //     println!("current depth: {} and manhattan distance: {}", current.depth, current.manhattan_distance);
            // }


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
            self.fringe.push(Rc::new(Blockworld::new(node.clone(), direction.clone())));
        }
    }
}