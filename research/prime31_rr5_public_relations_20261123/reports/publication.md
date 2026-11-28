# Publication record

- Draft PR: https://github.com/isogeny-labs/crypto/pull/4970.
- Published producer: `9db32aeb03d7f1c1964016efcb23ed9c8241ed9f`.
- Producer tree: `724221b848ebe7d5e6bdf5be4aad03a34d84ff55`, matching the local staged tree exactly.
- Published unsigned-registration commit: `baaa7b04f60fe10802f1a04e4facaf999cfa0691`.
- Registration tree: `56aed2a24fad6e3b2dcc5743f31d6207ed12a6fd`, matching the local staged tree exactly.
- Observation: `PCC-20261123-688e4c`, unsigned, unverified, no verifier execution or verdict.
- Observation SHA-256: `688e4c7dd5ca6be980ecf1c5c29dcfa355a4e994724f0e5a7bb951a1f736a507`.
- Registration appended ledger sequence 1564 and preserved the full preceding raw ledger bytes.

The PR body passed the existing reviewed prose parser with an empty standing
table and only `[pcc:unverified]` citations. This documentation-only invocation
did not load standings or execute a signature/certificate verifier. The native
registration receipt remains the record of what the metadata CLI checked.

The original B677 extraction, complete public recovery, full-library gate and
fully charged matched comparison remain open. This publication records the
bounded component work and is not a final performance result. [pcc:unverified]
