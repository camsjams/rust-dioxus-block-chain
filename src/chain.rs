use crate::block::Block;

pub struct Chain {
    blocks: Vec<Block>,
}

impl Chain {
    pub fn new() -> Chain {
        let genesis = Block::new(
            0,
            "We don't need no education".to_string(),
            "All in all you're just another brick in the wall".to_string(),
        );
        Chain {
            blocks: vec![genesis],
        }
    }

    pub fn get_block(&self, index: usize) -> Option<Block> {
        self.blocks.get(index).cloned()
    }

    pub fn get_blocks(&self) -> Vec<Block> {
        self.blocks.clone()
    }

    pub fn get_total(&self) -> usize {
        self.blocks.len()
    }

    pub fn add_block(&mut self, data: String) {
        match self.blocks.last().cloned() {
            None => println!("Invalid state: no genesis block."),
            Some(last_block) => {
                let next = Block::new(self.blocks.len(), data, last_block.get_hash());
                self.blocks.push(next);
            }
        }
    }
}
