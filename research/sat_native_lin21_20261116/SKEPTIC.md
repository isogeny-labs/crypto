# SKEPTIC review: native 2+1 linear-leaf pilot (`lin21`), 2026-11-16

Independent review by a separate session. Every statement below is an unverified reviewer
assertion (`[pcc:unverified]`). No producer file was edited and no git state was changed.
Scratch work is under
`/tmp/claude-0/-home-user/570c81c4-197a-558d-aa76-c9ff505d9731/scratchpad/skeptic-P1/`
(`own/brute.rs` is my ground truth, `own/harness.rs` is the producer's code with my `main`).

## Verdict

- **Reproduced: partly.** Everything I reran matched exactly. I did not rerun `lin` at l = 9,
  `lin21` at l = 11 and 12, or `lin21-pu` and `lin-nolex` above l = 5.
- **Soundness and completeness: no counterexample found.** Exhaustive at l = 5, partial at
  l = 6, measured targets only at l = 7 and 8, not checked independently at l >= 9.
- **Rule outcome: "lead holds" follows mechanically from the pre-written rule.** The rule is
  weak, and the name overstates what was shown.
- **Plain supported claim:** on admissible targets, `lin21` makes the unchanged solver
  perform the known linear 2+1 scan correctly. It needs about 1.5 * 2^l conflicts, where
  `lin` needs about 1.2 to 1.5 * C(2^l, 2). It is about 1,400 to 1,900 times slower in wall
  time than the plain scan, and that gap is not closing. This is not an improvement over
  any baseline: the best method in the pilot is still the plain scan.

## 1. Rebuild and rerun

Built with `rustc -O` (rustc 1.97.0) through `run.sh <scratch>/lin21-l5 5`. The selftest
passed.

| Rerun | Rows | Compared fields | Result |
|---|---|---|---|
| l = 5, all four SAT arms | 80 | status, conflicts, decisions, propagations, XOR propagations, GJE calls | identical to `results.json` and `raw/rows.jsonl` |
| l = 5 targets file | 20 | all except `oracle_ms` | identical |
| `lin21`, l = 7, 9, 10, every target | 62 | same fields | identical |
| `lin`, first target at l = 7 and l = 8 | 2 | all counters | identical (12,343 and 40,167 conflicts) |
| `summary` rebuilt and run on a copy of the producer's `raw/` | n/a | `tables.md`, `summary.json`, `results.json`, `fig_data.tsv`, SVG | byte-identical |

File hashes of `lin21.rs`, `run.py` and `examples/pdp_lin_cdcl.rs` match PROTOCOL.md.

Timeline: the protocol commit `ec75e74b1a` (01:14:14Z) contains only PROTOCOL.md. `run.py`
records `started_utc` 01:14:20Z, six seconds later. The protocol itself says smoke runs on
seed 999 at l = 5, 6, 7 and 9 preceded it.

## 2. Independent ground truth

`own/brute.rs` has its own field multiplication, inversion by exponentiation, an
Artin-Schreier table and its own group law. It sums every unordered triple of rational
points with abscissa in V, including (0, 1) and repeated points, and labels each reachable
abscissa by witness class.

| Check | Scope | Result |
|---|---|---|
| Pair oracle and scan against my truth | l = 5, all 65,558 admissible rational targets outside V | 0 mismatches |
| `lin21` against my truth | l = 5, all 1,951 decomposable targets plus 642 sampled non-decomposable | 0 mismatches; every model lifts and sums to R |
| `lin` against my truth | l = 5, 529 edge-only targets plus about 80 sampled | 0 mismatches |
| Producer's `oracle_sat` against my truth | measured targets at l = 6, 7, 8 (21, 20, 20) | 0 mismatches; all admissible |
| Admission rule against my own computation | every l = 5 target | identical |
| Same four checks on a 2,264-target subset | l = 6 | incomplete: the run hit my time cap; at least 1,326 targets processed, 0 mismatches printed |

Handling of special cases:

- **x = 0.** Allowed in all arms; it is the 2-torsion point (0, 1). At l = 5, 194 targets
  decompose only with that point, and all arms return SAT on them. They count as
  "decomposable" here; whether a factor base should contain the 2-torsion point is a
  modelling choice the pilot inherits from `ChainInstance`.
- **Infinity.** It cannot occur as t in `lin21`, because xR is outside V. Triples with
  P2 = -P3 sum to a point with abscissa in V, so they never hit a target.
- **Repeated points.** Decided correctly: 322 targets at l = 5 decompose only with a doubled
  abscissa and 13 only as 3P, and `lin21` and `lin` return SAT on all of them.
- **Non-liftable abscissae.** They arise only for non-admissible targets (1/xR in V). At
  l = 5 all 14 such targets are SAT in the oracle, the scan, `lin21` and `lin`, while the
  point truth is "not decomposable". So the three arms share one algebraic definition, and
  it differs from the point definition exactly there. The admission rule removes these
  targets.
- **Even n.** At l = 6 (n = 20) there are 33 non-admissible targets, and all of them are
  genuinely decomposable (R = P + 2Q, with 2Q the 2-torsion point). The rule therefore
  excludes a few true positives when n is even. It is a negligible fraction and does not
  affect the UNSAT medians.

## 3. Encoding soundness and completeness

Algebra checked by hand:

- A2 = (x1 + xR)^2 is never 0, because xR is outside V.
- B = 0 happens only for x1 = 0. The encoding then forces a conflict (E = 0, C = b). That
  is the only case where the two roots t coincide.
- No admissible decomposable target needs x1 = 0 in every ordering: {0, 0, c} and {0, 0, 0}
  sum into V. At least one summand is nonzero and can take the x1 slot.
- C = 0 or t = 0 occurs only when x1 = sqrt(b)/xR lies in V, which is the non-admissible
  case.
- s = 0 (x2 = x3) gives ST = 0 and Q = 0, leaving p^2 + t p = b and zz^2 = p. This is
  consistent with doubling.

Empirical: the edge classes in section 2 are exactly these cases. Zero-only (194),
double-only (322) and triple-only (13, where s = 0 is forced) all return SAT at l = 5.

Limits: the completeness argument rests on the symmetry of S4 and on the unchanged solver
being complete once the priority variables are exhausted. I did not audit
`pdp_lin_cdcl.rs`. On non-admissible targets `lin21` returns SAT with non-lifting models,
so the encoding is correct only together with the admission rule.

## 4. Fairness

- **`lin` call.** `ChainInstance::build(n, l, irr, xR, 1, lex)` with `conflict_budget` set
  and restarts off, as in `examples/b0_pdp_sat_baseline.rs:285`. The modulus rule differs
  in wording from `find_irreducible_sparse`, but I computed both and they give the same
  mask for n = 17, 20, 23, 26, 29.
- **Same targets at l = 9.** The 254 figure is the median of per-target ratios on the 8
  common targets. `lin` conflicts there range over 196,372 to 196,377, so the subset does
  not matter. The l = 9 `lin21` median in the main table uses 16 targets.
- **Same counter.** Both arms report `Stats.conflicts` of the same `LinCdcl`.
- **Hidden Gauss work.** There is some, and it is small against 254x.

  | Arm | l | GJE calls per conflict | Gate linearisations per conflict | Variables | Wall per conflict (ms) |
  |---|---|---|---|---|---|
  | `lin` | 5 | 4.35 | 2.1 | 170 | 0.23 |
  | `lin` | 9 | 4.33 | 2.0 | 10 blocks of n | 2.25 |
  | `lin21` | 5 | 6.0 | 4.1 | 255 | 0.45 |
  | `lin21` | 9 | 5.2 | 4.1 | 15 blocks of n | 1.38 |
  | `lin21` | 12 | 5.0 | 4.0 | 15 blocks of n | 2.5 |

- **The 418x wall ratio is not a property of the encoding.** It exceeds the conflict ratio
  only because `lin` slows tenfold per conflict between l = 5 and l = 9; I did not check
  why (it ends with about 196,000 learnt clauses). At l = 5 to 8 the wall ratio (8, 10, 42,
  62) is below the conflict ratio (16, 26, 68, 105). Quote the conflict ratio.
- **Robustness to a factor of 2.** Halved or doubled, 418 stays far above the prediction
  floor of 32, and 1,400 to 1,900 becomes 700 to 3,800, still three orders of magnitude. My
  rerun medians: `lin21` 148 ms, 1,113 ms and 2,666 ms at l = 7, 9, 10 (producer 145, 1,068,
  2,663); l = 5 `lin` 212 ms (producer 171). Qualitative statements are robust; the third
  significant digit is not. Wall times are single runs under shared load.
- **Trend against the scan.** The `lin21`/scan ratio rises from 1,401 at l = 9 to 1,682 at
  l = 12. There is no approach to parity.

## 5. Decision rule and reporting

- Condition A holds: `lin21` medians are at most 1.51 * 2^l at l >= 7, against a bound of 4.
- Condition B holds: 254 at L* = 9 against a threshold of 64.
- The two missed predictions (D <= 16, and the ratio floor of 256 against 254) are not part
  of the rule, so they do not change the outcome. The README reports both.
- The rule was written after smoke runs at the same sizes had shown the effect, and its
  threshold is four times looser than the predicted mechanism. Passing it carried little
  risk.
- `summary.rs` regenerates every table number, slope, ratio and the decision. It recomputes
  agreement and verification from the binary's fields and ignores the Python `agree` and
  `verified` keys.
- `run.py` could in principle have altered rows, since it parses and re-serialises the
  binary's JSON. All 144 SAT-arm rows I regenerated without Python are identical in the
  counters, so there is no sign that it did. The roughly 500 rows I did not regenerate rest
  on the producer's files.

## Problems

1. The README sentence "its conflict count equals the scan's own count on every target
   (D >= 0)" is wrong as written. D is positive on many targets: 6 to 20 on every l = 11
   target, and up to 136 at l = 12.
2. "No learnt clause excluded an unvisited x1 value" does not follow from D >= 0. That
   inequality shows only no net saving; pruning offset by excess conflicts would also give
   D >= 0. The width histogram is suggestive, not a proof, and the excess D is unexplained.
   Treat this claim as weaker than stated.
3. "Lead holds" names a comparison against `lin` only. Under the repository's success
   criterion this is not an improvement: the plain scan is 1,400 to 1,900 times faster and
   the gap widens slightly with l.
4. The 418x wall figure mostly reflects `lin` slowing per conflict, not the encoding.
5. The measured rows were launched and re-serialised by Python, and `results.json` rows
   still carry the Python-added keys. This is mitigated by the exact reruns, not eliminated.
6. Verdicts at l >= 9 are checked only against the producer's own oracle. My independent
   truth covers l = 5 fully, l = 6 partly, and l = 7 and 8 on the measured targets.
7. The preregistration is low-risk (smoke runs on the same sizes came first), and the
   repository tree already differs from the snapshot commit (`analyze.py` deleted,
   `rows.jsonl` extended).
8. One seed, one curve, one n/l ratio. `lin` at l = 9 has 8 targets. There are no A/A wall
   repeats.
9. The encoding is correct only with the admission rule (twist models otherwise), and
   "decomposable" includes the 2-torsion point as a summand.
10. No knowledge-base query, Conductor task or benchmark record exists for this pilot (the
    producer declares this).
