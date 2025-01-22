use block::Block;


mod block;
mod hash;

fn main() {
    let mut genisis_block = Block {
        data: "block1".to_string(),
        hash: 0,
        previoushash: 0,
    };

    let mut block2 = Block {
        data: "block2".to_string(),
        hash: 0,
        previoushash: 0,
    };


    genisis_block.init(0);
    block2.init(genisis_block.hash);
    genisis_block.print();
    block2.print();

}

