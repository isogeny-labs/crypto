# CT56 — Serre point-count gate for global prime-to-binary torsion

**Scientific status:** [pcc:unverified]. Classical sharp bound and an explicit application to the CT55 construction; **not** a new cryptanalytic algorithm or a lower bound on general ECDLP complexity.

## 1. Universal obstruction, without Galois-image hypotheses

Let A/K be a g-dimensional abelian variety with good reduction at a place w over 2, residue field k_w = F_(2^f). Let ell be an **odd prime** and let T ∈ A(K) be of exact order ell. Good reduction and ell ≠ 2 make reduction injective on ell-torsion, so ell divides #A(k_w).

Define q=2^f and

    B(f) = q + 1 + floor(2 sqrt(q))
         = 2^f + 1 + isqrt(4*2^f).

Serre's sharp improvement of the Weil bound for any abelian variety of dimension g yields

    #A(F_q) <= B(f)^g.

Consequently **ell <= B(f)^g**, with no assumption on fixed global A, generic monodromy, simplicity, or a selected Galois image. This bound applies to families A_(p,ell) varying with the source instance as long as the stated dimension, global rationality over K, good reduction and residue-field conditions are met.

Since f <= [K:Q] for a place of K above 2, this is also a necessary dimension-versus-global-degree tradeoff. At K=Q, necessarily f=1, hence

    ell <= 5^g.                                             (CT56-A)

For ell=2^127-1, a Q-rational odd torsion point on a good-at-2 abelian variety requires g >=55. For ell=2^255-19, it requires g >=110. These are **necessary** dimensions, not constructions. In particular, a globally rational lift through any fixed genus 1–4 is impossible at such orders, no matter how adaptively the global curve is selected. This is a narrow but unconditional obstruction to one form of the proposed cross-characteristic route.

### One-page derivation of the Serre bound

Write the reciprocal pairs of Frobenius eigenvalues as alpha_i and bar(alpha_i), each of absolute value sqrt(q), and put x_i=-(alpha_i+bar(alpha_i)), so x_i ∈ [-2sqrt(q),2sqrt(q)] and #A(F_q)=product_i(q+1+x_i).

Let m=floor(2sqrt(q)). The x_i are roots of an integer polynomial (the real Weil polynomial), so product_i(m+1-x_i) is a positive nonzero integer: each factor is positive because m+1>2sqrt(q), and the product is Galois invariant and integral. Its value is therefore >=1. Applying AM-GM to the positive factors gives sum_i(m+1-x_i) >= g. Therefore sum_i x_i <= gm. Applying AM-GM again to the positive factors q+1+x_i gives #A(F_q) <= (q+1+m)^g. This reproduces Serre's standard result, not a new theorem.

At q=2, m=2, B(1)=5. The bound 5^g is sharp for **arbitrary abelian varieties**, since E/F2: y²+y=x³+x has five rational points and E^g has 5^g. The product need not be the Jacobian of a smooth genus-g curve, and having group order 5^g does **not** supply a large odd-prime-order point.

### 1.1. Sharper, Jacobian-specific bound in genus two over F2

The universal abelian-surface bound 5²=25 is **not attainable by a smooth genus-two Jacobian over F2**. The exact maximum is **19**, from the complete genus-two Jacobian classification in Aubry–Haloui–Lachaud, §4 Theorem 4.1, q=2 special case. This is previously published mathematics.

A concrete smooth witness is the weighted hyperelliptic equation

    C/F2: y² + (x³+x+1)y = x⁵+x.

Its affine nonsingularity follows from gcd(h, h'²*f + f'²)=1 over F2[x], with h=x³+x+1 and f=x⁵+x. The weighted chart at infinity has equation v²+(1+u²+u³)v = u+u⁵, so at u=0 there are **two** distinct F2-rational nonsingular points v=0,1 (the v-derivative equals 1). Independent direct F2 and F4 enumerations give

    #C(F2)=6,   #C(F4)=6.

The Frobenius trace sums are s1=3−6=−3 and s2=5−6=−1, hence L_C(T)=1+3T+5T²+6T³+4T⁴. Evaluating at T=1 gives

    #Jac(C)(F2)=19.

Thus an ell-torsion point **in a genuine genus-two Jacobian over F2** can only have odd prime order ell<=19, improving the general g=2 bound ell<=25. The point-count witness itself does not construct a global number-field torsion lift from arbitrary source P,Q.

The bounded exact census independently checks smooth weighted degree-5/6 binary models with h of degree at most 3 and f of degree 5 or 6. It finds 640 passing models, maximum group order 19, and the explicit witness above. **This finite model sweep is not offered as an independent exhaustive classification of all genus-two curves**; the global upper bound comes from the cited theorem. Both Python and JavaScript implement F4 arithmetic and point counting separately. An earlier JS infinity-count expression had incorrect operator precedence, detected by the independent witness test and corrected before commit. Wrongly counting only one of the two rational infinity points is an additional rejected control.

Reproduce:

    python3 -B research/cross_characteristic_global_torsion_20261112/ct56/ct56_genus2_witness.py
    python3 -O -B research/cross_characteristic_global_torsion_20261112/ct56/ct56_genus2_witness.py
    node research/cross_characteristic_global_torsion_20261112/ct56/ct56_genus2_independent.js
    node --no-opt research/cross_characteristic_global_torsion_20261112/ct56/ct56_genus2_independent.js

Following a defect fix replacing Python `assert` statements with unconditional `require` checks, all four local runs produced the same exact JSON: {"max_j":19,"n1":6,"n2":6,"smooth_models":640,"witness_f":34,"witness_h":11}.

Python witness source SHA-256: 0ea9b7be053fb4618b0a006a770e92b2ceb1c3a2fb780c339c05515d30717c4e.
JavaScript witness source SHA-256: c0cf9320de69b60de0981998114a359088a81a5af2d5e99f78db4f9247e32b36.

[pcc:unverified]

## 2. A stronger, dimension-independent gate for entire level structures

Suppose additionally that *all* A[ell] are rational over F_(2^f). Then #A(F_(2^f)) is divisible by ell^(2g). Thus

    ell^(2g) <= B(f)^g, hence ell² <= B(f).             (CT56-B)

This necessary bound is independent of g. If A has a polarization of degree prime to ell (in particular, A is **principally polarized**, as a Jacobian is), a rational symplectic ell-torsion basis has a nontrivial Weil pairing in mu_ell(F_(2^f)). Therefore

    2^f = 1 (mod ell), hence ord_ell(2) divides f.       (CT56-C)

Do not apply the Weil-pairing condition to an arbitrary one-dimensional cyclic subgroup or to a higher-dimensional A without the stated polarization/nondegeneracy hypothesis. In particular, rational points supported on an isotropic subgroup need not satisfy it.

CT56-B/C are not sufficient for full rational torsion: for ell=5, the necessary gates first allow f=4, but CT55's fixed E0:y²+y=x³ has #E0(F16)=9 and Frobenius F⁴=[4] ≠ identity on E0[5]. That particular curve first has full 5-torsion at f=8 (CT55).

## 3. Exact enumerated necessary lower bounds

Each entry is the least **integer** dimension g or residue degree f not ruled out by CT56-A, using exact integer square roots. These are not actual curve constructions and not solving-cost estimates.

| Tested integer n | min g, f=1 | min g, f=2 | min g, f=4 | min g, f=8 | min f, g=1 | min f, g=2 | min f, g=3 | min f, g=4 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 2^127 - 1 | 55 | 41 | 28 | 16 | 127 | 64 | 43 | 32 |
| 2^255 - 19 | 110 | 81 | 55 | 32 | 255 | 128 | 85 | 64 |

Minimum f for a **full principally polarized ell-level structure**, from CT56-B/C only (not sufficient):

| ell | 3 | 5 | 7 | 11 | 13 | 17 | 19 | 23 | 31 | 43 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| least f not excluded | 2 | 4 | 6 | 10 | 12 | 8 | 18 | 11 | 10 | 14 |

The order values are bounded input integers; the scripts do not establish primality or an ECDLP instance at those sizes.

## 4. Consequences for the user-requested transfer

1. A low-genus lift with an **ell-torsion point globally rational over Q** and good reduction at 2 cannot handle growing cryptographic ell. This conclusion **does not require** CT52/53's full Galois image assumptions.
2. Allowing a number field K with residue degree f > 1, or allowing g to grow, bypasses the narrow Q-rational/f=1 obstruction, but incurs the necessary size tradeoff ell <= B(f)^g. Charging representation and arithmetic costs is still essential.
3. Demanding all ell-torsion rational at the binary destination is much more restrictive than transporting a single cyclic subgroup. Choosing a high-dimensional Jacobian does not relax the full-level ell² <= B(f) gate.
4. Weil restriction Res_(F_(2^f)/F2)(A) has dimension g*f and the same rational point group. Thus rewriting an extension-field target over F2 does not erase the dimension increase; **no running-time lower bound** follows from this fact alone.
5. None of these bounds rules out an efficient encoded-input single-point transfer with g≥2, f>1 and appropriately low-image global torsion. That is the remaining target.

## 5. Executable checks, independent replay and negative controls

Run from repository root:

    python3 -B research/cross_characteristic_global_torsion_20261112/ct56/ct56_serre_gate.py
    python3 -O -B research/cross_characteristic_global_torsion_20261112/ct56/ct56_serre_gate.py
    node research/cross_characteristic_global_torsion_20261112/ct56/ct56_independent.js
    node --no-opt research/cross_characteristic_global_torsion_20261112/ct56/ct56_independent.js

The Python program uses arbitrary-precision integers and math.isqrt; the JavaScript program independently implements BigInt integer square root, modular exponentiation, minimality checks, and direct F2 point enumeration. All four local executions passed. Within-language normal/alternate outputs were byte-identical. Cross-language outputs agreed after normalizing only tuple/list printing. Deliberate non-sufficiency control: the universal ell=5,f=4 admission is rejected as a certificate about CT55's particular curve using independent Frobenius computations.

Python source SHA-256: 8c28fa0a16cd6339226a11f40fd098344da15e9e11a2ad372967133378aaf808.
JavaScript source SHA-256: 924cde43db08ad302aa40c04dcc601ef15643abe1f151d564fea10820c4414af.

**No exact-head CI or independent PCC signature is asserted**; producer arithmetic replay is evidence, not signed verification. No cloud compute, timing comparison or leaderboard change.

## 6. Literature and governance receipt

- **Primary source**: Aubry, Haloui, Lachaud, *Number of points on abelian and Jacobian varieties over finite fields*, §2.1 Corollary 2.2 (Serre bound), https://arxiv.org/abs/1201.4967 ; see also https://www.numdam.org/articles/10.1016/j.crma.2012.10.001/.
- **Prior lab work**: CT51–55 in the parent directory, root crypto PR #3877, cross-characteristic crypto PRs #4012/#4015 and the problem record problems/lift-reduce-second-prime-transfer.md. The CT55 universal CRT construction is **not** modified here.
- **Shared KB**: research-KB MCP not exposed in this runtime; repository wrapper unavailable without the lab checkout. GitHub cross-repo content searches for rational lift, Serre, f*g degree and CT55 reviewed instead. No novelty assertion.
- **Conductor / EKS**: conductor command and EKS authenticated workspace not available. Only local bounded exact arithmetic was run; no unapproved spending.
- **Next discriminating obligation**: for fixed bounded genus 2–4, construct a global K-point of order ell from arbitrary encoded source P,Q with [K:Q], residue f, heights and point-specific reduction explicitly bounded; compare its full cold cost against source rho. The present theorem only eliminates the Q-rational variant.

[pcc:unverified]
