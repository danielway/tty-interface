# TTY Interface

Provides simple TTY-based user interface capabilities including partial re-renders of multi-line displays. Uses 
[Termion](https://crates.io/crates/termion) for basic TTY terminal interactions, though consumers of TTY Interface 
need not depend on Termion directly themselves; TTY Interface will accept and output to any `Write` writer.

## Overview

Fundamentally, this crate introduces a data structure and API for describing and interacting with a terminal-based
user interface. This allows for partial/differential updates and an abstraction away from having to arrange ANSI escape 
sequences for cursor movement, line clearing, etc.

To accomplish this, a terminal interface is broken into lines with each line containing a number of segments. This allows 
for differential updates defined to the level of a segment within a line, with the specificity/scope of that segment 
being left to the user. Larger segments mean more of the screen must be cleared/re-written to make an update, smaller 
segments may require more complexity for the user to manage.

Updates to the interface are performed in batches. A batch contains a list of staged changes (referred to as "steps") 
which can then be applied to the terminal at the user's discretion. The steps described by a batch are applied in the 
order they're staged, and it is up to the user how many changes are included in a batch, while bearing in mind there 
are some operations performed automatically for every batch (e.g. cursor restoration and writer flushing).

## Usage

To start, here's an excerpt from `examples/helloworld.rs`:

```rust
// Initialize TTY Interface with stdout
let mut stdout = std::io::stdout();
let mut tty = TTYInterface::new(&mut stdout);

// Start a batch which contains interface changes staged for display
let mut batch = tty.start_update();

// Add/stage setting a line of the interface to "Hello, world!"
batch.set_line(0, Line::new(vec![
    Segment::new("Hello, world!".to_string())
]));

// Apply the update to the interface, thereby pushing the changes to stdout
tty.perform_update(batch)?;

// End the session with TTY Interface which resets the terminal
tty.end()?;
```

To begin, we initialize `TTYInterface::new(write: Writer)` which will both maintain the state of our interface and hold 
a reference to the writer for the life of the interface. Once we have a `TTYInterface`, we can create an `UpdateBatch` 
by calling `.start_update()`. This batch holds a list of staged changes (referred to as "steps") which may include 
setting the cursor's position, setting lines, setting segments within a line, deleting lines, or deleting segments 
within a line. Once a batch has had its changes staged, it may be pushed to the terminal by calling 
`.perform_update(batch)` on the `TTYInterface`. This will apply the changes in the order they were staged, restore the 
cursor's pre-update location, and flush the writer.