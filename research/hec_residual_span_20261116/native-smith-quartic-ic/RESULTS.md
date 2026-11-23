# Original transported quartic IC solves qualify [pcc:unverified]

Freezef749102e1c5;INPUT hashes pass;sole bounded invocation exits0. All4actual quartic index-calculus solves return the original scalars and pass [k]D=T on the original H/F61:y²=x⁷+x³+5x+1. The original ell124459, source multiples1/17 and fixed fixtures are retained. Solver inputs contain public transported G,T, ell, relation base and seed; expected labels remain only in main.rs verification.

## Native correctness results

Both canonical models have66rational points. All3783projective lines are examined, including intersection multiplicities and infinity;209fully split lines per model are retained. Every accepted line is verified as the same full Pic class K-4P0 before entering the scalar equations. Together with L_P0=0 this gives210setup rows. Each solve needs2additional actual target-walk equations, with212rows total. Both elimination rounds are charged in the reported LA counts.

|Model|Original case/source multiple|Returned k|Walk steps|Smooth equations|LA multiply-mods|Core Pic multiplications|Core Pic inversions|
|---|---|---:|---:|---:|---:|---:|---:|
|0|0 /1|83209|7|2|25874|378905|3869|
|0|1 /17|44792|35|2|27001|469544|5157|
|1|0 /1|83209|16|2|26575|366904|4033|
|1|1 /17|44792|17|2|29138|372610|4159|

Every scalar is verified in both the quartic Jacobian and original hyperelliptic Jacobian. These are solver outputs, unlike the previous transport-only scalar checks. Source image nonzero, ell-annihilation of G/T and ell²>Hasse-bound checks pass. The two models arise from one source kernel and are not two independent escape routes.

## Costs and limits

Full line setup, rational-point class construction and line-class verification consume3186813Pic multiplications/32693inversions on model0 and2946246/31584on model1. Core Pic counters include public subgroup validation, walk and final quartic verification. LA counts are over ell and are not field61multiplications. Native partial counters are cumulative snapshots (including driver/setup/prior work), not independent phase deltas. They must not be added blindly to overlapping Pic counters.

No complete total ledger or wall comparison was performed. Model construction, public transport, constructor internals, native residues, lookup/host costs and coverage of each module still need explicit inclusion. Shared line setup between two cases is stated correctness reuse, not an amortized single-target speed claim. The all-lines scheduler is quadratic in p and does not implement the optimized quartic relation-generation complexity. This run demonstrates correctness, not a10xgain or a new mathematical attack.

The complete native correctness chain is now present for these fixed original inputs: rational kernel/trigonal map, canonical quartic, public divisor transport, actual quartic IC output and original-H verification. It follows the known Smith strategy; independent review remains open. The p211/p401 rationality obstruction, genus2/genus4 work and full10xgenus2+objective remain unresolved.

Next work: replace exhaustive all-line verification/setup with an exact on-demand relation scheduler under a new protocol, audit complete field/subgroup/host/setup/transport counters, and compare the entire same-target path with the strongest normalized original IC and matched rho. Do not infer a speedup from the short target walk or omit the setup burden.

Raw line/support relations and outputs are in CONTROL.txt; structured answers are in SOLVES.json. References/linear equations are in DERIVATION.md; known construction source Smith https://arxiv.org/abs/0806.2995 section7. Git custody/noS3polling/no cloud/no merge. Unused imported-module/helper warnings only; no scientific failure.
