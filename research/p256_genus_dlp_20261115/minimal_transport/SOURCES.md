# Sources and exact reuse

## Primary literature

- Benjamin Smith, *Easy scalar decompositions for efficient scalar multiplication
  on elliptic curves and genus 2 Jacobians*, arXiv1310.5250v1, sections1--5.
  https://arxiv.org/abs/1310.5250 ; PDF SHA256
  `508b9dda476a02891a3cacedfb5279624e09ebaf702dab30c8bd7f719ab24c65`.
  Read the scalar-decomposition kernel, efficient-map requirement, short-basis
  role and GLV context. This is established background, not a new GLV discovery.
- Bei Wang, Yi Ouyang, Songsong Li and Honggang Hu, *A new twofold Cornacchia-type
  algorithm and its applications*, eprint2021/090, section3/Algorithm1 excerpt.
  https://eprint.iacr.org/2021/090.pdf . The primary indexed/retrieved excerpt
  supplies the Eisenstein norm-n construction; later direct retrieval returned403.
  No local full-paper hash or complete-paper audit is claimed.
- Stephen Galbraith, *Mathematics of Public Key Cryptography*, section14.4,
  reviewed in the prior round; primary PDF SHA256
  `ca75ee62b96daa0d46faaa2d4dc28205683455ad3a2a48586b9b39299c25022d`.
  Orbit-step gains still require representative/cycle costs; this round does
  not add a new benchmark or repeat that source's claims as measurements.

## Existing code and geometry

- Frozen transfer_search/REPORT.md, section1: adjoint matrix, mixed map,
  subgroup action and non-scalar-image control; prior PCC-20261115-e111c2.
- Frozen folded_walk: exact coefficient transport, canonicalization, bounded
  solver, source adapter and previous public qualification. The present map
  is new/unqualified; earlier solver checks do not qualify this implementation.
- Existing native identity_certificate.rs is reused without alteration to
  serialize/replay the already kernel-proved polynomial identities. The new
  serializer links no minimal-transport candidate code.
- Shared CryptoLean.Curve.Conductor.quadraticNorm, inherited as stated. New
  generic laws are in CryptoLean.Curve.QuadraticNormTransport at
  `f74b0e8bd3815fdf2adae5b180431fc94d2be107` in aburan28/crypto-lean.
  Shared PR12 merged: https://github.com/aburan28/crypto-lean/pull/12 .
- Lean4.35.0 (eb6cafb7290e8bd5f2937522d8d804f58f02bafc), Mathlib
  `cfc55fc58714a59dc06e6e79edc6d1d95140d3c2`; complete dependency pins in the
  study lake-manifest. Canonical proof-gate/auditor source hashes are retained.

## Lookup and access limits

KB wrapper exits127; no KB connector or ecdb credential is available. Used
repository/primary-source fallbacks and made no novelty claim. Shared Lean
repo is public/readable at aburan28; the guessed isogeny-labs URL returned404.
RunPod read-only API check returned401. The S3 resume gate reports no token,
profile or ambient identity. These are access dependencies, not mathematical
obstructions or negative native results.
