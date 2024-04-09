# Notes

## Before anything

When I started, there were many clippy warnings visible immediately.

Lots were related to mutability, one or two were unit values.

To reduce confusion I cleared all the warnings up and made a commit: `6f49195`

## Test outcomes

- `--help` output looks sane
- Basic newline wrapped output is sane (hw/hw2)
- Extensible tape is good (-c 1 -e hw.bf)
- Flushing is bad (primes)
- Performance is good (primes)
- General behaviour OK (game) (bad flushing)

Testing "failed" but not critically

## Documentation review

`cargo doc --workspace --open --no-deps`

- No warnings while building docs, which is nice.
- A little sparse, particularly at crate/module level but also sometimes at type level.

## Code skim

- Missing flush as expected given test outcomes
- Interpreter does not handle EOF on input cleanly
  - Propose future work, read <https://esolangs.org/wiki/Brainfuck#EOF_2> and decide
- Generally there is some more thought needed about structured error types

Pass (with action to fix flush and errors)
