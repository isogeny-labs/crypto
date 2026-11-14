# Ordered D18 positive formula replay

The ordered-first-pair D18 positive control has a source-bound original XCNF
export, and its archived native assignment satisfies every exported row:
2,807,999 CNF clauses and 3,901 XOR constraints over 329,808 variables.
This supplies a checked positive formula for a bounded external solver probe
on the primary N83 `a=0` curve. [pcc:unverified] PCC claim
`PCC-20261114-e4c5de` has `unverified` standing; the replay is a local
independent check of the retained bytes, not a signed PCC verdict.

| Item | Recorded result |
| --- | --- |
| Source checkout | clean `afebef39283466fca4ddff01063d26df9b59c45e` |
| Release binary SHA-256 | `c30c6f0e2d04e4131740bf768c1485885d64dbc7fa9c0d2e63c07221a711ae66` |
| Exact D18 object | 5,418,738 points; 32,643 folded columns; compressed SHA-256 `eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b` |
| Constructed target | `(6594052263204820946437856, 9593508870534830450764009)` |
| XCNF | 50,523,891 bytes; SHA-256 `c632c152447cb09dbe7dd7cf7e29f713588c37bd56b67dfbf1fa7dac4cb15b74` |
| Native model | 329,808 bits; SHA-256 `e41a2df1bd8cbf940f04797318f071045bb547e7270991f92f2155979920ee3f` |
| Replay | `PASS_ORIGINAL_XCNF_REPLAY`; independent Python clause/XOR walk after verifier self-test |
| Export | exit `0`; 53,175 ms guarded process wall, 50,261 ms worker wall, 49.506 CPU s, 5,525,258,240 peak RSS bytes |

The new export-only command uses the same shared-root, no-choice-pin,
phase-zero-first, ordered-first-pair, cadence-32 configuration as native
attempt 7. The native certificate also includes group relation and altered
target checks. The replay here independently checks the original XCNF, not a
solver transcript or a relation recovered from a natural query.

The [frozen protocol](PROTOCOL.md), [compact raw manifest](evidence/RAW_MANIFEST.json),
and receipts retain the source, binary, inputs, shape, UTC interval, exit,
and replay. The 50 MB original XCNF is retained at
`/Volumes/SSD990-2/tmp/n83-d18-ordered-xcnf-20261114/export-positive/result.xcnf`
with its hash and length in the manifest; it is not in Git. The successful
replay releases protocol preparation for a separate bounded external solver
arm. Natural relation yield, useful rank, complete subgroup logs, target
descent, matched rho, and the a=1 curve remain open for end-to-end selection.
