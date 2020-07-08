# Snazzy - A low-level language for SNES development

`snazzy` is an attempt to create a language slightly above the 65816
assembly language used for SNES development. The goal is to still
provide assembly-level control (each statement is approximately one
instruction), but with a less obtuse syntax and basic structural
programming.

## What does it look like?

```
VAR inidisp := 0x2100;
VAR cgdata := 0x2122;

FUN main {
  # force-blanking is on from init, so we can set our registers

  # Load a nice dark blue into the palette.
  # cgaddr is already at address 0 from init
  A := 0x1C;
  cgdata:= 0;
  cgdata := A;

  # Disable force blank
  A := 0x0F;
  inidisp := A;

  # We're done - wait forever
  DO {}
}
```

This compiles to the following 65816 assembly:

```
main:
  LDA #$1C
  STZ $2122
  STA $2122
  LDA #$0F
  STA $2100
loop:
  BRA loop
```

## Supported Features

* Automatic adustment of mode flags on function calls and block entry/exit
* SEI/CLI instructions
* Some types of assignments
* Function calls
* nice names for registers/globals

## Installation

Since `snazzy` is still under early development, it must be install
from git:
```
cargo install --git https://github.com/branan/snazzy
```

## Missing Features

This is a bareones proof-of-concept at the moment, so most features
you'd want are missing, including but not limited to:

* More assignments
* Function locals
* Conditionals
* Non-infinite loops
* Math
* Multiple banks
* Adding data to the ROM image
* Many more

## What's with the name?

It's a compiler for the SNES, and "snazzy" sounds like "SNES-C"
