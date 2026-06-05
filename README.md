# ternary-collatz

**The Collatz conjecture, viewed through ternary lenses.**

The Collatz conjecture is the most famous open problem in mathematics that a child can understand: pick a number. If even, halve it. If odd, triple it and add one. Repeat. Does every starting number eventually reach 1? Nobody knows.

This crate doesn't solve it. Instead, it asks a different question: **what does the Collatz sequence *look like* when you reduce every number to its residue mod 3 — mapped to `{-1, 0, +1}`?** The ternary projection reveals structure invisible in the raw integers. Patterns repeat. Ternary sequences cycle. The `3n+1` operation, viewed mod 3, is a deterministic walk on a three-state system.

## What's Inside

- **`ternary_value(n)`** — map any integer to its ternary digit (-1, 0, +1)
- **`collatz_step(n)`** — one step of the Collatz iteration
- **`collatz_sequence(start, max_steps)`** — full integer sequence until reaching 1
- **`ternary_collatz_sequence(start, max_steps)`** — the same sequence, projected to {-1, 0, +1}
- **`diverges(start, max_steps)`** — does the sequence fail to reach 1 in time?
- **`ternary_period(start, max_steps)`** — find the repeating period in the ternary projection

## Quick Example

```rust
use ternary_collatz::*;

// Classic Collatz: 7 → 22 → 11 → 34 → 17 → 52 → 26 → 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
let seq = collatz_sequence(7, 100);
assert_eq!(*seq.last().unwrap(), 1);

// Ternary projection: what does that look like mod 3?
let ternary = ternary_collatz_sequence(7, 100);
// [1, 1, -1, 1, -1, 1, -1, 1, 1, -1, 1, -1, 1, -1, 1, -1, 1]
// The oscillation between +1 and -1 reveals the structure

// Does it converge?
assert!(!diverges(7, 1000));
assert!(!diverges(999999, 10000)); // probably not — Collatz is stubborn
```

## Why Ternary Collatz?

**Reduction reveals structure.** The Collatz sequence in raw integers looks chaotic. But reduced mod 3, patterns emerge — the ternary projection has predictable transitions because the operations (divide by 2, multiply by 3 and add 1) have clean mod-3 behavior. This crate makes those patterns visible and programmatically accessible.

**Use cases:**
- **Number theory research** — explore Collatz structure in finite state spaces
- **Generative art** — ternary Collatz sequences produce distinctive visual patterns
- **Mathematics education** — make the Collatz conjecture tangible and interactive
- **Sequence analysis** — periodicity detection in ternary projections of integer sequences
- **Algorithmic music** — map ternary Collatz walks to pitch/rhythm

## Install

```bash
cargo add ternary-collatz
```

## License

MIT
