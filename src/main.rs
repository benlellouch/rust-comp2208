mod block_world;
mod search;
mod bfs;
mod ids;
mod astar;

use std::rc::Rc;

fn main() {
    let state = block_world::Blockworld::new_root(block_world::Pos(0,0), block_world::Pos(2,2), block_world::Pos(2,3), block_world::Pos(3,0));

    // let mut bfs = bfs::BFS::new();
    // bfs.run(Rc::new(state));

    let mut ids = ids::IDS::new(Rc::new(state), 0);
    ids.run();

    // let mut astar = astar::AStar::new();
    // astar.run(Rc::new(state));
}
