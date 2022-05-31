#[path = "wgpud/lib.rs"] mod lib;
use crate::block::Block;

pub async fn run(block: &Block) {
    pollster::block_on(lib::run(block));
 }