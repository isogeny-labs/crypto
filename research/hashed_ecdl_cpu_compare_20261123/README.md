# Matched CPU numerator-scratch comparison

The two controlled source variants build independently and each passes 47 native
library tests, with zero failed/ignored. Two other harnesses are empty. Every one
of the 108 source/input files is shared except CPU `lib.rs`: the staged-numerator
parent and current reverse-pass numerator candidate. This is software qualification,
not a new measured comparison. Unsigned record: `PCC-20261123-9c9ba9`.
[pcc:unverified]

Source reconciliation corrected an earlier status mistake. Round24's actual
180-call early-residual-filter panel exists and led to its revert. Current code
already matches round25's numerator-scratch candidate; its published round25
package has no result tree at the inspected snapshot. See the preserved sibling
[erratum](../hashed_ecdl_cpu_scale_20261123/SOURCE-RECONCILIATION-ERRATA.md).
The old 24-call public-target cohort and all historical records stay unchanged.
[pcc:unverified]

This new local design compares those two library variants on the same six fixed
curves, two public targets per curve and unchanged m8/batch64 scale workers.
Logical arms a/c use the same copied parent executable for A/A; b uses the
candidate. Every worker invocation still uses native label u and its own output
directory. Raw replicate 0/warmup false remain untouched; canonical instance IDs
include both curve and seed, while round identifies the actual replicate.

The planned census is 3 arms × 12 public targets × (1 warmup + 9 recorded
repetitions) = 360 calls. This is one batching/slot setting, with two distinct
targets per curve. Fresh processes and fixed rotation/reversal interleave the
arms. All full certificates and structural counters must agree. Every call
has Native replay and the worker's scalar-corruption check; separately frozen
OpenSSL point/binding positives and two negative controls are planned per cell.
Native analysis writes incomplete flow before rejecting a missing/failed panel.

Build, copied-qualifier, preflight, protocol and post-call records have separate
names. A warmup-only preflight will estimate runtime and dry-run analysis before
the prospective protocol is frozen/published. No main-panel observation exists
yet. Original round25 cloud protocol and its attempt identity are unchanged.

Counted correctness/certificate/counter equality is primary. Complete internal
and enclosing driver clocks are secondary L0 diagnostics, reported as medians,
full ranges and parent/candidate ratios beside A/A spread. Total-operation S,
per-process peak RSS and globally ranked speed are unmeasured. The removed
numerator stream does not itself establish fewer total machine loads or lower
runtime; reverse-pass loads/branches are charged by complete execution.

Execution stays local with S3 excluded, one worker, separate build/test phases,
4 GiB worker virtual-address limit and the existing 15,032,385,536-byte sampled
whole-cgroup guard. All original failures, protocols, full 84-case tensor panel
and source/history/total-cost proof obligations remain separate and unchanged.

Sources checked: round24 actual EXECUTION/rows/summary; residual-filter-revert,
residual-scratch and batch-endpoints notes; round25 protocol/source delta/runner/
analyzer; current CPU lib and exact historical commits 4939261c/c4788625. KB
tools are unavailable in this runtime; direct source review supplies no novelty
clearance. The new driver/analyzer and prospective protocol require their own
qualification before any panel data. [pcc:unverified]
