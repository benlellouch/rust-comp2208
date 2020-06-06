mod block_world;
mod search;
mod bfs;
mod ids;

use std::rc::Rc;

fn main() {
    let state = block_world::Blockworld::new_root(block_world::Pos(3,3), block_world::Pos(2,2), block_world::Pos(1,3), block_world::Pos(2,0));

    // let mut bfs = bfs::BFS::new();
    // bfs.run(Rc::new(state));

    let mut ids = ids::IDS::new(Rc::new(state), 0);
    ids.run();
}
