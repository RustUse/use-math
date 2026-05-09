use use_set::{
    are_disjoint, contains_member, is_subset, set_difference, set_intersection,
    set_symmetric_difference, set_union,
};

fn main() {
    let left = ["circle", "triangle", "triangle", "square"];
    let right = ["square", "hexagon", "triangle"];

    assert!(contains_member(&left, &"circle"));
    assert!(is_subset(&["triangle", "square"], &left));
    assert!(!are_disjoint(&left, &right));

    assert_eq!(
        set_union(&left, &right),
        vec!["circle", "triangle", "square", "hexagon"]
    );
    assert_eq!(set_intersection(&left, &right), vec!["triangle", "square"]);
    assert_eq!(set_difference(&left, &right), vec!["circle"]);
    assert_eq!(
        set_symmetric_difference(&left, &right),
        vec!["circle", "hexagon"]
    );
}
