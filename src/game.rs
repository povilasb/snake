use std::clone::Clone;

use rand::{thread_rng, Rng};

#[derive(Clone, PartialEq, Debug, Hash, Eq)]
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
    pub direction: MovementDirection,
}

impl Cell {
    fn new(x: usize, y: usize, direction: MovementDirection) -> Cell {
        Cell { x, y, direction }
    }

    /// Generates new cell with random pozition within given ranges.
    ///
    /// Direction is undefined.
    pub fn random(max_x: usize, max_y: usize) -> Cell {
        let mut rng = thread_rng();
        Cell {
            x: rng.gen_range(0, max_x + 1),
            y: rng.gen_range(0, max_y + 1),
            direction: MovementDirection::Right,
        }
    }
}

/// Limited space game plane.
///
/// It's where snake and it's food exist.
pub struct Plane {
    width: usize,
    height: usize,
    pub snake: Vec<Cell>,
    pub food: Cell,
}

impl Plane {
    pub fn new(width: usize, height: usize) -> Plane {
        let body1 = Cell::new(0, 0, MovementDirection::Right);
        let body2 = Cell::new(1, 0, MovementDirection::Right);
        let head = Cell::new(2, 0, MovementDirection::Right);
        let snake = vec![head, body2, body1];

        let mut plane = Plane {
            width,
            height,
            snake,
            food: Cell::random(width - 1, height - 1),
        };
        plane.randomize_food();
        plane
    }

    pub fn move_to(&mut self, direction: &MovementDirection) {
        for i in (1..self.snake.len()).rev() {
            self.snake[i] = self.snake[i - 1].clone();
        }
        match *direction {
            MovementDirection::Left => self.move_left(),
            MovementDirection::Right => self.move_right(),
            MovementDirection::Up => self.move_up(),
            MovementDirection::Down => self.move_down(),
        };
        self.snake[0].direction = direction.clone();
        if self.head_on_food() {
            self.eat_food();
        }
    }

    /// Places food cell in a random location on a game plane.
    // TODO: make it private?
    pub fn randomize_food(&mut self) {
        let mut on_snake = true;
        while on_snake {
            self.food = Cell::random(self.width - 1, self.height - 1);
            on_snake = self.snake.iter().map(|cell| (cell.x, cell.y)).any(
                |coords| {
                    coords == (self.food.x, self.food.y)
                },
            );
        }
    }

    fn eat_food(&mut self) {
        self.snake.push(self.food.clone());
        self.randomize_food();
    }

    /// Tests snake head and food collision.
    fn head_on_food(&self) -> bool {
        self.snake[0].x == self.food.x && self.snake[0].y == self.food.y
    }

    fn move_left(&mut self) {
        self.snake[0].x = match self.snake[0].x {
            0 => self.width - 1,
            x => x - 1,
        };
    }

    fn move_right(&mut self) {
        self.snake[0].x = if self.snake[0].x >= self.width - 1 {
            0
        } else {
            self.snake[0].x + 1
        };
    }

    fn move_up(&mut self) {
        self.snake[0].y = match self.snake[0].y {
            0 => self.height - 1,
            y => y - 1,
        };
    }

    fn move_down(&mut self) {
        self.snake[0].y = if self.snake[0].y >= self.height - 1 {
            0
        } else {
            self.snake[0].y + 1
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::*;

    mod cell {
        use super::*;
        mod random {
            use super::*;

            #[test]
            fn it_places_food_inside_plane() {
                let cell = Cell::random(20 - 1, 20 - 1);

                assert_that!(cell.x, is(less_than(20)));
                assert_that!(cell.y, is(less_than(20)));
            }
        }
    }

    mod plane {
        use super::*;
        mod move_to {
            use super::*;

            #[test]
            fn it_shifts_body_parts_right() {
                let mut plane = Plane::new(20, 20);

                plane.move_to(&MovementDirection::Right);

                assert_that!(plane.snake[1].x, is(equal_to(2)));
                assert_that!(plane.snake[1].y, is(equal_to(0)));
                assert_that!(plane.snake[2].x, is(equal_to(1)));
                assert_that!(plane.snake[2].y, is(equal_to(0)));
            }

            #[test]
            fn it_increases_head_x_when_direction_is_right() {
                let mut plane = Plane::new(20, 20);

                plane.move_to(&MovementDirection::Right);

                assert_that!(plane.snake[0].x, is(equal_to(3)));
                assert_that!(plane.snake[0].y, is(equal_to(0)));
            }

            #[test]
            fn it_decreases_head_x_when_direction_is_left() {
                let mut plane = Plane::new(20, 20);

                plane.move_to(&MovementDirection::Left);

                assert_that!(plane.snake[0].x, is(equal_to(1)));
                assert_that!(plane.snake[0].y, is(equal_to(0)));
            }

            #[test]
            fn it_updates_head_direction_with_the_specified_one() {
                let mut plane = Plane::new(20, 20);

                plane.move_to(&MovementDirection::Down);

                assert_that!(
                    &plane.snake[0].direction,
                    is(equal_to(&MovementDirection::Down))
                );
            }
        }

        mod randomize_food {
            use super::*;

            #[test]
            fn it_places_food_inside_plane() {
                let mut plane = Plane::new(20, 20);

                plane.randomize_food();

                assert_that!(plane.food.x, is(less_than(20)));
                assert_that!(plane.food.y, is(less_than(20)));
            }

            #[test]
            fn it_picks_any_place_except_where_snake_is() {
                let mut plane = Plane::new(20, 20);

                plane.randomize_food();

                assert_that!(
                    &vec![(0, 0), (1, 0), (2, 0)],
                    not(contains(vec![(plane.food.x, plane.food.y)]))
                );
            }
        }

        mod head_on_food {
            use super::*;

            #[test]
            fn it_returns_true_when_snake_head_coordinates_match_food_coordinates() {
                let mut plane = Plane::new(20, 20);
                plane.food = Cell::new(5, 5, MovementDirection::Up);
                plane.snake[0] = plane.food.clone();

                assert_that!(plane.head_on_food(), is(equal_to(true)));
            }

            #[test]
            fn it_returns_false_when_snake_head_coordinates_match_food_coordinates() {
                let mut plane = Plane::new(20, 20);
                plane.food = Cell::new(5, 5, MovementDirection::Up);
                plane.snake[0] = Cell::new(0, 0, MovementDirection::Up);

                assert_that!(plane.head_on_food(), is(equal_to(false)));
            }
        }

        mod eat_food {
            use super::*;

            #[test]
            fn it_appends_food_to_snake_body() {
                let mut plane = Plane::new(20, 20);
                plane.food = Cell::new(5, 6, MovementDirection::Up);

                plane.eat_food();

                let last_part = unwrap!(plane.snake.last());
                assert_that!(last_part.x, is(equal_to(5)));
                assert_that!(last_part.y, is(equal_to(6)));
            }
        }
    }
}
