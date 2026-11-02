= The exact-count falsifier for lever 2: no candidate family fires it, and none differs from random bases
2026-11-01. Protocol frozen at `9d2a2daf2`, before any row (`PROTOCOL.md`). The count asked for by `RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` §4 item 2, run exactly: for a base `X ⊂ F_p` of size `B`, the closure count `c_m(r)` of `S_{m+1}(x_1, …, x_m, r) = 0` over every ordered tuple of `X` and every target `r ∈ F_p`, compared with the threshold `B^{m−1}`.

== Result
- #strong[Instrument checked.] The fast count equals brute-force evaluation at every `r` for `m = 2, 3` and all seven base families (G1). `S_3` vanishes at `x(P_1 + P_2)` over 9,900 point pairs and `S_4` at `x(P_1 + P_2 + P_3)` over 83,285 point triples (G2). `S_4` is symmetric and its polynomial form equals its numeric value (G3). Every base element carries the `3B − 2` trivial tuples `(x, x, r)` at `m = 3` in all 2,560 `m = 3` instances (G5, `trivial_violations = 0`). Clippy with `-D warnings` and rustfmt are clean.
- #strong[Exact, no sampling.] 5,760 instances: 3,200 at `m = 2` over `B = 128 … 2048` and 2,560 at `m = 3` over `B = 16 … 128` (`p` up to 4,206,593), each counted over all `p` targets.
- #strong[The literal falsifier does not fire for any candidate family.] None of the 1,800 structured instances (subgroup coset, arithmetic progression, interval, cubic image, Dickson image; 40 per family per size) reaches `B^{m−1}`. The median of their largest counts is 0.7%–8% of the threshold at `m = 2` and 2.4%–20% at `m = 3`, and that share falls as `B` grows. The largest single instance reaches 14 of 128 at `m = 2` and 64 of 256 at `m = 3`.
- #strong[None differs from random bases.] All 90 Mann–Whitney tests of a family against the 400-instance null, on `Cmax` and coverage, are within the null at `α_c = 0.01/90 ≈ 1.1·10⁻⁴`. The smallest family `p`-value is `4.3·10⁻³` (image, coverage, `m = 2`, `B = 2048`) and the largest `|Cliff's δ|` is 0.27.
- #strong[The detector sees concentration.] The control `x(kP)` fires in 40 of 40 instances at each of the nine sizes (360 of 360), and its coverage is below the null\'s at `p ≈ 10⁻²⁵`. Its `Cmax` is exactly `2B − 2` at `m = 2` and exactly `3B² − 6B + 4` at `m = 3` at all nine sizes (fit to the recorded values, not derived here), while its coverage falls to 0.1% and 0.02% of targets.
- #strong[The A/A split of the null passes] at every size (18 tests, smallest `p = 0.12`).

== 1. Claim classes
#align(center)[#table(
  columns: 3,
  align: (col, row) => (left,left,left,).at(col),
  inset: 6pt,
  [claim], [class], [basis],
  [each `c_m(r)`, `Cmax`, coverage and firing flag of each instance],
  [exact computation],
  [counted over all tuples and targets; gates G1–G5],
  [no structured instance has `Cmax ≥ B^{m−1}`; the control always does],
  [exact computation],
  [`fires` per instance in `rows.jsonl`],
  [a family does or does not differ from the null],
  [comparative, statistical],
  [controls of §5 below; 90 tests],
  [the closed forms of the control\'s `Cmax`],
  [observation],
  [fit to nine recorded values],
  [growth of `Cmax / B^{m−1}` with `p`],
  [descriptive],
  [the figure, no test],
  [lever viability, any attack cost],
  [out of scope],
  [not established],
)
]

== 2. Figure
#page(flipped: true, margin: 1.2cm)[#figure(image("counts.svg", width: 100%), caption: [Largest closure count over the threshold (top) and coverage (bottom), by base size; m = 2 left, m = 3 right. Exact counts from the summary.json of runs/m2 and runs/m3.])]

Top: the largest closure count over all targets, divided by the threshold `B^{m−1}`; the dashed line is the threshold. Grey: min to max over the 400 random bases. The five structured families sit on the null\'s median; the control sits above the threshold. Bottom: the fraction of targets with at least one solution.

== 3. Summary by size
#align(center)[#table(
  columns: 9,
  align: (col, row) => (right,right,right,right,left,left,right,right,left,).at(col),
  inset: 6pt,
  [m], [B], [p], [threshold], [null `Cmax`: median \[min, max\]], [structured `Cmax`: range of family medians], [structured fires], [smallest family `p`], [control `Cmax` (fires)],
  [2],
  [128],
  [17921],
  [128],
  [10 \[8, 14\]],
  [10 to 10],
  [0/200],
  [5.8e-02],
  [254 (40/40)],
  [2],
  [256],
  [67073],
  [256],
  [12 \[10, 16\]],
  [10 to 12],
  [0/200],
  [3.8e-02],
  [510 (40/40)],
  [2],
  [512],
  [270337],
  [512],
  [12 \[10, 16\]],
  [12 to 12],
  [0/200],
  [5.4e-02],
  [1022 (40/40)],
  [2],
  [1024],
  [1054721],
  [1024],
  [14 \[12, 16\]],
  [14 to 14],
  [0/200],
  [2.1e-02],
  [2046 (40/40)],
  [2],
  [2048],
  [4206593],
  [2048],
  [14 \[12, 18\]],
  [14 to 14],
  [0/200],
  [4.3e-03],
  [4094 (40/40)],
  [3],
  [16],
  [4129],
  [256],
  [52 \[46, 70\]],
  [50.5 to 52],
  [0/200],
  [5.9e-02],
  [676 (40/40)],
  [3],
  [32],
  [32833],
  [1024],
  [100 \[94, 118\]],
  [100 to 100],
  [0/200],
  [4.7e-02],
  [2884 (40/40)],
  [3],
  [64],
  [262657],
  [4096],
  [196 \[190, 214\]],
  [196 to 199],
  [0/200],
  [1.1e-01],
  [11908 (40/40)],
  [3],
  [128],
  [2100737],
  [16384],
  [394 \[382, 406\]],
  [394 to 394],
  [0/200],
  [2.4e-01],
  [48388 (40/40)],
)
]

`tables.md` has every family at every size, with `Cmax` and coverage medians, `p`, Cliff\'s `δ` and the verdict.

== 4. The `m = 3` baseline is a trivial family of solutions
At `m = 3` the null\'s `Cmax` is about `3B` (394 at `B = 128`), not a few units as at `m = 2`. The cause is the solution family `P − P + R = R`: `(x, x, r)` and its permutations solve `S_4 = 0` for every `r ∈ X`, which is `3B − 2` ordered tuples. They are solutions of the system as written, so the primary metric keeps them (frozen, `PROTOCOL.md` §4), and gate G5 confirms every base element carries them. With them removed, `Cmax_nt` is 15–18 at `B = 16`, 21–24 at `B = 32`, 24 at `B = 64` and 30 at `B = 128` for the null and every structured family, against 630 to 48,006 for the control. They stay below `B^{m−1}` by a wide margin, so the reading does not depend on the choice.

== 5. Reading under the frozen rules
#align(center)[#table(
  columns: 2,
  align: (col, row) => (left,left,).at(col),
  inset: 6pt,
  [rule (`PROTOCOL.md` §5)], [outcome],
  [gates G1–G5 on the study binary],
  [pass (`gates.txt`)],
  [positive control fires at `m = 2` in every instance],
  [200/200],
  [positive control `Cmax ≥ 0.5·B²` at `m = 3` in every instance],
  [160/160 (`Cmax ≥ 2.6·B²`; it also exceeds `B²`)],
  [positive control coverage below the null at `p < α_c`],
  [`p = 1.7·10⁻²⁵` at all nine sizes],
  [A/A split of the null, `p ≥ 10⁻³`],
  [pass, minimum 0.12],
  [a structured instance with `Cmax ≥ B^{m−1}`],
  [none of 1,800],
  [family vs null, 90 tests at `α_c = 1.1·10⁻⁴`],
  [0 rejected],
)
]

Verdict wording from the protocol: every structured family is #strong[within the null] on `Cmax` and on coverage at every tested `(m, B)`; the control is #strong[above] the null on `Cmax` and #strong[below] it on coverage.

== 6. What this shows and what it does not
- #strong[Shows.] At these sizes the literal §4.2 count does not distinguish a subgroup coset, an arithmetic progression, an interval, a cubic image or a Dickson image from a uniformly random base of the same size, on the largest count or on coverage, and it cannot be fired by any of them. It is fired only by a base with additive structure in the group, which also destroys coverage.
- #strong[Detectable effect.] With 40 instances against 400 the tests reach `α_c` at `|δ| ≈ 0.37` and detect `|δ| ≈ 0.46` with 80% power (protocol §5). Smaller differences are not excluded; the largest observed `|δ|` is 0.27.
- #strong[Not shown.] Toy primes only (`p ≤ 4.2·10⁶`), `m = 2, 3` only, the five parameter distributions listed, and the isogeny tower\'s `V` is not among them. The count says nothing about the cost of finding a decomposition or about the solving degree of any presentation; that question is `LEAD-ICP-pkm-tower-algebraic-oracle`\'s and `LEAD-ICP-digit-presentation-solving-degree`\'s.

== 7. Disclosures
- A dry run (seed 1, small sizes) preceded the protocol and led to the trivial-solution metric and gate G5. Its instances are not used.
- Route: AGENTS.md §9a sends x86 experiments to Iso Lab on EKS Spot. The container that ran this has no AWS access, so the run was local (two threads on a four-vCPU host shared with another job). The claims are exact integer counts and do not use a clock; `wall_seconds` is operational only (298 s at `m = 2`, 3,941 s at `m = 3`).
- `commit.txt` records the binary SHA-256 `3061da3e…` and the freeze commit. `runs/m2/rows.jsonl` is in the artifact store (manifest beside it, sha256 `19ccf5ad…`).
- No independent `review-run` record exists yet.

== 8. Sources
`RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` §2.3, §4 item 2 and its 2026-10-31 correction; `research/lever2_closure_20261031/REPORT.md` (the identity: mean closure count `C(B+m−1, m)/N`); in crypto-autoresearcher `KN-FIND-007` (the same identity and the small-multiples base\'s coverage collapse), `EV-FBG-001` (exact yield over 288 cells), `IDEA-20260801-021` (scheme degree of a size-`B` description); Petit–Kosters–Messeng, PKC 2016 (the factor-base definition, read from the authors\' PDF). The summation polynomials follow Semaev (2004), which was not re-read here; `S_3` and `S_4` as implemented are checked against the group law by gate G2.

== 9. Reproduce
`cargo build --release --example exact_count_membership`; `exact_count_membership --selftest`; then the two commands in `PROTOCOL.md` §3 with `--seed 20261101`. `plot_counts.py` (a layout script, document rendering only) rebuilds `counts.svg`, `counts.png` and `tables.md` from `runs/m*/summary.json`; `build_report.sh` rebuilds this PDF.
