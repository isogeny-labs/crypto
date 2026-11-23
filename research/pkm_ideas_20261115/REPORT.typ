#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 10pt)
#set heading(numbering: "1.")
#set par(justify: true)

= Five follow-up ideas on PKM smooth-map factor bases

EXP-PKM-IDEAS-001 · 2026-11-15 · isogeny-labs/crypto PR \#4091 · protocol `PROTOCOL.md` (frozen in `c68ba4c2d6`
before any study row; Amendment 1 after the frozen run) · instrument `examples/pkm_ideas.rs` · rows `runs/`
(frozen) and `runs/ext/` (Amendment 1) · analysis `analyze.py` → `results.json` · figures `make_figures.py`.
Counted F#sub[p] multiplications (`field_ops`) only. Unisolated local host, 4 threads. Nothing here concerns
cryptographic sizes or Pollard rho [pcc:unverified].

The question behind the study was whether the auxiliary smooth isogeny kernel should be faster than a random factor
base. EXP-PKM-ODD-001 had already found that the isogeny and Kummer forms cost the same under F4 (0.76–1.47×). This
study asks the stronger version: is *any* smooth-map structure cheaper to solve than a random `V` of the same size?
Four further ideas ride along.

== Verdicts (frozen prediction column)

#table(columns: 4, align: left,
  [\#], [idea], [verdict], [what was measured],
  [1], [smooth tower far cheaper to solve than a random `V`], [*falsified*],
    [Writing membership as one dense polynomial `∏(x − v)` makes F4 *cheaper* than the tower at every completed `S`. Dense random ÷ tower: `m = 2` 0.052, 0.018 (`S = 16, 32`, frozen); 5.5·10⁻³, 2.5·10⁻³, 9.8·10⁻⁴ (`S = 64, 128, 256`, Amendment 1). `m = 3`: 0.54, 0.081, 0.040 (`S = 4, 8, 16`). The ratio falls with `S`, the reverse of the prediction.],
  [2], [`j = 0` automorphism closes a Kummer tower with `3 | S`], [*held*],
    [Exact at `p = 1000621`, prime order `n = 1002493`: `ω(F) = F`, `ω(P) = [λ]P` on every factor-base point, decomposition counts invariant over 2,000 targets, columns `|F|/6` (3 at `|F| = 18`, 4 at `|F| = 24`), random sets: 0 elements closed.],
  [3], [trace replay costs ≤ 0.5 of fresh F4], [*falsified* (threshold)],
    [No mismatch and no disagreement in 44 replays. Replay is cheaper in every cell, ratio 0.38–0.91, but above 0.5 in 5 of 11 cells (`2^7` 0.91, `3^3` 0.77, `3^4` 0.58, `m = 3` `2^2` 0.70, `2^4` 0.53).],
  [4], [one trace serves every curve and prime of a shape], [*held*],
    [Cross-curve (seed 77) and cross-prime (`P2`) on `2^6`, `3^4`, `m = 3` `2^3`: 0 mismatches in 24 replays; fresh and replay counts are bit-identical to the same-curve cells.],
  [5a], [per-point cosets change coverage, not F4 work], [*held*],
    [Per-point ÷ shared = 1.000 at `S = 16 … 256` (identical counts).],
  [5b], [per-point cosets cover ≈ 2× the shared count], [*held*],
    [Per-point ÷ shared 3.11, 2.36, 2.44 (`S = 16, 32, 64`); random sets: per-point ÷ shared 1.99, 1.70, 1.89.],
)

== Idea 1: is the smooth tower cheaper to solve than a random `V`?

#figure(image("figures/idea1_dense_vs_tower.svg", width: 100%),
  caption: [F4 work on refutation systems. Blue: the degree-2 Kummer tower (committed EXP-PKM-ODD-001 rows, same `p`,
  `m`, `S`). Red: random `V`, membership written as `∏(x − v)`. Green: the same dense form for the Kummer coset.])

*No.* For the same `|V|`, writing membership as a single polynomial of degree `S` makes F4 far cheaper than writing it
as the tower of degree-2 maps, and the gap widens. Fitted growth over the ladder: tower `S^3.47`, dense `S^2.04`
at `m = 2`; tower `S^6.1`, dense `S^5.0` between `S = 8` and `16` at `m = 3`. A random `V` and the smooth coset
`g·μ_S` cost the same in the dense form (ratio 0.97–1.00), apart from `m = 3`, `S = 4` (0.23), where the 2-term
membership `x^S − g^S` gives the coset an edge that vanishes by `S = 8`.

What this says, and what it does not:

- The tower's merit is that `V` is *describable* by low-degree maps. Under F4, as measured here, that description
  costs more than the dense polynomial it replaces: the tower adds `t·m` variables to save degree. At `m = 2` the dense
  system has 2 variables and is close to a univariate problem.
- So "the auxiliary smooth isogeny kernel is faster than random" is not supported by this instrument: the isogeny
  form costs the same as the Kummer form (EXP-PKM-ODD-001), and the smooth form gives no F4 advantage over a random
  `V` of the same size.
- Comparisons here are across two studies: the same F4 code, the same `p`, `m`, `S` and seed, and refutation targets
  only. The dense refutations are F4's own verdict, not checked against exhaustive search as the tower rows were.
  Building `∏(x − v)` costs about `S²/2` uncounted multiplications (3.3·10⁴ at `S = 256`, under 0.4% of the dense
  count).
- Finding decompositions, as opposed to refuting them, was not measured here. Nor is this a statement about larger `p`
  or other solvers.

Amendment 1 (dated, after the frozen run): the frozen degree bound of 64 censored every `m = 2` dense cell at `S ≥ 64`
before any work, because the membership polynomial alone has degree `S`. Those nine rows stay censored in `runs/`. The
same cells were rerun with `--dmax 1024` into `runs/ext/` and are shown as an extension. The idea-1 verdict was
already reached on the frozen rows (`S = 16, 32` at `m = 2`, all of `m = 3`).

== Ideas 3 and 4: trace replay

#figure(image("figures/idea3_replay_ratio.svg", width: 95%),
  caption: [Replay ÷ fresh F4 work. One trace is recorded per shape on (`P1`, seed 20261115).])

Replaying a recorded F4 trace (Joux–Vitse "F4 remake") never mismatched: not on 44 same-curve replays, and not on 24
cross-curve and cross-prime replays. The saving is real but smaller than predicted: 0.38–0.91 of fresh work, against
the frozen threshold of 0.5. By the frozen rule idea 3 is falsified; the replay speed-up itself is not in doubt.
Idea 4 holds. One trace serves another curve and another prime of the same shape, and the counts are bit-identical,
consistent with EXP-PKM-ODD-001's finding that F4 work here depends only on the shape.

== Idea 2: `j = 0` automorphism

On `y² = x³ + b` with `p ≡ 1 (mod 3)`, `ω: (x, y) ↦ (ζ₃x, y)` maps `x` to `ζ₃x`. When `3 | S`, the coset `g·μ_S`
contains `ζ₃`, so `ω` maps the factor base to itself. Every check held exactly, and the linear algebra has one column
per `⟨ω, −1⟩` orbit: `|F|/6`, a third of the negation-only count. This is the usual automorphism saving in index calculus,
here shown to be compatible with the PKM coset. It is exact, not a measurement of speed.

== Idea 5: one coset per point

#figure(image("figures/idea5_coverage.svg", width: 70%), caption: [Exact `m = 2` coverage at `p = 1001089`.])

Giving each summand its own coset `g_i·μ_S` leaves F4 work unchanged: the counts are identical. It raises coverage
`|{P₁ + P₂}|` by 2.4–3.1×, more than the 1.7–2.0× that two independent random sets gain over one. With a shared coset
the sums collide more than random ones (99 vs 201 at `S = 16`). Per-point cosets remove that deficit and more.

== Reproduce

`run.sh` (frozen cells), then `run_ext.sh` (Amendment 1), then `python3 analyze.py` and `python3 make_figures.py`.
Binary SHA-256 values are in `binary.sha256` and `runs/ext/binary.sha256`; the Amendment 1 binary differs only by the
`--dmax` flag, defaulting to 64.
