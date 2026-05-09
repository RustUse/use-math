use use_logic::{equivalence, exclusive_or, implication, majority, nand, nor};

#[test]
fn direct_logic_usage_covers_boolean_helpers() {
    assert!(implication(false, true));
    assert!(!implication(true, false));
    assert!(equivalence(true, true));
    assert!(!equivalence(true, false));
    assert!(exclusive_or(true, false));
    assert!(!exclusive_or(true, true));
    assert!(!nand(true, true));
    assert!(nand(true, false));
    assert!(nor(false, false));
    assert!(!nor(true, false));
    assert!(majority(true, true, false));
    assert!(!majority(true, false, false));
}
