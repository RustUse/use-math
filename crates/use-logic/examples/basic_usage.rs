use use_logic::{equivalence, exclusive_or, implication, majority, nand, nor};

fn main() {
    assert!(implication(false, true));
    assert!(equivalence(true, true));
    assert!(exclusive_or(true, false));
    assert!(!nand(true, true));
    assert!(nor(false, false));
    assert!(majority(true, true, false));
}
