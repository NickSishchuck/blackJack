# Black-Jack

A command-line implementation of the classic Blackjack card game written in Rust.

( yeah, I know the name is incorrect. Sorry for that)

## Overview

Black-Jak is a simple terminal-based Blackjack game where you can play against a computer dealer. The game follows standard Blackjack rules where the goal is to get a hand value closer to 21 than the dealer without going over.

## Features

- Interactive command-line interface
- Standard Blackjack rules implementation
- Support for hitting and standing
- Automatic dealer AI that follows conventional rules (hits until 17)
- Multi-platform support (Windows and Linux)

## Game Rules

- The player and dealer are each dealt 2 cards
- Number cards are worth their face value, face cards (J, Q, K) are worth 10, and Aces are worth 11 or 1
- The player can choose to "hit" (take another card) or "stand" (keep current hand)
- The player can hit as many times as desired until they stand or bust (exceed 21)
- The dealer must hit until they reach a value of 17 or higher
- The player wins if their hand value is higher than the dealer's without busting
- The dealer wins if the player busts or if the dealer's hand value is higher without busting
- If both the player and dealer have the same hand value, it's a "push" (tie)

## Requirements

- Rust compiler and Cargo (Rust's package manager)
- For cross-compilation:
  - Windows target: `x86_64-pc-windows-gnu` toolchain
  - Linux target: `x86_64-unknown-linux-gnu` toolchain

## Installation

Clone the repository:

```bash
git clone https://github.com/yourusername/black-jak.git
cd black-jak
```

## Building

### Simple Build

Build the project using Cargo:

```bash
cargo build --release
```

The executable will be located in `target/release/black-jak`.

### Cross-Platform Build

The project includes a build script to compile for both Windows and Linux:

On Linux:
```bash
chmod +x your_script.sh
./build.sh
```

This will create a `builds` directory containing:
- `black-jak.exe` (Windows executable)
- `black-jak-linux` (Linux executable)

## How to Play

1. Run the executable:
   ```bash
   ./black-jak
   # or on Windows
   black-jak.exe
   ```

2. When prompted, indicate whether you're familiar with Blackjack rules by typing 'y' or 'n'.

3. The game will deal two cards to you and the dealer, showing you your cards and one of the dealer's cards.

4. On your turn, type 'h' to hit or 's' to stand.

5. After you stand, the dealer will take their turn according to standard rules.

6. The winner will be announced, and you'll be asked if you want to play again.

## Project Structure

- `src/main.rs` - Main game loop and user interface
- `src/card_operations.rs` - Functions for shuffling and displaying cards
- `src/ai.rs` - Game logic for calculating hand values and dealer decisions
- `src/rules.rs` - Functions for displaying game rules
- `build.sh` - Build script for multi-platform compilation


## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
