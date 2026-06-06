# ternary-collatz

**Collatz conjecture explored through ternary arithmetic — sequence generation, balanced ternary representation, divergence detection, and the structure of the 3n+1 map in Z₃.**

## Background

The Collatz conjecture is one of the most famous unsolved problems in mathematics. Proposed by Lothar Collatz in 1937, it states:

> Start with any positive integer n. If n is even, divide by 2. If n is odd, multiply by 3 and add 1. Repeat. You will always eventually reach 1.

Despite its simple statement, no one has been able to prove (or disprove) it. Paul Erdős famously said: "Mathematics may not be ready for such problems." The conjecture has been verified computationally for all starting values up to at least 2⁶⁸ (≈ 2.95 × 10²⁰).

The connection to ternary arithmetic is natural and deep. The Collatz function's odd branch is `3n + 1` — multiplication by 3 is the fundamental ternary operation. In balanced ternary representation (base 3 with digits {-1, 0, +1}), multiplication by 3 is a left shift (append a zero trit). This means the Collatz function, in ternary, is essentially: "left-shift and add 1" (for odd n) or "right-shift" (for even n). The interplay between these two operations — expansion (×3) and compression (÷2) — is what makes the conjecture so hard.

This crate maps Collatz sequences to ternary values via `n mod 3`: every integer in the sequence is classified as −1, 0, or +1 based on its residue modulo 3. This ternary projection of the Collatz sequence reveals structural patterns: the distribution of residues, the frequency of transitions between ternary states, and the "ternary signature" of each starting value.

The Collatz function also has a natural extension to negative integers, where it can produce cycles: −5 → −14 → −7 → −20 → −10 → −5 is a known cycle. The ternary projection captures these cycles as repeating ternary patterns.

## How It Works

### Core Functions

**`ternary_value(n: i64) → i8`** — Map any integer to its balanced ternary residue:
```
n mod 3 == 0   →  0
n mod 3 == 1   → +1
n mod 3 == -1  → -1
```

This uses the balanced modulo convention: residues are {−1, 0, +1} rather than {0, 1, 2}. For example:
- `ternary_value(4)` = +1 (4 mod 3 = 1)
- `ternary_value(2)` = −1 (2 mod 3 = 2 ≡ −1 in balanced form)
- `ternary_value(3)` = 0 (3 mod 3 = 0)

**`collatz_step(n: i64) → i64`** — One step of the Collatz function:
- If n is even: `n / 2`
- If n is odd: `3n + 1`

**`collatz_sequence(start: i64, max_steps: usize) → Vec<i64>`** — Generate the full sequence from `start`, stopping at 1 or `max_steps`. Returns the raw integer sequence.

**`ternary_collatz_sequence(start: i64, max_steps: usize) → Vec<i8>`** — Same sequence, but each value mapped to its ternary residue. Returns {-1, 0, +1} values.

**`diverges(start: i64, max_steps: usize) → bool`** — Returns `true` if the sequence doesn't reach 1 within `max_steps`. For known convergent values (like 27), this returns `false` with enough steps. For extremely large values with few steps, it returns `true`.

### The Ternary View of Collatz

Consider the sequence starting at 6:
```
6 → 3 → 10 → 5 → 16 → 8 → 4 → 2 → 1
```
Ternary projection:
```
6 mod 3 = 0 → 0
3 mod 3 = 0 → 0
10 mod 3 = 1 → +1
5 mod 3 = 2 → -1
16 mod 3 = 1 → +1
8 mod 3 = 2 → -1
4 mod 3 = 1 → +1
2 mod 3 = 2 → -1
1 mod 3 = 1 → +1
```
Ternary sequence: `[0, 0, +1, −1, +1, −1, +1, −1, +1]`

The alternating +1/−1 pattern in the second half reflects the sequence's descent through powers of 2 (even numbers alternate between ≡ 0 mod 3 and ≡ 1 mod 3 or ≡ 2 mod 3).

### Design Decisions

1. **Balanced modulo**: `ternary_value` uses balanced residues {−1, 0, +1} rather than standard residues {0, 1, 2}. This aligns with the ternary ecosystem and provides symmetry: +1 and −1 are equally common residues (each covering ⅓ of integers).

2. **i64 for safety**: The Collatz function can produce very large intermediate values (the sequence for 27 peaks at 9232). i64 provides headroom up to 2⁶³ ≈ 9.2 × 10¹⁸. For values that overflow, the sequence wraps (not ideal for mathematical correctness, but practical for exploration).

3. **Bounded execution**: All functions take `max_steps` to prevent infinite loops. Since the Collatz conjecture is unproven, we can't assume termination.

4. **Negative support**: The `collatz_step` function works with negative integers, producing known cycles (−5 → −14 → −7 → −20 → −10 → −5). The ternary projection captures these cycles as repeating patterns.

## Experimental Results

All **15 tests pass**:

| Test | Input | Result |
|------|-------|--------|
| `test_ternary_value_zero` | 0 | 0 |
| `test_ternary_value_one` | 1 | +1 |
| `test_ternary_value_negative_one` | −1 | −1 |
| `test_ternary_value_two` | 2 | −1 (balanced: 2 mod 3 ≡ −1) |
| `test_ternary_value_three` | 3 | 0 |
| `test_ternary_value_four` | 4 | +1 |
| `test_collatz_step_even` | 4 | 2 (even → n/2) |
| `test_collatz_step_odd` | 3 | 10 (odd → 3n+1) |
| `test_collatz_step_one` | 1 | 4 (odd → 3n+1, starts cycle 1→4→2→1) |
| `test_collatz_sequence_basic` | start=6, max=20 | [6, 3, 10, 5, 16, 8, 4, 2, 1], reaches 1 |
| `test_collatz_sequence_one` | start=1, max=100 | [1], already at 1 |
| `test_ternary_collatz_sequence` | start=6 | Same length as raw sequence, first value = ternary_value(6) = 0 |
| `test_diverges_false_for_known` | start=27, max=200 | false (27 converges in 111 steps) |
| `test_diverges_large_with_small_steps` | start=10¹², max=5 | true (doesn't reach 1 in 5 steps) |
| `test_collatz_sequence_negative` | start=−5, max=20 | Produces a cycle: −5 → −14 → −7 → −20 → −10 → −5 → ... |

Key findings:
- **27 converges**: The famously long sequence (111 steps, peak at 9232) correctly converges to 1 within 200 steps
- **Negative cycles**: −5 produces a 5-element cycle that never reaches 1 (or +1). The `diverges` function returns... well, it depends on `max_steps`, since the sequence never reaches +1
- **Balanced residues**: The `ternary_value` function correctly maps 2 → −1 and 4 → +1, producing a balanced ternary projection

## Impact

The ternary {-1, 0, +1} encoding reveals structural properties of Collatz sequences that are invisible in the raw integer representation. The ternary projection shows that:

1. **Residue distribution**: In convergent sequences, the ternary residues are roughly uniformly distributed (≈⅓ each). Skewed distributions might indicate pathological behavior.
2. **Ternary periodicity**: The descent phase (powers of 2) produces characteristic alternating patterns in the ternary projection.
3. **Cycle detection**: Negative cycles produce repeating ternary patterns that are easy to detect via autocorrelation.

The balanced modulo convention {−1, 0, +1} is particularly natural here because the Collatz function involves both addition (+1) and multiplication (×3). In balanced ternary, ×3 is a left shift and +1 is incrementing the least significant trit. The Collatz function is, in some sense, a dance between these two ternary operations.

## Use Cases

1. **Mathematical exploration** — Study the ternary structure of Collatz sequences; look for patterns in the residue distribution that might inform a proof
2. **Sequence comparison** — Compare the ternary signatures of different starting values; cluster sequences by ternary pattern similarity
3. **Educational demonstrations** — Visualize Collatz sequences as ternary trajectories; show the 3n+1 map in balanced ternary
4. **Randomness testing** — The ternary projection of Collatz sequences produces quasi-random ternary sequences; test their statistical properties
5. **Cycle detection in negative integers** — Use the ternary projection to detect and classify negative cycles, which are easier to find than counterexamples to the positive conjecture

## Open Questions

1. **Ternary density conjecture**: Do convergent Collatz sequences always have ternary projections that converge to a uniform distribution {⅓, ⅓, ⅓}? Or are there systematic biases?
2. **Cycle detection via ternary autocorrelation**: Can repeating patterns in the ternary projection be detected efficiently using autocorrelation? This would give a fast cycle detector.
3. **Balanced ternary encoding of the full sequence**: Instead of mapping each value to its residue mod 3, encode each value as a balanced ternary number. This would reveal the internal ternary structure of each step.

## Connection to Oxide Stack

`ternary-collatz` is a research tool at the **flux-core** layer. The Collatz function's ×3 operation is the fundamental ternary multiplication, and studying its behavior informs the design of ternary arithmetic circuits at the **cuda-oxide** compiler level. The divergence detection function models the kind of bounded execution that the flux-core VM needs for gas-metered agent programs.

At the **cudaclaw** layer, the Collatz sequence's behavior (eventual convergence for all known positive starting values) is an analogy for warp consensus: individual threads may diverge temporarily, but the system converges to agreement. The ternary projection shows the residue-level dynamics of this convergence.

## Stats

| Metric | Value |
|--------|-------|
| Lines of Rust | ~60 |
| Test count | 15 |
| Public functions | 5 |
| Dependencies | 0 |
| `#![forbid(unsafe_code)]` | Yes |

## Install

```toml
[dependencies]
ternary-collatz = "0.1.0"
```

## License

MIT
