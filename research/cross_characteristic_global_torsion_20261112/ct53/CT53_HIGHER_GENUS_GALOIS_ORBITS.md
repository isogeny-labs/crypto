# CT53 — Higher-genus symplectic torsion orbit obstruction

**Scientific status: [pcc:unverified].** Conditional mathematical theorem and small-field arithmetic validators. No cross-characteristic ECDLP solver or speedup is claimed. Extends CT52 [fixed-elliptic-curve degree analysis](../ct52/CT52_GALOIS_ORBIT.md), without modifying root native PR #3877.

## Conditional theorem

Let A/K be a g-dimensional principally polarized abelian variety over a number field, ell a prime, and identify the geometric ell-torsion with the symplectic space V = F_ell^(2g). **Assume** the mod-ell Galois image contains Sp_(2g)(F_ell), e.g. it is full GSp_(2g)(F_ell). For every nonzero **oriented point** P in A[ell],

    [K(P):K] = ell^(2g) - 1.

Proof: Each transvection T_(u,a)(v)=v+a<v,u>u lies in Sp(V). For nonzero v,w with <v,w> nonzero, choose u=w-v and a=<v,w>^(-1): a single transvection maps v to w. If <v,w>=0, choose an intermediate nonzero z with <v,z> and <z,w> both nonzero. Such z exists because a union of two proper linear hyperplanes never covers V, including over F_2. Two transvections then send v to w. Thus Sp acts transitively on V minus {0}; the Galois orbit of P has exactly ell^(2g)-1 elements and orbit–stabilizer identifies its size with [K(P):K].

The cyclic **line** (without a specified generator) has (ell^(2g)-1)/(ell-1) possible images. For odd ell, the unoriented pair {P,-P} has (ell^(2g)-1)/2 images. These do **not** specify the oriented ECDLP public key.

## Split case: the relevant escape hatch

For A = E1 × E2 split globally over K, a point (P,0) in the first elliptic factor has orbit at most ell²-1; if E1 has full GL2 image, exactly ell²-1. More generally a globally Galois-stable two-dimensional submodule bounds that point's orbit similarly. **A genus-two target is not automatically more expensive to globalize**: full symplectic monodromy must be established; global splitting prevents that hypothesis. A factorization of A only after reduction modulo the **source prime** need not imply splitting over K.

| ell | g=1, full GL2 | g=2, full Sp4 | g=3, full Sp6 | split elliptic factor |
|---:|---:|---:|---:|---:|
| 3 | 8 | 80 | 728 | 8 |
| 5 | 24 | 624 | 15,624 | 24 |
| 7 | 48 | 2,400 | 117,648 | 48 |
| 11 | 120 | 14,640 | 1,771,560 | 120 |

This is opposite to the necessary Hasse–Weil **second residue degree**: increasing g can make the minimum allowed target binary extension *smaller*, while for full image the **dense global point field** degree grows as ell^(2g). Neither statement is a universal bit-complexity impossibility theorem: a varying family A_(p,ell)/K_(p,ell), exceptional small-image representations, and succinct encodings are not excluded.

## Executable independent controls

From the root:

```bash
python3 -B research/cross_characteristic_global_torsion_20261112/ct53/ct53_orbits.py
python3 -O -B research/cross_characteristic_global_torsion_20261112/ct53/ct53_orbits.py
node research/cross_characteristic_global_torsion_20261112/ct53/ct53_independent.js
node --no-opt research/cross_characteristic_global_torsion_20261112/ct53/ct53_independent.js
```

Python constructs explicit paths of at most two transvections, checks all ordered nonzero source/target pairs for (ell,g)=(2,1),(2,2),(3,1),(3,2),(5,1),(7,1) and one-to-all for (3,3),(5,2); it checks pairing invariance and rejects a mutant. Independent JS **does not use the Python construction**: it reaches the full orbit by BFS under unit transvections and separately enumerates invertible 2x2 matrices for the split-factor control. Both normal and alternative interpreter modes passed local executions. Their results are **not** signed independent PCC scientific verdicts.

## Next decisive work

A scalable characteristic-changing algorithm needs to select a global low-image/split family and **compute both oriented global lifts from arbitrary encoded P,Q**, without first determining Q's logarithm. Charge field degree, coordinate heights, finding the compatible global embedding, second reduction and final solve. In the absence of an algorithm, a fixed-curve full-image obstruction cannot be generalized to all engineered families. The existing CT51 lookup has O(ell) setup/size.

## Research receipt and provenance

Prior: [CT52](../ct52/CT52_GALOIS_ORBIT.md), CT51, [second-prime lift open problem](../../../problems/lift-reduce-second-prime-transfer.md), [root PR #3877](https://github.com/isogeny-labs/crypto/pull/3877), and cross-repository GitHub search against crypto, cryptanalysis and crypto-autoresearcher. Related primary literature: C. Hall, *An open-image theorem for a general class of abelian varieties*, BLMS 43 (2011), 703–711, https://doi.org/10.1112/blms/bdr004, applies to **fixed** A/K with particular endomorphism/reduction assumptions for almost all ell. Arias-de-Reyna, Dieulefait, Wiese, *Classification of Subgroups of Symplectic Groups Over Finite Fields Containing a Transvection*, 2016, https://doi.org/10.1515/dema-2016-0012, gives big-image context, not a claim that the CT51 curve has a full higher-genus image. Shared KB MCP and local wrapper unavailable in this execution; crypto-autoresearcher AGENTS.md connector read returned UnicodeDecodeError. No novelty claim.

No paid EKS job or timing measurement was performed.
