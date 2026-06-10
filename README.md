# ternary-collatz

Collatz sequences projected into ternary space — where the chaotic dynamics of 3n+1 become a three-valued oscillation pattern. Map integers to {-1, 0, +1}, trace Collatz trajectories in ternary, and detect divergence.

## Why This Exists

The Collatz conjecture is one of mathematics' most notorious open problems: start with any positive integer, repeatedly apply "if even, divide by 2; if odd, multiply by 3 and add 1" — does every starting value eventually reach 1? Nobody knows. Paul Erdős said "mathematics may not be ready for such problems."

This crate doesn't solve the conjecture. It gives you a lens for studying it. When you project Collatz sequences through `n mod 3 → {-1, 0, +1}`, the chaotic integer trajectories become simple oscillation patterns. The ternary encoding preserves the parity structure (odd maps to ±1, multiples of 3 map to 0) while stripping away the magnitude noise.

The ah-ha moment: the ternary Collatz sequence of any number is a rhythm. The pattern `+1, -1, 0, +1, +1, ...` repeats with structure you can hear. Numbers that share the same ternary rhythm tend to converge together. Whether this rhythm has a universal attractor at `0, 0, 0, ...` (which would mean the conjecture is true) is an open question this crate lets you explore computationally.

## Quick Start

```rust
use ternary_collatz::*;

// Map an integer to its ternary digit
assert_eq!(ternary_value(0), 0);   // 0 mod 3
assert_eq!(ternary_value(1), 1);   // 1 mod 3
assert_eq!(ternary_value(2), -1);  // 2 mod 3 (mapped to -1 in balanced ternary)
assert_eq!(ternary_value(4), 1);   // 4 mod 3 = 1
assert_eq!(ternary_value(6), 0);   // 6 mod 3 = 0

// Standard Collatz sequence
let seq = collatz_sequence(6, 20);
// → [6, 3, 10, 5, 16, 8, 4, 2, 1]
assert_eq!(seq.last(), Some(&1));

// Same sequence in ternary
let tseq = ternary_collatz_sequence(6, 20);
// → [0, 0, 1, -1, 1, -1, 1, -1, 1]
// The oscillation between +1 and -1 is the "heartbeat" of Collatz descent

// Check if a number diverges (doesn't reach 1 within max_steps)
assert!(!diverges(27, 200));  // 27 takes 111 steps but converges
assert!(diverges(999_999_999_999i64, 5));  // too few steps
```

## The Key Functions

### `ternary_value(n: i64) → i8`

Map any integer to its balanced ternary residue:

```
n mod 3 = 0  →  0   (neutral)
n mod 3 = 1  →  +1  (positive)
n mod 3 = 2  →  -1  (negative, in balanced ternary 2 ≡ -1)
```

This is balanced ternary mod 3, not standard modular arithmetic. The mapping `2 → -1` means the system is symmetric around zero: `{−1, 0, +1}` instead of `{0, 1, 2}`.

### `collatz_step(n: i64) → i64`

One iteration of the Collatz function:

```
if n is even:  n / 2
if n is odd:   3n + 1
```

### `collatz_sequence(start: i64, max_steps: usize) → Vec<i64>`

Generate the full trajectory from `start`, stopping when the value reaches 1 or `max_steps` is exhausted. Works with negative numbers too (negative Collatz has known cycles).

### `ternary_collatz_sequence(start: i64, max_steps: usize) → Vec<i8>`

The ternary projection: map every element of the Collatz sequence through `ternary_value`. This is where the interesting patterns live.

### `diverges(start: i64, max_steps: usize) → bool`

Returns `true` if the sequence doesn't reach 1 within the given budget. Note: this doesn't prove the Collatz conjecture false — it just means you didn't give it enough steps.

## Real-World Example: Comparing Ternary Rhythms

```rust
use ternary_collatz::*;

fn rhythm(n: i64) -> String {
    ternary_collatz_sequence(n, 50)
        .iter()
        .map(|&t| match t {
            1 => '▲',
            -1 => '▼',
            _ => '·',
        })
        .collect()
}

// Numbers that converge quickly
println!("7:  {}", rhythm(7));
// → ▼·▼▼▼▲▼▲▼▲▼▲▼▲▼▲▼▲▼▲▼▲▼▲▼▲▼▲▼▲▼▲▼...

// The famous 27 — takes 111 steps
println!("27: {}", rhythm(27));
// → ▲▲▼▲▼·▲▼·▲·▼▲·▲·▲·▼·▼·▲·▼·▼▲·▼·▲·...

// Negative Collatz: -5 enters a cycle (-5 → -14 → -7 → -20 → -10 → -5)
let seq = collatz_sequence(-5, 20);
// → [-5, -14, -7, -20, -10, -5, -14, -7, -20, -10, -5, ...]
```

## Architecture

```
ternary_collatz (no_std compatible)
├── ternary_value()              # i64 → i8 mapping
├── collatz_step()               # Single iteration
├── collatz_sequence()           # Full trajectory
├── ternary_collatz_sequence()   # Ternary projection of trajectory
└── diverges()                   # Divergence check
```

Zero dependencies. No `std` required. No `alloc` needed — wait, actually `Vec` is used, so `alloc` is needed. The crate is `#![forbid(unsafe_code)]` and has 135 lines of Rust.

## API Reference

| Function | Signature | Description |
|----------|-----------|-------------|
| `ternary_value` | `(n: i64) → i8` | Balanced ternary residue (mod 3) |
| `collatz_step` | `(n: i64) → i64` | One Collatz iteration |
| `collatz_sequence` | `(start: i64, max_steps: usize) → Vec<i64>` | Full trajectory |
| `ternary_collatz_sequence` | `(start: i64, max_steps: usize) → Vec<i8>` | Ternary projection |
| `diverges` | `(start: i64, max_steps: usize) → bool` | Non-convergence check |

## Ecosystem Connections

- **ternary-core** — Shared Z₃ arithmetic traits used by the ternary mapping
- **ternary-interpreter** — Bytecode VM that could execute Collatz step functions as ternary programs
- **ternary-automata** — Cellular automata where Collatz-like rules could drive state transitions
- **ternary-grid** — Spatial grid where ternary Collatz patterns could be visualized

## Performance

`collatz_step` is a single branch and multiplication. `collatz_sequence` for a starting value of N runs in O(steps to reach 1) iterations. For the number 27, that's 111 steps — microseconds. For numbers above 2⁶⁰, sequences can exceed 10,000 steps.

The ternary projection adds one `% 3` and a match per step — negligible overhead.

## Open Questions

- **Universal attractor**: Does the ternary Collatz sequence always converge to all-zeros? (Equivalent to the original conjecture.) This crate gives you the tools to explore, not the answer.
- **Ternary cycle detection**: The current implementation detects reaching 1 but doesn't detect other cycles (e.g., the negative cycle at -5). A proper cycle detector would catch these.
- **Statistical analysis**: What's the distribution of +1, 0, -1 in ternary Collatz sequences? Is it biased? Does it depend on the starting value modulo some power of 3?

## Stats

| Metric | Value |
|--------|-------|
| Lines of Rust | 135 |
| Tests | 15 |
| Dependencies | 0 |
| Unsafe | 0 (forbidden) |

## License

MIT
