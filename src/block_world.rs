const GRID_SIZE: i8 = 4;

#[derive(Debug)]
pub struct Blockworld<'a>
{
    pub root: bool,
    pub parent: Option<&'a Blockworld<'a>>,
    player:  Pos,
    pub blocks: [Block; 3],
    depth: i32,
    pub move_taken: Option<Direction>,
    possible_moves: Vec<Direction>,
    manhattan_distance: i32
}

#[derive(Clone, Debug)]
pub struct Pos(pub i8, pub i8);

#[derive(Clone, Debug)]
pub struct Block
{
    pub name: char,
    pub position: Pos,
}

#[derive(Debug, Clone)]
pub enum Direction
{
    UP,
    DOWN,
    LEFT,
    RIGHT
}

impl<'a> Blockworld<'a>
{

    pub fn new(parent: &'a Blockworld, direction : Direction) -> Self
    {
        let mut new_state = Blockworld
        {
            root: false,
            parent: Some(parent),
            player: parent.player.clone(),
            blocks: parent.blocks.clone(),
            depth: parent.depth + 1,
            possible_moves: Blockworld::calculate_possible_moves(&parent.player),
            move_taken: Some(direction),
            manhattan_distance: 0,
        };

        new_state.move_direction(new_state.move_taken.clone());

        return new_state;
    }

    pub fn new_root(player_pos : Pos, a_pos: Pos, b_pos: Pos, c_pos: Pos) -> Self
    {
        let block_a : Block = Block{ name: 'A', position: a_pos};
        let block_b : Block = Block{ name: 'B', position: b_pos};
        let block_c : Block = Block{ name: 'C', position: c_pos};

        Blockworld
        {
            root: true,
            parent: None,
            possible_moves : Blockworld::calculate_possible_moves(&player_pos), 
            player: player_pos,
            blocks: [block_a, block_b, block_c],
            depth: 0,
            move_taken: None,
            manhattan_distance: 0
        }
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


    
}