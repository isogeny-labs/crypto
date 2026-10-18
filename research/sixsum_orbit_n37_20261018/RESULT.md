# Orbit-base fold and concentration at n=37: the K⁴ verdict

Engine: `examples/koblitz_sixsum_orbit_n37` — compact-orbit factor base
(K Frobenius-orbit seeds closed under π and negation, B ≈ 2nK points),
pair table, matched 4-sum join and 6-sum S₄-chain arms, planted-target
diagnostics, and an x-hit-rate probe. All relations group-verified;
targets are fresh random subgroup points (a=0 n=37, r = 230603167).

## Measured

1. **Fold** (step 1): the target's Frobenius orbit has length 37 (n); every
   found relation yields its images for π^k(R) as free relations — the
   ladder's orbit amortization, confirmed.
2. **Concentration** (step 2): the x-hit rate is **736–798 per 1e6 trials
   against the uniform model's ~47 — a K-independent ~15× concentration**
   from the Frobenius-closed pair table. Per-relation cost = 4·(1/P_xhit):
   the 15× divides the plain-join law 4r/|T| by 15.
3. **K-scaling** (24 relations at K=4 and K=16): 6-sum mean 9,746 → 550
   (÷17.7); 4-sum mean 13,343 → 788 (÷16.9) — both ≈ 1/K² (4² = 16). **The
   feasibility note's K⁴ model does not materialize in this scan shape.**
4. **Failure-freedom**: at K=4 (E₄ ≈ 1) the 4-sum join failed on 7/24
   targets (no 4-sum exists); the 6-sum found 24/24 with mean 9,746 trials.
   The arity buys availability at small bases, ~1.4–2× better mean cost,
   and the 15× concentration — not a K-exponent.

## Row-parity position after this measurement

Total harvest at n=131 ≈ K · 4r/(15·|T|) ∝ r/(15·n²K): parity with rho
(2^60.9) needs K ≈ 2.9e15 — 15× below the 4-sum's 1.8e16, same exponent.
The 6-sum's measured wins are density (E₆ = 1 at K ≈ 2.4e4 where the 4-sum
needs K ≈ 4e6+ just to exist), failure-freedom, the 15× concentration and
the 37× fold — a combined ~10³–10⁴ total-work gain over the 4-sum ladder,
still ~10⁹ short of rho at n=131. Total-work parity at the challenge
scale remains unreached by every measured scan shape; the unmeasured
remaining option is a scan whose per-state structure (not just its table)
grows with K² — the note's S₄-tree premise in its strongest form.

## Reproduction

```bash
cargo run --release --example koblitz_sixsum_orbit_n37 -- 4 24 0x0AB1_7037
cargo run --release --example koblitz_sixsum_orbit_n37 -- 16 24 0x0AB1_7037
```

## Addendum: the third scan shape settles the law

The (2+1+2) mix (one pair sum + two base points + pair lookup; 24 relations
at K=16) measures mean 624 / median 480 trials per relation against the
S₄-chain's (2+2+2) 519/376 and the 4-sum join's (2+2) 782/573 — all three
shapes within 1.5×, all scaling as r/K². The general law: for any m-sum
join scan with `k` sampled pair levels and `j` sampled single levels
(`2k + j = m`, last level free), probes per relation = |T|^k·B^j / E_m ∝
r/B² — the arity and the level mix change only constants (the measured
range across all shapes and arities at n=37: 1.5×). The K⁴ concentration
of the feasibility note's model therefore requires a scan whose per-state
structure itself grows with K² — a state table whose entries are not pair
sums but K²-growing structures (e.g. shared S₃-root trees indexed by
column pairs) — not any join over pair tables. That is the precise,
falsifiable form of the remaining build.

Reproduction: `cargo run --release --example koblitz_sixsum_orbit_n37 -- 16 24 0x0AB1_7037`
