# Protocol Q5: replay the grevlex comparisons of Q2 and Q4 with msolve (F4)

Status: **frozen before data** (2026-11-02). No msolve run of these systems
existed when this was written. The only msolve calls made before the freeze
were on two toy systems (`x²-y, xy-1` and `x-1, x-2`), used to learn the
input and output formats.

## Why

Q2 and Q4 measured cost with this study's own Buchberger engine (counted
termops). The report flagged that constant factors from a hand-written
Buchberger may not carry over to F4. Q5 repeats the **grevlex** comparisons
with msolve's F4. The systems, seeds, targets and arms are identical, and the
cost is measured as F4's linear-algebra size.

## Engine

- **msolve.** Version 0.6.5, the Ubuntu noble package `msolve 0.6.5-1build2`,
  the same version the repository's earlier msolve studies used.
- **Invocation.** `msolve -v 2 -g 2 -t 1 -f <in> -o <out>`, under `timeout 120`.
  - The order is DRL (msolve's only order), with variables listed in the
    harness order `x_{0,0} > x_{1,0} > x_{0,1} > …`.
  - All primes are below 2¹⁶, so the default linear-algebra kernel is safe.
    (The A5 note in `research/prime_field_leads_20261031` concerns
    characteristics above 2¹⁶.)
- **Primary counted unit.** `cells` = Σ over F4 rounds of rows × cols, read
  from the `-v 2` round table; the final interreduction is excluded. For a
  fixed msolve version and input it is deterministic.
- **Secondary.** Number of rounds, maximum round degree, largest matrix, and
  `wall_us`. Wall time is unadmitted, for the same reason as in `PROTOCOL.md`.
- **Censoring.** A timeout marks the row censored.

## Systems (unchanged from Q2 and Q4)

- **Q2 part.** Cells (p, k) ∈ {257, 1153, 7681, 12289} × {3, 4}, targets 0–3,
  and arms A0, A1, A2 (2 values of α), B (2 kernels) and B2 (2 kernels).
  Each (target, arm instance) gets 6 "dec" and 6 "nodec" values of R, with the
  Q2 seeds (label `R/…`).
- **Q4 part.** The same cells, confirmation targets 4–7, Q4 seeds (label
  `Q4R/…`), and candidates C_nat, C_rep, C_sel_grevlex, C_sel_lex, C_inv,
  C_ker_nat, C_best, A1 and D.
  - Candidates are selected exactly as frozen in `PROTOCOL_Q4_SEARCH.md`:
    the selection split uses Buchberger termops and is recomputed
    deterministically. **There is no reselection on msolve cost.**

## Gate

For every system:
- msolve returns the basis `[1]` exactly when the brute-force V × V check finds
  no witness;
- any other basis means the system is consistent.

Any mismatch is a gate failure and is reported.

## Decision rules (grevlex only; 8 cells × 2 strata = 16 comparisons each)

**Q5a: does presentation explain the cost under F4?**
- **Presentation explains it** if, in ≥ 12/16 comparisons, the ratio of median
  `cells` A2/B lies in [0.8, 1.25].
- **Presentation does not explain it** if that ratio lies outside the band in
  ≥ 12/16.
- Otherwise: inconclusive.
- Also reported: B/A1, A2/A1, and B2/B.
- "The isogeny slowdown persists under F4" if B/A1 ≥ 1.5 in ≥ 12/16.

**Q5b: Q4's grevlex verdict under F4.** For C_sel_grevlex / C_nat (median
`cells`, 95% bootstrap CI, 4000 resamples, seeded):
- **Improvement** if ratio ≤ 0.8 with CI upper bound < 1 in ≥ 12/16;
- **No improvement** if that holds in 0/16;
- otherwise inconclusive.
- The same rule is reported, not as a decision, for C_sel_lex / C_nat (does the
  lex winner help F4?) and C_best / C_nat.
- The gap to μ_k, C_sel / A1, is reported as distance remaining.

## Execution

- **Where.** Local in the session container, through a director/executor
  task card. EKS is unreachable from this session (see `PROTOCOL.md`).
- **Replay.** `cells` is deterministic, so a replay on any host with the same
  msolve version must reproduce it byte for byte.
- **Stop.** Run once; no reruns with changed seeds, timeouts or msolve options.

## Amendments

(none)
