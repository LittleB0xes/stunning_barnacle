# stunning_barnacle
**Play with life and Rust**


After seeing this wonderful [video](https://www.youtube.com/watch?v=0Kx4Y9TVMGg&t=455s), I wanted to have a little fun with Rust to see what it looks like.
It's a work in progress, and just for fun...

For this project, i use (~~[Rust-SDL2](https://github.com/Rust-SDL2/rust-sdl2)~~) [Macroquad](https://macroquad.rs) for rendering

It's still a work in progress...

Actually, it uses a naive approach for collision checking. A future step will be to use quadtrees to make this more efficient.

## How to use it
There are currently 4 types of particles and the possibility of adjusting the forces of attraction/repulsion between each of them by clicking on either side of the colored rectangle.

The `A` key allows you to randomize the interaction rules.

The `Z` key is used to generate a new starting position of the particles.

## Some result

![](https://github.com/LittleB0xes/stunning_barnacle/blob/main/screenshots/gif_3.gif)

![](https://github.com/LittleB0xes/stunning_barnacle/blob/main/screenshots/gif_1.gif)

![](https://github.com/LittleB0xes/stunning_barnacle/blob/main/screenshots/gif_2.gif)
