# BK base-field boundary census and actual Lean transport

The complete finite census covers every prime p<500 at k=1: all nonsingular
general Weierstrass tuples over F2/F3 and all nonsingular short coefficient
pairs over the other prime fields. Two native passes agree byte-for-byte;
the native SQL reconstruction and canonical export/file checks pass.
The separate Lean project passes 29 declarations, with zero failures and
zero trusted-native exceptions. [pcc:unverified]

| Exact scope | Count |
|:--|--:|
| Prime fields | 95 |
| Coefficient models | 7,013,620 |
| Short scaling representatives | 43,330 |
| General F2/F3 model rows | 178 |
| Weighted model/prime-factor cases | 16,450,388 |
| Prime-to-characteristic cases | 16,376,044 |
| Characteristic cases, recorded separately | 74,344 |
| Qualified k=1 domain cases | 9,668,007 |
| Qualified k=1 mismatches | 0 |
| Unrestricted criterion failures | 5,386,300 |
| Failures also satisfying r² divides #E | 2,227,674 |

For each prime r dividing the actual point count, the native program counts
all actual base-field points satisfying rP=O, including O. The unrestricted
test compares this count with r² against r dividing p−1. Prime-to-characteristic
and characteristic cases are kept separate. These weighted counts describe
coefficient models; they are not independent random samples. [pcc:unverified]

The original BK Theorem 1 retains r not dividing q−1. Its Remark 1 already
gives a counterexample even when r² divides #E. The census reproduces this
omitted-condition boundary broadly. No correction to that qualified source,
all-k extension-field check, generic all-prime rank theorem or determinant
proof is claimed. [pcc:unverified]

The source trace predicate p divides p+1−#E identifies 197,920 models in
this population. All prime-factor and all coprime-divisor embedding-order
bounds pass: at most two for p>3, at most six including characteristics
two and three. The literal equality k=2 fails in 197,905 weighted prime-factor
cases. The divisor scan includes divisors that need not occur as cyclic
point orders. No geometric supersingularity/classification bridge is proved
by this native computation. [pcc:unverified]

## Actual formal semantics

| Lean certificate in BKBasefieldScalingAudit | Checked statement |
|:--|:--|
| scaling_audit_certificate | Actual additive point bijection (x,y)→(u²x,u³y), equal point cardinalities, and equal killed-by-n cardinalities for every n |
| rational_low_torsion_certificate | Actual rational 2-torsion equals identity plus cubic roots; actual rational 3-torsion equals identity plus quartic/square-root fibres |
| geometric_low_torsion_certificate | Over algebraically closed fields with explicit discriminant and characteristic hypotheses, actual 2/3-torsion cardinalities are 4/9 |

The scaling proof handles vertical, tangent and chord additions, constructs
the inverse on actual Mathlib points, and precedes reuse of representative
counts. It assumes no torsion rank, basis, Hasse bound or pairing theorem.
The geometric 3-torsion count is inherited from the separately checked
general short-curve project; the actual 2-torsion count is proved here.
[pcc:unverified]

The complete gate audits 29 declarations. The 19 formal/config source pins,
three complete certificate bindings, all 30 build/source-attempt files and
the five final gate files are retained in sources.json and evidence/.
Lean4.35.0 / Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679; only
propext, Classical.choice and Quot.sound. No holes, custom axioms,
native_decide, waived linters or raised elaboration limits. [pcc:unverified]

## Native verification and controls

- Main source/binary/analysis/protocol were frozen and committed before
  data. Compiled source checkpoint b978fdd6; pilot freeze49c8a1f2,
  main freeze6a656748. Protocol hashes and exact binary/runtime/source pins
  are in the canonical records.
- Reference and A/A each emit 43,605 identical JSONL rows. The repeat tests
  determinism; it supplies no independent host or independent-seed estimate.
- SQL separately reconstructs 7,013,442 short scaling members, 43,330
  polynomial point/2/3-torsion counts and 3,929 distinct modular orders.
  All 25 integrity gates pass. Higher-r scalar multiplication is computed
  by the C++ implementation; the SQL implementation checks its aggregates.
- The native program also checks #E·P=O at all 14,176,080 representative
  point rows. Counted model cases and executed point rows are distinct units.
- The separate four-call pilot covers 506 coefficient models in six fields.
  All known controls pass. On F7:y²=x³+2, a deliberately wrong polynomial
  reports3 instead of9; a separate copied-weight SQL corruption fails the
  registered CHECK constraint. Canonical records are unchanged.

The main population includes the pilot fields as part of the complete
universe. It is not a held-out sample. Every scientific clock is invalid;
there is no performance or security inference. Native C++/SQL counts remain
unsigned producer evidence, separate from the Lean semantics proofs.
[pcc:unverified]

## Raw-data availability and reproduction

The two main JSONL files each contain 24,987,542 original bytes. They remain
in the approved local zero-paid checkpoint at runs/main/reference.jsonl
and runs/main/aa.jsonl. A local 646,608-byte evidence/main-raw.tar.xz review
bundle restores both original files byte-for-byte. LOCAL_CHECKPOINT.json
records full hashes, sizes, availability and the checked round trip.
[pcc:unverified]

[AGENTS.md §13](../../AGENTS.md#13-large-files-go-through-the-store) states:
“Too big for Git” means over 10 MiB. No owner/reviewer waiver is present.
These large originals and their local bundle are excluded from Git; the
source, reports, small artifacts and hashes are committed. No remote raw-data
availability or agent storage upload is claimed. A fresh checkout needs the
local bundle or a separate reproduction before in-place export file checks.
[pcc:unverified]

```sh
cd research/bk_basefield_census_20261112
# If the local original records need restoring:
tar -xJf evidence/main-raw.tar.xz
sha256sum -c main-raw.sha256
python3 /workspace/crypto-autoresearcher-current/tools/experiment_protocol.py \
  check-export export.json --protocol protocol.json --verify-files
cd lean
lake build
```

The frozen protocol's build field uses the study-relative source name.
The actual compile ran from the repository root with the full repository-relative
name. Source and executable hashes matched the frozen records throughout.
The exact observed compile argv/cwd are retained separately in
evidence/build-command-clarification.json; the original protocol remains
unchanged. This path difference matters for byte-identical assertion strings.
[pcc:unverified]

## Sources and remaining scope

Reread BK1998 Theorem1/proof/Remark1, printed142–143; original PDF SHA256
ac837b8edc47b346dbb341f787c41de0402c95371a159f9786b747bb2c987499.
Review of the actual Mathlib affine point formulas and prior finite.hpp
arithmetic is recorded in evidence/source-review.json. KB unavailable
(MCP absent, wrapper127); bounded primary/repository/Mathlib fallback.
No novelty claim. [pcc:unverified]

This completes the recorded p<500 base-field boundary component. All-k
extension-field inclusion, generic all-prime rank/determinant and the other
remaining original fourteen-item obligations stay in the central tracker.
[pcc:unverified]

Exact local packet replay: formalPCC-20261113-39640f passes all three
registered Lean bindings; nativePCC-20261113-7dec93 remains not_covered
as unsigned producer evidence. Both signed standings remain unverified.
Actual verdicts/timestamps are in [verification.json](verification.json);
complete bindings and source hashes are in [sources.json](sources.json).
The native artifact plan is [native-attestation.json](native-attestation.json).
[pcc:unverified]
