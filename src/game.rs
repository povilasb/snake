use std::clone::Clone;
use std::cmp::PartialEq;
use std::fmt::Debug;

#[derive(Clone, PartialEq, Debug)]
pub enum MovementDirection {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone)]
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

    pub fn move_to(&mut self, direction: MovementDirection) {
        for i in (1..self.snake.len()).rev() {
            self.snake[i] = self.snake[i - 1].clone();
        }
        // TODO: check over/underflow.
        match direction {
            MovementDirection::Left => self.snake[0].x -= 1,
            MovementDirection::Right => self.snake[0].x += 1,
            MovementDirection::Up => self.snake[0].y -= 1,
            MovementDirection::Down => self.snake[0].y += 1,
        };
        self.snake[0].direction = direction;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::*;

    mod plane {
        use super::*;

        mod move_to {
            use super::*;

            #[test]
            fn it_shifts_body_parts_right() {
                let mut plane = Plane::new(20, 20);

                plane.move_to(MovementDirection::Right);

                assert_that!(plane.snake[1].x, is(equal_to(2)));
                assert_that!(plane.snake[1].y, is(equal_to(0)));
                assert_that!(plane.snake[2].x, is(equal_to(1)));
                assert_that!(plane.snake[2].y, is(equal_to(0)));
            }

            #[test]
            fn it_increases_head_x_when_direction_is_right() {
                let mut plane = Plane::new(20, 20);

                plane.move_to(MovementDirection::Right);

                assert_that!(plane.snake[0].x, is(equal_to(3)));
                assert_that!(plane.snake[0].y, is(equal_to(0)));
            }

            #[test]
            fn it_decreases_head_x_when_direction_is_left() {
                let mut plane = Plane::new(20, 20);

                plane.move_to(MovementDirection::Left);

                assert_that!(plane.snake[0].x, is(equal_to(1)));
                assert_that!(plane.snake[0].y, is(equal_to(0)));
            }

            #[test]
            fn it_updates_head_direction_with_the_specified_one() {
                let mut plane = Plane::new(20, 20);

                plane.move_to(MovementDirection::Down);

                assert_that!(&plane.snake[0].direction,
                             is(equal_to(&MovementDirection::Down)));
            }
        }
    }
}
