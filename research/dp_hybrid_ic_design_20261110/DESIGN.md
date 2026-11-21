# M4: index calculus on a distinguished-point factor base (design, not run)

2026-11-10 · design only, no experiment, no claim. Every number taken from the
repository is `[pcc:unverified]`. Everything else is derived here and labelled
that way.

**Verdict.** M4 is preprocessing rho with a worse search step bolted on. With a
hash-defined DP predicate, every variant is generic. Preprocessing rho already
meets the Corrigan-Gibbs–Kogan bound up to a constant, and the one "IC" step
M4 adds (decomposing a missed DP over the table) gets fewer hits per group
operation than walking on. With a high-bits DP predicate, M4 turns into the
small-x factor base whose reach is already closed (§2.1–2.4 of the program
note). **Recommendation: do not run.**

## 1. Notation (two different S)

`E` = advice, the stored (x, log) entries: ecbench's `entries` and the note's
`S`. `S_ecb` = ecbench's GAE/√r per target, a different quantity. `n` = the
group order; `W = 2^dp` = the mean walk length to a DP.

## 2. The algorithm

**Precompute (target-blind).** Choose an r-adding negation walk. Run walks in
⟨G⟩ from random [a]G until `E` distinct DPs are stored with their logs. Choose
`W ≈ √(n/2E)`, which is ecbench's default `dp_bits = round(log2 √(r/(2S)))`
`[pcc:unverified]` (`src/cryptanalysis/ecbench/methods.rs`, `rho.preprocessing`).

**Online, plain (= Bernstein–Lange).** Walk from [a]G + [b]Q to the next DP
d. If d is in the table, k = (a_d − a)/b. Otherwise restart. This is exactly
`PreprocessingRho::{build, solve}` (`src/cryptanalysis/ic_boundary.rs`) and
probe A's `precompute`/`online`.

**The IC twists considered.**

- **(T1) Decompose a missed DP over the table.** Look for d = ±d_i ± d_j with
  d_i, d_j in the table. Online, this means computing d − d_i for each i and
  looking the result up: E group operations per miss (batched inversion).
  Alternatively, store the pair table: E² advice.
- **(T2) Relations among DP walks.** Keep the missed DPs from online walks. Two
  online walks that reach the same DP give a relation.
- **(T3) High-bits DP predicate** (x < p/2^dp). Now the DP set is a small-x set
  `{x ≤ B}`, and decomposing d is the summation polynomial S₃(x₁, x₂, x_d) ≡ 0
  with small roots.

## 3. Why no version is non-generic, or better

1. **There is no linear algebra to do.** Every table DP has a known log and
   every online point is aG + bQ, so any relation with b ≠ 0 is one equation in
   the one unknown k and solves the DLP at once. T2's "relation" is an ordinary
   rho collision. The IC phase structure (B relations, then a sparse solve)
   never appears.
2. **Genericity.** In the generic-group model a hash-defined DP predicate is a
   random predicate on random labels, so T1 and T2 are generic algorithms with
   E words of advice. They obey S·T² ≳ n. Preprocessing rho reaches about 4n on
   that line `[pcc:unverified]` (program note §1). So the most any such variant
   can gain is a constant factor.
3. **T1 loses even the constant (derived).**
   - A walk of length W hits the table's trails, which cover about E·W points,
     with probability about W·EW/n per walk. That is about c/W = c√(2E/n) per
     operation.
   - Decomposing over the table costs E operations for about 2E²/n success
     probability. That is about 2E/n per operation.
   - So walking is about √(n/2E) times more efficient per operation. At n = 2³²
     and E = 2¹¹ that is about 2¹⁰ times.
   - The pair table at equal advice (E′ = E²) is the known-log line S·T ≈ n
     (`ic.known_log_mitm2_v1`; the note's IC-3 row). It is worse than
     S·T² ≈ n whenever E′ < n: E′ stored points cover E′ points, E′ DPs cover
     √(nE′).
4. **T3 is the closed small-x IC.**
   - A walk lands on a DP with probability B/p, so walks have length p^{1−δ}.
   - Lattice decomposition of S₃ reaches at most δ ≈ 1/6 asymptotically and
     about 0.16 at 40 bits `[pcc:unverified]` (program note §2.2; probe C).
   - Only a (E/B)² fraction of decompositions have both parts in the table.
   - §2.4 shows that m = 2 never beats √p, even with a free oracle
     `[pcc:unverified]`.

**Where M4 sits.** Plain M4 is on the S·T² = n line (≈ 4n). M4 + T1 sits
above it by about (1 + E/W)², because T ≈ T_BL·(1 + E/W) (derived); at 32 bits
with E = 2¹¹ that is about 3× the Bernstein–Lange online cost. M4 + T3 is above
√n. The repository's IC rows lie on S·T ≈ p, further from the frontier still.

## 4. What ecbench would measure (if anyone insists)

- **Already registered.**
  - `rho.preprocessing` (entries, dp_bits, table_seed, precompute_cap,
    cap_multiple), with phases `setup`, `precompute`, `search` and
    `internal_verification`. The cost scope is "cold, one target".
  - `ic.known_log_mitm2_v1` and `ic.known_log_ksum_v1` (known-log pair and
    k-sum tables).
  - `rho.negation`.
- **New, only if run.** `ic.dp_decompose_v0` would be `rho.preprocessing`
  plus T1 on every miss. Each d − d_i is charged as a group operation; table
  lookups and canonicalisations are counted as `*_uncharged`.
- **Metric.** The `search` phase GAE per target (the warm online cost, with the
  table amortised). Report the cold total beside it, and give
  precompute/online separately so that S·T² can be read off.

## 5. Frozen 32-bit spec outline (not to be run; recorded for completeness)

- **Curves.** `prime_search` at 32 bits, prime order, 3 curves, 16 targets
  each. Seeds are derived from the spec hash.
- **Advice.** E ∈ {2⁸, 1664 ≈ n^{1/3}, 2¹⁴}, with the same `table_seed` and
  jump table on both arms (equal advice = equal stored words).
- **Arms.**
  - `rho.preprocessing` (baseline, per E).
  - `ic.dp_decompose_v0` (candidate, per E).
  - `rho.negation` (matched rho reference).
  - `rho.preprocessing` repeated (A/A control).
  - Arms run interleaved, at `isolation_required: L2`, through isolab-eks per
    the repository rule.
- **§5a band.** Price unpriced native work (lookups, canonicalisations, hash
  probes) at 0, 0.1 and 1 GAE per operation on every arm, rho included. Rank
  two arms only where the order holds across the band.
- **Positive control.** At E = 2⁸ with `dp_bits` 4 below the default, the
  harness must detect the baseline's predicted slowdown.
- **Prediction (derived).** Candidate/baseline online GAE ≈ 1 + E/W, which is
  about 1.25×, 2.4× and 16× at the three E values.

## 6. Kill line and prior

- **Paper kill (applies now).** §3.1–3.3 make M4 + T1/T2 a relabelling of
  Bernstein–Lange, plus a step that is dominated per operation. §3.4 makes T3
  the closed small-x IC.
- **Empirical kill, if run.** The lower 95% CI of candidate/baseline online GAE
  is above 1 at all three E → close.
- **Prior.** P(the candidate's online cost at equal E is below
  Bernstein–Lange's, outside the A/A band) < 0.001; that would contradict the
  counting in §3.3, not only CGK. P(any M4 variant is non-generic without
  being T3) ≈ 0.
- **Success criterion.** Not an IC improvement: a win would be a rho
  constant, and relabelling rho as IC is an accounting change.

## Sources checked

- `RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` §0–§2.4
- `.claude/skills/ecbench-measure/SKILL.md`, `.claude/skills/ecbench-extend/SKILL.md`,
  `.claude/skills/ecbench-bounds/SKILL.md`
- `examples/prime_ecdlp_breakthrough_probe.rs`: `GWalk`, `precompute`, `online`
- `research/prime_fast_bench_20261008/probeA.md`: at 32 bits, 2¹¹ entries,
  online 5.372e3 steps vs plain rho 5.808e4 (11×) `[pcc:unverified]`
- `src/cryptanalysis/ecbench/methods.rs`: the `rho.preprocessing`,
  `ic.pipeline`, `ic.known_log_mitm2_v1`/`ksum_v1` and `rho.negation`
  declarations; `solve_preprocessing` (phases and cost scope)
- `src/cryptanalysis/ic_boundary.rs`: `PreprocessingRho`
- research-kb: **not available** (no MCP tool; `scripts/research-kb` reports
  "no executable found"). No `source_id` was recorded. The lead atlas is not
  present in this container.
