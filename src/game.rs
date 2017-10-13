enum MovementDirection {
    Left,
    Right,
    Top,
    Bottom,
}

pub struct Cell {
    pub x: usize,
    pub y: usize,
    direction: MovementDirection,
}

impl Cell {
    fn new(x: usize, y: usize, direction: MovementDirection) -> Cell{
        Cell { x, y, direction }
    }
}

pub struct Plane {
    width: usize,
    height: usize,
    pub snake: Vec<Cell>,
}

impl Plane {
    pub fn new(width: usize, height: usize) -> Plane {
        let body1 = Cell::new(0, 0, MovementDirection::Right);
        let body2 = Cell::new(1, 0, MovementDirection::Right);
        let head = Cell::new(2, 0, MovementDirection::Right);
        let snake = vec![head, body2, body1];
        Plane {
            width,
            height,
            snake,
        }
    }
}
