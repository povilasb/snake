/// (x, y)
type Point = (u32, u32);

/// Returns (m, b) where y = mx + b.
pub fn solve_linear_eq(p1: Point, p2: Point) -> (f64, f64) {
    let m = (p2.1 - p1.1) as f64 / (p2.0 - p1.0) as f64;
    let b = p1.1 as f64 - m * p1.0 as f64;
    (m, b)
}
