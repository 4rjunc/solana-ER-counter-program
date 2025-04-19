use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Counter {
    pub count: u64,
}

impl Counter {
    pub const USIZE: usize = 8;
}
