# ternary-collatz

**The Collatz conjecture in ternary space. Where number theory meets cellular automata.**

Take any positive integer. If it's even, divide by 2. If it's odd, multiply by 3 and add 1. Repeat. The Collatz conjecture says you'll always reach 1 eventually. Nobody has proven it. It's one of the most notorious open problems in mathematics, and it's devastatingly simple.

In ternary, the Collatz sequence takes on a different character. Each intermediate value is mapped to {-1, 0, +1} based on its residue mod 3, creating a ternary sequence that traces the Collatz trajectory through mod-3 space. The resulting patterns are deeply connected to the ternary number system — where base-3 representation makes the parity structure visible in a way base-10 obscures.

## What's Inside

- **`collatz_step(n)`** — one step of the Collatz function. Returns the next value
- **`collatz_sequence(start)`** — full sequence from start to 1 (or until max steps)
- **`ternary_collatz(start)`** — map a Collatz sequence to ternary residues (mod 3)
- **`stopping_time(n)`** — how many steps to reach 1?
- **`max_value(n)`** — the highest value reached during the sequence
- **`collatz_grid(width, height)`** — visualize stopping times as a 2D grid (each cell = one starting value)
- **`is_divergent(n, max_steps)`** — does the sequence reach 1 within max_steps?

## Quick Example

```rust
use ternary_collatz::*;

// Classic Collatz sequence
let seq = collatz_sequence(27);
// 27 → 82 → 41 → 124 → 62 → 31 → 94 → 47 → ... → 1
println!("Steps: {}", seq.len());
println!("Max value: {}", seq.iter().max().unwrap());

// Ternary mapping: what does it look like in mod-3 space?
let ternary = ternary_collatz(27);
// Each value mod 3, mapped to {-1, 0, +1}
// The pattern reveals the parity structure

// Stopping time: how long does each number take?
for n in 1..=20 {
    print!("{}→{} ", n, stopping_time(n));
}
// 1→0 2→1 3→7 4→2 5→5 6→8 7→16 ...

// Grid visualization: stopping times as a heat map
let grid = collatz_grid(20, 20);
// Cell (x,y) = stopping_time(x + y*width)
```

## The Deeper Truth

**Collatz in base-3 is self-revealing.** The 3n+1 operation becomes trivially visible in ternary: multiplying by 3 shifts left one digit (like ×10 in decimal), and adding 1 increments the least significant trit. So every odd step in Collatz is just "shift left, increment" — the ternary representation shows the mechanical structure of the operation that decimal hides.

The stopping time grid is the real art piece: when you plot stopping times for consecutive integers as a 2D grid, fractal-like patterns emerge. The short-stopping numbers (powers of 2) form regular grids. The long-stopping numbers cluster in bands. The overall pattern has the visual complexity of a cellular automaton — which it effectively is, just running on a different substrate.

**Use cases:**
- **Number theory education** — the simplest open problem in mathematics, made visual
- **Generative art** — stopping time grids produce striking visual patterns
- **Sequence analysis** — study the structure of Collatz sequences in ternary
- **Cryptography** — Collatz-like functions as one-way hash candidates
- **Algorithmic music** — map stopping times to rhythms (each number = one beat)

## See Also

- **ternary-fib** — another number-theoretic sequence (Fibonacci) in ternary, with period-8
- **ternary-life** — cellular automaton (deterministic dynamics, similar visual complexity)
- **ternary-complexity** — complexity measures that can be applied to Collatz sequences
- **ternary-visualizer** — render stopping time grids as ASCII art

## Install

```bash
cargo add ternary-collatz
```

## License

MIT
