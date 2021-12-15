# Advent of Code 2021 Solutions

Library used for data: <https://github.com/wimglenn/advent-of-code-data>.

Main things:

* Fetch your token ([help](https://github.com/wimglenn/advent-of-code-wim/issues/1))
* Save the value in `~/.config/aocd/token`

## Downloading inputs

```bash
# Today's data
aocd > in.txt

# Data of a specific puzzle
aocd 3 2020 > in.txt
```

## Starting a new day

There's a script to start a new day:

```
./new_day.sh <day_number>
```

Running the script will create a new folder with the template files for a Python solution
under `python/<day_number>` as well as a new file for a Rust solution in `rust/src/day_<day_number>.rs`.

## Running the solutions

For Python:

* Go to the directory of the day you want to run (e.g. `cd python/04`)
* Run `python sol.py` to run the solution against your individual input
* ... or run `python sol.py X` to run the solution against the input stored in the file `sX.txt` in the same directory

For Rust:

* `rustup install nightly` (only needed before running anything for the first time)
* `cd rust`
* `cargo +nightly run` will run the solution for all days
* `cargo +nightly run X` will run the solution for day `X`
* `cargo +nightly test` will run all tests
* `cargo +nightly test X` will run tests for day `X`
* `cargo +nightly bench` will run all benchmarks
