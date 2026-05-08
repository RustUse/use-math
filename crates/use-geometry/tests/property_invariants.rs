use proptest::prelude::*;
use use_geometry::{
    Orientation2, Point2, Triangle, Vector2, distance_squared_2d, midpoint_2d, orientation_2d,
    triangle_twice_area, triangle_twice_signed_area,
};

prop_compose! {
    fn point_strategy()(x in -1_000_i32..1_001_i32, y in -1_000_i32..1_001_i32) -> Point2 {
        Point2::new(f64::from(x), f64::from(y))
    }
}

prop_compose! {
    fn vector_strategy()(x in -1_000_i32..1_001_i32, y in -1_000_i32..1_001_i32) -> Vector2 {
        Vector2::new(f64::from(x), f64::from(y))
    }
}

fn approx_eq(left: f64, right: f64) -> bool {
    let scale = 1.0_f64.max(left.abs()).max(right.abs());
    (left - right).abs() <= scale * 1.0e-10
}

proptest! {
    #[test]
    fn swapping_two_vertices_flips_orientation(a in point_strategy(), b in point_strategy(), c in point_strategy()) {
        let forward = orientation_2d(a, b, c);
        let swapped = orientation_2d(b, a, c);

        match forward {
            Orientation2::CounterClockwise => prop_assert_eq!(swapped, Orientation2::Clockwise),
            Orientation2::Clockwise => prop_assert_eq!(swapped, Orientation2::CounterClockwise),
            Orientation2::Collinear => prop_assert_eq!(swapped, Orientation2::Collinear),
        }
    }

    #[test]
    fn translation_preserves_triangle_measurements(
        a in point_strategy(),
        b in point_strategy(),
        c in point_strategy(),
        offset in vector_strategy(),
    ) {
        let triangle = Triangle::new(a, b, c);
        let translated = Triangle::new(a.translate(offset), b.translate(offset), c.translate(offset));

        prop_assert_eq!(triangle.orientation(), translated.orientation());
        prop_assert!(approx_eq(triangle.twice_signed_area(), translated.twice_signed_area()));
        prop_assert!(approx_eq(triangle.twice_area(), translated.twice_area()));
        prop_assert!(approx_eq(triangle.area(), translated.area()));
        prop_assert!(approx_eq(triangle.perimeter(), translated.perimeter()));
    }

    #[test]
    fn vector_chaining_matches_point_differences(a in point_strategy(), b in point_strategy(), c in point_strategy()) {
        let ab = Vector2::from_points(a, b);
        let bc = Vector2::from_points(b, c);
        let ac = Vector2::from_points(a, c);

        prop_assert_eq!(ab + bc, ac);
        prop_assert!(approx_eq(distance_squared_2d(a, b), ab.magnitude_squared()));
        prop_assert_eq!(midpoint_2d(a, b), a.midpoint(b));
    }

    #[test]
    fn triangle_area_tracks_vertex_order(a in point_strategy(), b in point_strategy(), c in point_strategy()) {
        let signed_area = triangle_twice_signed_area(a, b, c);
        let swapped_signed_area = triangle_twice_signed_area(b, a, c);

        prop_assert!(approx_eq(signed_area, -swapped_signed_area));
        prop_assert!(approx_eq(triangle_twice_area(a, b, c), signed_area.abs()));
    }
}
