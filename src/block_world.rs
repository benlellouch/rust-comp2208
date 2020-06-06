use std::rc::Rc;
use std::cmp::Ordering;

const GRID_SIZE: i8 = 4;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Blockworld
{
    pub root: bool,
    pub parent: Option<Rc<Blockworld>>,
    player:  Pos,
    pub blocks: [Block; 3],
    pub depth: i32,
    pub move_taken: Option<Direction>,
    pub possible_moves: Vec<Direction>,
    pub manhattan_distance: i32
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pos(pub i8, pub i8);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block
{
    pub name: char,
    pub position: Pos,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction
{
    UP,
    DOWN,
    LEFT,
    RIGHT
}

impl Ord for Blockworld
{
    fn cmp(&self, other: &Blockworld) -> Ordering
    {
        other.manhattan_distance.cmp(&self.manhattan_distance)
    }
}

impl PartialOrd for Blockworld
{
    fn partial_cmp(&self, other: &Blockworld) -> Option<Ordering>
    {
        Some(other.manhattan_distance.cmp(&self.manhattan_distance))
    }

}

impl Blockworld
{

    pub fn new(parent: Rc<Blockworld>, direction : Direction) -> Self
    {
        let mut new_state = Blockworld
        {
            root: false,
            player: parent.player.clone(),
            blocks: parent.blocks.clone(),
            depth: parent.depth + 1,
            possible_moves: Blockworld::calculate_possible_moves(&parent.player),
            parent: Some(parent),
            move_taken: Some(direction),
            manhattan_distance: 0,
        };

        new_state.move_direction(new_state.move_taken.clone());
        new_state.manhattan_distance = new_state.calculate_manhattan_distance();

        return new_state;
    }

    pub fn new_root(player_pos : Pos, a_pos: Pos, b_pos: Pos, c_pos: Pos) -> Self
    {
        let block_a : Block = Block{ name: 'A', position: a_pos};
        let block_b : Block = Block{ name: 'B', position: b_pos};
        let block_c : Block = Block{ name: 'C', position: c_pos};

        let mut new_state = Blockworld
        {
            root: true,
            parent: None,
            possible_moves : Blockworld::calculate_possible_moves(&player_pos), 
            player: player_pos,
            blocks: [block_a, block_b, block_c],
            depth: 0,
            move_taken: None,
            manhattan_distance: 0
        };

        new_state.manhattan_distance = new_state.calculate_manhattan_distance();

        return new_state;
    }

    fn calculate_possible_moves(player: &Pos) -> Vec<Direction>
    {
        let mut possible_moves = Vec::new();

        Blockworld::calculate_up(player, & mut possible_moves);
        Blockworld::calculate_down(player, & mut possible_moves);
        Blockworld::calculate_left(player, & mut possible_moves);
        Blockworld:: calculate_right(player, & mut possible_moves);

        return possible_moves;
    }

    fn calculate_up(player: &Pos, possible_moves: & mut Vec<Direction>)
    {
        if (player.1 + 1) < GRID_SIZE 
        {
            possible_moves.push(Direction::UP);
        }
    }

    fn calculate_down(player: &Pos, possible_moves: & mut Vec<Direction>)
    {
        if (player.1 - 1) >= 0 
        {
            possible_moves.push(Direction::DOWN);
        }
    }

    fn calculate_left(player: &Pos, possible_moves: & mut Vec<Direction>)
    {
        if (player.0 - 1) >= 0  
        {
            possible_moves.push(Direction::LEFT);
            
        }
    }

    fn calculate_right(player: &Pos, possible_moves: & mut Vec<Direction>)
    {
        if (player.0 + 1) < GRID_SIZE 
        {
            possible_moves.push(Direction::RIGHT);
        }
    }

    fn move_direction(& mut self, direction: Option<Direction>)
    {
       match direction.unwrap()
       {
           Direction::UP => self.move_up(),
           Direction::DOWN => self.move_down(),
           Direction::LEFT => self.move_left(),
           Direction::RIGHT => self.move_right(),
       };
    }


    fn move_up(& mut self)
    {
        self.player.1 = self.player.1 + 1;

        for block in self.blocks.iter_mut() 
        {
            if (block.position.0 == self.player.0) && (block.position.1 == self.player.1)
            {
                block.position.1 = self.player.1 - 1
            }   
        }

    }

    fn move_down(& mut self)
    {
        self.player.1 = self.player.1 - 1;

        for block in self.blocks.iter_mut() 
        {
            if (block.position.0 == self.player.0) && (block.position.1 == self.player.1)
            {
                block.position.1 = self.player.1 + 1
            }   
        }

    }

    fn move_left(& mut self)
    {
        self.player.0 = self.player.0 - 1;

        for block in self.blocks.iter_mut() 
        {
            if (block.position.0 == self.player.0) && (block.position.1 == self.player.1)
            {
                block.position.0 = self.player.0 + 1
            }   
        }

    }

    fn move_right(& mut self)
    {
        self.player.0 = self.player.0 + 1;

        for block in self.blocks.iter_mut() 
        {
            if (block.position.0 == self.player.0) && (block.position.1 == self.player.1)
            {
                block.position.0 = self.player.0 - 1
            }   
        }

    }

    fn calculate_manhattan_distance(&self) -> i32
    {
        let mut manhattan_distance = 0;

        // if self.root
        // {
        //     manhattan_distance = 0;
        // }
        // else
        // {
        //     manhattan_distance = self.parent.as_ref().unwrap().manhattan_distance;
        // }

        for block in self.blocks.iter()
        {
            let x = i32::from(block.position.0);
            let y = i32::from(block.position.1);
            match block.name
            {
                'A' if x == 1 && y == 2  => manhattan_distance += (x - 1).abs() + (y - 2).abs() - 1,
                'A' => manhattan_distance += (x - 1).abs() + (y - 2).abs(),

                'B' if x == 1 && y == 1 => manhattan_distance += (x - 1).abs() + (y - 1).abs() - 1,
                'B'  => manhattan_distance += (x - 1).abs() + (y - 1).abs(),

                'C' if x == 1 && y == 0 => manhattan_distance += (x - 1).abs() + (y).abs() - 1,
                'C'  => manhattan_distance += (x - 1).abs() + (y).abs(),

                _ => ()

            }
        }

        manhattan_distance += self.depth;

        return manhattan_distance;
    }

}