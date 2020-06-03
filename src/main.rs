mod block_world;
mod search;
mod bfs;

fn main() {
    let state = block_world::Blockworld::new_root(block_world::Pos(3,0), block_world::Pos(0,0), block_world::Pos(1,0), block_world::Pos(2,0));
    let bfs = bfs::BFS::new(state);
    bfs.run();
}
