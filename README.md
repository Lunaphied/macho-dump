# Mach-O Dump (`macho-dump`)

An `objdump` like tool for exploring and manipulating Mach-O files.

**Note:** This project is in an early stage and most of the features are yet to come.

## Features

The goal of this tool is to be a more ergonomic alternative to tools like `llvm-objdump` or binutils `objdump` for working with Mach-O binaries, including fat binaries and raw Mach-O binaries (something that `llvm-objdump` doesn't currently handle correctly).

Currently Implemented:
* Lists the segment names and the names of contained sections in a pretty format.

Planned:
* Improved command-line interface.
* List more details about binaries (section locations, load commands, code signing details, etc.).
* Formatted data output suitable for consumption by other tools (JSON based).
* Validating code signing signatures on binaries.
* And more... ~
