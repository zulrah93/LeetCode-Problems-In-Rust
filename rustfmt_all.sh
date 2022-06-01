#!/usr/bin/env bash

for filename in *.rs; do # Loop through all Rust source files

    echo "Formating With Rust Tool [rustfmt]: {$filename}"
    rustfmt $filename

done