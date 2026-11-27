# Separate branch-factor identity record

Use the unchanged maintained identity-certificate module and SHA256 module to
issue G=H_plus*H_minus, degree bound20, variables A/tau/w. Its source expression
matches the already gated ReceivedIdeal.branch_factorization declaration.
Record32 issuing points and32 checker-extra points over the format's fixed
2^61-1 field, and refuse plus-one corruption through checking and issuance.
The optional Lean source hash is a pointer; this checker does not invoke Lean.
This is same-producer bookkeeping, not independent signed validation or a
proof for native Curve25519 field operations. [pcc:unverified]

Compile/test/fmt/strict Clippy this separate crate; retain source and logs.
Publish its exact source before its one fixed issuance call. Bind the six
inputs Cargo.toml/Cargo.lock/src/main.rs plus maintained identity and SHA
modules and the unchanged Lean pointer subject. Run from repository root
with a fresh output directory and collision refusal. The frozen102-input
native qualification, executable, source manifest and evidence do not change.
No decoder, backend search, sampling, benchmark or paid campaign is added.
