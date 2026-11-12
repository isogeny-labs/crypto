# Chained ECDLP transfers over prime fields — verified construction, measured boundary

**Status:** research construction + measurement. Same characteristic only. No
cryptanalytic speedup; no cross-characteristic claim. Every number here is
`[pcc:unverified]` until CI's pcc gate signs the claims.

Protocol frozen in `PROTOCOL.md` (`frozen.sha256`) before any measurement.
Builds on PR #3877 (toy genus-2 transfer), promoted to native Rust.

## What this adds over PR #3877

PR #3877 proved `f_*f^* = [2]` algebraically and checked a toy `F_101` [pcc:unverified]
instance in Python. This effort:

1. **Native Rust, arbitrary divisors.** `src/cryptanalysis/chained_transfer/` [pcc:unverified]
   implements `F_p`/`F_{p²}`, elliptic curves with Vélu isogenies, and the
   bielliptic quotient maps `φ^*` (pull-back) and `φ_*` (push-forward) on
   **arbitrary reduced Mumford divisors**, including non-split ones (roots
   over `F_{p²}`), built on the repository's `genus2_fp` Cantor arithmetic.
   This closes the "arbitrary nonsplit divisor" gap named in
   `problems/prime-cover-balanced-jacobian-and-induced-maps-missing.md`.
2. **Verified chains.** A transfer *hop* checks `h(P) ≠ 0`, `ord = r`, and
   `h([x]P) = [x]h(P)` before it is trusted; a *chain* `E → E' → J(C)`
   (isogeny then pull-back) is verified end to end to preserve the order-`r`
   discrete logarithm.
3. **Measured total cost.** `chained_transfer_bench` solves the transferred
   instance four ways and compares counted group operations against a matched
   Pollard-rho reference on the source curve.

## Results

See `RESULTS.md`, `results.json` and `REPORT.pdf`; the replay-corrected rerun is `results_v2.json` and `REPORT_v2.pdf`. The result is reported as an
unverified assertion (no independent validator has signed it). Summary: the
chain is verified at p = 211 to 1619. Rho on each destination stays within the
scatter of the source rho. Genus-2 index calculus on J(C) costs 9.8× to 42.9×
the matched rho, and the fit gives about p^1.0. This is a boundary, not an
improvement. [pcc:unverified]

## Reproduce

```
cargo test  --release --lib chained_transfer
cargo run   --release --bin chained_transfer_bench -- run --json \
    research/chained_prime_transfer_20261111/results.json
```

## Scope and obstruction

The genus-2 destination is not cheaper: see
`problems/prime-field-chained-transfer-no-advantage.md`. The boundary is the
`Ω(p)` relation count of genus-2 index calculus over `F_p` against rho's
`√p` — a structural obstruction, not a tuning gap.
