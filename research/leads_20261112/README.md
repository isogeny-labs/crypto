# Index-calculus techniques by field regime: open cells, new proposals, next steps (2026-11-12)

Owner's request: look at the atlas families for prime fields, Koblitz curves and the rest, think of new
index-calculus techniques, and for the promising ones give next steps. Everything here is a proposal or a desk
computation: `[pcc:unverified]` throughout; nothing was run on a curve; no row, leaderboard or atlas verdict changes.

## 0. Inputs

- The lead atlas brief (image, "Factor-base families tried, by field regime", verdicts as recorded, 2026-11-12).
  The atlas bodies in crypto-autoresearcher are vault-encrypted; only the 176 lead ids were readable
  (`atlas_lead_ids.txt`). Novelty below means: not among those ids, not among the 173 ids of the 2026-11-10 sweep,
  and not among the 13 leads of PR #3938 (`research/leads_20261111/`).
- Today's fall-degree results for the flagship prime family 0.1 (two-summand μ_{2^k} chains), from
  isogeny-labs/ml-cryptanalysis PR #101: LFD ≤ k + 2 (kernel), LFD = ⌊(k+14)/4⌋ measured to k = 15 (kernel relative
  to four GPU closures), LFD unbounded with slope ≥ 1/14 (written proof, reviewed 2026-11-12). The binary deep chain
  (interim, 16 of 57 systems): certified LFD 4 at k = 11 on random subspaces where law B predicted 5.
- `docs/ecdlp-hardness-corpus/ATTACK_GRAPH.md` (frontier), `WEAKNESSES.md` §6 (live obstructions),
  `docs/ic/LEADERBOARD.md` (every IC row above matched rho except the GHS-weak family).
- `research-kb` was not reachable from this session (no MCP server, no CLI executable); the knowledge-base pass is
  owed before any of these becomes a task card.

## 1. The bar: Gröbner routes on multiplicative-subgroup bases against brute-force decomposition

With m summands in a coset of μ_N, N = 2^k, the chain presentation has m·k variables. Brute force decomposes one
target in 2^((m−1)k) summand tuples. The Gröbner/XL route must at least write the Macaulay columns of weight
≤ LFD = s·m·k, about 2^(m k H(s)) of them (H the binary entropy). Thresholds (computed this session):

| m | columns < enumeration needs LFD/k < | sparse linear algebra (ω = 2) needs LFD/k < | ω = 2.81 |
|--:|--:|--:|--:|
| 2 | 0.220 | 0.083 | 0.054 |
| 3 | 0.522 | 0.184 | 0.117 |
| 4 | 0.858 | 0.290 | 0.182 |

Measured m = 2 slope: 0.25. The matrix alone already exceeds brute force (2^(1.087 k) columns against 2^k), and the
proved floor 1/14 = 0.071 is above the ω = 2.81 line, so at m = 2 the Gröbner route on family 0.1 cannot beat
enumeration even at the proved floor unless the linear algebra is sub-quadratic in the columns. The decisive unknown
is LFD(k) at m = 3: below 0.184·k the route is alive, above it family 0.1 closes for m ≤ 3. No record of a certified
m = 3 prime-chain LFD exists in the files read here. Caveats: equal yield per attempt in both arms; the column count
is a lower bound on F4 work; enumeration priced at one unit per tuple (a quadratic solve and an N-th power test).

## 2. Proposals

(filled from the lane reports and the main-session notes; see §3 for the ranking)

## 3. Ranking and next steps

## 4. Proposed atlas verdict changes (proposals only)

- 0.1 Kummer g·μ_{2^t}: "open" → "open only for m ≥ 3 with LFD/k < 0.184, or slope < 0.083 at m = 2" (§1).
- C5 trace-zero: today's k = 11 binary measurement (LFD 5, every law) is consistent with "engineering".
- A4 subgroup/coset: if its refutation is cited through the fall degree, the coset case of the k + 2 bound is written
  only, not kernel-checked.

## 5. Considered and not proposed

## Sources checked

