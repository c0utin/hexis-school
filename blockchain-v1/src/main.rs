mod consensus;

fn main() {
    let genesis_block = consensus::pow::Block::new(0, 0, "0", "Genesis Block", 10, 2);
    let new_block = consensus::pow::generate_block_with_pow(&genesis_block, "Some data", 2);

    println!("{}", new_block);
}
