# TTY Interface

[![Crate](https://img.shields.io/crates/v/tty-interface.svg)](https://crates.io/crates/tty-interface)
[![Rust CI](https://github.com/danielway/tty-interface/actions/workflows/rust_ci.yml/badge.svg?branch=master)](https://github.com/danielway/tty-interface/actions/workflows/rust_ci.yml)
[![Rust CD](https://github.com/danielway/tty-interface/actions/workflows/rust_cd.yml/badge.svg)](https://github.com/danielway/tty-interface/actions/workflows/rust_cd.yml)

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
