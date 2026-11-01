# Native binding of the frozen quartic fixtures

Status: source implementation, not an executed catalog or registration.
No output slug is cited before registration. Preserve every original packet
and workload ID. This does not replace either live scientific panel.

## Population and mathematical checks

Consume the exact original collision catalog SHA-256
`17ecb4c302526bc58e592466071e1c2704eaaf43a0a0ca1efe563e1bd69b3c10`.
Bind all seven characteristics and all three public presentations, selecting
each original public input once while preserving its exact hash. Require
the complete fixed 21-pair set. No favorable model or presentation is selected.

Use native arithmetic to check prime characteristic, the degree-four
Rabin irreducibility criterion for the actual monic modulus, nonsingularity,
prime subgroup order, a nonzero on-curve generator, and its annihilation
by that subgroup order. The reused native fixture checks also reconstruct
the original Frobenius/cubic scalar actions, factor-base coordinates and
endomorphism/doubling images rather than trusting a friendly name.

The full group order is derived separately: since the field size is p^4,
the exact Hasse interval is [p^4+1-2p^2, p^4+1+2p^2]. Require exactly one
multiple of the independently verified generator order in that interval.
Lagrange then fixes the full group order. Refuse an empty or ambiguous
interval; do not assume the subgroup order is the full curve order.

Pass that order and the exact coefficient/modulus arrays to the existing
`crypto_lib::cryptanalysis::curve_id::extension` SDK. Do not replace its
identity computation with a prose label or a Python curve calculation.
Bind exact representation records using the shared EC1 metadata convention,
the established extension encoding and the registry's `fpk` tag. Generator,
subgroup, cofactor and field basis remain in the full UID preimage.

## Controls, execution and publication

Before producing the real 21-fixture catalog, run native controls for
irreducibility/reducibility, unique/ambiguous Hasse multiples, sorted
float-free canonical JSON and changed-generator/changed-field identities.
Record format/build/test receipts and source hashes, then commit the
source/input freeze before production. Use one worker, the shared reservation,
8 GiB address-space and a 1,200-second aggregate production limit. Report
failures; no automatic retry or budget increase. A second process must
reproduce the complete catalog bytes before import.

Import the native model and representation records through a metadata-only
registry loader, preserving all existing rows and every frozen evidence
file. Verify registry membership, exact SDK model hashes, EC1 aliases/full
UIDs and the source-file seal. Update current prose only after those checks.
Any root alias-table change gets the required root tests and actual new-head
CI. Preserve the pre-import source-reuse evidence as historical rather than
claiming the changed alias source is byte-identical.

The binding supplies representation identity and the stated native field/
order/group checks. It is not a new DLP solve, an endomorphism-ring proof,
a formal verification of the entire field/group implementation, a new
relation collector, or a complete cost/speedup measurement.
