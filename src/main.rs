mod block_world;
mod search;
mod bfs;

use std::rc::Rc;

fn main() {
    let state = block_world::Blockworld::new_root(block_world::Pos(0,0), block_world::Pos(3,2), block_world::Pos(1,1), block_world::Pos(1,0));
    let mut bfs = bfs::BFS::new();
    bfs.run(Rc::new(state));
}
