*Conway's Game of Life Simulation*

This repository contains an implementation of Conway's Game of Life, a fascinating cellular automaton that demonstrates complex behaviors emerging from simple rules.

*Overview*

Conway's Game of Life operates on a 20x20 board where cells evolve through successive generations based on their neighbors. The board wraps around at the edges, ensuring every cell always has eight neighbors.

*Rules of the Game:*
* Each cell's fate is determined by the sum of its eight neighbors:
* Sum = 2: The cell remains in its current state (alive or dead).
* Sum = 3: A cell is born or stays alive.
* Other sums: The cell dies (if alive) or remains dead.
* The simulation iterates for 20 generations starting with the following initial live cell positions: [(0,1), (1,2), (2,0), (2,1), (2,2)].

*Features*

* Initialization: A predefined set of live cells to start the simulation.
* Generation Updates: Uses two matrices to manage the current and next generation states.
* Edge Wrapping: Cells on the edges interact with those on the opposite sides using the modulo operator.
* Modular Design: A separate function calculates a cell's liveness based on its neighbors.
* Testing: Includes tests to validate the correctness of cell liveness calculations.


*How It Works*

1. The program initializes the board with live cells at specified positions.

2. In each generation:
 * Every cell's state is updated based on the sum of its neighbors.
 * A new board is generated while the old one is cleared and swapped.
3. The simulation runs for 20 iterations, with the state of the board displayed after each step.

*Testing*

To ensure correctness, the program includes unit tests for the cell liveness function, verifying outcomes for various neighborhood configurations.
