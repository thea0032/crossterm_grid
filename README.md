# Overview

This is a lightweight library for more complicated CLIs, with multiple elements. It's designed to make separating multiple UI elements easier.

It's also designed to be versatile. With rust's trait interface, you can make it output from any CLI API, even though it was designed with crossterm in mind.

## Usage

The prelude contains everything you can use (which isn't much).

I recommend you look at the documentation instead. Here's a short summary of what each structure does:

### Grid

Alignment: An enum that's used for input.

DividerStrategy: An enum that's used to decide where text is placed inside a DrawProcess.

Frame: A structure that's used to represent the entire terminal, and "saves" dimension data.

SplitStrategy: A structure that's used to decide how grids are split apart.

Grid: A structure that represents a section of a terminal.

### Out

Action: An enum that's used to represent either moving the cursor or drawing.

Handler: A trait for structures that can translate actions into output.

SafeHandler: A trait for handlers that don't return errors.

OutToString: A handler that writes text out to a string without regards for location.

StringBuffer: A handler that writes text onto a vector of strings with regards for location.

### Process

DrawProcess: Represents a chunk of the terminal that has been "activated". Text can be added and then printed.

### Trim

FormatError: Represents a problem with formatting. Currently only returned when there's no space for text.

TrimStrategy: A trait for structures that can translate text into trimmed text (text that fits a DrawProcess).

Ignore: A TrimStrategy that ignores whether or not text can fit. Just useful for debug and example purposes.

Split: A TrimStrategy that splits text into multiple lines if it doesn't fit.

Truncate: A TrimStrategy that removes all text that doesn't fit.

TrimmedText: The output of a TrimStrategy.

## Status

Should be completed.

### Completed features

1. Minimum functionality
2. Capability for use with other APIs
3. Ability to clear a DrawProcess to use it again

### Features being worked on

1. Replacing/deleting individual lines for DrawProcess
2. Option to forcefully "shove" code in if there's a section that doesn't fit
3. Manually setting lines instead of pushing them on (and handling conflicts - perhaps an additional trait: OneLineTrimStrategy?)

### Features to be added

1. Ansi code support

## License

This project is licensed under the MIT license.

  ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

## Chance log

v 0.1.1: Updated DrawProcess to add clear function.
         Fixed bug in documentation where a removed function was called.

v 0.1.0: Initial commit
