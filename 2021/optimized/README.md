# AoC 2021 optimized Rust solutions

Heavily optimized solutions in Rust using lots of `unsafe`.

Build and run benchmarks with `cargo run --release -- --all`.

It expects the input to be in `../dayXX/input.txt` as provided in this repo.

Alternatively you can provide a directory with input files using `--input-directory <DIR>` in which case input files are expected at `<DIR>/dayXX.txt`.

You can provide expected results in a file `expected.txt` in the current or input directory or using `--expected-file <FILE>`. The expected format is one line per day with any whitespace separating the result for part 1 and 2.

All options:
- `-a`/`--all`: Benchmark all days.
- `-d <DAY>`/`--day <DAY>`: Benchmark a single day.
- `-i <NUM>`/`--iterations <NUM>`: Number of iterations between timings.
- `--no-check`: Don't check if results are correct.
- `--input-directory <DIR>`: Directory for inputs. See above.
- `--expected-file <FILE>`: File for expected inputs. Defaults to `expected.txt`.

## "Rules"

The timings don't include reading the file into a string but include input parsing.

The solutions ignore any edge-cases not found in official inputs and heavily exploit any patterns found in them.
Before execution, the read inputs are safety checked to avoid undefined behavior but any inputs that are safe but don't follow
the expected patterns may produce wildy incorrect results.

## Days

### Day 1

For part 2, when comparing 3-value windows,
it's enough to compare the first value of the first window with the last value of the last window
since the respective other two values are the same.

### Day 2

Nothing interesting going on here.