# Solver-control follow-up

The latest expanded repository gate passes93declarations,0failing,
0trusted-native exceptions. It includes the actual bivariate quotient
equivalence/length2 and closed support1witness. All16frozen-protocol msolve
calls pass exact checks; the canonical export passes against that protocol
with file hashes. The standalone native checker compiles with
-Wall/-Wextra/-Werror. Receipts and scope are in
../solver_multiplicity_audit_20261109/evidence and its README. Claim
PCC-20261109-ee57d0 remains unsigned. Earlier counts below are historical
increment receipts and are preserved. [pcc:unverified]

# Validation scope

Latest follow-up: repository gate passes74declarations,0failing,
0trusted-native exceptions, including both actual torsion-boundary modules.
The source/field-degree report and exact unsigned claims are in PRIMARY_AUDIT.md.
The ledger now has470entries and remains a valid append-only extension of
the local parent. Main's previously documented fork is not reconciled.
[pcc:unverified]

The direct FST and actual BK-boundary claim packets also pass their full
local PCC checks (same repository kernel gate); receipts are
evidence/pcc-fst-torsion-check.json and evidence/pcc-bk-boundary-check.json.
They remain unsigned. Both pages of the new torsion/source PDF were visually
inspected with legible Greek/math symbols after the final rendering.
[pcc:unverified]

The list below is the earlier census increment's snapshot; its original
41-declaration and463-entry receipts remain unchanged. [pcc:unverified]

- `lake build`: pass, pinned Lean4.35.0/Mathlib56a00585; all earlier failed
  incremental builds retained. [pcc:unverified]
- Final repository transitive gate: pass41audited declarations,0failing,
  0trusted-native exceptions. Includes generated equation lemmas.
  Receipt: evidence/proof-gate-final.log and its directory. [pcc:unverified]
- Both corrected PCC claim checks: pass, unsigned. Initial packets lacked
  declaration hashes and are retained as superseded. The failed initial
  rational-packet receipt is evidence/pcc-rational-check.json.
  No signed validation or independent-host standing is claimed.
  [pcc:unverified]
- PCC ledger:463entries, chain/prefix check passes against9172a052.
  No prior row was rewritten. [pcc:unverified]
- Unchanged repository identity checker/SHA256 in the standalone issuer:
  22release tests pass; identity issuer checks32+11points and emits
  IDC1h819303b7fd181c8c. [pcc:unverified]
- Native C++ controls and bounded census: all completed; zero discrepancy
  counters. Source hashes match the pre-run receipt. No clock or performance
  comparison. [pcc:unverified]
- Export structural/file-hash validation:0findings. This is deliberately
  not a frozen-protocol check: no committed frozen JSON protocol preceded
  the census. [pcc:unverified]
- Report: two-page PDF rendered and every page visually inspected after
  the final representation-scope correction. Graph sources/data retained.
  [pcc:unverified]
- Prose lint and source/prose whitespace check: pass. Raw Lean/compiler logs
  retain the tools' original trailing whitespace and produce whitespace
  warnings in the unfiltered diff; their bytes are not normalized.
  [pcc:unverified]

Repository-wide checks attempted, with inherited/environment limitations:

- Root `cargo test --release --lib` exits101: missing sparse path package
  `crates/ecdl-tensor-ic`. No root library source was changed. The touched
  standalone issuer's22release tests pass. Full root library suite is not
  reported as passing. Receipt: evidence/root-library-tests.log.
  [pcc:unverified]
- Global `tools/benchrec.py gate` exits1 with887problems in75existing
  records: unrelated missing archive inputs, already-open base record and
  stale base progress. No benchmark record or canonical progress file is
  changed; another workstream's records were not closed/regenerated.
  Full receipt: evidence/benchrec-gate.log. [pcc:unverified]

This study uses a separate additive path while PR3739 awaits its requested
Copilot conflict resolution. Its conflicted old session-log file is not
edited. A stacked PR cannot enable GitHub auto-merge until its base becomes
main; retarget and enable only after the parent lands and applicable checks
pass. [pcc:unverified]

Integration check at main e472218c8b14a770c38a1198bc2073edbc03d480 found
divergent immutable PCC ledgers: main458entries, local463, common prefix457;
neither is a prefix of the other. The previous parent component already
diverges from main. Parent autofix reported `needs-human` at01:02Z and no
Copilot resolution appeared by this check. No ledger was rewritten, no side
was selected, and no main merge is attempted. Reconciliation needs the
repository's authorized ledger process. Receipt:
evidence/main-ledger-integration.json. [pcc:unverified]

2026-11-09 CI follow-up: the full PCC gate failed on the two original active
packets lacking declaration hashes; successor packets alone did not repair
`verify-all`. Their original bytes now live in evidence/metadata-originals/.
The unsigned active packets have their missing declaration hashes restored,
with appended claim/note events for the changed digests; statements and all
existing ledger rows are unchanged. This is a correction of our metadata
defect, not an inherited CI failure. The amendment receipt records both
digests. [pcc:unverified]
