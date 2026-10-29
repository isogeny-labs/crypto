# Missing compact tensor encoder and fast contraction backend for hashed ECDL

- **Status:** open
- **Kind:** implementation gap; external theorem not independently verified
- **Affected work:** [paper replication](../docs/hashed-ecdl.md)
- **Next action owner:** unassigned
- **Last updated:** 2026-10-29

## Problem

Full replication of the linked conditional subexponential ECDL algorithm is
unresolved because the compact arithmetic/decomposition tensor encoder and the
assumed fast contraction algorithm are not implemented in this change.

The paper concerns cofactor-one odd-prime-order elliptic curves over prime
fields (with a stated fixed-cofactor extension). Our bounded native executable
reduction currently supports p <= 65521 and uses generic residual enumeration,
not the tensor method. Its correctness tests cannot establish the claimed
expected running time or a concrete improvement on deployed curves.

## Evidence and uncertainty

- **Observed:** upstream revision `1ee44e562b1b9a50ea339f3b8f81112b5a24b96f`
  contains only `subexponential-ecdl.pdf` (v0.2). §2 Theorem 2 assumes [Ope26]
  Theorem D.1; §3.3 uses the compact arithmetic networks of §2. Neither a
  compact encoder nor a fast contraction implementation is supplied there.
- **Implemented:** inverse-hash factor bases and Figure 2's relation/dependency
  pipeline, with bounded generic enumeration and algebraic certificate replay.
  Tests and precise limits are described in the linked documentation.
- **Derived or proved:** the paper's conditional bound has not been independently
  verified or formalized here. No new theorem is claimed.
- **Suspected explanation:** none; the missing implementations are explicit.
- **Missing capability:** executable low-degree O(mb)-edge encodings, a correct
  exact contraction backend, and independent validation of its prerequisites.
- This does not establish impossibility, theorem falsity, a concrete attack,
  a performance crossover, or security of any curve.

## What would unblock it

- [ ] Obtain and independently review the exact external contraction theorem,
  its algorithm and parameter restrictions; preserve a pinned source.
- [ ] Implement the paper's compact arithmetic and decomposition networks;
  test all small assignments against arithmetic, including identity and
  doubling branches, membership, and witness extraction. Audit edge/degree
  scaling separately from functional correctness.
- [ ] Integrate an exact contraction backend; compare counts and extracted
  witnesses with independent exhaustive BTN evaluation on bounded networks.
- [ ] Establish the claimed complexity under explicit backend assumptions;
  keep any practical timing comparison separate, with matched rho, resource
  and accounting gates if such a comparison is requested.

These conditions address full conditional-algorithm replication. They do not
by themselves establish a practical improvement at cryptographic sizes.

## Next discriminating step

Implement a standalone BTN representation and the compact modular arithmetic
encoder from §2, with assignment-level parity/identity/zero/overflow controls.
Stop at the first disagreement with exact arithmetic and preserve the witness;
a clean bounded panel would admit composition into the §3.3 decomposition
encoding, not establish a runtime bound. Run future experiments under the
existing checkpoint and compute-routing rules. Recheck this record when a
verified encoder or contraction implementation becomes available.

## Resolution and history

2026-10-29 prime-field companion extension: the user's prime counterpart is
officially Certicom ECCp-131. The paired
[implementation and validation receipt](https://github.com/isogeny-labs/cryptanalysis/blob/73498317bbabad289bbfd47ca6ab3d81f80e1083/experiments/hashed-ecdl-tensor-reference/prime-development/README.md)
now composes canonical prime-field arithmetic, modular Fourier multiplication,
point validity/addition, fixed-cofactor image and affine hash constraints with
the scalar BTN engine. Its 39 combined development tests pass, including 16
OpenSSL multiples of published P/Q. The actual-target graph's 1,564,506 edges
compile consistently; default u_0=0 exact summation exceeds the configured
2,000,000-operation budget and produces no witness. Native Rust integration,
general ell, full soundness/complexity audit and both challenge solves remain
open. This progress does not close the checkboxes or assert a speedup.

2026-10-29 companion update: a scalar ell=1 executable Cauchy/mode/Hermite
reference and characteristic-two coordinate encoder are being published in
[cryptanalysis](https://github.com/isogeny-labs/cryptanalysis/tree/a182e2a63c53f4de804744df2c43b0876a9f2c59/experiments/hashed-ecdl-tensor-reference),
with review tracked by [PR #1299](https://github.com/isogeny-labs/cryptanalysis/pull/1299).
The prototype's bounded correctness checks do not close this record. It is
Python, not a native Rust integration; its binary-field Fourier construction
is an adaptation rather than a complete prime-field replication. The actual
target's default u_0 is zero and its exact-leaf search exceeds its stated
budget. No theorem falsity, complexity verification, performance improvement,
challenge relation, or challenge scalar follows. Next steps are a native
port, independent algorithm/encoding audit, and protocol-compliant validation.

2026-10-29: unresolved. The reference-code PR delivers the native reduction
and keeps full replication open rather than relabelling enumeration as tensor
contraction. The cryptanalysis sibling links this canonical obstruction.
