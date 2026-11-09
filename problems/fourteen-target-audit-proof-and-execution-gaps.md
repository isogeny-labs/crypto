# Fourteen-target audit: remaining formal bridges and execution prerequisites

- **Status:** open
- **Kind:** proof gaps, implementation gaps and access/tool limits; no impossibility claim
- **Affected work:** [fourteen-target audit](../research/theorem_audit_20261108/README.md)
- **Next action owner:** audit continuation; environment owner for credential/host provisioning
- **Last updated:** 2026-11-09

## Problem

The request for all fourteen source audits, experiments and faithful Lean proofs
is incomplete: local checkpoints are now authorized and bounded native
components have run, while independent-host execution/main integration and
several actual curve/Jacobian/implementation bridges remain unresolved.
[pcc:unverified]

## Evidence and uncertainty

- Observed: a complete resume document was supplied to `scripts/checkpoint.sh
  gate`; upload failed because neither the configured AWS profile nor an
  upload token exists. Local builds and fixed development tests continued.
  The same checkpoint dependency appears in
  [the existing launch-gates record](arity-completion-launch-gates.md).
  No paid cloud resources were requested or launched. [pcc:unverified]
- Observed: the selected runtime contains one execution environment. The
  Rust production controls and additional C++ fixture execute on it; this
  does not supply the requested other-machine validation. [pcc:unverified]
- Local mathematical components: `PROOF_MAP.md` describes the actual Lean
  statements. The S3 existence equivalence is general in its stated domain;
  the Frobenius, branch, pairing and subgroup lemmas expose their missing
  bridges. An arithmetic identity is not labeled a full Jacobian, pairing or
  algorithm-refinement theorem. [pcc:unverified]
- Missing formal bridges: E[r]'s Frobenius representation/characteristic
  polynomial; supersingularity and Hasse trace-zero result; actual Weil/Tate
  constructions and domains; Diem ramification and intersection geometry;
  reduced-divisor/Jacobian definitions and smoothness counting; msolve row
  arithmetic/provenance/degree refinement; probabilistic mixing/collision
  theorems on the appropriate walk ensemble. These are not infrastructure
  errors or assumed conclusions. [pcc:unverified]

## What would unblock it

- The checkpoint-upload prerequisite was removed by explicit user approval
  of local checkpoints for this audit; do not ask again. New local runs
  still need their own frozen protocols/evidence, and this exception does
  not authorize paid compute or settle a theorem. [pcc:unverified]
- Supply an independently controlled execution host, or a reviewed CI run
  of the appropriate independently implemented validator and original inputs;
  retain its real provenance and signed report. [pcc:unverified]
- Complete each missing formal bridge against the exact source hypotheses,
  build its actual module, audit axioms and map its conclusion to the corpus
  statement before promoting a result. [pcc:unverified]

## Next discriminating steps

1. The documented bounded Semaev census has run. Continue remaining
   resultant proofs beyond m4/degree proofs and the all-p<500torsion/binary/ternary census
   obligations under their exact original scope, with a new frozen protocol
   before affected native work. Never convert finite agreement into a
   universal theorem. [pcc:unverified]
2. For item7, provide the concrete Jacobian/factor-base/subgroup family and a
   faithful Lean group/divisor bridge; the abstract confinement theorem is
   already available and is insufficient by itself. [pcc:unverified]
3. For item12, bind recorded C matrix rows to monomial/prior-polynomial
   certificates and total degrees; separate ordinary GB, trace replay and
   saturation. A source comment naming F4 is not a refinement proof.
   [pcc:unverified]

## Resolution and history

Unresolved. The proof-and-control increment does not close the original
fourteen-item request, and opening or merging its PR would not do so.
[pcc:unverified]

### 2026-11-09 follow-up

The user explicitly approved local checkpoints for this audit. That removes
the upload prerequisite recorded above; it must not be asked for again.
The native Semaev census subsequently completed its documented bounded
domain with no discrepancy counters. The missing frozen JSON preregistration
is preserved as a deviation, not repaired retroactively. [pcc:unverified]

The [follow-up study](../research/theorem_census_20261109/PRIMARY_AUDIT.md)
now has an expanded74-declaration local Lean gate, actual BK excluded-case
geometric3-torsion, an F5 r=2 corpus counterexample, and a direct FSTp239
counterexample using actual F7(mu2) field degree1 and nonrational2-torsion.
The finite-field source trace definition is connected to actual point counts;
the corrected odd-prime theorem retains Hasse as an explicit premise. No
general p-rank equivalence or Hasse theorem is thereby supplied.
[pcc:unverified]

Primary BK/FST/Gaudry/Hildebrand sources were recovered/read at the recorded
scopes. All44unread-label tasks and239source records remain catalogued; the
full fourteen-item objective and independent second-host census stay open.
Main integration has an additional immutable-ledger fork, which was not
resolved by selecting a side or rewriting prior rows. [pcc:unverified]

### 2026-11-09 general-model extension

The S3component now has a universal actual-group proof for the general
Weierstrass model in every characteristic, not only short/ordinary binary
forms. Both model specializations and geometric false-zero controls over
F2/F3are checked. Actual geometric p-torsion families in characteristics2/3
are proved without assuming a trace/Hasse bridge. Expanded project gate167
passes,0failing/0trusted-native exceptions. General resultant recursion,
irreducibility/degree and the other original obligations remain open.
See [new proof scope](../research/theorem_census_20261109/GENERAL_S3.md).
[pcc:unverified]

The current independent Rust local validator agrees on4188162triples;
its distinct-host manual dispatch returned404and never executed. A new
attempt needs normal workflow registration/permissions and an explicit
protocol amendment. At the general-S3checkpoint, main's ledger has483rows versus local479,
common prefix457; no chain has been selected/rebuilt. Those access/integration
dependencies do not prevent the independent local proof work above.
[pcc:unverified]

### 2026-11-09 first resultant step

The actual fixed degree2/2Mathlib Sylvester-resultant equivalence for S4is
now proved in every characteristic on nonsingular general Weierstrass
models. A faithful geometric F5control disproves the finite-common-root
shortcut under simultaneous degree loss while preserving the valid
four-point zero-sum theorem. Whole census gate197passes,0failing and
0trusted-native exceptions. See
[scope and declarations](../research/theorem_census_20261109/GENERAL_S4.md).
General recursion beyond m4, irreducibility and individual-variable degree
proofs remain open. This increment adds no native population or host.
[pcc:unverified]

The16:26Zread-only ledger comparison is483main rows versus481local,
common prefix457; both histories remain intact. Parent3739is still
open/unmerged. Local mathematics continues independently of that merge
dependency. [pcc:unverified]
