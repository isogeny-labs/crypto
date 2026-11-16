# Rank 1: Appendix D.4 symbolic guard audit

Date: 2026-11-01

Source revision: `3d76699b00`

Implementation: `crates/hashed-ecdl-tensor/src/contraction/guards.rs`

## Question and claim boundary

The public recursive contraction computes
`u = floor(ceil(log2(s)) / 2^17)` while representing the edge count `s` with
`usize`.  Thus every natively representable invocation has `u = 0` and cannot
enter the advertised recursive regime.  The experiment asks whether the source
parameters and displayed Appendix D.4 proof invariants become mutually
admissible when evaluated symbolically, without allocating the corresponding
graph.

This is a parameter audit.  It does not validate the contraction identities,
prove the theorem, measure a runtime, or establish an ECDLP speedup.

## Method

For the exact model `s = 2^lambda`, `C = 1`, arbitrary-precision integer
arithmetic evaluates every potentially active depth.  The transcript records:

- the three source guards `M >= 2 Darr^2 Delta`, `M >= L Darr`, and
  `Delta >= 2L`;
- nonzero `u_d` and the active-edge lower bound;
- the proof-side conditions `u_d^5 >= lambda^4`,
  `a_d <= 4 epsilon lambda`, `L <= s^(1/1000)`, and
  `Darr <= s^(1/1000)`;
- whether the same `lambda` can be represented by the native implementation.

The scan covers every `lambda = 2^x` for `0 <= x <= 520`.  The point directly
before the first pass is retained as the negative control.  Unit tests also
check that every native `usize` value gives `u = 0`.

## Result

| Check | Exact result | Interpretation |
| --- | --- | --- |
| First passing power-of-two grid point | `lambda = 2^480` | all source guards and proof invariants pass at all 80 active depths |
| Immediately preceding grid point | `lambda = 2^479` | all source guards pass; only depth 79 fails `u_d^5 >= lambda^4` |
| Native implementation | `lambda <= usize::BITS`, hence `u_0 = 0` | recursive path is unreachable for a materialized native graph |
| Focused tests | 3 passed, 0 failed | boundary, nonzero-root and native-representation controls pass |

At the first passing grid point,

```text
lambda =
3121748550315992231381597229793166305748598142664971150859156959625371738819765620120306103063491971159826931121406622895447975679288285306290176
```

and `s = 2^lambda`.  The finite 10--16-bit panel therefore did not exercise
this recurrence, and no realizable native-size experiment can do so.

## Reproduction

```bash
CARGO_HOME=/tmp/tensor-cargo RUSTUP_HOME=/tmp/tensor-rustup \
  /tmp/tensor-cargo/bin/cargo +1.78.0 test \
  --manifest-path crates/hashed-ecdl-tensor/Cargo.toml \
  --locked --release contraction::guards::tests

crates/hashed-ecdl-tensor/target/release/tensor-guards scan 0 520
```

The CLI emits the complete JSON transcript.  The scan claim is deliberately
limited to the power-of-two grid; it is not the exact threshold among all
integer values of `lambda`.

## Decision

The native recursive implementation is **exhausted within its representation
scope**: it cannot reach even a nonzero root parameter.  The symbolic recurrence
is **not disproved**: its registered sufficient guards do become admissible,
but only at an input scale far beyond materialization.  The rank-1 theorem lane
remains open pending identity-level differential checks; its current finite
fallback remains rejected by the separately retained complete-cost panel.
