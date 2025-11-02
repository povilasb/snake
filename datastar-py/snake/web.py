import asyncio
from pathlib import Path
import typing as t

import uvicorn
from fastapi import FastAPI
from fastapi.responses import HTMLResponse

from datastar_py.fastapi import (
    DatastarResponse,
    ReadSignals,
    ServerSentEventGenerator,
)
from datastar_py.sse import DatastarEvent

from snake._game import Game, Cell

app = FastAPI()
_GAME = Game(grid_size=(30, 20))

# TODO: try patching only the signals: food and snake body
# TODO: dark theme


class _RenderHtml:
    def __init__(self, game: Game):
        self._game = game
        self._cell_width_px = 20
        self._cell_height_px = 20

    def render_html(self) -> str:
        grid_width_px = self._game.grid_width * self._cell_width_px
        grid_height_px = self._game.grid_height * self._cell_height_px

        if self._game.is_over:
            return f"""
            <svg id="grid" width="{grid_width_px}px" height="{grid_height_px}px" style="background-color: white;">
                <text x="50%" y="50%" text-anchor="middle" dominant-baseline="middle" font-size="20px" fill="black">Game Over</text>
            </svg>
            """

        return f"""
        <svg id="grid" width="{grid_width_px}px" height="{grid_height_px}px">
            {"".join(self._render_cell(cell) for cell in self._game.curr_state())}
        </svg>
        """

    def _render_cell(self, cell: Cell) -> str:
        match cell.state:
            case "food":
                color = "#b3e5fc"  # light blue
            case "head":
                color = "yellow"
            case "body":
                color = "green"
            case _:
                assert False, f"Unknown cell state: {cell.state}"

        return f'''<rect
            x="{cell.coord.x * self._cell_width_px}" y="{cell.coord.y * self._cell_height_px}"
            width="{self._cell_width_px}" height="{self._cell_height_px}" fill="{color}"
        />
        '''


@app.get("/")
async def main_page():
    html = (Path(__file__).parent / "_game.html").read_text()
    return HTMLResponse(html)


@app.get("/on-load")
async def on_load(signals: ReadSignals):
    if _GAME.is_over:
        _GAME.reset()
    return DatastarResponse(_stream_game_updates())


@app.post("/key-down")
async def key_down(signals: ReadSignals):
    match signals.get("key_pressed"):
        case "ArrowUp":
            _GAME.direction = "up"
        case "ArrowDown":
            _GAME.direction = "down"
        case "ArrowLeft":
            _GAME.direction = "left"
        case "ArrowRight":
            _GAME.direction = "right"
        case _:
            pass


async def _stream_game_updates() -> t.AsyncGenerator[DatastarEvent]:
    render_html = _RenderHtml(_GAME)
    while not _GAME.is_over:
        _GAME.tick()
        yield ServerSentEventGenerator.patch_elements(render_html.render_html())
        await asyncio.sleep(0.1)


if __name__ == "__main__":
    uvicorn.run("snake.web:app", reload=True)
