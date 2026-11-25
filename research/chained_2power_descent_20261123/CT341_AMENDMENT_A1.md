# CT341 amendment A1 — rational-point isogeny cokernel, before full-census rerun

2026-11-23 UTC; original [CT341_PROTOCOL.md](CT341_PROTOCOL.md) is preserved unchanged. Original standalone local preregistered script SHA-256 `c7554c23482d69dc511d9d41f552c6577dfc9ef8380847a9cef974c63c0125d8` exited nonzero at its **first** zeta-order guard, prior to product/Jacobian census. The command was `python3 /mnt/data/ct341.py`; recorded exception: `RuntimeError: Jacobian order from independent zeta != 2300`. Do not misrepresent this as a passed original protocol.

## Actual finite-field evidence from the first diagnostic

Point counting the original odd genus-two curve `eta²=F(z)` with one point at infinity yields `#C(F103)=88` and `#C(F103²)=10862`. Let `t1=104-88=16`, `t2=103²+1-10862=-252`, and `e2=(t1²-t2)/2=254`. Then `#J(F103)=1-t1+e2-103 t1+103²=9200`. Independently CT339 fixes `#Eplus(F103)=92`, `#Eminus(F103)=100`, product `9200`. An isogeny of degree 4 over F103 preserves the *total rational point count*, not the number of rational points in the image. The original protocol incorrectly identified `#im(Psi(F103))=9200/4=2300` with `#J(F103)`.

This finite rational-point cokernel issue is mathematically consequential for an end-to-end DLP transfer. A separable `(2,2)`-isogeny with a full rational 4-element kernel can induce an image of index 4 on rational points, consistent with both groups having 9200 members. Full Jacobian coverage cannot be claimed by enumerating the `Psi(F103)` image alone.

## Amended, stronger acceptance test (frozen before further code execution)

1. Keep the original CT339 fingerprints and elliptic count guards, and the independent `F103,F103²` point-count formula for `#J=9200` (independent of the product group).
2. Enumerate all 9200 `Eplus(F103)×Eminus(F103)` points, compute `Psi`, require exactly 2300 image classes with each fiber of size 4. This is a *proper index-4 subgroup of J(F103)*, not the entire group. Check `Phi Psi=([2],[2])` on all 9200 product points, as before.
3. Independently enumerate **all** `J(F103)` Mumford classes by (a) rational affine point pairs, including tangent/double and Weierstrass exceptions, (b) Frobenius-conjugate degree-two divisor supports over F103², (c) one-point classes, and (d) identity, normalizing each with the frozen CT339 norm guard. Require 9200 *distinct* classes, no silent omissions, and exact match to the zeta count. If this cannot be established, report a **partial** result.
4. Check `Psi Phi(D)=[2]D` for every one of the 9200 rational Jacobian classes, including the 6900 classes outside `im Psi`; verify `ker Phi` has exactly four rational points, `im Phi` index four, and the concrete Weierstrass kernel representatives are all present.
5. Re-run normal and optimized Python with byte-identical output, and non-vacuous sign/W1/source-fingerprint mutants with nonzero failure exits. Record any original/test changes, failures, counts and hashes. This does not convert coordinator replay into signed PCC or authorize an original-cost ECDLP speedup.

No external original norm-reuse fixtures, paid computation, root Rust build, held C169/C13a/C13b/N2_actual/D pricing, strongest rho/BSGS/IC session, or peer-owned branch is touched. [pcc:unverified]
