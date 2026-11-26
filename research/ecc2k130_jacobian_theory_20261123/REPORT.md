# ECC2K-130: principal polarization and the missing Jacobian

Date: 2026-11-23. Base: crypto 5403e9ab890e9b9dc92fa67c661ab7df0ea3c860.
Status: written mathematical deductions and producer arithmetic replay; not independently reviewed or signed PCC verified. All conclusions below carry [pcc:unverified]. No curve or DLP transfer was constructed.

## Findings

1. The F2-isogeny class of the dimension-130 trace-zero factor A admits a principal polarization. An explicit inert residue prime certifies the hypothesis of Howe's finite-field criterion. Thus principal polarizability does not exclude a genus-130 Jacobian. [pcc:unverified]
2. A genus-130 Jacobian in this class cannot have the natural primitive 131st root of unity as an integral F2-endomorphism. In particular, its F2-endomorphism ring cannot be the maximal order. A rational endomorphism in End^0 is not enough to invoke Torelli. [pcc:unverified]
3. The residue witness also shows that zeta is not in the universal Frobenius order Z[pi,2/pi]. Therefore the usual automatic-root-of-unity obstruction cannot rule out the entire isogeny class. [pcc:unverified]
4. Neither these deductions nor existing constructions settle existence in genus 130–300. For genus greater than130, the requirement is J(C) isogenous to A times B, with dim B=g-130, rather than J(C) isogenous to A. The integral-symmetry obstruction above is scoped to the whole-Jacobian genus130 case.

## Exact mathematical target

E0/F2 has equation y²+xy=x³+1 and trace -1. Put alpha²+alpha+2=0, beta=-1-alpha, so alpha*beta=2. Let zeta be a primitive131st root of unity in characteristic zero. The source is A=ker(Tr:Res(F_(2^131)/F2)(E0)->E0)^0; its dimension is130 and its Frobenius eigenvalues are alpha*zeta^j, beta*zeta^j, 1<=j<=130.

The inherited source registry slug is icv1-f2m131-tm22283658519494248867-115e0dc5, field modulus 0x800000000000000000000000000002007, subgroup order r=680564733841876926932320129493409985129, elliptic cofactor4. The registry's generator binding remains unresolved in the cited source; no point coordinates have been invented. For this theory calculation no field-basis conversion or challenge point is used.

The field K=Q(alpha,zeta) has degree260: Q(sqrt(-7)) and Q(zeta) are disjoint, since their ramified primes are disjoint. For pi=alpha*zeta, pi^131=alpha^131 is nonrational. Indeed alpha/beta is not a root of unity (ordinarity); rationality would imply (alpha/beta)^131=1. Thus pi generates K, and its degree260 minimal polynomial gives the simple ordinary F2-isogeny class A. Its endomorphism algebra over F2 is the CM field K. Over an algebraic closure it is isogenous to E0^130; geometric simplicity is not claimed.

## 1. Principal polarizability: an explicit witness

Define U0=0, U1=1, and U_(m+2)=-U_(m+1)-2U_m. Then alpha^m-beta^m=(alpha-beta)U_m. Exact native arithmetic gives

    U131 = 38531015900842053623
         = 263 * 146505763881528721.

Let ell=146505763881528721. It is prime. The retained checker gives a full Lucas n-1 certificate with base13 and complete factorization

    ell-1 = 2^4 * 3 * 5 * 17 * 31 * 4001 * 289510489.

Every factor is trial-proved prime; 13^(ell-1)=1 modulo ell and gcd(13^((ell-1)/q)-1,ell)=1 for every distinct prime factor q. This proves primality by the elementary full-factorization Lucas criterion. No probable-prime assumption is used. The unsigned output remains producer evidence, not a signed execution proof.

We have ell=-1 modulo131 and ell=5 modulo7. Since (-7/ell)=(ell/7)=-1, the polynomial X²+X+2 is irreducible modulo ell. Work in F_(ell²)=F_ell[a]/(a²+a+2), with b=-1-a. Ell-Frobenius sends a to b.

Since ell divides U131, (b/a)^131=1. Also b/a is not1, because the discriminant -7 is nonzero. Set z=(b/a)^66. Then z has order131, z²=b/a, and z^ell=z^-1. Consequently

    a*z = b*z^-1,     (a*z)^ell = a*z.

The explicit residue values from check.cpp are

    z = 139017811438029095 * (1+a),
    a*z = 14975904886999252.

Now O_K=Z[alpha,zeta], by the coprime-discriminant compositum theorem. Mapping alpha to a and zeta to z defines a residue prime P of O_K with residue field F_(ell²). It is unramified since ell does not divide7*131. Arithmetic Frobenius at P sends alpha to beta and zeta to zeta^-1, hence is complex conjugation c. In K/K+, where K+ is the c-fixed field, the prime P+ below P is therefore inert. Its residue field is F_ell. The equality a*z=b/z says P divides pi-c(pi), equivalently P+ divides the real ideal below pi-c(pi).

Howe, Kernels of polarizations, Theorem1.1, says that a simple isogeny class with CM field K is principally polarizable if an inert prime of K+ divides pi-c(pi). Its hypotheses have now been supplied for A. This proves principal-polarization existence somewhere in the F2-isogeny class as a written application of that theorem. It constructs neither a principal polarization on the originally presented A nor a Jacobian, an isogeny evaluator, or an efficient algorithm. [pcc:unverified]

Why the small toys differ: U3=U5=-1, so this inert-prime witness is absent. Their known non-Jacobian outcomes cannot be extrapolated to131 by that calculation. We make no claim that absence of this witness alone proves their nonpolarizability.

## 2. Why retaining the full cyclotomic action cannot work at genus130

Suppose C/F2 is smooth, projective, geometrically connected, genus130, with J(C) in A's F2-isogeny class. If zeta belongs to End_F2(J(C)), it is a unit of order131. The Rosati involution for the canonical principal polarization is complex conjugation on the CM endomorphism field. Thus zeta^dagger*zeta=1, and zeta is a polarized automorphism. Precise Torelli gives a curve automorphism inducing zeta up to sign; squaring removes that sign and retains order131. This argument works in characteristic2 as well. See Howe2022 §4.1 and Theorem4.9 for this standard method.

Here an elementary Riemann–Hurwitz argument suffices. The quotient by this F2-defined order131 automorphism has genus h. The cover is tame, and if R is the number of geometric ramification points, then

    258 = 131*(2h-2)+130*R,
    130 = 131*(h-1)+1+65*R.

Nonnegative h,R force h=0 and R=4. At a branch point, tame inertia acts faithfully on its tangent line. Because the deck transformation is defined over F2, a ramification point of degree d must have a primitive131st root of unity in F_(2^d). But ord_131(2)=130, so d is a multiple of130. A nonempty ramification locus therefore has at least130 geometric points, contradicting R=4. There is no such C with integral zeta. [pcc:unverified]

In particular maximal F2-endomorphism order O_K is excluded. This does not show that all nonmaximal orders are excluded: rational isogenies conjugate End^0, but do not preserve integral endomorphism rings. Do not carry the order131 automorphism through an isogeny without checking integrality.

The residue witness pinpoints this distinction. Both pi and c(pi) reduce to an element of F_ell at P, so every element of R0=Z[pi,c(pi)] has image in F_ell. But z has order131, while 131 does not divide ell-1. Thus z is not in F_ell and zeta cannot belong to R0. The universal Frobenius order does not itself force that automorphism. [pcc:unverified]

For genus131–300, a factorwise zeta action need not preserve the integral gluing kernel between A and the complement, or extend to a polarized automorphism of J(C). Howe2022 Proposition4.3 makes this extra condition explicit. We have not checked every complement or kernel, so do not extend the genus130 exclusion to all carriers.

## 3. Prior work actually consulted

KB unavailable: no loaded research-kb MCP tools; scripts/research-kb status exits127 because its executable is absent. Fallback queries were exact path reads and primary literature; this is not an exhaustive novelty search. Conductor is absent. No cross-session reservation is asserted.

- Original RESEARCH_ECC2K130_CURVE_CONSTRUCTION.md: four routes and a conditional genus130 model cost of 2^37.17, not a curve construction.
- research/cover_decomposition_gap_search_20261018/README.md: order131 symmetry exclusions. Some historical census wording is superseded by the stronger analytic bound in the next source.
- research/koblitz_modular_ic_20261018/REPORT.md §3: written cyclic-carrier bound g>=394, with geometric and formalization scope stated.
- Its rounds/15/CM.md: later level1841 carrier has genus45589; its quadratic diamond twist carries A131(E0)^2 under the written modular/descent bridge. This supersedes the older 508799809 ambient-genus figure as a known construction in this repo. Equations and n131 subgroup maps remain unimplemented. No new minimum-genus claim is made here.
- problems/koblitz-modular-carrier-transfer.md and current README through round26: existing modular evaluator, gluing and proof gaps. The n5 controls and bounded genus5/6 searches are not proofs about n131.
- Howe1996, Theorem1.1 and Proposition7.2, full primary text: https://arxiv.org/pdf/2001.05111 . Supplies the sufficient pp criterion used here.
- Howe–Maisner–Nart–Ritzenthaler2008 §3, Theorem6: https://arxiv.org/pdf/math/0602650 . Confirms the criterion is not restricted to abelian surfaces; its stronger Artin-ideal test requires hypotheses we do not assume here.
- Howe2001 corrected arXiv version: https://arxiv.org/abs/math/0002232v3 . The no-CM hypothesis excludes using its first construction on E0, which has CM. Its Theorem3.2 is flagged incorrect by the author and is not used.
- Howe2022 §4.1/4.3: https://arxiv.org/pdf/2110.04221 . Supplies the Rosati/Torelli method and the integrality warning underlying our second result.

No novelty claim is made. The deductions above specialize established theorems to the exact target.

## 4. What a computable DLP reduction must deliver

The requested acceptance criterion remains a concrete C/F2 of genus130–300 and an evaluated subgroup-preserving correspondence. If g=130, prove P_C=P_A. If g>130, prove P_A divides P_C over Z, with an actual complementary isogeny class. Polynomial equality or divisibility only supplies an abstract isogeny statement.

Supply maps i:A->J(C) and j:J(C)->A, defined over F2, on actual divisor/abelian representatives. A sufficient certificate is j*i=[d] with gcd(d,r)=1. Then i preserves order r and Q=kG implies i(Q)=k*i(G). Confirm i(G) is nonzero, actual cyclic-subgroup membership, exceptional fibers, field descent, denominator inverses and scalar recovery. A rational projector is not an evaluator until its denominator and domain are handled. No new generator or map is claimed here.

Price construction and every failed attempt, map evaluation, destination arithmetic, relations, linear algebra, individual descent and final source verification. The 2^37.17 figure remains a conditional destination-solver model. In particular an arbitrary nonhyperelliptic carrier needs a suitable relation algorithm; genus alone does not justify importing the model unchanged.

## 5. Status, formalization and next discriminating obligation

Principal-polarization existence: written proof plus exact arithmetic replay; unsigned, not Lean verified. Integral-action obstruction at genus130: written proof; unsigned, no full Lean geometric bridge. Genus130 Jacobian existence with nonmaximal order: unresolved. Genus131–300 general carrier: unresolved. Explicit maps and total cost: not constructed or measured.

Formalization map is PROOF_MAP.md. Arithmetic.lean contains only finite arithmetic facts, intended for crypto-lean's current leanprover/lean4:v4.35.0; lake/lean are absent here and it has not been compiled. Full formalization needs finite-field isogeny-class/CM-order definitions, the Howe criterion, polarized Torelli and tame-cover geometry; arithmetic lemmas cannot replace these bridges. No sorry, custom axioms or assumed geometric conclusion is used to claim completion.

The next proof obligation is localized: classify which nonmaximal Frobenius-stable Deligne lattices in this isogeny class admit principal forms while excluding integral zeta, then determine whether any resulting pp variety lies in the Torelli locus. For larger genus, specify a complement and gluing kernel and test curve realization plus subgroup maps. This round gives neither such a classification nor a decision procedure that resolves it; those are the remaining mathematical dependencies. An unbounded curve enumeration is not represented as an executed search.

The original construction blocker stays open in problems/koblitz-modular-carrier-transfer.md. Existing canonical cost charts are unchanged: no curve, executable map or cost datum has been added. The new routes.svg is an explanatory proof-status diagram, not a measured performance graph.

## Reproduction

    g++ -std=c++17 -Wall -Wextra -Werror -O2 check.cpp -o /tmp/ecc2k130-check
    /tmp/ecc2k130-check

Compare output with check.txt. The checker uses unsigned128 intermediates for 58-bit modular products and signed128 for the131-term Lucas recurrence. Its primality criterion is complete, its residue-field identities are exact, and it contains explicit assertions for every arithmetic hypothesis used. It does not check the geometric implications. No timing, benchmark, cloud experiment, curve census, or DLP run was performed.
