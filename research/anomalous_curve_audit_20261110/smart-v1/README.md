# Concrete Smart arithmetic and a 160-bit known answer

The frozen native run recovered the fixed known-answer scalar in all four
coefficient lifts on the corrected 160-bit curve. It also recovered 2 on the
small control and rejected the off-curve control. Every outcome is retained.
The native measurement packet is `PCC-20261110-162b83`: actual local verdict
`not_covered`, signed standing `unverified`. Producer checks and the separate
mathematical certificate are not an independently certified native execution.
[pcc:unverified]

The postdata Lean certificate evaluates the fixed arithmetic program on the
seven public integers in `inputs/large.txt`. The program receives no expected
scalar. For each i=0,1,2,3 it uses A=a+ip and B=b+(2i+1)p, lifts G and Q,
checks the full binary multiplication traces over ZMod(p²), extracts the
parameter coefficients, and proves recovery of
`210393287966660756596132643172172640405085536179`. The separate actual
Mathlib point certificate proves that this returned scalar satisfies dG=Q,
on a nonsingular curve with G of exact 160-bit prime order p.
Whole gates pass 28 predata and 51 concrete declarations, with zero failing proofs and zero trusted-native exceptions. [pcc:unverified]

## Proof map and scope

| Source | Obligation |
|:--|:--|
| Frozen `lean/SmartArithmetic.lean` | Pure Jacobian polynomial program; trace soundness; binary schedule; parameter/recovery definitions |
| Frozen `lean/SmallControl.lean` | Actual F5 curve, point order 5, 2G=Q and off-curve equation disproof |
| New `certificate-lean/SmartSpecification.lean` | Public-input arithmetic attempt; checked Hensel/inverse/coefficient composition lemmas |
| New `certificate-lean/SmartLargeLift{0..3}{G,Q}.lean` | Eight explicit 211-node traces, all 1,680 transitions, exact schedules, lifting and coefficients; initial/output curve equations |
| New `certificate-lean/SmartLargeRecovery.lean` | All four returned values from concrete witness composition |
| New `certificate-lean/SmartCertificate.lean` | Fixed four-lift policy, actual public coordinates, returned scalar satisfies the actual point relation |

The main theorem is `SmartConcreteAudit.concrete_attack_certificate`.
This is concrete finite-program correctness. It does not prove a generic
p-adic logarithm transfer theorem, the entire curve point count=p, a native
compiler refinement, attack complexity or a matched rho/SafeCurves constant.
NIST's explicit anomalous-curve exclusion is retained. No screened standard
is refuted. [pcc:unverified]

The source formatter copies checked decimal strings and performs no curve
arithmetic. All numerical equalities are reconstructed in Lean against the
unchanged predata program; constructor output is untrusted. Only standard
axioms are allowed; no sorry, custom axiom or native_decide exception.
Source commit `7fc27f7f` precedes theorem packet `PCC-20261110-d8466d`. Full local verification passes at 09:44:26Z; signed standing remains unverified. [pcc:unverified]

## Frozen run and reproduction

Protocol `smart-known-answer-20261110/v1` was frozen at 08:56:55Z on
2026-11-10, hash
`00e7a4a76936914ded10c4bf2f40a1762e8eaec591af6d8abe5c316380684c56`.
Source 70aa9f2f and protocol commit 37302ba3 precede the three registered calls.
Flow: 3 planned,3 attempted,3 completed,0 excluded/interrupted/resumed.
All four valid lifts were nondegenerate in both positive arms. Offcurve's
expected exit 2 is a completed rejection, not a recovered logarithm.
The all-row export passes canonical file/schema/protocol checks. No timing
metric, statistical claim, retries or paid compute. [pcc:unverified]

| Arm | Exit | Fixed lift results | Logical operators across G/Q |
|:--|--:|:--|--:|
| F5 control | 0 | 2,2,2,2|24 total; 6 per lift|
| F5 offcurve | 2 |Input point off curve|No multiplication traces|
| 160-bit fixture | 0 |Same certified d in all 4lifts|1,680 total;420 per lift|

The protocol, native source, analyzer, predata Lean functions, raw runs and
export are frozen. Do not rerun or repair this record in place. Failed Lean
attempts and source snapshots are retained; postdata mathematical modules
are separately pinned. A complete proof can be reproduced from their literal
source without trusting the native binary or source formatter.
[pcc:unverified]

```sh
cd research/anomalous_curve_audit_20261110/smart-v1/certificate-lean
lake build
cd ../../../..
tools/lean/proof-gate.sh research/anomalous_curve_audit_20261110/smart-v1/lean research/anomalous_curve_audit_20261110/smart-v1/certificate-lean
```

See `PROTOCOL.md`, `protocol.json`, `provenance.json`, `export.json`,
`evidence/native-pcc-verify.json`, the parent point study, and
[the report](reports/smart-20261110/report.pdf). Sources checked: Smart's
original HPL-97-128 report; EFD short-Weierstrass Jacobian add-1998-cmo and
dbl-1998-cmo formulas; the original anomalous-curve paper and author erratum;
NIST SP 800-186 section 3.1.2/Appendix C.1. Existing source hashes/read scopes
are retained in the protocol/provenance and parent source plan.
KB/Conductor unavailable; bounded primary/repository/Mathlib fallback.
No newness or independent-host claim. [pcc:unverified]
