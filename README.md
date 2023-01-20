# snek
A from-scratch(ish) implementation of the classic snake game.

## This is a work-in-progress and is not yet functional!

## What is it?

This is my attempt at writing a basic snake game without the aid of a game engine.
It's probably a terrible idea given that I've never written snake *with* a game engine either, but yolo.

It uses [minifb](https://github.com/emoon/rust_minifb) for window creation and drawing. It also includes some font rendering
code that was shamelessly copied and pasted from one of the examples.

## Progress

As of this writing (15de5ce), there is a single white square sprite that is constrained to the window and can moved in the cardinal directions with WASD. The code is a messy rats nest of scaffolding, but I'm still on the prototyping stage. Next on the todo is better font rendering but that is a rabbit hole all its own. Once that's done, the plan is to step back, asses, and refactor as necessary to start setting up a more permanent structure in the code with less awfulness.