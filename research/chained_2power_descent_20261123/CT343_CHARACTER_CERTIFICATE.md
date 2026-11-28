# CT343 follow-up: two quadratic characters replace source scalar-lift arithmetic

2026-11-23 UTC — bounded public F103 positive control; [pcc:unverified]. Additive protocol registered before execution in [PR #5066 comment](https://github.com/isogeny-labs/crypto/pull/5066#issuecomment-7115047665); frozen base head f0752c8272002de9c04ff9e2abbcc9ccd88f91f2. Original artifacts unchanged.

## Claim and assumptions

Let A=J(C)(F103), B=E+×E-, Phi the CT340 product map. CT341/342 report ker(Phi)=(Z/2)^2, with four classes 0,K1=W6-W1,K2=W2-W5,K3=W3-W4 and K3=K1+K2. On the odd model eta²=F(z)=prod_(i=1..5)(1-i*z), z=1/(x-1), leading coefficient c=86, set X=c*z and Y=c²*eta. Then Y²=c^4 F(X/c) is monic. The five standard 2-descent coordinates evaluate X-c*a on reduced divisors, for a=1/i.

For monic Mumford U of degree d in z, the squareclass at root a is (-c)^d U(a) when U(a) != 0. If U=(z-a)U1, use c³ F'(a) (-c)^(d-1) U1(a) instead, since the local leading coefficient of X-c*a at the branch is the inverse of the monic polynomial derivative, equivalent modulo squares. At infinity of the monic odd model the local leading coefficient is 1. Repeated branch support is invalid. Non-split support is handled by the polynomial norm, without root extraction. Reference: Dallaporta, 2024, §3.3, https://doi.org/10.1112/jlms.12921 (odd monic descent map); Stoll, 2001, https://doi.org/10.4064/aa98-3-4. No novelty claim.

Write chi for the two Legendre-symbol bits at a=1 and 52. Their restrictions to the four actual kernel points, in the above order, are 00,10,01,11. Consequently (Phi,chi) is injective on A **conditional on the homomorphism and full rational kernel premises**: equality of both outputs implies the difference is in ker(Phi) and has zero signature, hence is zero. This is a standard descent argument, not a kernel-checked proof.

For either CT343 generator G (order 2m, m=23 or 575), chi(G)=10. After a verified destination log r mod m, evaluate chi(Q). If it is neither 00 nor 10, reject. Otherwise choose the unique s in {r,r+m} whose parity matches chi(Q). No source scalar multiplication is needed for this conditional certificate. One coordinate alone distinguishes the two candidate lifts for promised subgroup members but accepts some off-subgroup inputs; two coordinates are necessary to separate this four-element kernel. The destination log must still be verified and G/order/map/character validity must be established. A single-target cold implementation must charge all those prerequisites.

## Executed results

Frozen local CT339 source SHA256 c798c3201da743091be50e1a3c3cd63200f2e995e72d17991518db3ca5345695. The coordinator used the actual CT339 Cantor arithmetic and product map, not a symbolic cyclic group. Destination residues were obtained by a complete precomputed elliptic-product image table (a correctness oracle, **not** a proposed competitive cold solver). Source classes generated sequentially; candidate selection receives only residue and two signatures, no known scalar or Cantor call. All 46+1150 classes accepted correctly, all 1196 false scalar alternatives rejected, all 2392 off-subgroup kernel-shift controls rejected; each shifted class was checked to have the same actual elliptic image. Coverage: 610 nonsplit, 82 repeated, 10 degree-one, 492 split, 2 identity. Three changed-code controls (bad branch derivative sign, duplicate coordinate, ignore second coordinate) rejected under normal and optimized Python. Canonical JSON byte-identical.

Original executed source SHA256 333797ae6f03eb429deb803d3019ff6a4f5a48d4989abbef4935e54678ddc4a1, Git blob 3744ef4b5de38bf8c02dbb5d3b0d13804a5c5375. Result SHA256 c7e51842572399904b033d87b192c86d8d3035a1449e91fd59177f4ecd921111, Git blob 117f3a318c15ef1320b41f398d2d6f8907b11790. Full result (canonical JSON, followed by newline):

```json
{"chosen_indices":[0,1],"chosen_roots":[1,52],"cohorts":[{"categories":{"nonsplit":24,"one":1,"split":20,"zero":1},"generator_signature":[1,0],"m":23,"off_subgroup_rejections":92,"targets":46,"wrong_lifts_rejected":46},{"categories":{"nonsplit":586,"one":9,"repeated":82,"split":472,"zero":1},"generator_signature":[1,0],"m":575,"off_subgroup_rejections":2300,"targets":1150,"wrong_lifts_rejected":1150}],"kernel_full_signatures":[[0,0,0,0,0],[1,0,0,1,0],[0,1,1,0,0],[1,1,1,1,0]],"kernel_pair_signatures":[[0,0],[1,0],[0,1],[1,1]],"lc":86,"p":103,"roots":[1,52,69,26,62],"schema":"ct343-character-certificate/v1"}
```

## Reproduction and actual roles

Use the existing CT343 report's CT339 extraction command and source hash check. The executable source is transcribed separately only if its Git blob matches the exact recorded executed blob. Commands executed: python3 ct343_character.py, python3 -O ct343_character.py, cmp, sha256sum, git hash-object; separate --bad-branch, --bad-pair, --ignore-second invocations in both modes. This was one coordinator, no independent worker or PCC signer. Native Rust/Conductor/research-KB unavailable. The shared daily log read returned empty; no overwrite. Current native ownership, authentic 695/701 norm-reuse fixture, physical C169/C13a/C13b/N2_actual/D pricing, original benchmark and EKS reservations not touched. No paid compute, competing timed run, CI rerun or merge. No elapsed-time gain is inferred. New cost target is two character evaluations per target and two for the generator (given fixed separating coordinates), rather than two source scalar multiplications; field exponentiation and exceptions must be priced, as must all other cold stages. Direct-source PH still a necessary composite-order comparator.

## Handoff

Next: independently reproduce the two-character certificate in native arithmetic with explicit input validation and frozen map/kernel assumptions; measure field operations and compare complete cold costs only with authorized matched resources. If ker(Phi) intersects 2A nontrivially on another curve, ordinary quadratic characters cannot separate that intersection: the two-character certificate is not universal. Use higher primary characters or fall back to source verification there. This remains a scoped arithmetic/correctness result and unsigned [pcc:unverified].
