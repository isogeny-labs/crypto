# Value-only subgroup check for projected source

subgroup.rs replaces unused phase-value calculation in the projected path with
curve/point checks, coprimality and psi_r(Q)=0. Caller-certified prime p/r remain
preconditions. DERIVATION.md explains why derivative nonvanishing follows on the
supported domain. The public paired source retains the full-value validator.

Five selected native methods pass, including acceptance/rejection agreement
against both the old phase routine and affine scalar multiplication. Claim
PCC-20261107-b5cddb is unsigned producer evidence. [pcc:unverified]

Exact commands/compiler/binary/dependency hashes and raw logs are in receipt.json,
source-lock.json and the five logs. run-record.py gives the replay commands; use
fresh outputs. generate.py regenerates the tested source byte-for-byte, with
subgroup-tests.rs as a local input; generation-check.json binds that check.
Report PDF rendered and inspected. No native failure, recovery or cloud job.

Projected phase work is removed, but value-only torsion checking, residuals and
the full-value reference remain. Next reuse the torsion-check value cache during
direct factor evaluation. Original cohort unchanged; no complete-cost claim.
