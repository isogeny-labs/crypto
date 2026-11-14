# CT92 — Chebotarev density and exact degree-one source-place counts for global torsion fields

**Standing: [pcc:unverified].** This is an exact conditional, classical Galois/Chebotarev computation applied to CT80's global 7-torsion transfer fixture, not an efficient arbitrary-input lift, ECDLP speedup, or a finite-range prime-finding runtime bound.

## Result (general prime ell)

Let E/Q have good reduction away from finitely many primes, and let ell be an odd prime such that the mod-ell Galois image is **all GL2(F_ell)**. Let T be a nonzero geometric ell-torsion point, K=Q(T), and let p avoid ell and the bad-reduction primes. The Galois embeddings of K are the ell²−1 nonzero vectors of E[ell]. An unramified degree-one prime of K over p corresponds to a vector fixed by Frob_p; therefore it exists iff det(I−Frob_p)=0 modulo ell, equivalently **ell divides #E(F_p)**.

Enumerating GL2(F_ell) gives the exact formula

    |GL2(F_ell)| = ell(ell−1)^2(ell+1).
    #{A in GL2(F_ell) : det(A−I)=0} = ell(ell²−2).

Proof of the second count: if the second eigenvalue a is distinct from 1, there are (ell−2) choices for a, each conjugacy class has ell(ell+1) elements. The repeated-eigenvalue-1 case contains the identity and the ell²−1 nonidentity unipotents. Total (ell−2)ell(ell+1)+ell²=ell(ell²−2). For any eligible A≠I, the 1-eigenspace has dimension exactly one, yielding **ell−1 distinct degree-one primes** of K; for A=I all ell²−1 embeddings give degree-one primes.

Chebotarev yields the natural density of good rational primes p with **at least one** such source place:

    delta_ell = (ell²−2)/((ell−1)²(ell+1)) = 1/ell + O(1/ell²).

The **unconditional expectation over the Frobenius conjugacy distribution** of the number of degree-one prime ideals is exactly one (Burnside): (ell−1)(ell(ell²−2)−1)+(ell²−1) equals |GL2(F_ell)|. This is an average count over primes, *not* an assertion that every p supplies one. Eligible rational primes have ell−1 places except for the exceptional identity Frobenius case. This distribution is a prime-selection constraint in the **fixed-E/Q, varying-p and varying-ell** research model; no claim of independent Frobenius draws or a finite cutoff is made.

The same full-image hypothesis gives the oriented global torsion field degree ell²−1 and the x-only field degree (ell²−1)/2. Explicit **dense** monic minimal-polynomial materialization therefore writes at least (ell²+1)/2 coefficient slots; a complete table of one cyclic source subgroup has ell point entries. This is a representation-size warning for a dense-field-materialization variant, not a lower bound for tower/sparse/black-box arithmetic or a wall-clock comparison with rho/IC. The two data types have different bit costs, and field construction might be amortized. In particular, CT86–91 already use an 8×3×2 tower rather than a mandatory dense degree-48 polynomial.

## CT80's actual source p=37

The public source curve is E: y²+y=x³−x²−10x−20. Direct exact count gives #E(F37)=35, trace 3, and Frob37 on E[7] has characteristic polynomial X²−3X+2=(X−1)(X−2) over F7. Exactly **six** nonzero vectors are fixed, matching CT81–82's six degree-one primes at the chosen p. The selected source place reducing T to (9,4) is one of six; choosing the source place alone does not identify CT90's binary ordinate/root or an arbitrary-input global lift.

Over F2 the same curve has five points (trace −2); CT91 handles the separate degree-24 binary residue obstruction. Source and target primes are *not interchangeable*. For ell=7, the conditional Chebotarev density of eligible source primes is 47/288, and among eligible Frobenius classes the identity is exceptional.

## Independently executable certificates

Two standalone implementations **do not import CT81–91 code**. Python enumerates every matrix of GL2(F_ell) and brute-force checks every nonzero vector; JavaScript independently determines the 1-eigenspace rank algebraically. Both count the curve points at p=37 and p=2 without using a database. Tested ell=3,5,7,11.

| ell | |GL2| | eligible matrices | distribution of degree-one-place counts (number of matrices) | Chebotarev density |
| ---: | ---: | ---: | --- | --- |
| 3 | 48 | 21 | 0:27, 2:20, 8:1 | 7/16 |
| 5 | 480 | 115 | 0:365, 4:114, 24:1 | 23/96 |
| 7 | 2016 | 329 | 0:1687, 6:328, 48:1 | **47/288** |
| 11 | 13200 | 1309 | 0:11891, 10:1308, 120:1 | 119/1200 |

From repository root:

```sh
D=research/cross_characteristic_global_torsion_20261112/ct92
python3 -B "$D/ct92_prime_density.py" >/tmp/ct92-py.json
python3 -O -B "$D/ct92_prime_density.py" >/tmp/ct92-opt.json
node "$D/ct92_prime_density_independent.js" >/tmp/ct92-js.json
node --no-opt "$D/ct92_prime_density_independent.js" >/tmp/ct92-jsopt.json
cmp /tmp/ct92-py.json /tmp/ct92-opt.json
cmp /tmp/ct92-py.json /tmp/ct92-js.json
cmp /tmp/ct92-py.json /tmp/ct92-jsopt.json
printf '%s  %s\n' f813e26c75d9a70835246b6cf1ba2d34a1bb9b2181d533c8b31453af88fc4506 /tmp/ct92-py.json | sha256sum -c
```

All four bounded local modes returned byte-identical JSON with the stated digest. No paid compute, no full Rust/Lean/Conductor run, and no independently signed scientific PCC verdict in this connector execution. GitHub exact-head CI must be inspected separately.

## Mathematical scope and follow-up

This does **not** prove impossibility of an efficient DLP-preserving reduction. Full GL2 is a hypothesis for the density formula; CT80's degree-48 and exact 37-place count are separately witnessed for that fixed ell=7 example. For fixed non-CM E/Q, Serre's open image theorem gives full mod-ell image for all sufficiently large ell, but p must then vary with ell to allow ell|#E(F_p). Chebotarev gives asymptotic densities, not a quantified prime-search cost in a finite interval.

Next: compute selective source-conditioned global root recognition with **one fixed source and target place**, charge actual number-field construction, field/rational arithmetic and target DLP against admitted baselines, and test whether tower representations avoid the dense-field obstruction. Keep the concurrent native genus-two/genus-three transfer work in cryptanalysis and autoresearcher untouched.

## Sources checked

- Internal primary experiments: CT80, CT81–82 (degree-one source primes), CT83 (height/precision), CT86–87 (8×3×2 field tower), CT90–91 (fixed binary place and target torsion), root crypto PR #3877 and stacked PR #4074; cryptanalysis #1523 and #1520, autoresearcher transfer ideas #3185/#3244. Do not treat proposals as certified claims.
- Serre's open-image implication and its assumptions as presented in: **On the effective version of Serre's open image theorem** (2024), https://doi.org/10.1112/blms.13002.
- Andrew V. Sutherland, **Computing images of Galois representations attached to elliptic curves**, Forum of Mathematics, Sigma (2016), https://arxiv.org/abs/1504.07618.
- Classical Chebotarev density theorem and elementary conjugacy-class counting; no novelty claim for either.
- **KB unavailable:** no research-KB MCP namespace and no repository-local CLI wrapper in this connector/container runtime; fallback to existing PR reports, repo AGENTS.md, dated log and external primary literature, so this is not a full KB/research-archive survey.
