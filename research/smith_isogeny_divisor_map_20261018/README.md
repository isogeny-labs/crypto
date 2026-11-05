# W3b: Smith's explicit isogeny ψ on divisor classes, and a toy Route S solve

2026-10-25. Workstream W3b of
[`research/plane_quartic_program_20261018/PROGRAM.md`](../plane_quartic_program_20261018/PROGRAM.md).
Branch `claude/smith-isogeny-divisor-map-20261018`. Frozen protocol:
[PROTOCOL.md](PROTOCOL.md) (frozen 2026-10-18, not edited; deviations listed
below). Report with graphs: [REPORT.pdf](REPORT.pdf) (source
[REPORT.typ](REPORT.typ)). Depends on W1 (PR #2227, canonical `Pic^0` keys on
smooth plane quartics), which is merged into this branch.

Synthetic toy fields only. No key-recovery, sub-rho, deployed-curve or
standardized-curve claim. Nothing was compared with an external implementation
(no Magma, Sage or PARI on the host), so there is no fastest-implementation
claim.

## What was built

- `src/cryptanalysis/smith_isogeny.rs`: ψ = (π_X)_*(π_H)^* of Smith 2009
  (arXiv:0806.2995, §§5–6) on Mumford divisors of H: y² = F(x), deg F = 7,
  as effective degree-6 divisors on the plane quartic C ≅ X (the canonical
  image of the same correspondence, fitted by linear algebra). On classes,
  Ψ_A(x) = [ψ(D′)] − [ψ(A)] with D′ ~ x + A.
  - `W1Bridge`: W1's `PlaneQuartic` on the fitted quartic. An effective
    degree-6 divisor becomes the 4 cubics through it (`Gen { k: 3 }`, tangency
    rows for multiplicity 2) and `reduce_deg6` gives its canonical class;
    Ψ(x) = reduce(plus) − reduce(minus), compared by `PicClass::key`.
  - `PicEngine` for both W1 and the branch's own degree-6 adjoint `Pic`;
    `draw_sample` + `run_engine` run T2–T6 on identical ψ-images;
    `image_structure` enumerates J_H(F_q) and checks image = J_H/S;
    `subgroup_rational_elements` gives S(F_q) from the subgroup's pairs.
  - **Which Pic^0 arithmetic was kept:** W1's `PlaneQuartic` is the
    arithmetic (group law, keys, the T8 discrete log). The branch's adjoint
    `Pic` is kept only as the protocol's independent second engine (T9).
- `src/cryptanalysis/smith_route_s.rs`: the Joux–Vitse m = 3, r = 5 descent
  of `research/ghs_cubic_hyperelliptic_genus3_20261013/hyper_genus3.rs`
  (lines 7–220, reformatted), plus a new driver: planted ECDLP on E0/F_{q³},
  transfer φ = (1+Fr+Fr²)∘π0* by Cantor, H made monic, Props. 2 and 4, ψ to
  C, dlog in J_X(F_q) by brute force on W1 keys, k′G = Q checked on E0.
- `examples/smith_isogeny.rs`: `verify`, `deg8`, `route-s`, `cost`,
  `cost-fit`, `summarize`. Every run appends JSONL to `data/`.
- Defect fixed on the way: `Fld::sqrt`'s non-residue search never ended in
  some fields (q = 17, modulus x² + 3x + 4: the search line held only
  squares); it now enumerates field elements from z.

## Results (all from `data/`; `data/summary.json` has the totals)

### T1 (curve), q ∈ {7, 11, 13, 17, 19, 23}, 6 curves per q, degree 7

36 curves (from 221 scanned). 32 pass every T1 item (nullity 1, 2 096
hold-out points all on C, smooth by both certificates, #C(F_{q^k}) =
#H(F_{q^k}) for k = 1, 2, 3, equal L-polynomials). 4 (2 at q = 17, 2 at q = 23)
have X hyperelliptic (images on a conic; T1(a)'s separate outcome), so no plane
quartic: T2–T6 do not apply to them. Degree 8 (T1 only): 33 of 36 pass, 3 have
X hyperelliptic. T1(f) on degree-8 curves, k = 1: 39 of 78 twist maps checked,
39 agree with N₁(X) = 2(q+1) − N₁(H), 0 disagree, 39 not applicable (degenerate
fibre or N₁(H) = q + 1); k = 2, 3 not run.

### T2–T6, two engines (pass / fail / skip, summed over q)

| check | W1 canonical keys | degree-6 adjoint engine |
|---|---|---|
| T2 Ψ_A(x) ~ Ψ_A′(x) | 932 / 0 / 28 | 793 / 0 / 167 |
| T3 Ψ(x+y) ~ Ψ(x)+Ψ(y) | 917 / 0 / 43 | 778 / 0 / 182 |
| T4 Ψ(d) ~ 0, d ∈ S(F_q) \ 0 | 74 / 0 / 0 | 68 / 0 / 6 |
| T4 Ψ(x) ≁ 0, x ∉ S(F_q) | 3147 / 0 / 53 | 2927 / 0 / 273 |
| T5 Ψ(G) ≁ 0, G of odd prime order ℓ | 177 / 0 / 3 | 164 / 0 / 16 |
| T5 ℓ·Ψ(G) ~ 0 (added on C) | 177 / 0 / 3 | 126 / 0 / 54 |
| T6 #J_H · Ψ(x) ~ 0 | 932 / 0 / 0 | 516 / 0 / 277 |

No check failed on either engine. A skip is an instance the engine could not
decide (W1 bridge: an image point of multiplicity > 2; degree-6 engine: its
exceptional / overlap / frame failures) and is not counted as a pass.

**Class-level structure (W1 keys, every curve at q ≤ 13):** all of J_H(F_q)
enumerated on 18 curves (26 120 elements, count equal to L_H(1) on each): on
all 18, ker Ψ ∩ J_H(F_q) = S(F_q) exactly and every fibre has #S(F_q)
elements, i.e. the image is J_H(F_q)/S(F_q). (442 elements had an image with
a point of multiplicity > 2 under the first base and were recomputed with
another base; none remained unreduced.)

### T8: Route S end to end

| q | candidates | ℓ ≥ 5 filter | run | not reached (cap) | Props. 2+4 pass | X a plane quartic | k′G = Q verified (= planted k) | skipped: X hyperelliptic |
|---|---|---|---|---|---|---|---|---|
| 7 | 2 352 | 2 352 | 2 352 | 0 | 1 596 | 1 300 | 1 300 | 296 |
| 11 | 24 200 | 22 000 | 22 000 | 0 | 18 150 | 15 662 | 15 662 | 2 488 |
| 13 | 56 784 | 56 784 | 56 784 | 0 | 44 616 | 40 562 | 40 562 | 4 054 |
| 17 | 221 952 | 221 952 | 22 690 | 199 262 | 17 686 | 16 531 | 16 531 | 1 155 |
| 19 | 389 880 | 383 724 | 16 152 | 367 572 | 12 401 | 11 497 | 11 497 | 904 |
| 23 | 1 024 144 | 1 001 880 | 8 821 | 993 059 | 6 683 | 6 392 | 6 392 | 291 |

Totals over the six q: 91 944 planted ECDLPs solved and verified on E0.
q = 7, 11, 13 cover every instance; q = 17, 19, 23 ran under the protocol's 1 h cap; their rows cover the first
instances in the construction's order (lowest census index of r₀ first), not a
uniform sample. Every DLP attempt whose quotient X is a smooth plane quartic
recovered the planted scalar and passed the check k′G = Q on E0; on each of
them #J_X(F_q) = #J_H(F_q) = #E0(F_{q³}), φ(G) ≠ 0 with ℓφ(G) = 0, and
Ψ(φG) ≠ 0 with ℓΨ(φG) = 0. The only skip reason is "X hyperelliptic". Every
descended h was a squarefree degree-7 F_q polynomial; every instance has a
trigonal map passing Prop. 2. Props. 2 and 4 pass per instance (some rational
tractable subgroup with a rational trigonal map and lc(s) a square) at
67.9% (q = 7), 82.5% (11), 78.6% (13), 77.9% (17), 76.8% (19) and 75.8% (23),
against the 18.6% that Smith's Theorem 2 predicts for a random genus-3 curve.
The comparison is not like for like: every H of this family has cycle type
(3,3,1,1) on its Weierstrass points and so three rational tractable subgroups
(`odd_char_descent_genus3_20261014`), where a random curve has about one; and
at q ≥ 17 the instances are the first ones in order, not a sample. The DLP stage
is brute force; nothing here is an index calculus or a cost comparison with
rho (that is W2b).

### T7: cost of ψ per divisor

200 generic degree-3 divisors per q, q ∈ {7, 11, 13, 17, 19, 23, 101, 1009,
10007, 65537, 1000003, 2³¹ − 1}; median 97 332 F_q multiplications at q = 7,
946 296 at q = 2³¹ − 1. OLS of log₂(cost) on log₂ q (12 points, 95%
t-interval): multiplications 0.099 [0.064, 0.133]; priced (inversions at the
measured F_{q⁶} time ratio) 0.107 [0.079, 0.135]; wall 0.106 [0.087, 0.124].
Consistent with the polylog model the protocol expected; the fit does not
establish one. Unpriced: factoring u over F_q, Frobenius-orbit grouping,
sorting, hashing. Wall times: shared host (load average above 80), one thread,
nice 10, not pinned. Graphs and PDF copied to
[`benchmarks/20261018-smith-isogeny-psi/`](../../benchmarks/20261018-smith-isogeny-psi/);
no row was added to `benchmarks/README.md`, because its validation gate
(`docs/VALIDATION_GATE.md`: replay certificate, validator report, matched rho
reference) is not met, and a rho reference does not apply to the cost of ψ.

## Deviations from the protocol (recorded; the protocol was not edited)

- JSONL rows are per curve (with per-check tallies) and per Route S instance,
  not one row per check instance.
- T1(f) at k = 1 only (on degree-8 models, where `x_point_count` exists).
- T9(ii) (class count at q = 7) not run for the degree-6 engine; that engine
  is validated by its p = 37 group-law test and by never disagreeing with W1.
- T7 wall times not on pinned cores.
- T8 filter: the protocol's "a prime ℓ ≥ 5 divides #E0(K)" (largest prime
  factor used); the old prototype's #E0 = 4ℓ / 8ℓ quota rule is recorded per
  instance (`old_cofactor_rule`).
- The degree-7 sampler is `random_curve`'s scheme with degree 7 and monic F
  (`random_curve7`).

## What is left

- Cost at growing q of the pipeline against matched rho: needs an index
  calculus on the quartic in place of the brute-force dlog (W2b).
- The X-hyperelliptic case: ψ then lands on a hyperelliptic Jacobian; it is
  detected and skipped, not modelled.
- T1(f) at k = 2, 3; T9(ii) for the degree-6 engine.

## Reproduce

```bash
export CARGO_TARGET_DIR=/path/to/target
cargo test --release --lib -- smith_isogeny plane_quartic_pic
cargo build --release --example smith_isogeny
B=$CARGO_TARGET_DIR/release/examples/smith_isogeny
for q in 7 11 13 17 19 23; do $B verify --q $q; $B deg8 --q $q; done
for q in 7 11; do $B route-s --q $q --cap-s 3600; done; $B route-s --q 13 --cap-s 7200
for q in 17 19 23; do $B route-s --q $q --cap-s 3600; done
for q in 7 11 13 17 19 23 101 1009 10007 65537 1000003 2147483647; do $B cost --q $q; done
$B cost-fit && $B summarize
python3 reports/plot.py && python3 reports/diagram.py   # layout only
typst compile REPORT.typ
```

Route S per-instance records (`data/route_s_q*.jsonl.gz`, 6.4 MB) are not in
Git: the artifact-store policy (AGENTS.md §13) sends them to
`s3://setec-astronomy-artifacts`, and the push failed on this host (AWS token
invalid). Their SHA-256 and sizes are in `data/route_s_records.sha256`; the
files are kept in the worktree that produced them and can be pushed with
`python3 tools/artifacts/artifacts.py push research/smith_isogeny_divisor_map_20261018/data`.
A backup copy is in `/Users/adamburan/pq-work/w3b-route-s-records/` on the
producing host. `data/summary.json` holds every total quoted here. Exactly
reproducible from the seed (20261018): `verify`, `deg8`, Route S at
q = 7, 11, 13 (complete runs) and the T7 operation counts. Not exactly
reproducible: the q = 17, 19, 23 Route S rows (how far a 1 h cap gets depends
on host load) and all wall times. `gunzip -k` the records before `summarize`. The two report PDFs are in the artifact store
(`.artifacts.json` beside each; `artifacts.py pull` hydrates them). Run ids: start rows of each JSONL file
(commit and arguments); runs 2026-10-26T02:49Z to about 05:10Z UTC. q = 13 hit
the first run's 1 h cap at 53 173 of 56 784 instances; it was rerun with a
2 h cap and completed (the recorded file is the complete rerun).
