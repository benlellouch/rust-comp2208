use crate::block_world::*;
use std::rc::Rc;

pub fn check_for_solution(node: &Blockworld) -> bool
{


    let mut count: u8 = 0;

    for block in node.blocks.iter()
    {
        match block
        {
            Block {name: 'A', position: Pos(1,2) } => count += 1,
            Block {name: 'B', position: Pos(1,1) } => count += 1,
            Block {name: 'C', position: Pos(1,0) } => count += 1,
            _ => ()
        }
    }

    if count == 3
    {
        true
    }
    else
    {
        false
    }

    
}

pub fn print_solution(node: Rc<Blockworld>, mut stack: Vec<Option<Direction>>)
{

    if node.root
    {
        println!("The solution is: {:?}", stack);
    }
    else
    {
        stack.push(node.move_taken.clone());
        print_helper(node.parent.as_ref() , stack);  
    }
}

fn print_helper(node: Option<&Rc<Blockworld>>, mut stack: Vec<Option<Direction>>)
{
    let unwrapped_node = node.unwrap();

    if unwrapped_node.root
    {
        let unwrapped_stack: Vec<Direction> = stack.into_iter().filter(|e| e.is_some()).map(|e| e.unwrap()).collect();
        println!("The solution is: {:?}", unwrapped_stack);
    }
    else
    {
        stack.push(unwrapped_node.move_taken.clone());
        print_helper(unwrapped_node.parent.as_ref() , stack);  
    }
}