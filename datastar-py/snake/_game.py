import typing as t
from dataclasses import dataclass
from random import randint

CellState = t.Literal["empty", "head", "body", "food"]
Grid = t.NewType("Grid", list[list[CellState]])
Direction: t.TypeAlias = t.Literal["up", "down", "left", "right"]


@dataclass
class Coord:
    x: int
    y: int

    def left(self) -> t.Self:
        return Coord(x=self.x - 1, y=self.y)

    def up(self) -> t.Self:
        return Coord(x=self.x, y=self.y - 1)

    def down(self) -> t.Self:
        return Coord(x=self.x, y=self.y + 1)

    def right(self) -> t.Self:
        return Coord(x=self.x + 1, y=self.y)


@dataclass
class Cell:
    state: CellState
    coord: Coord


class Game:
    def __init__(self, grid_size: tuple[int, int]):
        self.grid_width, self.grid_height = grid_size
        self.direction: Direction = "right"
        self.is_over = False

        head = Coord(x=self.grid_width // 2, y=self.grid_height // 2)
        self._snake = [head, head.left(), head.left().left()]
        self._food = self._place_food()

    def tick(self) -> None:
        match self.direction:
            case "up":
                head = self._snake[0].up()
            case "down":
                head = self._snake[0].down()
            case "left":
                head = self._snake[0].left()
            case "right":
                head = self._snake[0].right()

        self._move_body(head)
        self.is_over = self._is_game_over()
        self._maybe_eat_food()

    def curr_state(self) -> list[Cell]:
        """The current state of a grid that has food and snake in it."""
        return [
            Cell(state="food", coord=self._food),
            Cell(state="head", coord=self._snake[0]),
            *[Cell(state="body", coord=coord) for coord in self._snake[1:]],
        ]

    def _overflow_if_needed(self, coord: Coord) -> Coord:
        if coord.x < 0:
            return Coord(x=self.grid_width - 1, y=coord.y)
        elif coord.x >= self.grid_width:
            return Coord(x=0, y=coord.y)
        elif coord.y < 0:
            return Coord(x=coord.x, y=self.grid_height - 1)
        elif coord.y >= self.grid_height:
            return Coord(x=coord.x, y=0)
        else:
            return coord

    def _place_food(self) -> Coord:
        while True:
            food = Coord(
                randint(0, self.grid_width - 1), randint(0, self.grid_height - 1)
            )
            if food not in self._snake:
                return food

    def _move_body(self, head: Coord) -> None:
        for i in range(len(self._snake) - 1, 0, -1):
            self._snake[i] = self._snake[i - 1]
        self._snake[0] = self._overflow_if_needed(head)

    def _maybe_eat_food(self) -> None:
        if self._snake[0] == self._food:
            self._snake.append(self._food)
            self._food = self._place_food()

    def _is_game_over(self) -> bool:
        return self._snake[0] in self._snake[1:]
