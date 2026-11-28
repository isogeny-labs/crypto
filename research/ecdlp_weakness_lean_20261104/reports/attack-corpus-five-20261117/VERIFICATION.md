# Verification receipt: five assumption boundaries

Date: 2026-11-17. Lean source commit: `4ce098bdb0`. Toolchain: Lean v4.34.1
and Mathlib v4.34.1.

## Kernel and project checks

From the repository root:

```text
PATH="$HOME/.elan/bin:$PATH" lake build
```

Run in `research/ecdlp_weakness_lean_20261104/lean`, this completed
successfully with `Build completed successfully (8941 jobs).`

```text
PATH="$HOME/.elan/bin:$PATH" lake env lean AxiomAudit.lean
```

This exited zero. The new declarations use only the standard Lean/Mathlib
foundations `propext`, `Classical.choice`, and `Quot.sound`; several use no
axioms from that list.

```text
PATH="$HOME/.elan/bin:$PATH" tools/lean/proof-gate.sh \
  --out /tmp/attack-corpus-five-lean-receipts-2 \
  research/ecdlp_weakness_lean_20261104/lean
```

Exact terminal result:

```text
PASS                    158 thm   0 trusted  research/ecdlp_weakness_lean_20261104/lean
```

Retained receipt digests:

| file | SHA-256 |
|---|---|
| `audit.log` | `d50ac7b8770665626bb5cc42ca311ccc2703c10c6137647d46d526bc6fa73d68` |
| `build.log` | `e4a307007fbe5a67c832459bbe6a84a9d2e91b1e8bedd27c6589151f778da34b` |
| `cache.log` | `29e284dc7d8024aa6a333a92d0b23035028d78ada47b37f7dd77d8e1a0293483` |
| `source-build.log` | `bd2489a2eb819190dd991ece1f215af348ca89ddbd717fc687ccdad7dbb1ebd3` |
| `status` (`PASS`) | `c26de83abdc9496cd1301470918ec39ecca1cf389ef0ae1c6504da1800d1c431` |

## PCC binding

The central theorem pairs are bound by five `interpretive` claim records:

- `PCC-20261117-a627cf`: finite transcript separation and extraction implies
  soundness;
- `PCC-20261117-1ed703`: composite collision and prime-coordinate
  injectivity;
- `PCC-20261117-a0edbe`: raw concatenation alias and framed-codec
  injectivity;
- `PCC-20261117-d4d88d`: scalar trapdoor constructor and exceptional point;
- `PCC-20261117-2dca91`: harmonic divergence and positive-hazard uniform
  bound.

These records are unsigned [pcc:unverified]. `origin/main`'s pre-existing PCC
ledger currently fails the modern claim-schema check because legacy claims do
not carry `claim_type`; all five records above do carry
`claim_type = interpretive`.

The PCC implementation's own declaration parser matched all ten pinned theorem
hashes. A full local `pcc verify` of the representative ZK claim returned
`verdict: pass` for both Lean references at report commit `128dbd0512`; each
reference independently reran the 158-theorem project proof gate. This local
verdict is not a signed main-branch verdict and does not change the
`[pcc:unverified]` standing.

## Additional checks

Four independent review passes covered Lean statement fidelity, primary-source
correspondence, report scope, and identity-certificate compatibility. The six
recorded primary-source hashes matched the retained artifacts. The native C++
identity issuer accepted 32 issuance points plus 64 extra domain-separated
self-replay points and rejected the changed-right-hand-side negative control.
That certificate covers only `(c-y)+y=c`; it is not a KZG security certificate.

The issuer was compiled with
`g++ -std=c++20 -O2 -Wall -Wextra -Werror identity_issue.cpp -lcrypto`.
Its output was byte-identical to the committed certificate. SHA-256:

- issuer: `6dfa4c267e02ba0e715b17367ac9bae80da2bfc9c95397878cc550044c7ae921`;
- certificate/reissue:
  `2cc15660eaacd94c5865b99624e5c12a4339da7893de555c24a579914de541cc`.

Rust was unavailable locally, so this is the disclosed C++ mirror replay, not
an independent run of the official Rust verifier. The independent schema and
stream-rule review found it structurally compatible with that verifier.
