# Frozen original-model operator gate [pcc:unverified]

Question: can cumulative signed walk rows and their transpose be evaluated exactly without materializing dense cumulative step coefficients? Same mathematical matrix required, not a proposed row-selection heuristic.

Use all11 original (g,p,c,ell) fixtures listed in main.rs. For each, construct D from the first rational factor-base point and T=17D. This gate does not assert D is the original fixture subgroup generator, or use ell to justify log existence: formal row operators and exact divisor identities do not require that premise. This is an original-model component trace, not the existing original public-target workload or a matched complete solve. Target label17 constructs the public input only; operator APIs have no scalar-label argument.

Generate2048steps with Rng seed2026112700 xor p xor(g<<32), roll in0..4: D,T,or uniform first min(64,m) FB place. Start D+T. At every smooth event preserve signed multiplicities and independently reconstruct the divisor with native additions; require exact equality. No coverage postselection or rank claim. Require at least1smooth event per model.

Build an explicit canonical matrix including target and constant coordinates, coalescing cancellations. Test8seeded arbitrary vectors in each direction against implicit forward/transpose; also require dot(Ax,y)=dot(x,A^Ty). Charge literal subgroup modular products/additions separately and assert counts. Report every model's trials,rows,nnz,sparse terms and both costs. No wall-time or complete-solver comparison. Existing dense-block/forest baselines remain stronger controls.

Freeze/push source, protocol, theory, imports and executable SHA256s before one local process limited600CPUseconds. Git custody during S3 outage; preserve raw stdout/stderr,exit,execution commit and input/output hash receipts. No unchanged rerun after failure. No S3/cloud/merge. Full10xgoal remains active; a full solver needs its own complete cost admission and correctness protocol.
