# Lean declaration hashing can stop inside legal signatures

- **Status:** open
- **Kind:** observed implementation gap in statement binding; separate formal-model proof and native observations
- **Affected work:** [statement audit](../research/statement_binding_audit_20261114/README.md), [BK actual case](../research/bk_actual_case_audit_20261110/README.md), original audit items12/13
- **Next action owner:** audit continuation for versioned BK receipt; canonical shared-verifier maintainer for parser contract
- **Last updated:** 2026-11-14

## Problem

The frozen PCC declaration extractor can hash a proper prefix of a legal Lean
signature, so a passing proof-gate receipt does not by itself establish that
the declaration hash binds the complete claimed type. [pcc:unverified]

The concrete issued case is **PCC-20261110-99983d**, theorem
`BKActualCertificateAudit.bk_actual_case_audit_certificate`: its recorded
176-character statement ends at `geomCurve (L`, before a legal named argument.
Its hash is `f56bb53db3c02eb97e77fb6c7aa358d08dc0cd87342e57e46485b74c5d0e7349`.
The independently parsed full written signature has913characters and hash
`1f432993e5bd23e740e77d40a684350537a85e181a85b9375bb885634f08dedc`.
The original actual-object Lean proof and whole37-declaration gate remain
separate evidence; this observation does not refute that theorem.
[pcc:unverified]

## Evidence and uncertainty

- **Observed:** native Lean official-parser audit02 at immutable source
  commit`1f0eba8e31c5d77626b13c1ab932757056366739` covers62 named bindings in65
  added source plans:61 complete signatures,1 issued prefix,0 unresolved.
  Three native/catalog-only plans are explicitly outside the theorem count.
  All63 candidate source files pass their metadata/source-pin checks.
  The first pass58complete/1prefix/3unsupported is retained; Matrix notation
  context and qualified namespace selection resolve its three tool gaps.
- **Observed:** the unchanged extractor in `tools/pcc/pcc.py:584` ends at
  `:=`, standalone `by` or ASCII bar without parsing the enclosing signature.
  Genuine bar/named-argument/default-proof/inline-by controls demonstrate
  the relevant legal syntax. Actual frozen extractor output was captured
  separately from the native parser and agrees with the predeclared inputs.
- **Derived or proved:** Lean proves noninjectivity of the explicitly defined
  simple delimiter-prefix model and checks the genuine semantic controls.
  It does **not** formally refine the Python regex or the native auditor.
- **Missing capability:** a canonical full-signature binding contract and
  parser implementation; a versioned complete binding for the affected BK
  theorem is still needed. The independent comparator deliberately reports
  comments/string-literal normalization cases as unsupported; all62 actual
  corpus headers in this bounded audit are covered.
- **What this evidence does not establish:** no false BK theorem, kernel
  bypass, corrupted source file, signed verification or failure of every
  historical claim. The scan is bounded to the65 added source plans, not all
  repositories or every ledger entry. Native parsing/hashing is procedural
  evidence; theorem semantics, full namespace resolution, dependency binding
  and execution authenticity remain separate. [pcc:unverified]

A complete written header still does not bind ambient variables or every
referenced definition/dependency. Canonical semantic binding needs an explicit
contract for the elaborated telescope and immutable source environment; the
independent surface parser does not establish that stronger property.
[pcc:unverified]

## What would unblock it

- [ ] Add a versioned BK wrapper with the same actual-object quantified type,
  parser-compatible complete signature, full source/config pins, whole gate
  and complete exact statement hash. Preserve the original claim/ledger/source.
- [ ] Update the canonical shared verifier through its dependency process to
  parse complete signatures, preserve semantically relevant literals and
  resolve exact qualified declarations. Include embedded delimiters,
  comments/strings, same-short-name namespaces and genuine Lean controls;
  reject unsupported forms explicitly.
- [ ] Recheck affected bindings under that contract without rewriting old
  claim bytes or claiming retrospective signed verification.

The local wrapper suffices to repair the concrete audit receipt; it does not
fix the shared parser globally or settle the remaining fourteen-item gaps.
[pcc:unverified]

## Next discriminating step

- **Action:** complete and publish the bounded audit/model-control packet,
  then add the actual BK full-signature wrapper.
- **Inputs and controls:** immutable65-plan inventory, actual frozen verifier
  hash`5e1a97036633b9798adc39aded657708decb244ca9ed98e4a41408be9ac17010`,
  native official Lean4.35 parser, seven genuine controls, existing BK proof.
- **Execution limits:** zero paid compute and no agent upload; source-processing
  audit only, no new cryptographic experiment. Preserve all failed attempts.
- **Outcomes:** a complete wrapper/type hash closes the specific receipt gap;
  a type mismatch must be diagnosed rather than weakened. Upstream parser
  changes need their normal shared dependency review. Source syntax failures
  remain explicit unsupported outcomes, not mathematical counterexamples.
- **Recheck trigger:** published wrapper or a canonical verifier revision.
  [pcc:unverified]

## Resolution and history

Unresolved. The bounded audit is complete with one affected issued binding;
the original proof and immutable evidence remain unchanged. No approval or
integration waiver is inferred from this record. [pcc:unverified]

2026-11-14: model exact local **PCC-20261114-286b6f** replay PASS at 2026-11-14T14:37:43Z, gate8/0/0; native bounded61/1/0 and public-tree replay retained separately. Complete actual BK wrapper remains the next step; this is not a retroactive signed upgrade or a false-theorem finding. [pcc:unverified]
