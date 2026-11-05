# Repaired-program binding to the preserved public decoder

2026-11-03. Checked API integration and recorded-oracle replay; no fresh source run.

## Exact interface found in the original recovery path

The certified-exclusion recovery runner obtains a program, derives its maximum
required offset, constructs a public point oracle with the complete prefix,
and passes that oracle to `compiler.evaluate`. Following the inherited evaluate
aliases leads to `target-aware-decoder/compiler.py:evaluate`. It requests each
listed offset once, multiplies term coefficients by the corresponding signed
characters, sums modulo r, and returns the scalar plus product/addition counts.

The new coefficient certificate had schema/r/terms, but not the old q/offsets/
degree metadata. The adapter supplies those fields from the verified support:
r479,q16,offsets0..15,degree3. The evaluator module is copied byte-for-byte;
its synthesis functions are not called. This preserves the actual decoder
arithmetic and query order rather than recreating an approximate interface.

## Binding, public boundary and overhead

`bind` accepts only the exact published program SHA256, runs the independent
full-field checker once, and retains immutable bytes. `legacy_view` rechecks
those bytes and creates a fresh legacy dictionary; a caller mutating a prior
view cannot alter the bound packet. `decode` wraps the offset oracle to require
exact Python integers in {-1,0,1}, rejecting bool, floats, None and residues
such as478. The original source oracle already emits signed values in this
alphabet; there is no scalar-label input to the decoder.

Binding is deliberately scoped to this one checked F479 program. It is not a
generic program-import API. Per call, the hash, JSON parse, support scan and
legacy-view allocation are extra native work. The original decoder's products
and additions remain its own field ledger; they must not be treated as the
complete adapter cost. Output records include the bytes hashed and prefix.
No runtime advantage or complete recovery cost is measured.

## Recorded-oracle contract checks

Four fixed public cases were projected from the saved candidate source receipts
for E/F509 with P=(3,68), order479: Q=O,P,2P,-P. The original complete479-record
file and each selected record are hashed. The local oracle object retains only
the public point, signed outputs and call log. Expected scalar labels stay in
the checker and are read after evaluation. These are API regression inputs,
not newly computed source values and not four new ECDLP recoveries.

The one bounded local test invocation passed three test methods with subcases:

- All four recorded cases decode to their expected values; queries are exactly
  offsets0..15, signs unchanged, and decoder counters agree with its loop.
- Preserved compile/Features entrypoints are replaced with rejection guards
  during the decoder check. Mutable returned views do not affect the packet.
  Altered source bytes fail before any oracle call, and four malformed signed
  values are rejected.
- The saved rectangular factor is loaded from its hash-bound artifact and
  source. One corrected-RHS solve exactly reproduces the published program;
  the independent square-set checker verifies all479 values again.

The latter closes the prior persisted-factor reload obligation. It is not an
independent full rank computation. No full recovery panel, performance or
sampling experiment was run, and the earlier immutable records are unchanged.
The local limits were one CPU,512MiB,30seconds,one attempt and2MiB output.
Sources were committed at77de09f before the invocation, which exited0.

## Complete fresh validation remains separate

`validation-plan.json` preserves all4,563 original targets across the three
curves (p353/r359,p509/r479,p701/r683) and baseline/candidate/reversed-A/A arms.
Only the candidate/AA r479 coefficient program changes. Baseline and the other
curves retain their original public programs and source semantics. Fresh source
setup, all shifted-sign/point checks and scalar verification are required.
The plan is correctness-only: it does not compare cold compiler costs while
using precompiled programs. A cost comparison needs separate complete accounting.

The plan is specified but NOT frozen or launched. It still needs its complete
source dependency manifest, digest-pinned runtime, executable runner/analyzer,
validated protocol.v1 and exact GitHub publication receipt. It proposes one
CPU,2GiB,900seconds,one attempt/no retries and a1USD ceiling; those are limits,
not an existing capacity or cost reservation.

Live Iso Lab overview was reconciled: no active job for this lane, and unrelated
terminal/suspended jobs were left alone. Its published pool values are compiled
defaults, not admission evidence. With the configured identity, NodePool read
authorization returned no, and reading ResourceQuota jobs in namespace isolab
returned Forbidden. The existing role was not changed or broadened. IAM role
name lookup did not supply an alternative authorized research identity, and no
alternative identity was used. No cloud job was submitted.

The tracked [.agents/skills/isolab-eks/SKILL.md](https://github.com/isogeny-labs/crypto/blob/7f2eded0f9f6ef0af9c5f6b6d1a3d480cfff1de1/.agents/skills/isolab-eks/SKILL.md)
requires: “Verify actual NodePool limits and the namespace quota; the MCP
overview's compiled-in numbers are not a live readback.” It also explicitly
keeps local unit tests as development work. The four contract cases and factor
reload test do not substitute for the full fresh-source cohort or bypass this
admission requirement.

## Sources, artifacts and next action

Sources inspected: certified-exclusion-recovery/run.py, its compiler alias
chain, target-aware-decoder/compiler.py, mixed-projective-strides-v2/run.py
(public Oracle and guards), prior corrected-span certificate, and the tracked
Iso Lab HARNESS/ACCESS/cost-control instructions. The original decoder's finite
field products are unchanged; this is interface integration, not a new formula.

Artifacts include immutable program bytes, preserved evaluator/checker, adapter,
recorded contract inputs and source hashes, exact test log/receipt, reload helper,
admission record, full validation plan, diagram and inspected PDF. Hydrate the
sibling feature-consistent-repair factor artifact before replaying reload tests.

Next finish the fresh-source runner/runtime manifest and obtain authoritative
capacity/cost readback within the existing role policy, then freeze and submit
the entire declared cohort. Do not relabel cached-oracle replay as fresh source
execution, drop fixtures, or move the full panel locally to bypass admission.
