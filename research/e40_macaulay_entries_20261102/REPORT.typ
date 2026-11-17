#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)
= E40: the chain solve's Macaulay matrix stored from each row's lead

*Run and results report, 2026-11-02.* Research note `research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md`: §9.50 (pre-registration, commit `5a0dd20af9`, after the Stage 0 count and before any solver code) and §9.51 (results). Code: `64a385da03` (Stage 0 instrument), `0787278d7b` (`src/cryptanalysis/macaulay_profile.rs`), `f2f31dbec1` (`klein_v4` hook, committed alone), `d6d00182c5` (record fields), `9b2a47238b` (harness), `86d0f86c25` (controls fix). Binaries built from clean, blob-checked exports and pinned by sha256 (`experiments/47_macaulay_entries/binary_sha256.txt`, `binary_commit.txt`). Records: `experiments/47_macaulay_entries/`. Host: the 4-vCPU container, shared with two other experiments (load about 17); isolab-eks unreachable, `conductor` not installed. *Operation counts only, no wall-time claim.* Python only plots recorded values; every number below comes from `examples/e40_stage0.rs`, `e40_stage_a.rs`, `e40_checks.rs` and `e40_tables.rs`.

== Outcome
- *Stage 0 (counts).* Of the 176,064 cells per call of the degree-11 Macaulay matrix ($393 times 448$), 73,237 (41.6 %) lie left of each row's lead. A forward echelon by lead and the left-looking normal forms never read them; only the dense first lead scan does (73,630 of 79,115 lead-scan reads). There are no duplicate rows and no untouched columns. 9 rows reduce to zero, holding 434 cells. One border target of degree 11 occurs per call, so $d = 11$ is forced.
- *Lever V (`chain-profile`).* Each row is stored from its lead. The arithmetic, pivots and outputs are A's. Every use of an offset is charged as a native operation (about 22,400 per call).
- *Identity holds everywhere.*
  - 7/7 existing configurations are byte-identical.
  - Stage A: 10,000/10,000 calls (outputs, fallback flag, every phase count, pivot digest, stored-cell counter, lead-scan relation).
  - End to end: 42/42 records equal A's (or A′'s) outside the tag, the timing fields, the lead scans and the two E40 fields.
- *Controls.*
  - P1 (lead cell dropped) was detected on 320/320 calls.
  - P2 (counter short by one per row) was detected on 320/320 calls.
  - The perturbed coefficient changed 213/213 non-empty solution sets.
  - The negative control A′ (A on another stream) is not ranked against A. V′ against A′ replicates the bands.
- *Band, 32 rows.*
  - Against A: $S_A slash S_V$ is 1.0000 / 1.2518–1.2618 / 1.3629–1.3807 at 0 / 0.1 / 1 GAE.
  - Against E25: $S_(E 25) slash S_V$ is 2.5054–2.5766 / 1.7455–1.8033 / 1.4105–1.4558.
  - *Ranked by §9.50's rule:* V is equal to A at 0 GAE (no improvement there), ahead of A at 0.1 and 1 GAE, and ahead of E25 at every weight.
  - Every one of G2.204–G2.215 holds.
- *Distance to rho* (walk (iii)): rho is ahead of every row at every weight. The best row is GLS $p = 397$ seed 1 arm B, with $S_V slash rho$ = $6.00 dot 10^3$ / $1.58 dot 10^4$ / $6.17 dot 10^4$ (A: $6.00 dot 10^3$ / $1.99 dot 10^4$ / $8.51 dot 10^4$).

#figure(image("figures/stage0_cells.svg", width: 85%), caption: [Stage 0 (`experiments/47_macaulay_entries/stage0.json`, GLS $p = 101$ seed 1, 200 calls): Macaulay cells per call by the weighted degree of the row's lead. The prefix (gold) is never read by the echelon or the normal forms; V stores only the tail (blue). The other 21 curves give the same split to within one cell per call.])

== Stage 0: where the native work of the build goes (per call, 22 curves × 200 calls)
#table(columns: 2, align: (left, right),
  [part], [count],
  [columns / rows built / shift candidates / skipped by row selection], [448 / 393 / 480 / 87],
  [monomials enumerated (and sorted) / selection tests / index inserts], [928 / 659 / 448],
  [coefficient writes = column hash lookups], [11,154–11,251],
  [dense cells `rows · cols` (priced by the band)], [176,064],
  [prefix left of the lead / tail], [73,237–73,239 / 102,825–102,827],
  [first lead scan / all lead-scan reads (A)], [73,630–73,632 / 79,114–79,116],
  [zero rows (cells) / duplicate rows / untouched columns], [9 (434) / 0 / 0],
  [rows in the normal forms' dependency closure (others' tails)], [334 (16,105–16,124)],
)
The other candidate levers are not proposed: zero rows (0.25 % of cells, unknown before the echelon; E21 closed the trace route), rows outside the dependency closure (they fix the staircase; a trace-shaped lever), degree truncation ($d = 11$ forced), per-curve reuse (moves no priced work), and $V_4$/$psi$ redundancy (the system is already invariant). Lead atlas: no lead covers this build. F4 trace reuse is closed (binary); F5 signatures and prime-IC constant engineering are open.

== Stage A (10 curves × 1,000 fresh calls, per-call means)
#table(columns: 2, align: (left, left),
  [quantity], [result (prediction)],
  [V stored cells, clean calls], [102,826–102,827 ($[1.02, 1.04] dot 10^5$)],
  [cells A / V, clean calls], [1.7122–1.7123 ([1.69, 1.73])],
  [V offset operations], [22,340–22,489 ($[1.5, 3.0] dot 10^4$)],
  [lead-scan reads A / V], [79,114–79,117 / 5,877–5,879],
  [$S_A slash S_V$ at 0 / 0.1 / 1 GAE], [1.000000 / 1.2447–1.2658 / 1.3534–1.3831],
  [σ1 (lead scans priced) at 0.1 / 1 GAE], [1.5881–1.6305 / 1.8389–1.8968],
)

#figure(image("figures/band.svg", width: 100%), caption: [End to end, 32 rows (20 GLS, 12 E24) on stream `0xE37E0001`, paired with E37's A and E25 records: $S_A slash S_V$ (left) and $S_(E 25) slash S_V$ (right) at 0, 0.1 and 1 GAE ($1 "GAE" = 97$ $F_p$ multiplications). Source: `experiments/47_macaulay_entries/summary.json`.])

== End to end (32 rows, Karatsuba, paired)
#table(columns: 4, align: (left, left, left, left),
  [quantity], [0 GAE], [0.1 GAE], [1 GAE],
  [$S_A slash S_V$], [1.0000], [1.2518–1.2618], [1.3629–1.3807],
  [$S_(E 25) slash S_V$], [2.5054–2.5766], [1.7455–1.8033], [1.4105–1.4558],
  [σ1 $S_A slash S_V$], [1.0000], [1.6024–1.6217], [1.8572–1.8921],
  [σ4 $S_A slash S_V$], [1.0000], [1.2362–1.2449], [1.3312–1.3461],
  [$S_V slash "rho"$], [$6.00 dot 10^3$–$1.98 dot 10^5$], [$1.58 dot 10^4$–$5.41 dot 10^5$], [$6.17 dot 10^4$–$2.36 dot 10^6$],
  [$S_A slash "rho"$], [$6.00 dot 10^3$–$1.98 dot 10^5$], [$1.99 dot 10^4$–$6.80 dot 10^5$], [$8.51 dot 10^4$–$3.24 dot 10^6$],
)
Negative control on stream `E40E0002` (20 GLS rows): A′ against A is ahead on 10 rows and behind on 10 (not ranked). V′ against A′ is 1.0000 / 1.2506–1.2608 / 1.3613–1.3790. The per-row table is in §9.51 and `tables.md`.

#figure(image("figures/rho.svg", width: 90%), caption: [Distance remaining to the matched rho (walk (iii)), per row, for A and V at 0 and 1 GAE (at 0 GAE they coincide). Rho is ahead on every row. Source: `summary.json`.])

== Deviations and limits
- The first controls harness compared the decomposition sets after the lift. Those were empty on all 320 calls, so the perturbed-coefficient control was vacuous. §9.50 cites E31's control, which is defined on solution sets. The harness was corrected (`86d0f86c25`, binary `e40_checks_v2`) and rerun before any verdict. Both runs are kept (`checks_v1.*`, `checks.*`).
- A falling-back call is charged A's convention ($2 dot "rows" dot "cols"$), so V gains nothing on those calls (conservative).
- At 0 GAE, V and A are identical. The improvement exists only at the band's positive weights.
- Scope: E25's and E24's curves, $p <= 521$, degree 11, one host. The 176,064-cell figure for A is the registered model. No wall-time claim is made.
- The validation gate, the leaderboard and `benchmarks/` are left to the main session.
