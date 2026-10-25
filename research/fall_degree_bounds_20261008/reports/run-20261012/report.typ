#set page(paper: "a4", margin: 1.6cm)
#set text(size: 9.3pt)
#show raw: set text(size: 8.3pt)
= Last fall degree of Weil-descended Semaev systems — run report (E3, E4, E5, E6)

*Study* `research/fall_degree_bounds_20261008` (aburan28/crypto): pre-registration 2026-10-08, Amendments 2–6 (2026-10-12) each committed before the runs they govern. *Engine* `examples/hky_last_fall_degree.rs` (native Rust; E5 gate: reproduces all 32 reproducible Sage records exactly); independent cross-check `examples/hky_crosscheck.rs`; E6 driver `examples/hky_grid.rs`. Host: Claude Code cloud container, 4 vCPU, 15 GB; untimed study (`secs` logged, not measured). Binary hashes and the in-place rebuilds/restart: `e3/PROVENANCE.md`, `e6/PROVENANCE.md`.

== What ran
E3: m = 2, random base, n ∈ {18,20,22,24,26,28,32}, 8 draws × rand/sat (112 draws, all completed). E4: the four registered m = 3 dreg-ladder cells, committed unsat draws (16). Extension/E6 A: committed unsat draws at m = 3 (9,3), (11,4), (13,4), (13,5), (15,5) and m = 4 (5,2), (7,2), (9,3) (32). E6 B: new draws, m = 3 (18,6) N = 36 and m = 5 (10,2) N = 40 (8, complete); m = 4 (13,4) N = 42 *still running at report time, not included*. E6 C: committed sat m = 3 draws (6). E6 D′: structure-matched random controls (12).

== Recorded values
#figure(image("fall_degree.svg", width: 100%), caption: [Left: m = 2 last fall degree d\_F per draw against n (native E3; Sage medians for n ≤ 16; semi-regular D\_reg). Right: m ≥ 3 against unknowns N — Semaev d\_F (unsat), recorded plain-tower D\* (▲ = lower bound), Semaev sat d\_last (◇), and random controls of the same shape (squares; ▲ = lower bound). Records in `e3/`, `e4/`, `e6/`.])

#table(columns: 4, inset: 3.5pt,
  [*arm*], [*cells (N)*], [*Semaev d\_F / d\_last*], [*reference*],
  [E3, m = 2], [n = 18–32 (N = n)], [4 on 112/112], [D\_reg 5; P3a, P3b held; LOG ≻ LIN (MAE 0.145 vs 0.491); kill condition not met],
  [E4, m = 3], [(5,2) (7,2) (5,3) (7,3); N 11–16], [3–4], [plain D\* 5–6; P4a, P4b 16/16; cross-check agrees],
  [m = 3], [(9,3) (11,4) (13,4) (13,5) (15,5) (18,6); N 18–36], [4 everywhere], [plain D\* 6, 6, 6, ≥7, ≥7, —],
  [m = 4], [(5,2) (7,2) (9,3); N 18–30], [3–4], [plain D\* 6, 6, ≥7],
  [m = 5], [(10,2); N = 40], [4, 3, 4, 4], [—],
  [sat, m = 3], [(7,3) (11,4)], [d\_last 4 on 6/6], [solution counts match the ladder exactly],
  [controls], [n5l2, n7l3, n11l4 shapes; N 11, 16, 23], [Semaev 3–4], [control d\_last 5, 6, ≥ 7 (rises with N)],
)

== Checks
Lean `lean/FallDegree/Verdict.lean` (kernel `decide`, 186 rows): P3a, P3b, P4a, P4b hold; "bounded at toy size" and "Semaev-specific" readings hold; a mutated verdict fails. cairn result objective `sha256:fc828009…` accepted by the kernel; `cairn audit` (epochs 5 s): chain intact. The rule transcription into Lean postdates part of the data; the prose rules it encodes do not.

== Scope
Toy sizes only (m ≤ 5, N ≤ 40). Not a bound: HKYY's conditional requires d\_F = ω(m²) at m ≈ n^(1/3) unless binary ECDLP is subexponential. Conditional model (`e6/crossover_model.md`): even if d\_F = 4 held forever, IC would beat rho only from n ≈ 233–283; ECC2K-130 stays above rho by 36–51 bits. No exponent, speedup or security claim.
