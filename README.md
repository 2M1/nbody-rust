# NBody Simulation in rust

This is a (currently) simple NBody simulation written in rust.

## Usage

You can run the simulation with `cargo run --release -- coordinatesfile.csv`.

This repository contains some sample coordinates files in `examples/`, if you want to generate your own custom coordinates, you can use the `generate_coordinates.py` script.

## Why?

This is a project I started to learn rust. I'll try to gradually go from a simple simulation to a more complex one (single threaded, threaded, openMPI).

## TODO

### Sequential (Single Threaded)

-   []: Add reasonable units to the simulation
-   []: command line arguments for the simulation / interface for parameters
-   []: Zoom in / out with the mouse

### Multithreaded

-   []: Add multithreading :)

### OpenMPI

-   []: Figure out how to use OpenMPI in rust
-   []: implement OpenMPI version
