# L07 ASSESSMENT — Prym index calculus for genus 4-5 (desk count)

Assessor: independent of producer and replay runner. Date 2026-11-16.
Every statement here is `[pcc:unverified]`; nothing in this lead has a pcc
claim id. Not a benchmark: no leaderboard row, no `benchmarks/` row, no speed claim.

## Verdict: KILLED (frozen PROTOCOL §8 applied as written)

The only Prym relation family identified (anti-invariant relations sieved as
reduced divisors on the étale double cover `D`, genus `2g+1`) has 2LP exponent
`2 - 2/(2g+1)` = 16/9 (g=4) and 20/11 (g=5), against the frozen bar
`2 - 2/g` = 3/2 and 8/5. The rule says killed when every identified family is
`>= 2 - 2/g`; that holds, with exponent excess 5/18 (g=4) and 12/55 (g=5).
The prediction "killed", recorded pre-data, is confirmed. The numeric check
does not gate the decision (§8), and its gates passed anyway.

## 1. Integrity checks

- `sha256sum -c frozen.sha256`: PROTOCOL.md OK, SPEC.md OK.
- Replay (replay/REPLAY.md) is byte-identical on genus_table.tsv,
  split_law.jsonl, gates.txt, plot.svg; tests.txt differs only in the
  wall-clock line. I did not re-run the full replay.
- Code read: `src/split_law.py` implements SPEC exactly (closed form
  `C(p,d)/p^d`, penalty ratios, MC with derived seed `seed*1000+d`, Horner
  `eval_poly`, `roots == d` predicate). `src/gates.py` evaluates the §7 gates
  from the raw file and re-derives one frozen-size MC cell (d=2, 200000 trials)
  for determinism. Gates genuinely ran.

## 2. Independent re-derivations (mine, Python stdlib, scratch only)

- Exhaustive enumeration of all monic degree-d polynomials over F_p for
  p in {5,7,11}, d in {1..4}: the number with d distinct roots equals
  `C(p,d)` in every cell. The closed form the lead rests on is correct.
- Penalty ratio from the closed form: p=101 g=4 gives 20548.5549 and
  p=10007 g=5 gives 334139.7938, both matching raw/split_law.jsonl. The
  asymptotes `(2g+1)!/g!` are 15120 and 332640; p=10007 is within 0.3 % and 0.5 %.
- MC d=3 z-scores vs exact 0.161749: -0.35 and -0.96; AA gap 0.000505 vs
  tolerance 0.003293. d=2 z = 0.609, 0.604 (from gates.txt, consistent).
- Exponent arithmetic: `(2-2/(2g+1)) - (2-2/g) = (2g+2)/(g(2g+1))` = 5/18
  and 12/55 for g = 4, 5, as PROTOCOL §2 states.

## 3. Audit concerns (none overturns the kill)

1. **The numeric arm is close to tautological.** It measures the
   random-monic-polynomial splitting law, which is `C(p,d)/p^d` by
   definition; no curve `D`, no cover and no Prym element was ever built. It
   validates the sampler, not the hypothesis that Prym relation candidates
   behave like random degree-(2g+1) divisors. The decision rests entirely on
   the analytic degree argument, as §8 already says.
2. **H1 is the exponent of the naive route, not a lower bound for Prym IC.**
   A point of P can generically be written `[E - sigma E]` with `E` effective
   of degree g (the Abel–Prym map `D^(g) -> P` is between g-dimensional
   varieties and dominant, because the Abel–Prym curve generates P). Testing
   `E` for splitting succeeds with probability about 1/g!, so heuristically
   this family has exponent `2 - 2/g`. That is equal to the bar, not strictly
   below it, so the frozen rule still kills. Finding `E` also means solving a
   non-linear fibre problem, not a Riemann–Roch reduction, and no cost is
   known for it. The margins 5/18 and 12/55 describe the naive route. They are
   not the true gap of the best Prym family, which is plausibly 0.
3. **Fairness of the baseline arm.** The bar `2 - 2/g` assumes that a
   hyperelliptic genus-g model of the target exists. A generic ppav of
   dimension 4 or 5, the case that motivates Prym (the Prym map is dominant),
   has no such model: the hyperelliptic locus has dimension 2g-1, smaller
   than dim A_g. For those targets the honest comparator is rho on P (g/2),
   and the naive Prym exponents 1.78 and 1.82 would sit below rho's 2.0 and
   2.5. That holds only given an algorithm that builds `(D, sigma)` over
   F_p from the target, and none is known (§3). The frozen rule fixes the
   hyperelliptic bar, so the verdict does not change; the caveat is recorded
   for the atlas.
4. **Minor protocol inaccuracy.** PROTOCOL §2 says Diem's canonical (plane)
   IC on the genus-(2g+1) cover has the same exponent class `2 - 2/(2g+1)`.
   Diem's non-hyperelliptic exponent for genus G is `2 - 2/(G-1)`, which gives
   `2 - 1/g`. That is still above `2 - 2/g`, so the verdict is unaffected.
5. **Protocol/spec mismatch in the MC size.** PROTOCOL §5 lists
   T = 2,000,000 at all degrees and primes, and §7 gate 3 says "every (p,d)".
   The frozen SPEC narrows MC to p=101, d<=3, T=200,000. Both documents were
   frozen before data, and the numeric check is non-gating, so this does not
   invalidate the rule. Gate 3 was only exercised on 3 cells.
6. **Deviations D1–D5 are benign.** D1 is a `-I` import fix with the same
   tests and the failure kept. D2 adds analysis helpers. D3 completes the
   garbled test 7, and the unit-test MC uses a smaller T while the
   frozen-size cells are gated in gates.py. D4 makes no change. D5 means
   DERIVATION.md was never assembled; this ASSESSMENT states the frozen
   verdict in its place. None changes a threshold, arm, seed or rule.
7. **No FPM counts.** Nothing was charged in FPM. That is acceptable for a
   desk count, but it means no cost-per-relation claim of any kind exists here.

## 4. Effect sizes

The analytic exponent excess over the hyperelliptic 2LP bar is +5/18 ≈ 0.278
at g=4 and +12/55 ≈ 0.218 at g=5, for the naive D-sieving family. It is
plausibly 0 for an Abel–Prym degree-g family whose decomposition cost is
unknown (concern 2). The per-trial splitting penalty `frac(g)/frac(2g+1)` is
15165 (g=4) and 334140 (g=5) at p=10007, approaching `(2g+1)!/g!`. Seed
spread exists only for MC d<=3, where the two-stream AA gaps are 0, 5e-6
and 5e-4, all inside tolerance.

## 5. Distance to matched rho

No improvement over the index-calculus baseline. The distance to rho is
unchanged by this lead. On a target that has a hyperelliptic genus-g model,
hyperelliptic 2LP (1.5 / 1.6) already sits below rho on P (2.0 / 2.5), and
the Prym route is worse than that baseline. On a generic dim-4/5 ppav, the
Prym route would sit 2/9 (g=4) and 15/22 (g=5) below rho in exponent, but
only with an unavailable cover-construction algorithm. That is a conditional
statement, not a measured gain.

## 6. Next step

None for this lead as framed. If the atlas wants anything, it is a one-line
annotation: "Prym IC only matters for generic (non-hyperelliptic,
non-Jacobian) dim-4/5 targets, and is blocked there by Prym-map inversion
over F_p." No experiment is warranted.
