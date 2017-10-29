/// (x, y)
type Point = (u32, u32);

/// Returns (m, b) where y = mx + b.
pub fn solve_linear_eq(p1: Point, p2: Point) -> (f64, f64) {
    let m = f64::from(p2.1 - p1.1) / f64::from(p2.0 - p1.0);
    let b = f64::from(p1.1) - m * f64::from(p1.0);
    (m, b)
}
