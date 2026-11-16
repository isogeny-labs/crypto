# Exact weighted monomial counts

The new Lean module proves the exact finite inequalities [pcc:unverified]

\[
M(w,D)\prod_i w_i\le {D+\sum_iw_i\choose v},\qquad
M(w,D)\prod_iw_i\,v!\le(D+\sum_iw_i)^v.
\]

For weights `1,...,v`, this gives `M(v,D)*(v!)^2 <= (D+v^2)^v`,
including `v=0` and `D=0`. The count includes every exponent vector under
the weighted budget; it uses no supplied support list. Claims
PCC-20261114-cc1d70 / 621b19 / 9932d5 are unsigned [pcc:unverified].

The proof attaches independent mixed-radix digits and injects the result
into an ordinary simplex. A slack label embeds the simplex in multisets
counted by Mathlib stars-and-bars. This is an elementary counting argument;
no novelty claim is made for it [pcc:unverified].

[Source](../lean/NormWeightedCount.lean),
[prospective contract](../WEIGHTED_COUNT_PROTOCOL.md),
[declaration hashes](../weighted-count-evidence/claim-pins.json),
[editable diagram](count.dot), [SVG](count.svg),
[PDF report](report.pdf), [editable report](report.tex).

Source freeze: `740732e95389e5185ed9a6547c18d8d889a6c580`.
Ten authored theorems plus generated `simplexCode.congr_simp`.
Draft001/002 source and failures are retained. Build003 exits0 without
warnings. Whole-study gate reports394 theorems/0 extra trusted axioms;
import-inclusive fresh NormWeightedCount replay exits0.
[Hashed validation receipt](../weighted-count-evidence/validation-001.json),
PCC-20261114-be41e4 [pcc:unverified]. No signed PCC standing is claimed.

The final two-page report was inspected on both pages; report003 has no
Overfull/Underfull messages. Earlier layout failures remain in raw logs.
No timing measurement or solver experiment was run [pcc:unverified].

The actual growing-family input grading and small full-closure cutoff,
public base coverage, signed output, extraction and complete expected bit
cost remain application obligations. No asymptotic specialization is
claimed in this finite component. The original rank-directed W-jet map is
separate; this is no demonstrated jet completion [pcc:unverified].

Sources checked(KB): unavailable (no loaded server tools, wrapper executable
absent). Primary fallback: pinned Mathlib `Data/Sym/Card.lean`,
`Data/Nat/Choose/Bounds.lean`, `Data/Nat/Factorial/BigOperators.lean`;
crypto-autoresearcher14f7a739d P1514 producer-v2/audit-v2 and P1536 full
norm-jet audit. The dense P1514 cutoff is sufficient, not a universal
minimum; P1536 supplies an exact simple-fiber source formula but no cheap
jet evaluator. Their historical formula audits are not measurements of
weighted closure [pcc:unverified].

Canonical leaderboard, benchmark and progress graphs were checked for
scope and left unchanged: this component adds no measured performance row,
new curve fact or ecdb-imported field [pcc:unverified].
