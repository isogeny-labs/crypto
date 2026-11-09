# Fourteen-target theorem audit with Lean

This increment supplies the full short-Weierstrass S₃ zero-sum equivalence,
including repeated x coordinates, over algebraically closed fields of
characteristic other than two. The statement uses Mathlib's actual elliptic
point group. It also supplies structural proof components for several other
targets and a known-degree control that runs the production census functions.
The repository proof gate passed: 68 declarations audited (33 new and 35 reused), zero failing and zero trusted-native exceptions. Signed PCC standing is pending. [pcc:unverified]

**The fourteen-item request is not complete.** The table below retains every
original obligation. Local development tests are distinct from the requested
new censuses, solver measurements, Smart-attack run, and second-machine replay.
Those runs have not been performed in this increment. [pcc:unverified]

## Exact acceptance map

| Item | Work performed here | Lean coverage and actual remaining requirement |
| --- | --- | --- |
| 1. Semaev | Reused the existing distinct-x proof and completed tangent/vertical cases and the algebraically closed zero-sum equivalence. | The2026-11-09follow-up in ../theorem_census_20261109 proves full general-Weierstrass S3equivalence in every characteristic, checked specializations and actual F2/F3model-domain counterexamples, and completes the bounded native m4/m5census. It proves actual S4resultant equivalence, degree-drop control, individual degree4in each m4input and sharp specialization criterion. The general fixed-size resultant step now yields a defined canonical family with last-input degree2^(m−2)for every aritym≥3and an exact S4bridge; reset-of-size counterexample retained. All-mgeometric zero-sum equivalence, other input degrees beyondm4, symmetry and irreducibility remain open. Earlier finite Python evidence is historical. |
| 2. BK | Read the exact node and Galbraith Theorem 26.4.1. Both exclude r dividing q−1; the node also requires gcd(r,q)=1. The earlier F7 witnesses do not refute this theorem. | `triangular_iterate_identity_iff` and `unipotent_not_identity` establish the linear core. The 2026-11-09 follow-up supplies actual F7/r3excluded-boundary torsion and an F2/r2excluded-case control for the node's coprimality condition, while preserving the original r²point-count condition. Full qualified Frobenius/basis proof and extension-field census remain pending. |
| 3. Supersingular | Node already says p≥5. F2/F3 examples do not refute that clause; r=2 still needs qualification. | `trace_zero_prime_embedding_degree` proves exact multiplicative order 2 from r prime, r>2, r dividing q+1; `two_subgroup_embedding_degree` gives order 1 for odd q. The follow-up now covers every integer r>2 in the conditional trace/Hasse arithmetic and checks an actual point of order8 over F7. General Hasse/geometric bridges and binary/ternary extension censuses remain open. |
| 4. Pairings/MOV/FR | Read Galbraith 26.2 and 26.4. The node already distinguishes the correct domains. | `cyclic_restriction_trivial` proves a cyclic restriction obstruction. The 2026-11-09 follow-up adds an actual F2geometric torsion/μ2counterexample to pointwise nondegeneracy when coprimality is dropped. Qualified actual Weil/Tate calculations/nondegeneracy and group-scheme bridges remain open. |
| 5. Diem factor base | Read original Condition 2.7, its construction in §2.2 and Props.4.9–4.11. The original base-defined x projection has stable branch locus and fails the condition. Diem changes the projection, even when E is base-defined. | `stable_branch_locus_excludes_diem` proves the orbit obstruction; `short_branch_locus_stable` proves stability of the finite cubic branch roots. The ramification/scheme bridge and Hasse–Weil size bound are not formalized here. No counterexample to Diem's bound established. |
| 6. Gaudry smoothness | Read Galbraith Exercise15.6.4 and15.6.5: fixed g, q→∞, probability (1/g!)(1+o(1)); the relation-dependency estimate is separately conjectural. | Exact Jacobian counting, lower-order errors and faithful Jacobian proofs remain open. A discrepancy at one small q would not alone refute this asymptotic claim. |
| 7. Subgroup smoothness | Proved subgroup confinement for any additive group. | `sum_confined` and `outside_subgroup_not_smooth` apply to a Jacobian once a concrete subgroup/factor base is supplied. No actual hyperelliptic-Jacobian counterexample family or proportion theorem has been supplied; abstract group confinement alone is insufficient. |
| 8. Bézout/multiplicity | Read original Prop.2.5(a,b), in the projective product with multiplicity. Added the nonreduced control K[X]/(X²). | `double_root_quotient_dimension`, `double_root_support_size`, `double_root_multiplicity_mass` distinguish length2 from support1. General multihomogeneous intersection bound and comparison to exact msolve output remain open. |
| 9. Gröbner heuristics | Read exact FGHR Hypothesis3, generic-ideal-degree and JV Assumption2 nodes, preserving regularity/genericity and representation assumptions. | No general proof or counterexample and no new msolve run. Source review does not settle the heuristics. Actual homogenized systems, ideal dimensions, multiplicities and regularity comparisons remain pending. |
| 10. Walks | Read Galbraith §14.2.5: random partitions differ from implementable keyed partitions; Hildebrand's statement is about random step sets and mixing, not a collision constant. | `branch_collision_mass` proves only the exact one-step iid label collision mass Σpᵢ². The stated penalty, mixing asymptotic and exact finite walk census remain open; iid resampling is not a fixed-partition rho walk. |
| 11. Standards | Read NIST SP800-186 §3.1.2 and AppendixC.1. Criteria4–5 explicitly exclude anomalous curves and require large embedding degree. | No general half-bit security theorem is asserted. A large-order Smart-attack fixture would test the screening boundary, not refute recommended screened curves. Smart run/proof and matched SafeCurves rho benchmark are pending; SafeCurves HTTP endpoint returned502. |
| 12. msolve/F4 | Audited pinned v0.6.5 selection, monomial reducer construction, preprocessing, trace and simplify call sites against `F4Run`. See SOURCE_AUDIT.md. | Source audit is partial conformance evidence, not a Lean proof that C implements F4Run. Actual row degrees, arithmetic correctness, traces and saturation paths require refinement/certificates. No implementation theorem promoted. |
| 13. Validator | Inspected the existing Rust/Python crosscheck documentation. It already has another-language checking on five instances, on one host. Added C++ unit control and tests of the actual Rust routines. | All new checks run on the same selected machine. No independently controlled second host or independent execution attestation has been supplied. Lean arithmetic correctness does not establish host independence. |
| 14. Negative control | Reused the original toy lower bound, proved explicit degree3 rows contain a Gröbner basis, and ran the actual Rust `low_dim`/`low_dim_iter` on that same fixture. | `control_degree2_rejected`, `control_degree3_rows_suffice`, `control_all_rows_degree_le_three`; three production unit tests and an additional C++ control pass locally. This is a prospective regression control, not a claim that earlier measurements had it. Signed validation remains pending. |

All local results and source interpretations in this table are unsigned
producer assertions under the PCC policy. [pcc:unverified]

## Reproduction and proof map

- Toolchain: Lean4.35.0, commit `eb6cafb7290e8bd5f2937522d8d804f58f02bafc`.
- Mathlib: `56a005855ea17e0b0958f73ec801ae140b0a1679`; all transitives are
  pinned in [lean/lake-manifest.json](lean/lake-manifest.json).
- [PROOF_MAP.md](PROOF_MAP.md) names the declarations, hypotheses and gaps.
- `bash check.sh` builds Lean, prints every new declaration's axioms, runs
  three tests against the unchanged production Rust source and a C++ control.
- From the repository root, run
  `tools/lean/proof-gate.sh research/theorem_audit_20261108/lean` for the
  independent repository-wide transitive-axiom check.
- [evidence/](evidence/) retains build attempts, final audits and unit output.
  Failed incremental builds remain failures, never evidence for a theorem.
- [sources.json](sources.json) records exact URLs, revisions, hashes and reading
  scope. Existing `S3Roots.lean` and `SolvingDegreeBound.lean` are unchanged
  copies from this repository's pinned base, not new results. [pcc:unverified]

## Source corrections and execution limits

The previous standalone audit's F7 examples violate the actual BK hypothesis.
The old report now points to an explicit correction; its numerical data is
preserved. The repository already had a distinct-x S₃ root converse, and the
supersingular node already restricted the prime-field clause to p≥5.
[pcc:unverified]

KB unavailable: no MCP server; `scripts/research-kb status` exits127. The
fallback was narrow corpus/source inspection, not an assertion of novelty.
The connector search initially returned an older corpus revision; this study
pins the actual checkout at `ca496f7898f4467a28b9733bd8a43f52f9e8e723`.
[pcc:unverified]

No paid cloud jobs were launched. The repository checkpoint gate was attempted
with a complete resume document and failed: no configured AWS profile or
upload token. The user subsequently approved local checkpoints for this audit, and bounded
native components ran in the follow-up. Local proof work continues under
that exception and zero paid spend. The selected environment also
has no second independent execution host. See the
[open problem](../../problems/fourteen-target-audit-proof-and-execution-gaps.md)
for exact remaining proof bridges and resume conditions. [pcc:unverified]

This is a proof-and-source increment, not a new performance result. IC/rho
leaderboards, measured corpus statuses and canonical attack graphs are left
unchanged: no new matched measurement or signed scientific promotion is made.
The [coverage diagram](coverage.svg) distinguishes completed proof components
from the remaining requested work; [report.pdf](report.pdf) is its readable
snapshot. [pcc:unverified]

PCC records: `PCC-20261108-5a6a51` (full S3 equivalence) and
`PCC-20261108-63fffd` (degree-control lower and upper certificates), both
unsigned. See [pcc-records.json](pcc-records.json). [pcc:unverified]

Publication checks: the global benchmark-record gate was also run after
materializing its75existing records. It reports missing unrelated sparse/raw
inputs, an already-open base record and stale committed progress. This study
changes no benchmark/progress records, opens none, and does not regenerate or
close another workstream's evidence. See the bounded
[evidence excerpt](evidence/benchrec-baseline-gate-excerpt.txt).
[pcc:unverified]
