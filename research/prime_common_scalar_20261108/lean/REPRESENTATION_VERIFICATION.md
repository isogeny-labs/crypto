# Representation continuation: scope and receipts

BufferBudget mathematical source8e62d4dfc182ee3bd41765dda4847100ddf516f6.
StreamPrecision mathematical source15ab03e54a929ce076fd6b58edc6bb646a110cb8.
Lean4.35.0, CryptoLean148f5dc389b341401e06d76abe7783c4ccf42f2e,
Mathlib cfc55fc58714a59dc06e6e79edc6d1d95140d3c2 (unchanged pins).
All standing unsigned producer evidence [pcc:unverified].

## Actual declarations

BufferBudget derives the exact actual-node slot-width sum and its odd-level
and real-port lower bounds. This concerns simultaneous fixed-width prescribed
buffer capacity, including zero-valued slots; it is not a bound against every
representation or algorithm. Source a=O(log s) gives polynomial4^a in s
[pcc:unverified].

StreamPrecision derives actual residue divisibility/cross-precision cancellation,
actual global coefficient scaling, the constructed integral scaled functional's
rational value and its actual computed scaled target residue. K=8h*2^a+8M(a+1).
Native canonical unscaling, construction/retained-state/recomputation costs and
coordinate extension remain separate. Noncomputable definitions are mathematical
specifications; a finite sum admitting a fold is not a native memory benchmark
[pcc:unverified].

## Audits

buffer-budget-gate/: actual source audit279 declarations,0 additional trusted.
stream-precision-gate/: final actual source audit289 declarations,0 additional
trusted. Standard propext/Classical.choice/Quot.sound only. The latter default
build includes both new libraries. Each leaf's import-inclusive fresh command
has unique log/status paths. Both BufferBudget and StreamPrecision status artifacts record exit0.
Each fresh replay includes its imports; the project-wide289 declaration
source audit covers both leaves
[pcc:unverified].

Canonical bindings: buffer-budget-claim-pins.json and
stream-precision-claim-pins.json (all9 records unsigned). Failed builds are
retained; failed unsolved-goal logs are not accepted axiom evidence.

## Reproducibility

    lake -d research/prime_common_scalar_20261108/lean build
    bash tools/lean/proof-gate.sh --out UNIQUE_OUTPUT research/prime_common_scalar_20261108/lean
    lake -d research/prime_common_scalar_20261108/lean env leanchecker --fresh --verbose BufferBudget
    lake -d research/prime_common_scalar_20261108/lean env leanchecker --fresh --verbose StreamPrecision

Use Lean4.35.0 on PATH. Do not overwrite historical evidence. Current native
census is separately frozen under ../replay and has never executed; actual S3
gate failed for absent transport and substitution is pending. These proof
checks generate no native census/timing data.
