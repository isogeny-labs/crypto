# Round C: reciprocal reduction and prospective width retune

Status: prospective development protocol, frozen before the reduction candidate
was implemented or either arm produced output on 2026-11-18. Round ID:
`ghs-strong-rho-r3-reciprocal-reduce-retune`.

This round asks two separate questions. First, can the hot base-field product
replace the run-time `a * b % q` division with a fixed-modulus reciprocal
reduction while returning exactly the same residue and incrementing exactly
the same operation counters? Second, after that arithmetic change, what
deterministic count/correctness sensitivity is seen at batch widths 4, 8, and
16? The width cell is a prospective retune screen only. It cannot select a new
primary arm, establish throughput, amend the frozen Round A grid, or move a
frontier or leaderboard row `[pcc:unverified]`.

## Baseline, scope, and sources

The source baseline is revision
`ad2a014c0022cf000845014499a9649b6c2cef2d`; its Round B
`strong_rho.rs` has SHA-256
`0bb8da07393873aa2b935403b86f5b7a0c24a52fff2d598a145a9fcca103229c`.
The inherited family has `1 < q < 2^31`, and every operand entering the hot
product is canonical, so `x = a*b < q^2 < 2^62`.

For `R = 2^64`, the candidate precomputes `m = floor(R/q)` once per instance,
then evaluates

```text
qhat = floor(x*m/R)
r    = x - qhat*q
x mod q = r >= q ? r-q : r.
```

Because `m <= R/q` and `m > R/q-1`, `qhat` never exceeds `floor(x/q)` and,
for `x < R`, undershoots it by at most one. Thus the subtraction cannot
underflow and `0 <= r < 2q`; one conditional subtraction is sufficient. The
production-domain bound is stricter (`x < 2^62`). This derivation is a testable
implementation obligation, not a throughput result `[pcc:unverified]`.

Repository precedents checked before freezing this design are
`src/cryptanalysis/f4_fp.rs` (`Barrett`),
`src/cryptanalysis/f4_fp_tower.rs` (`Fp::reduce`),
`src/cryptanalysis/zero_dim_la.rs` (`Fp::bred`), and
`src/cryptanalysis/hashed_ecdl.rs` (`Modulus`). They use the same fixed-divisor
high-product method and retain direct-remainder differential tests. The method
is the standard reciprocal/Barrett reduction; the latter source points to
Handbook of Applied Cryptography section 14.3.3. The research-KB wrapper was
invoked with fixed-divisor Barrett and batch-inversion-width queries but no
executable was available, so no novelty or absence claim is based on that
search failure.

## Reduction correctness and identity gates

The reducer is admissible only if all of these deterministic checks pass:

1. Exhaustively compare it with a widened `u128` remainder for every
   `(a,b) in [0,q)^2` and every `q` from 2 through 257.
2. On boundary moduli `2`, `3`, `7`, `223`, `4051`, `65535`, `65536`,
   `65537`, `2^30-35`, and `2^31-1`, compare the Cartesian product of
   operands `0`, `1`, `q/2`, `q-2`, `q-1`, followed by 10,000 deterministic
   pseudorandom canonical pairs per modulus.
3. Run the existing cubic-field, batch-inversion, curve-addition, and planted
   logarithm self-test. The new reducer test must not use the production
   reducer to compute its reference.
4. On the public synthetic `q=223`, seed `1`, target index `0`, policy
   `ic-repeat` cell, compare the reciprocal candidate with a source-bound
   baseline binary. Excluding only calibration and `wall_s`, the instance and
   rho records must agree exactly: recovered scalar, walk identity, every
   field count, and every native count. In particular, `fm` must still bump
   `fq_mul` exactly once.

Post-freeze correction: integration review identified one reciprocal-constructor
division outside those pre-existing counters. The original identity requirement
above is retained as historical text; [`ROUND_C_DEVIATION.md`](ROUND_C_DEVIATION.md)
supersedes its normalized-record expectation with one explicit
`setup.reciprocal_div` difference and otherwise exact counter identity.

Stop on any residue mismatch, wrong logarithm, divergent walk, changed count,
panic, missing record, or source/hash ambiguity. A failed arm is retained; it
is not repaired by relaxing the comparison.

## Prospective b4/b8/b16 screen

The width screen uses the same `q=223`, seed `1`, target index `0`,
`ic-repeat` cell and a single deterministic walk seed. It runs `strong-b4`,
`strong-b8`, and `strong-b16` in that order in both source-bound binaries.
Each record must recover the planted logarithm, and the candidate must match
its same-width baseline record after calibration and `wall_s` are removed.

The screen reports, without ranking by wall time, steps, distinguished points,
restarts, look-ahead retries, cycles, cycle additions, and all phase field and
native counts. Counts may differ across widths because each width executes a
different batching schedule; only baseline-versus-candidate identity within a
width is required. This one instance is not a tuning sample. Width 8 remains
the frozen primary unless a later amendment is committed before a complete
standard-isolation retune panel.

A future throughput retune must start from source-bound baseline and candidate
binaries, pass the checkpoint gate, profile first, establish an A/A noise
floor, then run at least five interleaved A/B observations on the same isolated
host for every admitted width. It must cover the preregistered width-sensitivity
sizes and seeds rather than promoting the `q=223` development cell. Results
remain diagnostic until the Round A validation, replay, PCC, and publication
gates admit them.

## Evidence and decision rules

Exact commands, revision and source/binary hashes, compiler and host metadata,
stdout, stderr, exit status, and normalized comparisons are retained under
`round-c-reduce-retune/` (administratively relocated after the original freeze
to avoid an evidence-namespace collision). Profiling may explain the reducer's instruction mix but does not
establish speed. No wall-time or throughput comparison is reported from this
development cell.

The reducer may be accepted as an exact implementation alternative if all
reduction, field, solve, and same-width count-identity gates pass. A speedup
claim `[pcc:unverified]` additionally requires the frozen
checkpoint/profile/isolation protocol;
without it, performance status is unmeasured. The retune status is separately
reported as protocol frozen, deterministic sensitivity complete or blocked,
and full-grid throughput unmeasured.
