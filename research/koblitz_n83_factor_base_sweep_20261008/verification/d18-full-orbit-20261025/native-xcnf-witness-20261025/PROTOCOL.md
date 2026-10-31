# Full-D18 XCNF witness equivalence gate

Freeze and commit this protocol and its source before timing. Use the primary
`a=0` Koblitz GF(2^83) curve, the complete direct-subgroup D18 base with
32,643 signed-Frobenius columns and 5,418,738 point rows, and the exact
public `(a=0,0)` fixture. The constructed control is the phase-first,
free-choice shared-root `m=4` model with all selectors and Frobenius phases
admissible, no identity summands and no choice pins. It pins the four
constructed x coordinates and target x as the prior positive XCNF export did.
The frozen prior export is 329,726 variables, 2,807,752 original CNF clauses,
3,901 XOR rows and SHA-256
`0d7bb7ef45ae19937220945129f61baccdd72b85c58bb7f16dcb56ca3ec24d7c`.
If the new export differs in shape or digest, classify an identity failure;
do not use a native witness to excuse a changed formula.

From one clean source commit, build one release exporter. Freshly GET the
exact S3 object and wrapper under
`s3://crypto-autoresearcher/factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/d18-full-direct-subgroup-v1/a0/`.
Require compressed object SHA-256
`eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b`,
wrapper SHA-256
`1b93351febd47dac4de8c5dd39dbf58564d0057a044054baf448d978f6c0d616`,
provider full-object checksums and lengths, and producer-manifest binding.
Run `primary-full-d18-m4-l0-xcnf-witness BASE FIXTURE NEW_JSON` under the
300-second whole-worker cap and a 310-second independent process watchdog,
using `/usr/bin/time -l` on the shared Mac. Preserve the exact command,
source and binary hashes, stage phases, XCNF, packed model, worker JSON,
stdout/stderr, exit code, wall and memory observations.

The worker exports XCNF before searching, then requires native SAT within
the bound. It checks the model against the original CNF and XOR rows, the
original-field chain, the four-point group lift, the signed-Frobenius
coefficient row, rank one, and altered-target rejection. Save the full model
as a packed bit vector: bit `i` of byte `i/8` is variable `i+1`, low bit
first. An independent Python auditor must check every exported XCNF CNF
clause and XOR row against this saved model, verify the original-field
four-point sum and signed-orbit row, and reject a flipped assignment bit,
altered target and altered XCNF hash. Retain all raw custody and timing
receipts with SHA-256 inventory. A missing, incomplete, inconsistent or
capped native model is an unresolved or invalid gate, never SAT evidence.

This is a constructed-model equivalence test. It cannot estimate natural
relation yield or select a factor base. Even a passing control leaves the
natural-search, useful-rank, factor-base logs, unseen-target descent, full
cold one-target accounting and matched same-target rho gates outstanding.
Keep `selected_best_total_runtime` null.
