= ECDLP across a large prime conductor gap on a deep 2-isogeny volcano
<ecdlp-across-a-large-prime-conductor-gap-on-a-deep-2-isogeny-volcano>
Study of 2026-10-22, on public synthetic prime-field curves.
Classification: theorem note plus a preregistered, controlled
measurement (rho against rho). No security statement about any deployed
curve.

#strong[Question.] Is ECDLP hardness the same on both sides of a large
prime conductor gap, on a deep isogeny volcano?

== Answer
<answer>
#strong[Proved] (#link("THEOREMS.md"), with Lean for the parts Lean can
state):

- #strong[Pairwise, for every algorithm.] For two given curves across
  the gap, ECDLP on one reduces to ECDLP on the other at the cost of the
  isogeny chain between them (Theorem 2). In the class measured here
  every chain is explicit and costs `O(N + d)` field operations per
  instance (Theorem 3). In general classes, Galbraith (2024) puts it at
  `Õ(q^{1/4})` (as reported). Either way the best attack cost `T*`
  agrees across the gap up to an additive term below `√q`.
- #strong[Everything that depends on the group alone agrees.]
  - group order and embedding degree;
  - the generic lower bound (Theorem 1).
- #strong[The one exception is a constant, and it travels.] A `j = 0` or
  `j = 1728` crater has extra automorphisms, so rho run #emph[on that
  curve] is `√3` or `√2` cheaper. A cheap ascent carries any floor
  instance there, so `T*` does not split (Theorem 4). This study\'s
  class (`D₀ = −11`) has `Aut = ±1` everywhere.

#strong[Open.] Whether a weakness on a fraction of floor curves can be
kept from a given crater curve (KKM 2011, Galbraith\'s Problem B) is
still open in general. In this class it cannot: descending is `O(N)` too
(Theorem 3(g)).

#strong[Measured] (protocol frozen before data, #link("PROTOCOL.md")):

- the class: `p ≈ 2^52`, gap prime `N = 631409 ≈ 77·p^{1/4}`, a depth-6
  2-volcano on each side;
- the method: `rho.negation`, the strongest generic method for
  `Aut = ±1`;
- the coverage: all 380 curves, one ECDLP instance carried across the
  gap by a verified degree-`N` Vélu isogeny.

The counted cost is #strong[equivalent within ±5%] across the gap:

#align(center)[#table(
  columns: 2,
  align: (col, row) => (left,left,).at(col),
  inset: 6pt,
  [], [value],
  [`G = mean S(S2) / mean S(S1)`],
  [#strong[0.9958]],
  [90% CI (curve bootstrap, 4,000)],
  [\[0.9714, 1.0205\]],
  [95% CI],
  [\[0.9663, 1.0257\]],
  [permutation p (10,000)],
  [0.78],
  [verdict (frozen rule)],
  [#strong[equivalent within ±5%]: the 90% CI is inside \[0.95, 1.05\]
  and both planted controls are detected],
  [runs],
  [4,560 (2,280 per side), all verified],
)
]

#image("figures/gap-by-level.svg")

== The class and the instance
<the-class-and-the-instance>
`volcano-gap construct --bits 52 --depth 6 --d0 11 --max-cofactor 16384 --seed 20261022`
(#link("run/pair.json")[`run/pair.json`]):

#align(center)[#table(
  columns: 2,
  align: (col, row) => (left,left,).at(col),
  inset: 6pt,
  [quantity], [value],
  [`p`],
  [4497080234220953],
  [`t`],
  [−5051274, and `t ≡ −2 (mod N)`],
  [`t² − 4p`],
  [`−11 · (2⁶ · 631409)²`],
  [sides],
  [S1: conductor `2^a`, one volcano, crater `j(−11) = −32768`],
  [],
  [S2: conductor `631409·2^a`, `N − 1` volcanoes of the same shape],
  [curves per level (each side)],
  [1, 3, 6, 12, 24, 48, 96 (190); predicted by the class-number formula,
  walked by `volcano-rho`, and matched by the independent `ca volcano`
  walker],
  [`#E`],
  [4497080239272228 \= 2388 · `n`],
  [`n`],
  [1883199430181 (prime, ≈ 2^40.78)],
  [order proof],
  [`n > 4√p`, so the Hasse interval holds one multiple of `n`],
  [`N`-isogeny `φ` (S1 crater → S2 crater)],
  [kernel \= a point of order `N` on the quadratic twist; Vélu over
  `F_p` from 315,704 abscissae],
  [`φ` checks],
  [homomorphism on 8 random pairs, `φ(P)` of order `n`, codomain order],
  [dual `ψ`],
  [built from the S2 root\'s one twist-rational line; reaches `j(−11)`,
  `ψ∘φ = ±[N]` on 5 points],
  [cost of `φ`],
  [build: 0.15 s (4.4·10⁷ `F_p` multiplications); evaluate: 3.2·10⁶ per
  point (0.019 s)],
  [side certificates],
  [all 760 curve models: S1 twists have `(Z/N)²`, S2 twists a cyclic
  `N`-part, proved by a witness
  (#link("run/certificates.json")[`run/certificates.json`]); no `j`
  shared],
)
]

The generator `P` sits on the S1 root and `φ(P)` on the S2 root.
`volcano-rho --generator` carries each down its volcano by verified Vélu
2-isogenies (189 edges per side). `planted_shared` targets give the same
`k` on all 380 curves, so on model 0 the S2 instance #emph[is] the
`φ`-image of the S1 instance.

#image("figures/class-diagram.svg")

== Results
<results>
Counted unit: `S = GAE/√r`, `r = n` on every curve. Measured unless
marked derived.

#strong[Per side] (`rho-neg`, 2 models × 6 targets per curve):

#align(center)[#table(
  columns: 4,
  align: (col, row) => (left,right,right,left,).at(col),
  inset: 6pt,
  [side], [runs], [mean S], [95% CI (runs)],
  [S1],
  [2280],
  [0.94288],
  [± 0.0203],
  [S2],
  [2280],
  [0.93889],
  [± 0.0208],
)
]

For reference, the ideal negation-map rho is `√(π/4) = 0.886`. Both
sides read about 6% above it, the walk\'s own overhead.

#strong[Per level] (S2/S1; level 0 has one curve per side):

#align(center)[#table(
  columns: 7,
  align: (col, row) => (right,right,right,right,right,left,left,).at(col),
  inset: 6pt,
  [level], [curves per side], [S1 mean S], [S2 mean S], [S2/S1], [95% CI
  (curve bootstrap)], [95% CI (run-level, post hoc)],
  [0],
  [1],
  [0.9270],
  [0.8725],
  [0.941],
  [(one curve: no spread)],
  [\[0.580, 1.528\]],
  [1],
  [3],
  [0.8784],
  [0.9130],
  [1.039],
  [\[0.868, 1.300\]],
  [\[0.797, 1.355\]],
  [2],
  [6],
  [1.0321],
  [0.9589],
  [0.929],
  [\[0.795, 1.119\]],
  [\[0.786, 1.098\]],
  [3],
  [12],
  [0.9319],
  [0.9222],
  [0.990],
  [\[0.873, 1.135\]],
  [\[0.872, 1.123\]],
  [4],
  [24],
  [0.8955],
  [0.8937],
  [0.998],
  [\[0.909, 1.092\]],
  [\[0.912, 1.092\]],
  [5],
  [48],
  [0.9625],
  [0.9526],
  [0.990],
  [\[0.931, 1.051\]],
  [\[0.931, 1.052\]],
  [6],
  [96],
  [0.9429],
  [0.9456],
  [1.003],
  [\[0.963, 1.045\]],
  [\[0.961, 1.047\]],
)
]

- Every level\'s interval contains 1.
- The run-level interval (delta method on the two means) was added after
  the data, because the curve bootstrap has no spread in a one-curve
  stratum. It is descriptive and does not enter the decision.
- From level 3 down, the two intervals agree. So there is no
  between-curve variance beyond run noise.

#strong[Secondary statistics] (all reported in
#link("analysis.json")[`analysis.json`]):

#align(center)[#table(
  columns: 3,
  align: (col, row) => (left,left,left,).at(col),
  inset: 6pt,
  [statistic], [value], [null / reference],
  [crossed bootstrap (shared target draw, conservative)],
  [90% \[0.9581, 1.0337\], 95% \[0.9496, 1.0419\]],
  [inside ±5% too],
  [floor/crater within each side (v1.1 stratified)],
  [1.050, 90% \[0.889, 1.265\], p \= 0.65],
  [one crater curve per side: wide],
  [horizontal τ (curves within a level)],
  [6.2%],
  [null p95 9.1%, p \= 0.64],
  [model τ (isomorphic models of a curve)],
  [6.2%],
  [null p95 7.8%, p \= 0.38],
  [§5a band, S2/S1 at w \= 0, 0.1, 1 GAE per unpriced op],
  [0.99576, 0.99576, 0.99576],
  [same method on both sides: reported, not ranked],
  [S at w \= 0 / 0.1 / 1],
  [S1: 0.943 / 1.037 / 1.885; S2: 0.939 / 1.033 / 1.877],
  [],
)
]

#image("figures/volcano-s1.svg") #image("figures/volcano-s2.svg")

== Controls
<controls>
#align(center)[#table(
  columns: 4,
  align: (col, row) => (left,left,left,left,).at(col),
  inset: 6pt,
  [control], [prediction], [measured], [verdict],
  [A/A arm (`rho-neg-aa`, other measurement seed), its own `G`],
  [95% CI contains 1],
  [1.0002, 95% \[0.9525, 1.0526\], p \= 0.99],
  [passes],
  [`rho-neg` against `rho-neg-aa`, matched workloads],
  [1],
  [0.980, 95% \[0.944, 1.016\]],
  [passes],
  [BSGS exactness (`bsgs-neg`, 2 targets)],
  [one cost per target on all 380 curves, `G = 1`],
  [max distinct cost per target 1; `G` \= 1.0000, CI \[1.0000,
  1.0000\]],
  [exact],
  [planted S2 × 1.05],
  [detected],
  [1.0456, 95% \[1.0156, 1.0774\], p \= 0.0040],
  [detected],
  [planted S2 × 0.95],
  [detected],
  [0.9460, 95% \[0.9188, 0.9748\], p \= 0.0006],
  [detected],
  [walk counts],
  [\[1, 3, 6, 12, 24, 48, 96\] each side],
  [the same from `volcano-rho` and from `ca volcano`],
  [passes],
  [side certificates],
  [all ok, no shared `j`],
  [760/760, 0 shared (re-run by the validator with another seed)],
  [passes],
  [audit],
  [`ecbench verify --replay 24`: exact],
  [24/24 on each of the three sessions, 0 problems],
  [passes],
)
]

#image("figures/controls.svg")

== What the measurement says, and what it does not
<what-the-measurement-says-and-what-it-does-not>
- #strong[Says (measured).] On this class, `rho.negation`\'s mean
  counted cost is the same on both sides of a prime conductor gap of
  631409, to within ±2.9% at 90%. That covers every level of two depth-6
  2-volcanoes and one instance carried across by an explicit
  degree-631409 isogeny.
  - The design can see a 5% shift (both planted controls).
  - BSGS confirms that the instance is the same everywhere.
- #strong[Does not say.]
  - That ECDLP is equally hard for every algorithm. That is Theorem 2\'s
    job, with its additive transfer cost.
  - Anything beyond `p ≈ 2^52`, `D₀ = −11` and this method.
  - The class is special (`t ≡ −2 mod N`), chosen so that the cross-gap
    isogeny is cheap and explicit. Rho never uses an isogeny, so the
    special choice does not enter the rho measurement. It does make the
    KKM isolation question trivial here (Theorem 3(g)).
- #strong[Transfer cost against rho (derived, same host, descriptive).]
  - Carrying one instance across the gap is two evaluations of `φ`:
    6.3·10⁶ `F_p` multiplications, about 0.04 s.
  - One `rho.negation` solve averages about 1.29·10⁶ group operations
    (`S·√n`), and 0.39 s of wall time on this host.
  - So the transfer is about a tenth of one rho solve in wall time here.
    Asymptotically it is `O(N) = O(p^{0.37})` in this family, against
    `√n`.

== Provenance
<provenance>
#align(center)[#table(
  columns: 5,
  align: (col, row) => (left,left,left,right,left,).at(col),
  inset: 6pt,
  [session], [spec], [arm], [runs], [audit],
  [`ECBS1h41a98e7c2141`],
  [`ECS1h7d9934575ec8` (`run/specs/spec.json`)],
  [`rho-neg`],
  [4560],
  [#link("run/sessions/spec.audit.json")[audit]: 4560 verified, 24
  replays exact],
  [`ECBS1h4034bfcbccc4`],
  [`ECS1h0d34427ad5a6` (`spec-aa.json`)],
  [`rho-neg-aa`],
  [1520],
  [#link("run/sessions/spec-aa.audit.json")[audit]: 1520 verified, 24
  exact],
  [`ECBS1h658495a369ec`],
  [`ECS1ha55be72ab53f` (`spec-bsgs.json`)],
  [`bsgs-neg`],
  [760],
  [#link("run/sessions/spec-bsgs.audit.json")[audit]: 760 verified, 24
  exact],
)
]

- #strong[Host.] Intel Xeon \@ 2.10 GHz, 4 vCPU shared VM, isolation L0.
  The three sessions ran concurrently, one per core. Wall times are
  descriptive only.
- #strong[Binaries and commit.] ecbench binary sha256 `a59c4014…`.
  Sessions recorded `git_commit f7b4d94` (the protocol freeze) with
  `git_dirty: true`; the untracked run directory was in the tree. The
  ecbench source is unchanged on this branch.
- #strong[Storage.] Sessions are committed with `records.jsonl` and
  `plan.json` gzipped (`gzip -n -9`).
  #link("run/sessions/MANIFEST.sha256")[`run/sessions/MANIFEST.sha256`]
  pins the uncompressed bytes. To re-audit, run
  `gunzip -k run/sessions/*/*.gz`, then
  `sha256sum -c run/sessions/MANIFEST.sha256` and `ecbench verify`.
- #strong[Lost file.] The `volcano-rho` exports first wrote their
  `instances.jsonl` to the binary\'s default `./volcano-rho-out`, where
  S2 overwrote S1. That file was removed. `run_pipeline.sh` now passes
  `--out`. The files the study uses (`curves.jsonl`, the specs) are
  unaffected.
- #strong[Independent checks.]
  - The validator re-ran the replays and recomputed the headline numbers
    from raw records (#link("validation/")[`validation/`]).
  - `ca volcano` (aburan28/cryptanalysis) re-walked both volcanoes
    (#link("run/crosscheck/")[`run/crosscheck/`]): same counts and 189
    verified edges per side. It prints no `j`, so curve identities were
    not compared.
- #strong[Run report.]
  #link("reports/run-20261022T0844Z/README.md")[reports/run-20261022T0844Z/].

== Requirement status
<requirement-status>
#align(center)[#table(
  columns: 3,
  align: (col, row) => (left,left,left,).at(col),
  inset: 6pt,
  [requirement], [status], [evidence],
  [A deep volcano],
  [done],
  [depth-6 2-volcanoes, 190 curves each, every curve measured],
  [A large prime conductor gap],
  [done],
  [`N = 631409 ≈ 77·p^{1/4}`, `log_p N = 0.371` (KKM\'s regime
  `p^{1/4} < N < p^{1/2}`)],
  [\"Hardness is the same\": proof],
  [proved where it can be],
  [Theorems 1–4; Lean: chain transfer and the twist-order arithmetic.
  The curve-level facts (Vélu, `End`) are checked numerically, not in
  Lean. The general distributional question (Problem B) is open.],
  [\"Hardness is the same\": measurement],
  [measured],
  [equivalent within ±5% for `rho.negation`; planted controls detected],
  [Isogeny transport across the gap],
  [done],
  [an explicit degree-`N` map and its dual, verified; cost measured],
  [Validation gate (replay, validator, rho reference)],
  [done],
  [#link("../../benchmarks/20261022-deep-volcano-prime-gap/VALIDATION.json")[`benchmarks/20261022-deep-volcano-prime-gap/VALIDATION.json`]],
  [Index calculus],
  [not run],
  [out of reach at `n ≈ 2^41` on this host; not an IC result, no
  leaderboard change],
  [Isolated (L2) timing],
  [not run],
  [shared VM; counted units only],
)
]

== Reproduce
<reproduce>
See #link("PROTOCOL.md")[PROTOCOL.md §9]. Figures: `python plot.py .` (a
scratch venv with matplotlib). It reads `analysis*.json`,
`run/pair.json` and the sidecars, and computes nothing. Lean:
`cd lean && lake build && lake env lean AxiomAudit.lean`. Tests:
`cargo test --release --bin volcano-gap` (one end-to-end test on a
30-bit class).
