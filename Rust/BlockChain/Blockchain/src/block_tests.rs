#[cfg(test)]
mod block_tests {
    use crate::block::Block;
    use sha2::digest::generic_array::GenericArray;

    #[test]
    fn example() {
        assert_eq!(1+1, 2);
    }

    #[test]
    fn check_initial_hash_string(){
        let mut b0 = Block::initial(16);
        b0.mine(2);
        assert_eq!(b0.hash_string(), "0000000000000000000000000000000000000000000000000000000000000000:0:16::56231");
    }

    #[test]
    fn check_next_initial(){
        let mut b0 = Block::initial(16);
        b0.mine(1);
        let mut b1 = Block::next(&b0, String::from("Next of b0"));
        b1.mine(1);
        assert_eq!(b1.hash_string(), "6c71ff02a08a22309b7dbbcee45d291d4ce955caa32031c50d941e3e9dbd0000:1:16:Next of b0:34303");
    }
}
