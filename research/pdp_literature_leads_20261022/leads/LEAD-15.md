# LEAD-15 — Plane-quartic Jacobian arithmetic to unblock Diem's Õ(q) branch of the cubic-GHS transfer

**Verdict.** exponent-candidate. The card's framing is partly mis-aimed (§1.4): the practical Diem–Thomé algorithm uses about 12 class-group additions and needs no unique class forms, so the missing arithmetic is a small cost term and is not what blocks the lead.
**Confidence.** medium. Diem–Thomé measured the same line harvest on plane quartics up to `q = 2^31` and the repository's toy quartic IC passes 90/90; the two untested links are the line yield on the bielliptic GHS quartics and whether `C(F_q)` reaches the `ℓ`-part at `q ≥ 257`.
**Expected gain if it works.** Unverified desk check (§5): cost ratio to the Item-7 rho model about 1.1 at `q = 257` (unresolved), 0.25 at `2^11`, 0.085 at `2^15` with `M_ℓ = 1 M_q`; 3.3 / 1.1 / 0.40 with `M_ℓ = 9 M_q`. Exponent about 1.0, against 1.30 measured for the hyperelliptic branch and 1.5 for rho. It would move the measured `m = 3` IC-2LP row (2.60e8 at `q = 2^15`) to about 1.4e7 on instances that have a plane-quartic cover. That is a different cover type than the baseline row, so it is a new row unless an instance has both types.
**Cost of the discriminating test.** About 6–10 person-days in total, under 1 CPU-hour, under 1 GB. The harvest alone (§6 Part C, no Jacobian arithmetic) is about 3 person-days, almost all of it an `F_{q³}` instance builder at `q ≥ 257`.

## 1. Mechanism (precise)

Conventions. `q` must be odd (the `(2,…,2)` cover is odd-characteristic), so "`q = 2^8`" means `q = 257`, as in Item 7. `C/F_q` is a smooth plane quartic, `|K|` is the line system (dimension 2, `q²` divisors), `P0 ∈ C(F_q)`, `n1 = #C(F_q) = q+1` on these covers, `#Jac(C)(F_q) = #E0(K) = 4ℓ`.

**1.1 Class representation (Diem).** Call an effective `F_q`-rational divisor `D` *maximally reduced along `P0`* if `D − P0` is not linearly equivalent to an effective divisor. Every `c ∈ Cl⁰(C)` is `[D] − deg(D)[P0]` for exactly one such `D`, of degree 0, 1, 2 or 3 (Diem–Thomé §2; Diem JTNB §"Notation and representation"). Proof sketch, from Riemann–Roch (derived, matches the sources): if `h⁰(D) ≥ 2` the pencil has a member through `P0`, so a reduced `D` has `h⁰(D) = 1` and is the unique effective member of its class. For degree 3, `h⁰ = 2` happens iff `D ∈ |K − P|` for a point `P` (a line through `P`). So the special classes are exactly `{K − P : P ∈ C(F_q)}`, `n1` of them, each with `q+1` effective representatives (Diem–Thomé Lemma 6: about `q` special systems, `q²` special divisors). A special class is represented by the degree-2 residual of the line `P·P0`, and the loop continues if that still contains `P0`.

Exact counting consequences (derived; checked by arithmetic on the repository's own point counts, not by group-law code):
- `#Eff_3(F_q) = #Jac(F_q) + q·n1` (RR: `h⁰ = 1` off the special classes, `q+1` on them).
- `#reduced of degree 3 = #Jac − #Eff_2`; degrees 0, 1, 2 give `1`, `q`, `#Eff_2 − n1`.

| instance (`q`, `#Jac`, `N1,N2,N3`) | `#Eff_3` by counting closed points | `#Jac + q·n1` | `#Eff_2` | degree-3 reduced `= #Jac − #Eff_2` |
|---|---|---|---|---|
| 5, 148, (6,26,192) | 56+60+62 = 178 | 148+30 = 178 | 31 | 117 |
| 7, 356, (8,50,380) | 120+168+124 = 412 | 356+56 = 412 | 57 | 299 |
| 11, 1348, (12,122,1380) | 364+660+456 = 1480 | 1348+132 = 1480 | 133 | 1215 |
| 7†, 332, (8,50,308) | 388 | 332+56 = 388 | 57 | 275 |
| 11†, 1324, (12,122,1308) | 1456 | 1324+132 = 1456 | 133 | 1191 |

**1.2 Coordinates and the group law (Flon–Oyono–Ritzenthaler, FOR).** With a rational flex `P∞ = (0:1:0)`, tangent `z = 0`, the affine model is `y³ + h(x)y = f(x)` (`deg h ≤ 3`, `f` monic quartic). A typical class is `[u, v]`: `u` monic cubic with simple roots, `v` of degree 2, `u | F(x, v(x))`, class `[D+ − D∞]` with `D∞` an effective divisor on `l∞`. *Typical* means the three points are non-collinear (else special), none at infinity, and with distinct `x`; atypical classes have density `O(1/q)` (my estimate about `3/q`, so tens of percent at `q = 5, 7, 11`).

Addition (FOR Prop. 1, Algorithm 1): the cubic `E = y² + sy + t` through `D1+ ∪ D2+ ∪ {P1∞, P2∞, P4∞}` (9 linear conditions, `deg s ≤ 1`, `deg t ≤ 3` in the flex case), residual `D3`; then the conic `Q = y − v` through `D3 ∪ {P1∞, P2∞}`, residual `D+`, and `D1+ + D2+ ∼ D+ + D∞`. Steps: 13M+2SQ (inverse of `v1 − v2` mod `u2`), 9M, 39M+3SQ+I (cubic), 37M+5SQ (resultant), 16M+2SQ, 18M+I, 16M+3SQ: **148M + 15SQ + 2I** (Table 1). Doubling: **165M + 20SQ + 2I**. The paper's abstract and conclusion table give 163M+2I and 185M+2I, the same figures with SQ counted as M (`148+15`, `165+20`). Cost of a hyperflex: 131M+14SQ+2I and 148M+19SQ+2I. FOR remark 3: the negative of a sum is cheaper than the sum only when `P1∞ = P2∞ = P4∞`, i.e. in the flex case. Negation is not needed for double-and-add.

Khuri-Makdisi (KM) represents a divisor of degree `d ≥ 2g` as a codimension-`d` subspace of a Riemann–Roch space and includes an equality test, because the representative is not unique per class. It costs `O(g^{3+ε})`, with a precomputation, and has no `g = 3` constant. It is a fallback for no-flex instances; I would not use it for table keys.

**1.3 Transport of the `ℓ`-part.** `φ = (1 + Fr + Fr²)∘π0^*`, `π0_*∘φ = [2]`, `ker φ ⊂ E0[2]` (genus-3 build README, derived, toy-checked). With `#Jac = 4ℓ`, `φ` maps `E0(K)[ℓ]` isomorphically onto `Jac(C)(F_q)[ℓ]`. `φ(P)` is the class of a degree-6 `F_q`-rational effective divisor `Δ_P` (two Galois orbits of size 3 when `π0^*P` is `K`-rational; otherwise an ideal over `K`) minus the constant `Δ_O`. To reduce to `(D, d)`: apply FOR addition to the two orbits (each an `F_q`-rational degree-3 divisor), or take a cubic through `Δ_P` plus 3 points. Projection to the `ℓ`-part: `x_P := log_{φ(G)}(4·[P − P0])`, a homomorphism `Cl⁰ → Z/ℓ`. Only the `m = 2` covers (non-hyperelliptic) are in scope: 42/104 (`q = 5`), 115/286 (`q = 7`), 22/49 (`q = 11` sample) of the `4 | #E` curves (census types table, "m=2 only" plus "both").

**1.4 What the algorithm actually asks of the arithmetic.** Practical Diem–Thomé (DT §3, §7):
1. Factor base `F`: `N ≈ 2√q` (DT) or `⌈2.83√(q+1)⌉` (repository PROTOCOL) random points of `C(F_q)`; the rest are large primes.
2. **Harvest, no Jacobian arithmetic.** For each pair `F_i, F_j` the line meets `C` in two more points, the roots of `A s² + B st + C t²` (polarisations of the quartic). The line is *PP* if both are large primes, *FP* if one is, *Full* if none. Relation `x_a + x_b + x_c + x_d = h`, with `h = log(H − 4P0)`. DT proved: lines with 4 distinct rational points `∼ q²/24` (Prop. 14), PP relations `E ∼ q`, FP `∼ (4/3)q^{1/2}` (Prop. 9, 20).
3. Graph on large primes `∪ {∗}`. Recombine along cycles through `∗` and even cycles, solve a sparse `N × N` system mod `ℓ`.
4. **Descent: the only class arithmetic.** Compute `a = φ(G)`, `b = φ(T)` in `(D, d)` form, then `α a + β b` for random `α, β` until `D` splits into 3 rational points that are in `F ∪ giant component`. The relation `Σ x_{P_i} = α + β k` gives `k`. DT §2: "for a practical variant of our algorithm we only need about 12 additions in the degree 0 class group in total".

Reconciliation (desk): success per trial `≈ (1/6)·ρ³`, `ρ` the covered fraction of points, `ρ ≈ 0.8` for `N = 2√q` (so `≈ 12` trials, DT's figure) and `≈ 0.99` for `N = 2.83√q` (about 6). Plus `≈ 3 log₂ ℓ` additions for the first `αa + βb`, about 70 at `q = 257` and about 130 at `2^15`.

**Consequences.** (i) Unique class forms are never needed, because the algorithm never tabulates or compares classes; any valid `D` in the class gives a valid relation. Atypical outputs (`O(1/q)`) can be rejected (Las Vegas), so no Gröbner or Hermite-normal-form fallback is required for the DLP. The card's second kill condition does not bind. (ii) The card's `10³` kill line is sharp only at `q = 257`: 70 ops at 10³ is 70k, comparable to the whole rho cost there; by `q = 2^11` it is irrelevant. (iii) The *provable* algorithm (Diem JTNB 2012, Prop. 11) differs: it uses only tree relations and needs `#F + 2` target relations at success `≈ q^{-1/2}/24` each, so `≈ 48 q` class additions. At 181 M per op that is about `8.7e3·q` mults, no better than rho at `2^15` (compare 1.6e8 at `2^15`). The proved theorem is therefore not the competitive variant; the heuristic practical one is.

**Degree-of-regularity analogue.** There is no Gröbner step. The bound is combinatorial: PP relations `∼ N²·(1/4)` need `N²/4 ≳ q`, so `N ≈ 2√q`, relation work `∼ N²/2 ∼ 2q` lines, linear algebra `∼ 2N²w`. It comes from Diem's `q^{2 − 2/(d−2)}` at `d = 4` (heuristic with double large primes, with the one heuristic assumption DT Assumption 12).

## 2. Prior art: literature

- Diem, "Index Calculus in Class Groups of Plane Curves of Small Degree", ECC 2005 slides, `https://cr.yp.to/conferences/2005-ecc/www.cacr.math.uwaterloo.ca/conferences/2005/ecc2005/diem.pdf`. **read-at-source** (HTTP 200 by curl; first WebFetch was 502). Claims: lines through two factor-base points; `Õ(q^{2−2/(d−2)})` with double large primes (`Õ(q)` at `d = 4`), `Õ(q^{2−2/(d−1)})` without; Thomé's runs at `q = 2^31`, FB about 88,000, one day.
- Diem, "An index calculus algorithm for plane curves of small degree", ANTS VII, LNCS 4076 (2006) 543–557, doi 10.1007/11792086_38. **abstract-only** (search-result abstract; Springer page redirects to a login; ePrint 2005/119 not fetched). Claim: heuristic `Õ(q^{2−2/(d−2)})` for `d ≥ 4`.
- Diem–Thomé, "Index calculus in class groups of non-hyperelliptic curves of genus three", J. Cryptology 21(4) (2008) 593–611, doi 10.1007/s00145-007-9014-6; `https://hal.inria.fr/inria-00107290/document` (HTTP 200, 19 pp). **read-at-source** (full text). Claims: algorithm; Props. 9, 14, 19, 20; Table 1 (graphs for `q = 2^19–2^24` match Bernoulli graphs); practical variant: 86 field mults per PP relation, 3.4 µs per pair, `87,803 × 87,803` matrix with 352 entries per row, block Wiedemann about a day, rho about `10^5×` more work at `q = 2^31`, "about 12 additions".
- Diem, "On the discrete logarithm problem for plane curves", JTNB 24(3) (2012) 639–667, doi 10.5802/jtnb.815; `https://jtnb.centre-mersenne.org/item/10.5802/jtnb.815.pdf` (HTTP 200, 30 pp). **read-at-source** (title page through the start of §3.3; the analysis of Prop. 18 not read). Claims: Theorem 1 (proven `Õ(q)` for non-hyperelliptic genus 3, expected time); Theorem 2; reduced-along-`P0` representation; Prop. 11 (provable version, group-operation count above). **This corrects the census README**, which calls `Õ(q)` "a heuristic of Diem–Thomé, not a theorem".
- Diem, "Index calculus with double large prime variation for curves of small genus with cyclic class group", arXiv math/0606607. **read-at-source** (abstract and §1): `Õ(q^{2−2/g})` for plane models of bounded degree. Whether it is the preprint of Math. Comp. 80 (2011) 443–475 is not verified (AMS PDF returned 403).
- Flon–Oyono–Ritzenthaler, "Fast addition on non-hyperelliptic genus 3 curves", CORR 2007-10, `https://uwaterloo.ca/combinatorics-and-optimization/sites/default/files/uploads/documents/2007_10.pdf` (HTTP 200, saved and read with `pdftotext`). **read-at-source** (all of §§1–5, Tables 1–2). Claims: Prop. 1, Algorithm 1, counts of §1.2. Theorem 1 (a rational line meeting `C` in 4 rational points if `q ≥ 127`) is *announced*; its proof (their ref. [9]) was not read. Published as *On Number Theory and its Applications* (2008) per a HAL listing: **recalled**.
- Khuri-Makdisi, Math. Comp. 73 (2004) 333–357, arXiv math/0105182. **abstract-only**: `O(g⁴)` per operation.
- Khuri-Makdisi, Math. Comp. 76 (2007) 2213–2239, arXiv math/0409209. **read-at-source** (abstract, §1 through the representation paragraph): subspace representation, `d ≥ 2g`, equality test, `O(g^{3+ε})`.
- Abu Salem–Khuri-Makdisi, LMS J. Comput. Math. 10 (2007) 307–328, arXiv math/0610121. **abstract-only**: 117M+2I add, 129M+2I double for `C_{3,4}`.
- Tian, "Cover attacks for elliptic curves with prime order", J. Cryptology 36 (2023), arXiv 2012.07173. **abstract-only**: an `(ℓ,ℓ,ℓ)`-isogeny from the cubic Weil restriction to a genus-3 Jacobian, some prime-order groups, `Õ(q)`. If its codomain is a plane quartic the same arithmetic applies; I did not check.
- Hess, J. Symb. Comput. 33 (2002) 425–445; Hartshorne IV. **recalled** (cited by DT for reduced divisors and RR).
- Oyono's thesis: not located (no search beyond the queries below).

## 3. Prior art: these repositories

- `problems/plane-quartic-jacobian-class-arithmetic-missing.md`: status `resolved` for toy smokes; deg-3 canonical forms not unique; dlog keyed by deg-6 orbit pairs. Supports; shows the non-uniqueness is of the search-based `pic_canon`, which pads with `Ω = P0+P1+P2` and enumerates candidates (`quartic_ic.rs` 1292–1500).
- `research/ghs_cubic_quartic_ic_20261012/README.md`, `PROTOCOL.md`: 90/90 correct at `q ∈ {5,7,11}`, no IC-vs-rho ratio, "dense FB-only line IC does not reach the `ℓ`-part on toys". Supports; the `ℓ`-part observation is a toy-size artefact to be tested at `q ≥ 257` (§6 K3).
- `research/ghs_cubic_genus3_build_20261012/README.md`: quartic equation `G(X²,Y²,Z²) = 0` (bielliptic over `K`, `Aut ⊇ (Z/2)²`, involutions permuted by Frobenius), `L(T) = 1 − t_E T³ + q³T⁶`, `N1 = q+1`, `N2 = q²+1`, `ker φ ⊂ E0[2]`. Supports; this is a special quartic family (§5).
- `research/ic_next10_20261014/07-cubic-ghs-genus3/README.md`, `PROTOCOL.md`: hyperelliptic branch, `q^{1.300}` IC-2LP, rho model 1.61e8 and IC 2.60e8 at `q = 2^15`, rho 74.5 M per step, LA share 17 → 48%, `M_ℓ = 9` sensitivity, plane-quartic branch "BLOCKED". Supports; supplies the instrument and the rho reference.
- `research/ghs_cubic_hyperelliptic_genus3_20261013/README.md`, `research/ghs_cubic_cover_census_20261012/README.md`: type counts; Cantor transfer; the census `Õ(q)`-is-heuristic sentence (now outdated, §2).
- `problems/ghs-to-smith-pipeline-characteristic-mismatch.md`, `/home/user/ml-cryptanalysis/experiments/smith-codomain/report.md`: odd-characteristic only; line-relation yield `f²/2` on random and Smith-codomain quartics (0.00496 vs 0.00499 at `f = 0.1`). Unrelated to the `m = 2` family, but supports the yield law on non-GHS quartics.
- `src/cryptanalysis/hyperelliptic_index_calculus.rs` header: Gaudry–Harley walk, one Cantor op per candidate. Unrelated (no lines), a contrast.
- `AGENTS.md` §5a: the pricing band. `weak_curve_database_20261012/REPORT.md` records Tian (abstract level).

## 4. Why it could lower the cost

The hyperelliptic branch needs about `q^{1.14}` genus-3 group operations (relations come from walks); the quartic harvest needs none: about `2q` line evaluations at 43–60 `F_q` mults each. DT measured 86 mults per PP relation at `q = 2^31`, which is 43 per pair at PP yield 1/2. The group-law count is about `3 log₂ ℓ + 12`, order 10², independent of `q` apart from `log q`. Assumptions: (a) line yield on the GHS quartics matches `1/24` (S₄ monodromy, Diem–Thomé Prop. 14; any smooth quartic in odd characteristic, error `O(q^{-1/2})` with a constant unknown at `q = 257`); (b) DT Assumption 12 (giant component); (c) the factor base reaches the `ℓ`-part; (d) sparse LA weight `w ≈ 12` at `N ≈ 500` (Item 7 implies about 15 at `N = 2049`).

## 5. Obstructions

- **Generic bounds.** The rho reference, and any oracle built from `E0` group operations, sit on the Shoup / Corrigan-Gibbs–Kogan line at `√ℓ ≈ q^{3/2}/2`. The harvest is not such an oracle: it uses the 2-dimensional line system `|K|`, `q²` divisors in one class. That is non-generic structure, not enumeration in disguise (cost `q^{1+o(1)} < √ℓ`). The descent is generic and priced. I did not check compatibility with CGHW 2026/384 beyond this.
- **Proved vs practical** (§1.4 iii). The theorem needs `Õ(q)` additions; the competitive variant is heuristic.
- **Special family.** The GHS quartic is bielliptic with `N1 = q+1`, `N2 = q²+1`. The full-splitting fraction `1/24` follows from S₄ monodromy for any smooth quartic, but the Weil-bound constant and the giant-component behaviour are unmeasured on this family. The Smith-codomain yield test was on other quartics.
- **`ℓ`-part reach.** Needs `C(F_q)` classes to have a nonzero `ℓ`-component; fine in the hyperelliptic analogue at 2^8–2^15, observed to fail at `q ≤ 11`, untested for quartics at scale. Test: nullity of the relation system mod `ℓ` is exactly 1.
- **Pricing.** At `M_ℓ = 9 M_q` the advantage shrinks (ratios 3.3 / 1.1 / 0.40 below); `ℓ < 2^64` for `q ≤ 2^20`, so `M_ℓ = 1` is the primary unit, as in Item 7. Against an ideal negation-map rho multiply every ratio by 1.41. §5a: lookups (4 M each, PROTOCOL pin), hashing and graph search must be listed for both arms and priced at 0, 0.1, 1 GAE; a ranking holds only where the order is stable across the band. At `q = 257` the order is expected to flip inside the band.
- **Supply.** `m = 2` covers only (about 40% of `4 | #E` classes at `q = 5, 7, 11`); `ℓ` prime with cofactor 4; a rational flex for the 163M/185M formulas (probability 0.63 per quartic, FOR conjecture), else a line with 4 rational points (guaranteed for `q ≥ 127`, announced) and a higher cost, not tabulated (my guess 300–1500 M, unverified). No quartic exists in the repository above `q = 11`: the builder uses table fields.

**Desk checks (unverified; scratchpad script, not committed).** (1) A harvest at `q = 257` on 6 random smooth quartics with `N = 46`: 1035 pairs, split-completely 0.46–0.53 per pair, PP 0.31–0.37, FP 0.13–0.18, Full ≤ 0.02, giant component ≥ 0.99 of large-prime vertices, non-tree edges 260–328 (≥ 5N); with `N = 33` the covered fraction was 0.80. (2) Cost model, `c_line = 50`, `w = 12`, `c_I = 9`:

| `q` | `N` | harvest | LA `2N²w` (dense cap `N³/3`) | other (FB, lookups, transfer, descent) | total, `M_ℓ = 1` | total, `M_ℓ = 9` | rho model (Item 7) | ratio (1 / 9) |
|---|---|---|---|---|---|---|---|---|
| 257 | 46 | 52k | 32k | 39k | 123k | 379k | 1.16e5 | 1.06 / 3.3 |
| 2053 | 129 | 413k | 399k | 99k | 0.91M | 4.1M | 3.62e6 | 0.25 / 1.13 |
| 32771 | 513 | 6.6M | 6.3M | 0.75M | 13.7M | 64M | 1.61e8 | 0.085 / 0.40 |

Central values; I would not trust them closer than a factor 2–3.

## 6. Minimal discriminating test (frozen protocol)

Seed 20261023; `q` = next prime above `2^b`; unit = `F_q` mult equivalents with `c_I = ⌈log₂ q⌉`, sqrt `⌈1.5 log₂ q⌉`, lookup 4, `M_ℓ ∈ {1, 9}` (Item 7). No judgement calls once Stage 0 exists; Stage 0 is an engineering task for the director tier.

**Part A: counting identities (runnable now, no new arithmetic).** On the five toy quartics of `ghs_cubic_genus3_build_20261012` (including both addendum curves) count `Eff_2` and `Eff_3` by brute-force enumeration of closed points of degrees 1–3. PASS iff `#Eff_3` equals 178, 412, 1480, 388, 1456 and `#Eff_3 = #Jac + q(q+1)` in each. Kill K5: any mismatch (the RR picture is wrong).

**Part B: arithmetic units (Stage 0, about 3–5 person-days, Rust).** Implement FOR's flex-model addition and doubling plus reduction to `(D, d)` and the 6 → 3 map for `φ`. Run at `q ∈ {5, 7, 11}` (all five curves; atypical paths use the rejecting variant or a Hermite-normal-form fallback, whichever the implementer builds) and at `q ∈ {101, 257}` on the typical path:
- B1: over all effective `D` of degree ≤ 3 the canonical outputs number exactly `#Jac` (148, 356, 1348, 332, 1324) with `1, q, #Eff_2 − n1, #Jac − #Eff_2` per degree (table of §1.1).
- B2: 100 random classes: `#Jac·c = 0`; 100 random triples associative; commutativity; inverse; `ℓ·φ(G) = 0`.
- B3: `φ(P+Q) = φ(P) + φ(Q)` on 400 random pairs per curve, cross-checked against the repository's `pic_jacobian` on the 11 curves.
- B4: counters of M, SQ, I per addition and doubling at `q = 257`, 1000 random typical pairs. **Kill K4:** mean cost `> 10³` M-equivalents. Prediction 181 / 203 (c_I = 9); in addition the measured counter totals must lie within 10% of 163M+2I and 185M+2I.

**Part C: harvest on real GHS quartics at `q = 257` (Stage 0 builder, about 2–3 person-days; needs no Part B).** Instances: 6 curves, `m = 2` cover, `4ℓ = #E0(K)` with `ℓ` prime, `N1 = q+1` checked, smoothness from the construction's six distinct roots; positive control: 6 random smooth quartics over `F_257` by the same code; A/A: two disjoint FB draws per curve; `N = ⌈2.83√(q+1)⌉ = 46`, all 1035 pairs. Record: split / FP / PP / Full per pair, giant fraction, usable cycle rank, rank and nullity of the recombined system mod `ℓ`, counters. Prediction: split `0.50 ± 0.05`, PP `0.33 ± 0.05` per pair, giant `≥ 0.95`, usable cycles `≥ N + 10`, nullity exactly 1. **Kills:** K1: mean PP per pair on GHS quartics `< 0.20`, or below the random-quartic control by more than 0.10; K2: usable cycle rank `< N + 2` on more than one curve, also at `N = 64`; K3: nullity `≠ 1` (the `ℓ`-part is not reached by `C(F_q)` at this size, so the route is closed for the family at `q = 257`).

**Part D (follow-on, only if A–C pass): end to end.** Same instance generator at `q ∈ {257, 1031, 4099, 16411, 32771}`, 3 curves per size, planted `k`, descent by Part B arithmetic (and by the cubic-pencil descent as a second oracle), answer verified, Lanczos on the normal equations, counters in the unit above, fit of `log cost` on `log q` with a bootstrap over curves, against the Item-7 rho model on the same `ℓ`. POSITIVE iff all answers verify and `S_IC/S_rho_model < 1` at `q ≥ 2^11` for both `M_ℓ`, and the §5a band does not flip the order. Part D is a PR-sized job, not a mechanical-tier run.

## 7. Ranking inputs

expected gain: exponent 1.30 → about 1.0 (desk), constant 10–20× at `q = 2^15` on quartic-cover instances | confidence: medium | cost: 6–10 person-days (3 for the harvest alone) | tier: exponent | depends on: an `F_{q³}` quartic builder at `q ≥ 257` (new), FOR flex-model arithmetic (optional for the first measurement, worth about 5× at `q = 257`), matched rho from Item 7. Related: Tian's isogeny covers if their codomain is a plane quartic (prime-order cases, not checked here).

## 8. Open questions

- DT Assumption 12 on the GHS quartics, and the Weil-bound constant of the `1/24` split fraction at `q = 257`: not settled from here.
- Whether `C(F_q)` reaches the `ℓ`-part at `q ≥ 257` (K3).
- Sparse LA weight `w` at `N ≈ 500`; DT's 352 per row is at `N = 87,803` after selecting the lightest rows. My `w = 12` is an extrapolation of Item 7.
- The no-flex group law: FOR's formulas assume a rational flex. Their general-line path (Theorem 1, `q ≥ 127`) is announced and its proof was not read; Khuri-Makdisi has no `g = 3` count here. Whether HAL's FOR record or Oyono's Magma program (`royono/Quartic.html`, 403 from here) could settle this: not reachable.
- Whether the "12 additions" holds with the repository's descent (reconciled by `6/ρ³` only), and compatibility with the structured generic-group bound (CGHW 2026/384): not analysed.
- The `m = 2` cover supply beyond `q = 11` (census exhaustive only at `q = 5, 7`).
- Not read: Diem's ANTS paper, JTNB §3.3 onward, FOR [9], Oyono's thesis.
- Script caveat: the harvest check in §5 used random quartics and brute-force line enumeration, not the GHS family.

## 9. Sources

URLs fetched (HTTP outcome):
- `https://arxiv.org/abs/math/0105182` 200; `https://arxiv.org/abs/math/0409209` 200; `https://arxiv.org/pdf/math/0409209` 200 (curl, read §1); `https://arxiv.org/pdf/math/0105182` 200 (curl, not read).
- `https://arxiv.org/abs/math/0610121` 200 (abstract).
- `https://uwaterloo.ca/combinatorics-and-optimization/sites/default/files/uploads/documents/2007_10.pdf` 200 (WebFetch returned binary; saved copy read by `pdftotext`).
- `https://cr.yp.to/conferences/2005-ecc/www.cacr.math.uwaterloo.ca/conferences/2005/ecc2005/diem.pdf` 502 (WebFetch) then 200 (curl).
- `https://doi.org/10.1007/11792086_38` 302 → `link.springer.com` 303 → `idp.springer.com` login; not followed. ePrint 2005/119 not fetched (Cloudflare class).
- `https://jtnb.centre-mersenne.org/articles/10.5802/jtnb.815/` 200; `https://jtnb.math.u-bordeaux.fr/item/10.5802/jtnb.815.pdf` 404; `https://jtnb.centre-mersenne.org/item/10.5802/jtnb.815.pdf` 200.
- `https://arxiv.org/pdf/math/0606607` 200.
- `https://hal.inria.fr/inria-00107290/document` 200 (19 pp); `https://hal-univ-tlse3.archives-ouvertes.fr/INRIA/inria-00107290v1` and `https://hal.inria.fr/inria-00107290` returned Anubis access-denied pages.
- `https://www.ams.org/mcom/2011-80-273/S0025-5718-2010-02281-1/S0025-5718-2010-02281-1.pdf` 403; `https://www.math.uni-leipzig.de/~diem/preprints/small-degree-exact.pdf`, `.../english.html`, `.../genus-3.pdf`, `.../cover-attacks.pdf` 404; `https://www.iacr.org/archive/eurocrypt2008/49650162/49650162.pdf` 403; `https://www.math.uwaterloo.ca/~royono/Quartic.html` 403.
- `https://arxiv.org/abs/2012.07173` 200 (abstract); `https://arxiv.org/pdf/1807.05270` 200, not read.
- Web searches (3 on Diem / FOR / Diem–Thomé, 1 on Diem–Scholten): used for metadata only.

Repository paths read: `/home/user/crypto/research/pdp_literature_leads_20261022/BRIEF_SONNET.md`, `LEADS.md` (standing constraints, LEAD-15), `leads/LEAD-09.md` (format only); `/home/user/crypto/problems/plane-quartic-jacobian-class-arithmetic-missing.md`; `.../ghs-to-smith-pipeline-characteristic-mismatch.md`; `/home/user/crypto/research/ghs_cubic_quartic_ic_20261012/{README.md,PROTOCOL.md,quartic_ic.rs (lines 1270–1540)}`; `.../ic_next10_20261014/07-cubic-ghs-genus3/{README.md,PROTOCOL.md}`; `.../ghs_cubic_hyperelliptic_genus3_20261013/README.md`; `.../ghs_cubic_cover_census_20261012/README.md`; `.../ghs_cubic_genus3_build_20261012/README.md` (lines 1–140); `/home/user/crypto/src/cryptanalysis/hyperelliptic_index_calculus.rs` (header); `/home/user/crypto/AGENTS.md` (§5a); `/home/user/ml-cryptanalysis/experiments/smith-codomain/report.md`; `weak_curve_database_20261012/REPORT.md` (grep for Tian). Scratch only (not in the repository): `/tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/desk/harvest.py`, downloaded PDFs under `.../scratchpad/dl/` and `.../scratchpad/for2008/`.
