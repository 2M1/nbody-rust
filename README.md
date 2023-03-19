# NBody Simulation in rust

This is a (currently) simple NBody simulation written in rust.

## Usage

You can run the simulation with `cargo run --release -- coordinatesfile.csv`.

This repository contains some sample coordinates files in `examples/`, if you want to generate your own custom coordinates, you can use the `generator.py` script.

## Why?

This is a project I started to learn rust. I'll try to gradually go from a simple simulation to a more complex one (single threaded, threaded, openMPI).

## TODO

### Sequential (Single Threaded)

- [x] Add reasonable units to the simulation
- [ ] command line arguments for the simulation / interface for parameters
    - [x] Implement Parsing
    - [x] Use parameters for initialisation
    - [ ] Limit passes based on parameters (And show framecounter / time remaining o.s.)
- [ ] Zoom in / out with the mouse
    - [x] Basic Zoom
    - [ ] Show scale
    - [ ] better zoom curve (non-linear steps based on current zoom to zoom more accurately when close and faster when futher out)
    - [ ] Zoom into mouse position dependend centre (currently it just zooms around (0,0) )
- [ ] Persist results (output of each frame, can be made into mp4 using ffmpeg)


### Multithreaded

-   [ ] Add multithreading :)

### OpenMPI

-   [ ] Figure out how to use OpenMPI in rust
-   [ ] implement OpenMPI version
