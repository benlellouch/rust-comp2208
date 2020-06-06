use crate::block_world::Blockworld;
use crate::search::*;
use std::rc::Rc;

pub struct IDS
{
    fringe: Vec<Rc<Blockworld>>,
    max_depth: i32,
    root: Rc<Blockworld>
}

impl IDS
{
    pub fn new(root: Rc<Blockworld>, max_depth: i32) -> Self
    {
        let mut new_search = IDS
        {
            fringe: vec![],
            max_depth: max_depth,
            root: root
        };

        new_search.initialise(max_depth);

        return new_search;
    }

    fn initialise(&mut self, depth: i32)
    {
        self.max_depth = depth;
        self.fringe.push(self.root.clone());
    }

    pub fn run(&mut self)
    {
        let mut depth: i32 = 0;

        while self.fringe.len() > 0
        {
            let current = self.fringe.pop().unwrap();

            if current.depth > self.max_depth
            {
                if self.fringe.is_empty()
                {
                    self.initialise(self.max_depth + 1);
                }
            }
            else
            {
                if check_for_solution(&current)
                {
                    let stack = Vec::new();
                    print_solution(current, stack);
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
    }

    fn expand(& mut self, node: Rc<Blockworld>)
    {
        for direction in &node.possible_moves
        {
            self.fringe.push(Rc::new(Blockworld::new(node.clone(), direction.clone())));
        }
    }

}