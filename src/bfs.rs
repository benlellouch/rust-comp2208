extern crate queues;
use queues::*;
use crate::search;
use crate::block_world::*;

pub struct BFS<'a>
{
    fringe: Queue<Blockworld<'a>>,
    expanded: Vec<Blockworld<'a>>,
    expanded_nodes: i32,
    root_node: Blockworld<'a> 
}

impl<'a> BFS<'a>
{

    pub fn new(node: Blockworld<'a>) -> Self
    {
        BFS
        {
            fringe: queue![],
            expanded: Vec::new(),
            expanded_nodes: 0,
            root_node: node
        }
    }

    pub fn run(&mut self)
    {
        self.fringe.add(self.root_node.clone());
        while self.fringe.size() > 0
        {
            let current = self.fringe.remove().unwrap();
            if search::check_for_solution(&current)
            {
                let mut stack = Vec::new();
                search::print_solution(&current, &mut stack);
                return;
            }

            self.expanded.push(current);

            self.expand(&self.expanded[self.expanded.len()-1]);


        }
    }

    fn expand(& mut self, node: &'a Blockworld)
    {
        for direction in &node.possible_moves
        {
            self.fringe.add(Blockworld::new(node, direction.clone()));
        }
    }
}