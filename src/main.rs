mod block_world;
mod search;

fn main() {
    let state = block_world::Blockworld::new_root(block_world::Pos(3,0), block_world::Pos(0,0), block_world::Pos(1,0), block_world::Pos(2,0));
    println!("{:?}", state);
    print!("Is it a solution? {} \n", search::check_for_solution(&state));

}
