# W3a: the Smith walk via the elliptic 2-isogeny class — results

Date: 2026-10-25 (runs 2026-10-26 UTC). Protocol: [`PROTOCOL.md`](PROTOCOL.md),
frozen at `eba0a48fbf` and not edited. Literature (Q3):
[`LITERATURE.md`](LITERATURE.md). Report with figures: [`REPORT.pdf`](REPORT.pdf)
(source [`REPORT.typ`](REPORT.typ)). Run reports:
[`reports/run-20261026T0212Z/`](reports/run-20261026T0212Z/) (Q1, exhaustive) and
[`reports/run-20261026T0244Z/`](reports/run-20261026T0244Z/) (Q2, sampled walks).
Synthetic curves over `F_{p^3}` only. Nothing here is a security claim about any
standardized curve, a sub-rho claim, or a key-recovery result.

## Headline

1. **On the descended locus Smith's test passes far more often than on random
   curves.** For every `p` in {7, 11, 13, 17, 19, 23, 29, 31}, every descended
   `H` has a rational tractable subgroup (share 1.000; only cycle types
   `1+1+3+3` and `2+6` occur). Then 0.53–0.61 of the descended `H` pass
   `full_success`, against Smith's 0.1857 for random `H`. The problem record's
   class ceiling (0.43–0.50, measured on random degree-8 `H`) does not bound this
   locus. Pre-registered E3 (`pi` near 0.186) is **falsified**, upward.
2. **Q1 walk distances are short.** A failing weak `E` has a passing weak `E'`
   in its 2-isogeny component in `C_2` = 0.74–0.98 of cases (2/3-graph `C_23` =
   0.76–0.99; same-trace class `C_t` = 0.78–1.00). When one exists it is on
   average 1.5–3.6 rational 2-isogenies away (maximum 3–11), and 1.4–2.9 steps
   with 3-isogenies added. Passing is not constant on components: of the
   components with at least 2 weak curves, 38–75% are mixed at each `p`. So the
   walk is **not killed by invariance**, and E2 holds (`D_E` < `C_2` < 1).
3. **Q2, frozen verdict: `o(p)`; a labelled post hoc analysis disputes it and
   indicates `Θ~(p)`.** The frozen fit outputs the label `o(p)` for arm A: cost
   exponent −0.14, 95% interval [−0.80, 0.95]. That is the registered result.
   **We do not take that label as evidence** of a sub-linear walk. It comes from the frozen
   estimator `(total steps)/(successes)`. In that estimator, right-censored
   walks add amounts set by the caps, and those dominate at the small primes
   and flatten the slope. In a post hoc analysis of successful walks at
   `p ≡ 3 (mod 4)`, where the walk does not branch, tested nodes scale with
   exponent 1.07 [0.95, 1.19] and cost with 1.13 [1.00, 1.26]. The frozen
   rule would label that cost interval `omega(p)`, since its lower end is
   not below 1. Step cost grows like log p, so the cost is about
   `p log p`, which is `Θ~(p)`. The medians are 0.1–0.6 `p` nodes. The hit
   rate is `h(p)` ≈ (3–10)/`p`.

   A non-circular check (derived) combines the walk's `h` with Q1's pass
   rate given JV (`pi` ≈ 0.8). It predicts `1/(h·0.8)` = 1.19e5 and
   3.24e5 tested nodes at `p` = 262147 and 1048583, against 1.19e5 and
   4.75e5 observed (ratios 1.00 and 0.68). E1 (`Θ~(p)`) is supported for
   the walks that finish.
4. **The walk policy fails when `p ≡ 1 (mod 4)`.** The step rule takes a
   neighbour only if it has full rational 2-torsion. Since
   `∏_i (r_i − r_j)(r_i − r_l) = −disc`, the parity of admissible steps is set
   by `χ(−1)` [derived, tested]:
   - for `p ≡ 3 (mod 4)`, every non-start node has exactly one forward edge,
     so the walk is a non-branching path;
   - for `p ≡ 1 (mod 4)`, a node is either a leaf or a branch with three
     neighbours.

   At the frozen primes with `p ≡ 1 (mod 4)` (2053, 8209, 65537), arm A walks
   are trapped in closed sets of 6–30 classes: 7/40, 37/40 and 20/24 walks
   respectively. Arm B reached no JV-weak node in 24 walks at 65537. Q1 shows
   the same split: the share of failing `E` with no passing `E` in their
   2-component is 56–59% at `p` = 13, 17, 29, against 4–25% at
   `p ≡ 3 (mod 4)`.
5. **Unit costs, in `F_p` multiplications** (instrumented; Smith derived from
   wall time):
   - one walk step: 1.8–3.0 k, growing like log p;
   - JV-weak test of one node: 114;
   - one descent: 2.0–4.0 k;
   - one `analyze_curve` call: 25–61 k (derived, 5.1–5.5 ns per
     multiplication on the shared host).

   A walk therefore costs about 2.5 k·`Θ(p)` multiplications. At this
   scaling the walk is comparable to `Õ(q)` index calculus, not negligible
   against it.

## Q1 table (exhaustive; measured)

`models` = the `2p^2` reduced weak models; `weak E` = `K`-isomorphism classes with
a model; `explored` = all classes reached by closing under 2- and 3-isogenies.
`C_X` = share of weak `E` whose component holds a passing `E`;
`r_X = (C_X − D_E)/(C_t − D_E)`. Distances are to the nearest passing weak `E`,
over failing weak `E` with one in the component. "Pass given non-JV" is derived
from the recorded `D_E`, JV share and pass-given-JV.

| p | models | weak E | explored | JV share | D_H | D_E | pass given JV | pass given non-JV (derived) | C_2 | C_23 | C_t | r_2 | r_23 | mono / comps ≥2 weak (2-graph) | failing with none in comp. | mean d₂ (max) | mean d₂₃ (max) |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 7 | 98 | 91 | 170 | 0.538 | 0.5408 | 0.5385 | 0.694 | 0.357 | 0.9670 | 0.9670 | 0.9670 | 1.000 | 1.000 | 2/8 | 3/42 | 1.46 (3) | 1.38 (2) |
| 11 | 242 | 230 | 664 | 0.522 | 0.6074 | 0.6000 | 0.850 | 0.327 | 0.9000 | 0.9261 | 0.9304 | 0.908 | 0.987 | 10/22 | 23/92 | 1.87 (4) | 1.68 (3) |
| 13 | 338 | 320 | 1628 | 0.512 | 0.5828 | 0.5969 | 0.835 | 0.346 | 0.7750 | 0.8125 | 0.8125 | 0.826 | 1.000 | 32/52 | 72/129 | 1.79 (3) | 1.83 (3) |
| 17 | 578 | 566 | 3440 | 0.519 | 0.5502 | 0.5512 | 0.786 | 0.298 | 0.7367 | 0.7633 | 0.7827 | 0.802 | 0.916 | 52/114 | 149/254 | 2.51 (5) | 2.33 (4) |
| 19 | 722 | 704 | 3428 | 0.514 | 0.5346 | 0.5312 | 0.776 | 0.272 | 0.9489 | 0.9830 | 0.9830 | 0.925 | 1.000 | 26/84 | 36/330 | 2.86 (9) | 2.47 (9) |
| 23 | 1058 | 1034 | 5584 | 0.511 | 0.5388 | 0.5368 | 0.778 | 0.285 | 0.9807 | 0.9807 | 0.9942 | 0.970 | 0.970 | 26/94 | 20/479 | 3.33 (11) | 2.72 (11) |
| 29 | 1682 | 1646 | 15012 | 0.507 | 0.5404 | 0.5431 | 0.802 | 0.277 | 0.7418 | 0.7874 | 0.8372 | 0.676 | 0.831 | 115/228 | 425/752 | 2.72 (6) | 2.55 (7) |
| 31 | 1922 | 1886 | 14288 | 0.507 | 0.5624 | 0.5589 | 0.823 | 0.287 | 0.9502 | 0.9883 | 1.0000 | 0.887 | 0.974 | 92/208 | 94/832 | 3.57 (11) | 2.89 (9) |

Per descended `H`: Prop. 2 holds for 0.78–0.88, and Prop. 4 given Prop. 2 for
0.62–0.72. Models per weak `E` are 1.02–1.08. Models of one `E` disagree in
`pass` for 0.4–3.7% of `E`.

**Controls**, all zero failures at every `p`:
- `L`-polynomial check on every descended `H` (98 to 1,922 per `p`, and 98,784
  in the unreduced run);
- Vélu trace equality on every edge (508 to 46,508 per `p`);
- dual edge present on every edge;
- every JV-weak node has JV models, and their Smith signatures lie in the
  node's set;
- no `analyze_curve` failure.

The **unreduced `p = 7` control**: all 98,784 models give the same 91 classes
and the same summary. The per-class signature sets agree too, checked by the
pinned test `unreduced_enumeration_agrees_with_reduced_at_p7`.

**Replay**: the 2026-10-20 run of the same cells, lost with that session's
untracked files, printed identical summary lines for `p = 7…31`.

## Q2 table (sampled walks; measured unless marked)

Frozen primes are the first prime above `2^k` for k = 10–16, 18 and 20. Arm A
runs from a random failing JV-split weak start to the first node with a
passing JV model. Arm B runs from a random full-2-torsion curve to the first
JV-weak node. Column definitions:
- "nodes/succ" is the frozen estimator: tested nodes summed over all walks,
  divided by successes.
- "median succ" is the median over successful walks only.
- `pi` = successes per JV-weak hit. Hits include retests, so it is
  meaningful only where walks do not revisit nodes.
- Censoring causes: I = isolated start, T = trap rule, S = step cap,
  N = node cap.

| arm | p | p mod 4 | walks | succ | nodes/succ (frozen) | median succ | cost/succ (frozen) | h(p) | h·p | pi | I | T | S | N |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| A | 1031 | 3 | 40 | 35 | 5.95e5 | 291 | 1.40e9 | 3.10e-3 | 3.2 | — | 0 | 4 | 0 | 1 |
| A | 2053 | 1 | 40 | 33 | 6.52e4 | 4755 | 2.33e9 | (retests) | — | — | 0 | 7 | 0 | 0 |
| A | 4099 | 3 | 40 | 34 | 2.37e6 | 448 | 5.20e9 | 8.17e-4 | 3.3 | — | 0 | 2 | 0 | 4 |
| A | 8209 | 1 | 40 | 3 | 3.18e6 | 24043 | 6.90e10 | (retests) | — | — | 0 | 37 | 0 | 0 |
| A | 16411 | 3 | 24 | 23 | 8.76e5 | 5852 | 2.29e9 | 5.79e-4 | 9.5 | — | 0 | 0 | 0 | 1 |
| A | 32771 | 3 | 24 | 23 | 8.78e5 | 6643 | 2.16e9 | 1.97e-4 | 6.4 | — | 0 | 0 | 0 | 1 |
| A | 65537 | 1 | 24 | 3 | 9.36e6 | 528492 | 6.78e10 | (retests) | — | — | 0 | 20 | 0 | 1 |
| A | 262147 | 3 | 12 | 12 | 1.19e5 | 155435 | 3.33e8 | 1.05e-5 | 2.8 | 0.80 | 0 | 0 | 0 | 0 |
| A | 1048583 | 3 | 6 | 6 | 4.75e5 | 505918 | 1.49e9 | 3.86e-6 | 4.0 | 0.55 | 0 | 0 | 0 | 0 |
| B | 1031 | 3 | 40 | 27 | 1.50e6 | 174 | 2.91e9 | | | | 8 | 3 | 0 | 2 |
| B | 2053 | 1 | 40 | 17 | 1.55e5 | 3005 | 1.20e11 | | | | 0 | 11 | 12 | 0 |
| B | 4099 | 3 | 40 | 28 | 7.29e5 | 501 | 1.51e9 | | | | 9 | 2 | 0 | 1 |
| B | 8209 | 1 | 40 | 4 | 6.72e6 | 784291 | 7.84e11 | | | | 0 | 20 | 15 | 1 |
| B | 16411 | 3 | 24 | 20 | 4.93e3 | 4025 | 1.24e7 | | | | 4 | 0 | 0 | 0 |
| B | 32771 | 3 | 24 | 18 | 2.05e4 | 6669 | 5.00e7 | | | | 5 | 1 | 0 | 0 |
| B | 65537 | 1 | 24 | 0 | (none) | — | — | | | | 0 | 9 | 12 | 3 |
| B | 262147 | 3 | 12 | 6 | 3.40e6 | 66755 | 9.47e9 | | | | 5 | 0 | 0 | 1 |
| B | 1048583 | 3 | 6 | 5 | 1.67e5 | 80781 | 5.25e8 | | | | 1 | 0 | 0 | 0 |

At `p ≡ 3 (mod 4)` arm A never revisits a node: distinct j-invariants equal
tested nodes on every successful walk. Its node-capped walks went round a
cycle of more than 1,000 classes that held no passing JV-weak node.

Fits (`fit.json`; OLS of the log mean on log p, 2,000 bootstrap draws):
- **Frozen fit**, primes and walks resampled:
  - arm A cost: −0.14 [−0.80, 0.95], frozen label `o(p)`; steps −0.04
    [−0.62, 0.92];
  - arm B: undetermined, because 65537 had no success.
- **Post hoc, labelled**:
  - successful walks, `p ≡ 3 (mod 4)`: arm A nodes 1.07 [0.95, 1.19], arm A
    cost 1.13 [1.00, 1.26] (rule label `omega(p)`: it is `p log p`, so
    `Θ~(p)`), arm B nodes 0.93 [0.75, 1.10];
  - successful walks, all primes: arm A nodes 0.85 [0.25, 1.53].

## Amendments / deviations (PROTOCOL.md is unchanged)

1. **Step cap (2026-10-25).** A walk is also censored after `4 × max_nodes` =
   8·10^7 steps. Some walks reflect between two nodes that are never retested,
   so `nodes_tested` does not advance (noted in #2287). Test:
   `walk_step_cap_bounds_reflection_loops`.
2. **Isolated start (2026-10-25; control failure fixed before data).** The
   arm B pilot panicked (`"the dual neighbour has full 2-torsion"`) on a start
   with no full-2-torsion neighbour. Such a start is now censored as
   `isolated`. This happens only when `p ≡ 3 (mod 4)`. Test:
   `walk_censors_isolated_starts_instead_of_panicking`.
3. **Trap rule and distinct-j diagnostic (2026-10-25, after the pilot).** The
   pilot found walks at `p = 65537` retesting 2–22 classes for minutes. A walk
   that has tested at least 2·10^5 nodes but seen at most 1,000 distinct
   j-invariants is censored as `trapped`. The j count is computed outside the
   cost counters.
4. **Fit.**
   - Censoring is recorded by cause.
   - The slope interval resamples primes, then walks, as the protocol
     specifies. The driver previously resampled walks only. Both intervals
     are in `fit.json`.
   - A prime with zero successes makes the verdict "undetermined" instead of
     being dropped silently.
   - The post hoc variants and per-operation unit costs are added and
     labelled.
5. **Host.** Apple M4 Pro (14 cores), shared with other jobs, with no core
   pinning: cores 29 and 30 do not exist on this host. At most 4 processes
   ran at `nice 10`. Wall times and the derived Smith cost are therefore
   noisy. Multiplication counts are deterministic.
6. **Calibration.** `calib`'s `p = 65537` row (5.12 ns per `F_p`
   multiplication, inversion = 7.6 multiplications) prices every `p`. The
   other rows are 5.2–5.5 ns.
7. **"Nodes visited" is `nodes_tested`**, which includes retests. Distinct
   classes are given by `distinct_j`.
8. **Scope.** The brief asked for Q1 at `p ∈ {7, 11, 13}`. The protocol's
   full list, 7–23 plus 29 and 31, was cheap (the largest cell took 62 s),
   so all of it ran.
9. **Provenance.** The 2026-10-20 session was cut off and its untracked
   outputs were lost. Every number here is from the 2026-10-25/26 rerun on
   this branch.
   - Exhaustive binary: sha256 `443d1f50…`.
   - Walk binary: `5e9444cd…`. Later edits touched only tests and the `fit`
     subcommand.
   - Fit binary: `aa0829690ca458a4…`.
   - `data/pilot/` holds the exploratory pilot from intermediate binaries. It
     is not used in any result.
10. **Interpretation of the frozen verdict.** The `o(p)` label is reported
    as the frozen rule's output. Results item 3 explains why it is a
    censoring artifact, and why the post hoc fits indicate `Θ(p)`.

## Files

- `data/exhaustive/`
  - `exh_p*.json`: summary, controls, per-model and per-curve rows;
  - `graph_p*.json`: classes with key, `a`, `b`, `t` and pass, plus 2- and
    3-edges;
  - `exh_unreduced_p7.json.gz`;
  - `log_exh_*.txt`.
- `data/walks/`
  - `walk_{A,B}_p*.jsonl`: one row per walk;
  - `calib.json`;
  - `fit.json`;
  - `log_walk_*.txt`;
  - `START_UTC` and `END_UTC`.
- `scripts/`: `run_exhaustive.sh`, `run_walks.sh`, `walk_cell.sh`.
- `plot.py`: draws the figures from recorded values only.
- `figures/`
  - `q1_shares`, `q1_distance`, `q2_walks` (`.svg` and `.png`);
  - `volcano_p11.*`, drawn with `tools/volcano-diagram`, with the label map
    in `volcano_p11_labels.json`.

Reproduce:

```sh
CARGO_TARGET_DIR=... cargo build --release --example smith_elliptic_walk
sh scripts/run_exhaustive.sh <bin> data/exhaustive
sh scripts/run_walks.sh <bin> data/walks
<bin> calib --out data/walks
<bin> fit --dir data/walks
python3 plot.py
```

`cargo test --release --lib smith`: 32 passed. Of these, 15 are in
`smith_elliptic_walk`, including the new tests:
- `admissible_forward_steps_follow_the_sign_of_minus_one`;
- `step_matches_scan_and_keeps_trace`, extended to `p = 13`;
- the isolated-start, step-cap and censoring tests.
