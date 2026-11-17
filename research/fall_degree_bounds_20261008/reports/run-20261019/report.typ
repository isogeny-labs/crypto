#set page(paper: "a4", flipped: true, margin: 1.4cm)
#set text(size: 9.3pt)
#show raw: set text(size: 8.3pt)
= Last fall degree at m = 5, 6 — run report (E7, E6 completion)

*Study* `research/fall_degree_bounds_20261008` (aburan28/crypto). Amendment 7 (cells, gate, reading rule) was committed before any E7 draw. Amendment 8 moved the N = 54 cell to the local host before any of its draws. *Engine* `examples/hky_par.rs`, a parallel, chunked mutant closure (sha256 `24cf10d6…`). Host: Claude Code cloud container, 4 vCPU, 15 GB. Wall times are logged; the study is untimed.

== Gate (on the exact E7 binary)
- It reproduces `d_F` and every per-degree closure dimension of the Sage-validated engine on 24 committed draws: m = 3 (5,2), (7,3), (11,4), (13,4), (13,5) and m = 4 (9,3). Result: `AGREE 24 DISAGREE 0`.
- It reproduces the four E6 m = 5 (10,2) draws bit for bit. They took 127–168 s, where the serial engine took 787–5264 s.

== Recorded values
#figure(image("fall_degree.svg", width: 100%), caption: [Left: m = 2 (E3). Middle: d\_F against unknowns N for every unsatisfiable Semaev draw. Stars mark the E7 draws (m = 5 purple, m = 6 magenta). Plain-tower D\* is shown as × (▲ = lower bound), and structure-matched random controls as orange squares. Right: recorded degree-4 closure wall time (points), and the dense memory that a degree-4 basis needs (dashed), against the 15 GB host and the ≥ 64 GB tier.])

#table(columns: 6, inset: 3.5pt,
  [*m*], [*(n, ℓ)*], [*N*], [*columns ≤ 4*], [*d\_F*], [*degree-4 closure, s*],
  [4], [(13, 4) (E6 B completion)], [42], [124,314], [4 4 4 4], [272–328],
  [6], [(8, 2)], [44], [149,986], [4 4 4 4], [68–172],
  [5], [(11, 3)], [48], [213,053], [4 4 4 4], [334–553],
  [6], [(9, 2)], [48], [213,053], [4 4 4 4], [90–124],
  [5], [(13, 3)], [54], [342,541], [4 3 4 4], [728–839 (peak 12.6 GB)],
)

*Reading (Amendment 7): "d\_F bounded through m = 6"*. All 16 E7 draws have d\_F ≤ 4. Not run: (6, 10, 3) N = 58 and (5, 15, 3) N = 60, the ≥ 64 GB tier, because no host was available. They are not extrapolated. N ≈ 100 needs about 4.1M columns (a ≈ 1 TB dense basis), so it needs a sparse method.

== Checks
- Lean `lean/FallDegree/Verdict.lean`, kernel `decide`, 206 rows. All earlier verdicts still hold, and `e7Bounded = true`, `e7Grows = false`. Changing one E7 draw to 5 fails the build. CI regenerates `Data.lean` from the records.
- cairn: result objective `sha256:16fc1da8…` was accepted by the kernel. A first attempt was "unavailable" because `lean` was not on PATH; it is logged. `cairn audit`: chain intact.

== Scope
Toy sizes only (m ≤ 6, N ≤ 54). This is not a bound. HKYY's conditional requires d\_F = ω(m²) at m ≈ n^(1/3) unless binary ECDLP is subexponential. Conditional crossover model (`e6/crossover_model.md`): even if d\_F = 4 held at every size, IC would beat rho only from n ≈ 233–283. No exponent, speedup or security claim.
