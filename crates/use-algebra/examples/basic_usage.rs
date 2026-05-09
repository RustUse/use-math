use use_algebra::{identity_element, is_abelian_group, is_distributive_over, is_ring};

fn main() {
    let residues = [0_u8, 1, 2];
    let add_mod_3 = |left, right| (left + right) % 3;
    let mul_mod_3 = |left, right| (left * right) % 3;

    assert_eq!(identity_element(&residues, add_mod_3), Some(0));
    assert!(is_abelian_group(&residues, add_mod_3));
    assert!(is_distributive_over(&residues, mul_mod_3, add_mod_3));
    assert!(is_ring(&residues, add_mod_3, mul_mod_3));
}
