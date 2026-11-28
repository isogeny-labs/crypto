# Pinned formal scope

Lean4 v4.35.0, CryptoLean5f2ca251cb34806f69fd5455175434cac3567dfa,
Mathlibcfc55fc58714a59dc06e6e79edc6d1d95140d3c2. Two Core Nat instance
obligations check inverse-eight Bezout coefficients. Assembly's anonymous
abstract homomorphism examples import existing Mathlib laws; no actual native
curve/group/field/map instance is constructed. [pcc:unverified]

From the repository root with the pinned Lake dependencies materialized:

```
bash tools/lean/proof-gate.sh --out /fresh/receipt research/curve25519_torus_native_20261115/torsion-column-transport-v1/lean
```

The recorded command used the existing4.35toolchain binary path explicitly
after the initial shell lacked lake in PATH. Existing pinned dependency
locations were reused without deleting or rebuilding the shared Mathlib cache.
The project gate audited2named/0failing/0trusted; both numeric endpoints use
no axioms. Complete build/audit logs are retained. The inherited audit helper
emits a String.mk deprecation warning, not a theorem failure. [pcc:unverified]
