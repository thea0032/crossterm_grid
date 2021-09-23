# Overview

This is a lightweight library for more complicated CLIs, with multiple elements. It's designed to make separating multiple UI elements easier.

It's also designed to be versatile. With rust's trait interface, you can make it output from any CLI API, even though it was designed with crossterm in mind.

## Usage

The prelude contains everything you can use (which isn't much).

I recommend you look at the documentation instead. Here's a short summary of what each structure does:

### Grid module

1. Alignment: Can be left/right, top/bottom. Used in inputs.
2. DividerStrategy: Choose how chunks are formatted.
3. Grid: Create for the entire terminal, then break into chunks.
4. Chunk: Portion of grid. Converts into ChunkProcess, which you can use to display things.
5. GridStrategy: Used to help you choose how to break grid into chunks.

### Process module

1. FormatError: Returned if something goes wrong with formatting.
2. ChunkProcess: Use these methods to add content to chunk, and then print the content.
3. TrimStrategy: Decides what happens if the content you add is too big.

### Output module

1. Action: Used by handler to decide where the cursor moves and what things are printed.
2. Handler: This is a trait. Decide what happens when you move the cursor and print things.

### Crossterm module (optional feature)

1. CrosstermHandler: A sample implementation of Handler, for the crossterm API.

## Status

### Completed features

1. Minimum functionality
2. Capability for use with other APIs

### Features being worked on

BLANK

### Features to be added

1. Ansi code support
