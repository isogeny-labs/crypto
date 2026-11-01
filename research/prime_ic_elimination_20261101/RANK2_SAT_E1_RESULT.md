# Rank 2: shared-coefficient SAT representation-equivalence result

Date: 2026-11-01

Frozen protocol: `research/prime_shared_coefficient_sat_20261031/E1_PROTOCOL.md`

Verdict: `sat_boolean_subgate_passed_msolve_cvc5_open`

## Outcome

The frozen correctness panel passed every registered CaDiCaL/CNF and
CryptoMiniSat/CNF-XOR gate.

| Requirement | Result |
| --- | ---: |
| Affine F17 targets | 18: 6 SAT, 12 UNSAT |
| Exact target/backend projected-set matches | 36 / 36 |
| Target solver attempts | 48: 12 SAT, 36 terminal UNSAT |
| Independently checked UNSAT proofs | 42 / 42, including six fixed mutations |
| Empty proof truncations rejected | 42 / 42 |
| Fixed positive controls | 6 / 6 SAT and exact (`m=3,5,7`, two backends) |
| Fixed `a0+1` mutation controls | 6 / 6 checked UNSAT |
| Strict parser controls | 5 / 5 |
| Timing collected | no |

Every target first passed exact projected-model-set equality between the
coefficient enumerator and the separate elliptic group/interpolation oracle.
Every SAT model passed canonical replay and independent oracle membership.
Every terminal target UNSAT and fixed mutation UNSAT had an accepted LRAT or
XLRUP proof, and its empty truncation was rejected.

## Identities and retained evidence

- Source revision:
  `3d76699b00ea089d63cd30072c294b5a0d3a2bae`
- Runner SHA-256:
  `50531f04354ad621848cb0777ff2c5d457e32109883edecdddcd1e1f5731ccc9`
- Aggregate SHA-256:
  `ed74dc6df578a3d03781277c194ed653d6fcea8055c535ad19e49523fc1af04f`
- Raw tree before archival: 451 files, 6,920,083,569 bytes.
- CaDiCaL, LRAT-trim, CryptoMiniSat and cake_xlrup binary hashes matched the
  frozen protocol exactly.

The write-once raw tree was losslessly archived at
`/workspace/prime-e1-sat-elimination-20261101-full.tar.zst` (1,777,011,716
bytes, SHA-256
`b61ba10fc8bbcad0768903a04e6b9be5da5cecdd63dc60be1dc2a2785261e82c`).
`zstd -t` passed, the archive listing contains the aggregate, and streaming
that aggregate back out reproduces its SHA-256.  The repository receipt is
`sat-e1-receipt.json`.  Checkpoint/S3 identity is unavailable, so a durable
remote raw location is still blocked; local archival does not satisfy that
obligation.

An earlier `/tmp` route was interrupted before a verdict when proof volume
exceeded that filesystem's safe headroom.  Its 1.2 GiB partial tree is retained
separately as
`/tmp/prime-e1-sat-elimination-20261101-partial-route-retained` and is not part
of this result.

## Interpretation

This result clears the Boolean lowering and external proof-bearing SAT
representation gate at the exact F17 scope.  It strengthens SAT as a backend
candidate: ordinary CNF and native-XOR encodings solve precisely the intended
predicate, including all empty targets and mutations.

It supplies no performance evidence.  The runner intentionally records no
time, instructions, memory, decisions or conflicts.  The fixed-`m=3`
`s=6..14` exponent screen still needs the source-bound `F_p` trail theory,
checked explanations, complete-cost ledger and isolated execution.  The
msolve comparator remains checkpoint-gated; cvc5 1.4.2 remains ineligible for
the independent-UNSAT-proof gate.
