# Collision certificate implementation and registration evidence

This compact evidence covers the prospective implementation gates,
conditional proof, identity certificate, all 42 prepared input packs,
launcher controls and inspected report. Production case results are
separate and pending at this registration checkpoint.

| Attempt | Actual outcome |
|---|---|
| Native/proof gate v1 | 4 tests/build pass; Lean fails in auxiliary zero-exponent branch |
| Native/proof gate v2 | 4 tests/build/Lean pass; source seals pass |
| Preparation v1 | Exit 2; parser expected the wrong completion tag in a valid Phase 1 record; no case input emitted |
| Native/proof gate v3 | 5 tests/build/Lean pass; schema regression included; source seals pass |
| Preparation v2 | Exit 0; all 42 equation-only case packs and sealed workload generated |
| Standard cubic IDC | Issue/check pass, 64 derived plus 64 extra points; false identity refused |
| Actual driver controls | Outer reservation 64 without output; existing phase 65 with unchanged marker and no candidate directory |
| Layout v1/v2 | v1 overflow preserved; both actual v2 pages inspected and legible |

The accepted native executable is SHA-256
`4c49d77faa9b5ef3fd54459bd0ac0cf69ee077eef43ec84361753f7dbc3555f5`.
The sealed 42-case workload is SHA-256
`17ecb4c302526bc58e592466071e1c2704eaaf43a0a0ca1efe563e1bd69b3c10`.
The cubic certificate is `IDC1h10810f17e0f6c8f6`; its Lean source digest is
`fe7fbef221b2d391f3793976fd3babbe14673ef08b0ddaa3b5ae74664764773c`.

Current root Cargo/src/examples still match the retained library-validation
source. The original 85-case panel's compiled source manifest also checks
unchanged; see `phase2-source-check.txt`. No original study, Diem generator,
cancelled waiting/freeze follow-up or EKS job is restarted.

Compact metadata is copied byte for byte from completed gate/preparation/
identity/control attempts. Historical source paths refer to those versions'
source snapshots; a newer working tree need not match an old failed gate's
hash. Raw executable/source snapshots and every failure remain under the
ignored `../../runs/collision-certificate-*` tree, to be packaged into a
separate versioned archive after production terminates. These local paths
do not claim publication or final replay of that archive.

See [protocol and implementation](../../collision_certificate/README.md),
[freeze](../../collision_certificate/FREEZE.json),
[report](../../collision_certificate/BUILD_AND_PROOF.md),
[inspected PDF](../../collision_certificate/BUILD_AND_PROOF.pdf),
and [managed-owner reconciliation](../../collision_certificate/RUNNING.md).
All complete end-to-end and admitted-speedup fields remain null.
