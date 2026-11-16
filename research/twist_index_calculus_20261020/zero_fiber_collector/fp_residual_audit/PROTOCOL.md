# Finite prime-field residual audit — fixed 42-arm census

2026-10-25. Supplement the three unchanged live panels; repeat no collector,
F4 solve, target recovery or timing. Before consuming production records,
commit this protocol, native source and passing synthetic test/build gate.

## Workload and question

Use exactly the 42 immutable case-arms in the original collision workload,
SHA-256 `17ecb4c302526bc58e592466071e1c2704eaaf43a0a0ca1efe563e1bd69b3c10`:
seven primes 37,73,97,157,229,241,337, seeds 0,1,2, phi and doubling. Check
every catalog-bound public input, complete phase-1 record and passing audit.
No subset selection, reordering by outcome or replacement of missing cases.

For each saved pair, independently reconstruct the finite-field model and
point pool from public coordinates. Recompute the residual as the negative
signed pair sum. Check equality with the saved residual. Classify only a
finite point with x coordinates 1..3 zero as eligible for rejection. Retain
infinity, even when its coordinate storage is zero. A class mismatch or lost
accepted relation falsifies the proposed rejection for this workload.

The existing square-ideal result excludes three distinct ordinary roots
when a finite S4 residual has x in Fp. It assumes degree four for B over Fp
and odd characteristic. Check the concrete basis 1,B,B²,B³ independently.
The prior report and ten-theorem Lean gate remain the source of this
conditional theorem; no new full field/group formalization is asserted.
For every saved field root in the Fp class, construct the four elementary
symmetric coordinates by adjoining residual x. Check all four published
component polynomials C0..C3, the defect and the cubic's number of distinct
Fp roots. Retain all violations and witnesses. This is a new native finite
instance audit, not a replacement proof or an exhaustive new root solver.

Independently generate the 24 scalar actions from the public cubic and
transported-Frobenius eigenvalues. Reconstruct original orbit columns,
signs and image coefficients using scalar multiplication. Check every
physical relation as an actual five-point zero sum. Transport retained
representatives, compare exact physical sets, normalize augmented rows and
compute separate RREF ranks and, when full rank, point-checked factor logs.
Preserve all zero rows. Four ordinary IDs must be distinct and the image
must be disjoint, as required by the original protocol. This audit says
nothing about additional multiset relations.

## Outputs and interpretation

Retain every per-system classification, root/component witness, accepted
representative, per-case before/after physical set/hash, projective row
count, zero-row count and independent rank. Report totals separately for
phi and doubling and per case, including losses or falsification.
Sum recorded F4 row multiplications in each class; label these partial
historical counts. Do not turn a skipped-system fraction, saved row count,
or audit duration into a complete-cost or twist-specific speed claim.
There is no deployed filter, new relation yield or new DLP recovery in this
round. Full cost, matched strongest IC/rho and the original panels remain
open. A public-only collector fork with charged predicate overhead and
complete independent replay would be a separate experiment.

Use Rust only, one science worker under the common busy reservation, 8 GiB
address space and one 1,200-second outer limit. Compilation has one build
worker and occurs separately. Run two independent process executions and
require byte-identical scientific summaries, omitting no failures. Preserve
commands, source/binary/input hashes, phases, exits and resource receipts.
No timing is admitted on this shared migrated host. Produce a source-linked
report, diagram/graph and inspected PDF; no leaderboard promotion.

Controls cover the finite/infinite predicate, the actual quartic field
relation and group cancellation, signed orbit orientation, cubic repeated
versus distinct roots, independent rank/projective normalization and an
accepted-relation rejection witness. Missing/hashing/schema failures refuse
the audit; counterexamples produce retained failure output and nonzero exit.
