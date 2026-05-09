use use_series::{arithmetic_nth_term, arithmetic_sum, geometric_nth_term, geometric_sum};

fn main() -> Result<(), use_series::SeriesError> {
    assert_eq!(arithmetic_nth_term(3, 2, 4)?, 11);
    assert_eq!(arithmetic_sum(3, 2, 5)?, 35);
    assert_eq!(geometric_nth_term(2, 3, 4)?, 162);
    assert_eq!(geometric_sum(2, 3, 4)?, 80);

    Ok(())
}
