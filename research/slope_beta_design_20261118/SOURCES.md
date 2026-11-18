# Sources checked (2026-11-18)

Knowledge-base note: the shared `research-kb` MCP server and the Python lead
tools were **not** queried — this task was run under an owner constraint of "no
Python tooling; analysis and writing only", so sources are the repo files read
directly (cited by `repo:path`) and the public literature (WebSearch). This is a
design, so every claim is sourced to the study's recorded numbers, the cost-model
code, or the cited literature; none is a new measurement.

## Repo sources (crypto, worktree wt/slope-beta)

- `crypto:research/ecbench_frobenius_orbit_vs_rho_20261014/README.md` — verdict,
  panel, H1–H6, the three trend fits, the §5a robustness (natives at zero).
- `crypto:research/ecbench_frobenius_orbit_vs_rho_20261014/PROTOCOL.md` — §1 cost
  model (n c²/2 + r/(2n²c), c\* ∝ (r/n³)^{1/3}, IC ÷ rho ∝ r^{1/6} n^{-1/2}); §5
  hypotheses and decision rules; §6 "what cannot be claimed".
- `crypto:research/ecbench_frobenius_orbit_vs_rho_20261014/trend/trend-orbit-vs-rho-frob.json`
  — β/γ point estimates, 95% intervals, χ²/dof, the 8 per-curve points with
  log2_r, n, ratio, se_ln.
- `crypto:src/cryptanalysis/ecbench/orbit_ic.rs` — the measured method: "bounded
  enumerated pair-table" (`:1`), phases (`:474-480`), pair-table nested loop
  `pair_visits` (`:333-389`), `trials` loop (`:400-407`), exact RREF rank
  (`:85-148`), the `*_uncharged` linear-algebra counters.
- `crypto:src/cryptanalysis/ecbench/full_price.rs`, `full_prices.json` — the §5a
  native pricing; counter rules.
- `crypto:RESEARCH_SEMAEV_ROOTFIND_VS_GROEBNER.md` — root-finding vs Gröbner
  decomposition subroutine; "m=2 one lookup, m=3 is |F| lookups, m=4 is |F|²";
  explicit statement that the asymptotic group-op count of IC is unchanged.
- `crypto:RESEARCH_KOBLITZ_INDEX_CALCULUS.md` — invariant-subspace classification
  (cyclotomic cosets; empty where 2 is primitive mod n, `:326-334`);
  unknowns(n,ℓ,m) = m·ℓ + (m−2)·n (`:384`); first-fall degree ≤ 3 to n = 63,
  "the variable count that bites" (`:387-389`); m = 2 needs dim ≈ n/2.
- `crypto:RESEARCH_IC_LEADS_ASSESSMENT_20261018.md` — where an exponent story
  could live (chain-free m ≥ 5, "no known formulation fits", `:90-93`); chain
  model floor 2^56 vs rho 2^60 on ECC2K-130 (`:34-39`); yield conserved by
  `|B|^m/r` (`:57,61`); equivariant F4 limits n prime, d ≤ 40 (`:140`);
  Joux–Vitse trace replay net loss (`:141`).
- `crypto:AGENTS.md` §5a (`:1271-1315`) — price every arm's unpriced work at
  0/0.1/1 GAE; `w_cal` for field counters; rank only where the order holds
  across the band.

## Literature (WebSearch, 2026-11-18)

- Gaudry, "Index calculus for abelian varieties ... and the ECDLP" — Õ(q^{2−2/n})
  for summation-polynomial decomposition. Galbraith–Gaudry survey (DCC 2016),
  "Recent progress on the ECDLP" — reviews the complexity claims; Diem's
  subexponential regime holds for n ∈ Θ(√log₂ q).
- Diem (via Galbraith–Gaudry / Karabina): for **q = 2 and n prime**, expected cost
  O(e^{n(log n)^{1/2}}) — **exponential, does not beat Pollard rho**. This is the
  regime of the entire panel.
- Petit–Quisquater, Asiacrypt 2012 (eprint 2012/146) — heuristic subexponentiality
  for binary ECDLP, resting on first-fall-degree ≈ degree-of-regularity.
- Shantz–Teske; Kousidis–Wiemers; Point-Decomposition-Problem paper
  (arXiv 1504.02347) — extended experiments and first-fall bounds that temper the
  Petit–Quisquater heuristic.
- Joux–Vitse (J. Cryptology, doi 10.1007/s00145-011-9116-z) — improved asymptotics
  when n = Ω(∛log₂ q); "nearly sparse" linear algebra / trace reuse.

Takeaway the literature establishes for this design: the one route that can
change p_IC (algebraic summation-polynomial decomposition) is, for prime-n
binary curves, expected-exponential on the best rigorous result and
subexponential only under a contested heuristic — consistent with the repo's own
first-fall/variable-count measurements.
