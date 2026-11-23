# Shared brief: EcGFp5 / EcMasFp5 lead evaluation (desk, 2026-10-31)

You are one of two parallel evaluators. DESK EVALUATION: decide with evidence; do not implement the
attack; do not edit any repository. Save your output to the path named in your task and return it.

## The curves (from the frozen primary sources below)
* Field F_{p^5} = F_p[z]/(z^5 - 3), p = 2^64 - 2^32 + 1 (Goldilocks). Note p ≡ 1 (mod 5).
* EcGFp5 (Pornin, ePrint 2022/274): y^2 = x(x^2 + 2x + 263z), double-odd, order 2n, n a 319-bit prime;
  one rational 2-torsion point (0,0); b = 263z is a non-square in F_{p^5}.
* EcMasFp5 (Polygon Hermez 2025 HackMD): y^2 = x^3 + 3x + 8z^4, 320-bit prime order, cofactor 1.
* Rho with negation: about sqrt(pi*n/4) ≈ 2^159.3 group operations on EcGFp5.

## Frozen sources available locally (read these; cite line numbers)
Root: /tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/atlas/ca/
* inputs/PORNIN-2022-274-ECGFP5/paper_fulltext.md  (security argument: lines ~110-152)
* inputs/JOUX-VITSE-2010-157/paper_fulltext.md     (Gaudry/Diem cost: ~322-370; JV variant: ~372-440;
  ordering-change cost: ~505-535; oracle-assisted SDHP: ~718-790)
* inputs/GORLA-MASSIERER-2014-1403/paper_fulltext.md (n = 5 remark on covers / trace-zero)
* inputs/ECMASFP5-HACKMD-2025/note_fulltext.md
* research/gfp5_deployed_curves_20260920.md        (the repository's study design and cost bands)
* experiments/EXP-GFPN-05ff43/analysis.md            (the C1 ladder's adversarial review: K1-K7)
* knowledge/open-problems/KN-OPEN-9b4a2b.md          (the open 2-torsion quotient question)
* knowledge/leads/ecdlp/ic-prime/LEAD-ICP-gfp5-*.yaml (the four atlas leads)
* ledger/hypotheses/H-GFPN-*.yaml, ledger/evidence/EV-GFPN-*.yaml, ledger/goals/GOAL-GFPN-380702/goal.yaml
* knowledge/literature/KN-LIT-bf1772.md (Iijima-Momose-Chao classification; abstract only)

## What is already established (do not redo; you may contest it with evidence)
* The repository's C1 ladder (EXP-GFPN-05ff43) measured NO five-summand (m = 5) ideal degree: both arms
  exhausted an 11 GiB address-space cap at p' = 4111 in the degree-33 F4 round. Inconclusive.
* Its review (K7) found the design note's degree-16, D ≈ 2^20 system is the S_5-symmetrized one; the raw
  system's mixed volume is 5!·16^5 ≈ 2^26.9; the frozen band 2^121-2^132 was mis-anchored by 120 and
  omitted the inverse success probability; re-anchored model figures 2^134.4-2^162.6 (model only).
* KN-OPEN-9b4a2b: a genuine F_p-rational (Z/2)^{m-1} ⋊ S_m quotient (rescaled base, b = lam^2·beta) is
  conjectured to have D = 2^{(m-1)^2} = 2^16 at m = 5; seen only in toy "square analogues".

## Hard constraints
* DISK: under 500 MB free. Do NOT run cargo or build the crate; do NOT clone or download anything large.
  A single-file standalone Rust program compiled with plain `rustc -O` into your own scratch subdirectory
  is allowed for an exact arithmetic check (u128 suffices for products mod p). Python only for analysing
  numbers or committed data. Write only your own small files.
* LITERATURE FIRST: check primary sources (the frozen texts above first; then WebSearch/WebFetch for
  anything else). Verify parameters and conclusions; record each source and how you used it. If a source
  is inaccessible, say so and state the search scope. Never invent a citation.
* Be explicit about UNITS (F_p multiplication, 64-bit word operation, operation mod the 319-bit group
  order, group operation) and about MEMORY. Separate measured / derived / assumed. Try to refute first.

## Output
For each question: a verdict (PROMOTE | RUN-AS-SPECIFIED | REVISE | CLOSE with named obstruction |
UNVERIFIABLE), the arithmetic with units, the decisive next experiment with a frozen kill criterion and
its compute/memory envelope, risks, confidence, sources. Finish with a 3-line summary.
