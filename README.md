# pongDaddy

A classic Pong game implementation using Rust and Raylib.

## Description

This is a simple Pong game featuring both single-player and multiplayer modes. The game runs for 60 seconds, after which the player with the highest score wins.

## Features

- Single-player mode (play against AI)
- Multiplayer mode (two players)
- 60-second time limit
- Score tracking
- Smooth paddle and ball movements

## Prerequisites

- Rust and Cargo installed
- raylib library installed

## How to Play

### Controls
- Player 1:
  - W: Move paddle up
  - S: Move paddle down

- Player 2 (Multiplayer mode):
  - Up Arrow: Move paddle up
  - Down Arrow: Move paddle down

### Game Modes
1. Press `1` for multiplayer mode  
2. Press `2` for single-player mode  
3. Press `ESC` to exit

## Project Structure
```
Pong/
├── src/
│   ├── main.rs        # Game initialization and main loop
│   ├── ball.rs        # Ball physics and rendering
│   ├── paddle.rs      # Paddle movement and rendering
│   └── game_modes.rs  # Single and multiplayer mode logic
├── Cargo.toml
└── README.md
```

