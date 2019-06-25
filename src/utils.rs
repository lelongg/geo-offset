use itertools::Itertools;

type Point = geo::Point<f64>;

pub fn orient_rings(coordinates: &[Point], is_hole: bool) -> Vec<Point> {
    let mut area = 0.0;
    let mut ring = coordinates.to_vec();

    for (pt1, pt2) in ring.iter().cycle().take(ring.len() + 1).tuple_windows() {
        area += pt1.x() * pt2.y();
        area -= pt2.x() * pt1.y();
    }

    if (!is_hole && area.is_sign_positive()) || (is_hole && area.is_sign_negative()) {
        ring.reverse()
    }

    ring
}
