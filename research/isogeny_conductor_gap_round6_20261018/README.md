# Round 6: early-stopping F4 and the trace coset on every Koblitz curve

**Date.** 2026-10-18.

**Status.**
- E8/E9a: 6,000 descent systems on 5 Koblitz curves, each solved twice.
  The solution sets agree on every system.
- E9b: one audited `ecbench` session, 168 records, all verified, 12
  replays reproduced.

**Result class.**
- E8 and E9a are stage diagnostics: F4 work per system, and yield.
- E9b is whole-method cold cost against matched rho.

No deployed-curve claim.  Protocol: [`PROTOCOL.md`](PROTOCOL.md), frozen
before E8 and E9 ran, with one amendment (a wall budget that never
triggered).  PDF: [`report.pdf`](report.pdf).

This round tests round 5's leads 1–3
([round 5](../isogeny_conductor_gap_round5_20261012/README.md)).

## Findings

1. **Round 5's lead 1 does not carry over to the binary descent.  Stopping
   F4 early saves up to 43 %, but only when most systems have a
   solution (E8).**
   - Both binary and prime-field F4 already return the moment 1 enters
     the basis.  So the extra cost of a solvable system is the rest of the
     basis, not a missed "decide first" test.
   - A solvable binary system costs 1.0–2.3× an unsolvable one, not
     3–10×.
   - `f4-f2-early` stops once degree-1 elements pin all but 8 unknowns,
     then enumerates and checks 2⁸ points.  It finds exactly the same
     solutions on all 6,000 systems.
   - Its saving never exceeds the share of work spent in solvable
     systems.  It is 29–43 % where that share is 60–98 %, and negative
     (down to −37 %) where it is near 0: the enumeration then costs more
     than letting F4 reach 1.
2. **The trace coset's yield gain is 1/f₂, the inverse fraction of the
   default base that lies in 2E.  The F_2^19 explanation from round 5 is
   wrong (E9a).**
   - The yield ratio matches 1/f₂ within 20 % in 7 of 9 rows with enough
     hits.
   - f₂ is 0.45–0.54 on every curve.  At F_2^19 it is 0.45, so the
     default base leans away from 2E, not toward it.  Round 5's 1.48×
     there remains unexplained.  The per-system profile gives 1.81
     [1.42, 2.30] at d = 9.
3. **The affine coset extends the yield gain to the a = 1 curves (lead 2),
   but not robustly the F4 gain (E9a, E9b).**
   - On the a = 1 curves yield rises 1.6–3.1×.
   - F4 work per system rises 1.1–45×.  The large factors come where the
     coset base needs solving degree 4 against the default's 3: F_2^17 at
     d = 9–10 and F_2^19 at d = 10.
   - The same jump happens on the a = 0 curve at F_2^19, d = 10, where the
     coset is linear.  So the cause is the subspace (it contains z^d),
     not the affine constant.
4. **Whole method on a = 1 (E9b): the coset helps MITM at n = 19 and 23,
   and nothing beats rho.**
   - MITM on the coset base is cheaper than on the default across the
     §5a band at F_2^19 (S 28.4 vs 31.2) and F_2^23 (14.1 vs 17.3).  It is
     dearer at F_2^17 (71.6 vs 61.6).
   - `f4-f2-early` beats `f4-f2` across the band on 3 of 6 (curve, base)
     pairs.  It never wins at F_2^23, where few systems are solvable.
   - **Distance remaining:** the round's best cell is coset MITM over
     F_2^23 at 12.7× rho.  The study's best cell is unchanged at 7.29×
     rho (round 5, prime field).

## E8. Where solvable systems spend F4 work

![E8 saving against solvable share; E9a yield ratio against 1/f₂](reports/run-20261018T1742Z/e8_e9a.svg)

| prediction | result |
|:--|:--|
| P-E8a: solvable ≥ 3× unsolvable | **fails**: mostly 1.0–2.3; ≥ 3 only with ≤ 7 solvable systems |
| P-E8b: no solution-set mismatch (gate) | **holds**: 0 of 6,000 |
| P-E8c: early ÷ full ≤ 0.5 on solvable systems | **fails** pooled: 0.94 [0.93, 0.95]; 0.24–0.99 per configuration |
| P-E8d: saving ≤ solvable share | **holds** on all 30 configurations |
| refutation rule: pooled ratio < 1.25× *and* saving < 10 % everywhere | not met: savings reach 43 % |

Per configuration: [`TABLES.md`](TABLES.md).  Run report:
[`run-20261018T1742Z`](reports/run-20261018T1742Z/README.md).

## E9a. Trace coset against the default base, per system

| prediction | result |
|:--|:--|
| P-E9a: yield ratio within 20 % of 1/f₂ | **holds** in 7 of 9 testable rows |
| P-E9a: f₂ > 0.6 at F_2^19 | **fails**: 0.45–0.46 |
| P-E9b: a = 1 ratio interval contains 2 or 1/f₂ | **holds** in 7 of 9 |
| P-E9c: a = 1 coset F4 work ≥ 1.3× default | **holds** in 8 of 9; 34–45× where the degree rises |

**Post hoc (not a prediction).**
- Estimated F4 work to collect one relation per column = columns ×
  work per system ÷ decompositions per system.
- At matched dimension the coset is lower in 7 of 14 (curve, d) pairs.
- It is 16–23× higher where its solving degree jumps (F_2^17 d = 9;
  F_2^19 d = 10), and 1.5–3× higher in the other losing pairs.

## E9b. a = 1 Koblitz curves, whole method

![S over rho per arm per curve](reports/run-20261018T1815Z/e9b_s_over_rho.svg)

| curve | rho S | MITM default | MITM coset | F4 default | F4-early default | F4 coset | F4-early coset |
|:--|--:|--:|--:|--:|--:|--:|--:|
| F_2^17 | 1.60 | 61.6 (38.6×) | 71.6 (44.8×) | 1,079 | 1,002 | 1,001 | 797 |
| F_2^19 | 1.59 | 31.2 (19.6×) | **28.4 (17.9×)** | 190 | 176 | 576 | 550 |
| F_2^23 | 1.11 | 17.3 (15.6×) | **14.1 (12.7×)** | 21.6 | 26.1 | 38.3 | 44.5 |

- S is mean counted S, with the solver at its pinned word-XOR ratio.
  `f4-f2-early` is also charged its extraction.
- Band verdicts across weights 0 / 0.1 / 1 are in
  [`sessions/e9b_a1.band.md`](sessions/e9b_a1.band.md).
- Run report: [`run-20261018T1815Z`](reports/run-20261018T1815Z/README.md).

## Leads, ranked by the evidence

1. **Choose the stopping threshold from the expected solvable share.**
   - The saving tracks the solvable share (E8).  So: stop early only when
     the base's decomposition probability is high (large d against n), or
     use a smaller `free` (2–4) so the enumeration never outweighs reaching
     1.
   - Cheap to test on the recorded systems: rerun E8 with free ∈ {0, 2, 4}.
2. **A 2E base that keeps the default's degree.**
   - The coset spans `⟨1, …, z^d⟩ ∩ ker Tr`, which contains z^d.  The
     degree jump appears where that matters.
   - A base inside `⟨1, …, z^{d−1}⟩` alone, with half the points (all in
     2E), may keep degree 3 at d = 10.  Whether its yield per column
     matches is the open question.
3. **Round 5's F_2^19 yield (1.48×) is unexplained.**  The per-system yield
   is 1.81× at d = 9.  `ecbench` targets are walk combinations: check
   whether they are evenly spread over E[r].
4. **Binary MITM stays 13–45× above rho, and F4 13–700× above it.**  No
   lever in rounds 5–6 changes that exponent.

## Graphs checked

- **Scoreboard:** a round-6 panel.
- **Progress timeline:** unchanged, since no new best.
- **Leaderboard and lab browser:** regenerated.

## Reproduce

```bash
cargo build --release --bin ecbench --example isogeny_gap_f4_phase_profile --example isogeny_gap_round6_tables
./target/release/examples/isogeny_gap_f4_phase_profile --out research/isogeny_conductor_gap_round6_20261018/e8/records.jsonl
./target/release/examples/isogeny_gap_round6_tables --dir research/isogeny_conductor_gap_round6_20261018 \
  --out research/isogeny_conductor_gap_round6_20261018/TABLES.md --json research/isogeny_conductor_gap_round6_20261018/tables.json
research/isogeny_conductor_gap_round6_20261018/run_sessions.sh
```
