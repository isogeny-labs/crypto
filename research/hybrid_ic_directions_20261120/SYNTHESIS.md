# Hybrid IC directions — synthesis (design-only, [pcc:unverified] throughout)

Date 2026-11-20. Four directions, each a Fable design fork + an independent Sonnet
codebase-grounding agent. Nothing was run or measured. No claim here is charged;
all `[pcc:unverified]`. research-kb / lead-atlas NOT queried — must be, with
source_id/repo:path receipts, before any novelty or "not tried" claim is written
into a repo note or PR.

## Meta-finding (the signal across all four)

Relation COLLECTION is 84–89% of an `ic logs` run; linear algebra is 0.11–1.9%,
and row_ops are unpriced in the ranked S metric. Every lever the user named either
attacks the wrong stage (D3 selection, D4 amortization), has no mechanism to beat
the EQUAL-ENVELOPE control that already reduced the large-prime graph to 2.1×
(D1 frontier-sharing), or produces disjoint unusable work (D2 twist). The only
measured wins that exist are constant-factor BASE-STRUCTURE wins in the oracle
(norm=N compact-orbit 9.2× over plain-scan at n=41/m=3; double-LP 2.1× over equal
envelope). A new idea should target cost-PER-RELATION in the decomposition oracle
structurally, not re-order the search or post-process the matrix.

## Per-direction verdict

### D1 — Frobenius-transport relation graph + A* + cross-target frontier sharing
LIVE but narrowed to one sub-claim. Design-fork self-refuted the headline:
cross-target forward-sharing fraction f_share ≈ b·E·n/r ≈ 7.6e-4 at n=41
(≈0.18 only at toy n=31) — sharing dies exponentially in n, preregistered as a
null. Grounding: no A*/beam/best-first search over partial decomposition states
exists in src/ (genuinely new); BUT (a) residual.rs already implements
"nodes=residuals, edges=subtract base pt, MITM meet" as a random collision walk
costing ≈1 rho solve/relation; (b) the only available A* heuristic (ONB x-weight,
Frobenius distance) is the exact Python selector feature set already shown to carry
NO routing signal (SOLVER_SELECTION, no headroom on 288 queries); (c) no n=131
decomposition path exists (u64 FastPoint < n=64, MAX_MODULUS_BITS=63) — testable
only at n≤41. Most likely outcome: collapses to batched MITM + compact-orbit
(already on main). Ceiling: constant factor, one rung, no exponent.
Cheapest discriminating test: masked 2×2 {sharing}×{heuristic}; run the n=31
sharing cell FIRST to kill the sharing sub-claim for ~nothing, then only spend on
n=41 if the A* ordering arm cuts forward expansions ≥2× at matched yield, gated on
beating the equal-envelope control at all three §5a price points.

### D2 — shared-x twist twin collector (binary Koblitz a=0/a=1)
NEAR-NULL, and grounding found a structural reason the mechanism cannot pay here.
binary_semaev.rs: S3 is a-INDEPENDENT; the whole twist difference is one
Artin-Schreier bit Tr(s1+a)=0 plus a disjoint factor-base partition (for odd n
every x lies on exactly one twin). So there is ONE system, not two — refutes the
"different Semaev systems" risk — but that sameness removes the leverage. A shared
x-solve yields roots split into DISJOINT subsets: each root triple lifts on only
one curve. So it is not 2× relations/solve for a single target; it is two SEPARATE
DLPs sharing solve cost. Design-fork predicted ≈3/4 "mixed" (cross-curve)
solutions wasted → net Y_pair ≈ 1.0–1.3×, maybe <1, and NOTHING for a single
ECC2K-130 target. Grounding adds two killers: (1) the two subgroups are wildly
unbalanced — at n=83, a=0 is 82-bit but a=1 is only a 53-bit prime (rho ≈ 2^26,
trivial) — so the twin DLP is of no interest; shared cost only pays if both DLPs
are of interest and comparably hard, which the Koblitz twist pair never is;
(2) binary field-op counting is NOT wired into ecbench (BinaryGroup does not
override counts_field_ops; F2mCounts exists in binary_velu.rs but is unwired), so
measuring "relations per field op" needs new instrumentation first. All measured
twist studies are prime-field; the prime-field negative result ("a twist factor
cannot add an independent column for an E target") stands and the binary analogue
is untested. Verdict: a clean near-null; only worth it to convert the twist lever
from "maybe" to "measured", and only after binary field-op instrumentation lands.

### D3 — MaxSAT on relation selection
NO-GO. Genuinely novel vs every closed SAT-as-oracle result (disjoint problem),
but attacks a stage that is 0.11–1.9% of the run; row_ops are unpriced in S so a
selection win cannot even register in the ranked unit; greedy CADO purge is
near-optimal on the one instance measured (538 vs 582 row_ops); rank is over Z/r
while SAT is GF(2) (no clean encoding; a post-hoc Z/r rank check is still needed);
NP-hard min-weight exact-rank cover ⇒ CMS overhead likely exceeds the Gaussian
elimination it replaces; no cardinality/MaxSAT tooling or even pysat/CMS on host.
"MaxSAT for filtering" appears once in a session log as an unrun idea. End-to-end
ceiling ≤1.9% (cost of making LA free), inside collection-stage A/A noise.

### D4 — auxiliary data
DROP (ranked lowest). (A) Cheon/static-DH auxiliary-input is a DIFFERENT oracle
domain: FRONTIER.md:7 + README:114 forbid an oracle-fed method from displacing a
non-oracle one, surfaced as its own oracle_queries axis; in-scope only if the
deployment exposes a static-DH oracle, as a separate result on a separate domain.
(B) "auxiliary data" = cross-target precomputation is what IC already is (ic logs
log store, stamped cost_basis amortized vs cold); FRONTIER has NO multi-target row
(different domain by rule); the existing mt-fieldops-v5 row is UNRANKED across the
§5a band (IC dearer at h=0, tied at 0.1, cheaper at 1), its lone h=1 win rests on a
fragile "2 native ops per DP point" convention, and wall-time reruns put IC behind
at 2^32–2^36 with the gap GROWING in group size. Crossover discriminant
κ²r/A > 4·c_solve·C_pre must hold across h∈{0,0.1,1}; structurally both IC and
batch rho scale ~√(n/T) per target so the ratio is ~flat in T — no guaranteed
crossover. SABOTAGE TRAP (already fenced by AGENTS.md:2196): comparing amortized
IC against NON-batched rho manufactures a spurious √T win; the matched reference
must be batch rho on the same T targets. Only salvageable act: read C_pre,
c_solve, κ from existing records and check the discriminant (a day; likely "no
band").

## Ranked recommendation
1. D1 — only live question. Run the cheapest form (n=31 sharing-null cell first,
   then the A* node-expansion count), expecting a null that is itself worth
   recording. Gate on the equal-envelope control across the §5a band.
2. D2 — cheap near-null, but needs binary field-op instrumentation first and the
   mechanism has no payoff scenario in the Koblitz twist pair. Low priority.
3. D3 — no-go. Only as a bounded "quantify the greedy gap" study at LA-dominant
   scale, reusing an existing pool, no new collection.
4. D4 — drop; at most the one-day discriminant check.

## Process gates before ANY run (crypto + cryptanalysis CLAUDE.md / AGENTS.md)
Conductor task + scope reservation; research-kb + lead-atlas receipts; a frozen
experiment.protocol/v1 with positive control + A/A + masked arms; .resume/RESUME.md
checkpoint pushed to S3; isolab-eks execution (L2 admission); bench.record/v1;
VALIDATION.json (replay cert + validator report + matched rho with §5a band);
pcc claim id. Default route is EKS, not local.
