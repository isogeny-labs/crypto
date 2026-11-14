---
title: "Prime-field BK boundary census below 500"
fontsize: 10pt
geometry: margin=17mm
mainfont: DejaVu Serif
---

The recorded k=1 census covers all 95 prime fields below 500: all nonsingular
general models over F2/F3 and all nonsingular short coefficient pairs for
p≥5. Both native passes complete and agree byte-for-byte. Native SQL
reconstruction and the canonical export/file check pass. [pcc:unverified]

| Exact unit | Count |
|:--|--:|
| Coefficient models | 7,013,620 |
| Representative/model rows | 43,508 |
| Weighted model/prime-factor cases | 16,450,388 |
| Qualified prime-to-characteristic k=1 cases | 9,668,007 |
| Qualified k=1 mismatches | 0 |
| Unrestricted criterion failures | 5,386,300 |
| Failures also satisfying r² divides #E | 2,227,674 |

There are 43,330 short scaling representatives and 178 general F2/F3 rows.
For each prime r dividing #E, the program counts base-field points with
rP=O, including O. The unrestricted criterion compares a count of r²
with r dividing p−1. The qualified subset also requires r≠p and
r not dividing p−1; 74,344 characteristic cases are recorded separately.
Model weights are not independent random samples. [pcc:unverified]

![](full-torsion.pdf)

The figure copies exact native counts; eligibility differs by r. Original
BK Theorem1 retains the missing condition, and Remark1 already supplies
an r²-divides-#E counterexample. This census checks the omitted-condition
boundary at k=1; extension-field inclusion at all k remains open.
[pcc:unverified]

\newpage

## Actual Lean semantics and native assurance

The separate Lean gate passes 29 declarations, zero failures and zero
trusted-native exceptions. It constructs the actual additive scaling
bijection (x,y)→(u²x,u³y), including tangent/chord/vertical cases and
an inverse, before reusing representative counts. [pcc:unverified]

| Certificate in BKBasefieldScalingAudit | Checked scope |
|:--|:--|
| scaling_audit_certificate | Equal actual point counts and killed-by-n counts for every n |
| rational_low_torsion_certificate | Actual rational 2/3-torsion root/fibre counts, including identity |
| geometric_low_torsion_certificate | Actual geometric 2/3-torsion cardinalities4/9 with explicit hypotheses |

The proof assumes no rank, basis, Hasse or pairing premise. It uses actual
Mathlib points and has 19 pinned formal/config sources. All 30 build/source
attempt files and the final gate are retained losslessly. Lean4.35.0 /
Mathlib56a00585; only propext, Classical.choice and Quot.sound. No holes,
custom axioms, native_decide, waivers or raised elaboration limits.
[pcc:unverified]

Native SQL independently reconstructs 7,013,442 short scaling members,
43,330 polynomial point/2/3-torsion counts, 3,929 modular orders and all
weighted aggregates. All 25 integrity gates pass. The C++ implementation
checks #E·P=O at all 14,176,080 representative point rows. Higher-r scalar
multiplication is computed in C++; SQL checks its aggregates. A/A has
43,605 identical rows and tests determinism on this same host.
[pcc:unverified]

The source trace predicate p divides p+1−#E identifies 197,920 models.
Prime-factor and all coprime-divisor embedding-order bounds have zero
violations: at most2 for p>3 and at most6 including characteristics2/3.
Literal equality k=2 fails in 197,905 weighted prime-factor cases. Some
scanned divisors need not occur as cyclic point orders. This finite count
does not prove geometric supersingularity classification. [pcc:unverified]

Source checkpoint b978fdd6 precedes the pilot/main freezes49c8a1f2/6a656748.
Protocols, all native outcomes, synthetic controls and exact source/binary/
runtime hashes remain unchanged. A separate build-command clarification
records the repository-root source path used by the actual compiler;
the original study-relative recipe remains frozen. Every scientific clock
is invalid. Native/compiled refinement and independent signed execution
remain separate assurance gaps. [pcc:unverified]

Each raw main JSONL is 24,987,542 bytes, above the repository tenMiB Git
limit. Original files and the 646,608-byte checked review bundle remain in
the approved local checkpoint; hashes and availability are committed in
LOCAL_CHECKPOINT.json. Raw data are available in the shared workspace and
are not claimed present in a fresh checkout or remotely uploaded.
[pcc:unverified]

Primary review: BK1998 Theorem1/proof/Remark1, printed142–143; original PDF
ac837b8e. KB unavailable (MCP absent/wrapper127); bounded original-paper,
repository and pinned Mathlib fallback. Exact source and verification
receipts accompany this report. All-k BK, generic all-prime rank/determinant
and the remaining original fourteen-item obligations stay open.
[pcc:unverified]
