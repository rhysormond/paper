import {World} from "paper";
import {drawCells, drawGrid, getClickedCell, height, width} from "./canvas";

const render = require("./canvas.js");

const world = World.new(width, height);

let animationId = null;

const renderLoop = () => {
    debugger;
    world.tick();
    drawGrid();
    drawCells(world);
    animationId = requestAnimationFrame(renderLoop);
};

const isPaused = () => {
    return animationId === null;
};

const playPauseButton = document.getElementById("play-pause");

const play = () => {
    playPauseButton.textContent = "⏸";
    renderLoop();
};

const pause = () => {
    playPauseButton.textContent = "▶";
    cancelAnimationFrame(animationId);
    animationId = null;
};

playPauseButton.addEventListener("click", _ => {
    if (isPaused()) {
        play();
    } else {
        pause();
    }
});

render.canvas.addEventListener("click", event => {
    const [row, col] = getClickedCell(event);
    world.toggle_cell(row, col);
    drawGrid();
    drawCells(world);
});

play()
pause()
