# CT11: odd-order elliptic input, quadratic cover, and Galois-stable Richelot partitions (2026-11-11)

**Standing:** original proof argument plus producer-run small-field development checks,
**[pcc:unverified]**. No external/independently signed PCC verdict, Rust compile,
isogeny-map implementation, or performance measurement is claimed.

## Precise obstruction and escape hatch

Let k=F_q have odd characteristic, let h(u)=u³+a*u²+b*u+c be
separable with c != 0, and define

    E : v² = h(u),   C : y² = h(x²).

C is a smooth genus-2 curve, and (x,y)->(u=x²,v=y) is a degree-2 map C->E.
If #E(k) is odd, E has no nonzero k-rational 2-torsion. Thus h has no
k-rational root; being cubic, it is irreducible.

**Proof 1 (rational factor gate).** A k-rational factor of h(x²) of degree 1
or 2 would have a root beta in F_{q²}. But alpha=beta² would then be a root
of the irreducible cubic h in F_{q²}, contradicting
F_{q³} ∩ F_{q²} = F_q. Hence h(x²) has no k-rational factor of degree
1 or 2. In particular, C has no k-rational Weierstrass point (the two
points at infinity are not hyperelliptic branch points) and cannot be
put into the existing rational-branch-point odd-quintic chart over k.
An algorithm enumerating **three individually k-rational quadratics**
also cannot start here. This does NOT forbid the cover or a k-rational
Richelot kernel whose individual quadratics are defined over extensions.

**Proof 2 (Frobenius partition gate).** Let alpha be a root of h in F_{q³}.
Its norm to k is -c, and alpha is a square in F_{q³} iff -c is a square
in k (odd-degree finite-field norm preserves square class).

- When -c is a nonsquare: each square root beta of alpha has degree six.
  The six geometric branch roots form one Frobenius 6-cycle. Its only
  invariant perfect matching is the opposite-root matching
  {{beta_i,-beta_i}}. The corresponding quadratics x²-alpha_i all have
  zero x coefficient and so have coefficient determinant delta=0.
  This is the **decomposable**, not a nondegenerate genus-2 Richelot,
  kernel. There is no nondegenerate F_q-rational Richelot kernel of this
  simple 6-root-partition type for that Frobenius class.

- When -c is a square: h(x²)=m(x)*(-m(-x)) for a monic irreducible cubic
  m(x)=x³+B*x²+C*x+D over k. For its three roots beta_i, Frobenius
  permutes beta_i cyclically and independently permutes -beta_i.
  There are exactly three Frobenius-invariant perfect matchings:
  pair beta_i with -beta_(i+k), for k=0,1,2 (cyclic indices).
  k=0 is the decomposable opposite pairing. For k=1,2, let
  G_i(x)=(x-beta_i)(x+beta_(i+k)). The ordered coefficient matrices
  (x²,x,1) have determinants

      delta_0=0,  delta_1=B*C - 9*D,  delta_2=9*D - B*C.

  **Proof:** if s1=sum(beta_i)=-B, s2=sum(beta_i*beta_j)=C,
  s3=prod(beta_i)=-D, direct determinant expansion gives
  delta_1=9*s3-s1*s2=B*C-9*D and delta_2=-delta_1.
  Thus **if B*C != 9*D**, there are two nondegenerate Galois-stable
  Richelot kernel partitions defined over k, despite the absence of any
  k-rational quadratic factor. Kernel descent is standard; actual
  codomain/morphism normalization and descent remain a separate check.

The cases are about this even-sextic / degree-2-cover family; they do not
prove or disprove other covers, field transfers or general DLP advantages.
In particular, a nontrivial rational kernel need not have pointwise-rational
kernel elements, just as cryptanalysis #1472 shows for degree-three
elliptic isogenies.

## Two predeclared public F11 fixtures

**Positive Galois-stable example (no rational quadratic factors):**

    E_+: v²=u³+u+6,   #E_+(F11)=13 (prime).
    C_+: y²=x⁶+x²+6,  #C_+(F11)=10, #C_+(F121)=156,
    #J(C_+)(F11)=117 = 13*9.

    h_+(u) irreducible over F11.
    x⁶+x²+6=(x³+5x²+7x+7)(x³+6x²+7x+4).
    m=x³+5x²+7x+7, (B,C,D)=(5,7,7).
    B*C-9*D=-28=5 (mod 11).
    delta_0=0; delta_1=5; delta_2=6.

Over L=F11[t]/(t³+5t²+7t+7), t,t^11,t^121 form the first Frobenius
3-cycle; their negatives form the second. The three G_i are generally
not individually in F11[x], but Frobenius cyclically permutes them.
Direct F11³ polynomial multiplication yields prod_i G_i=x⁶+x²+6.

**Six-cycle negative control:**

    E_-: v²=u³+2u+4, #E_-(F11)=17 (prime).
    C_-: y²=x⁶+2x²+4.
    h_-(u) irreducible, -4=7 is a nonsquare mod 11.
    x⁶+2x²+4 irreducible over F11 (no factors degree 1,2,3).
    Unique F11-stable root-pair partition has delta=0.

This negative result rules out a nondegenerate **rational Richelot
kernel** of the specified partition class; it does not rule out
other Jacobian isogenies or DLP-preserving transfers.

## Essential twist/sign trap in the genus-2 codomain

With the explicitly chosen bracket convention

    H_i=(G_j' G_k - G_j G_k'),  (i,j,k) cyclic,

the *raw* products H_1 H_2 H_3 / delta over F11 have ascending
coefficient vectors:

    k=1: [6,6,6,6,6,6,5]
    k=2: [6,5,6,5,6,5,5].

Exact F11/F121 point enumeration gives each raw model
(#C(F11),#C(F121),#J(F11))=(14,156,165), unlike the source
(10,156,117). Their negatives (quadratic twists over F11) are

    k=1: [5,5,5,5,5,5,6]
    k=2: [5,6,5,6,5,6,6]

and have (10,156,117) under independently coded character sums.
Thus **the unsafely signed naive normalization fails a necessary
Frobenius/isogeny-class check**. The sign consistent with these tests is
the negative under this specific bracket/determinant convention. A matching
zeta function is a necessary control, not an independent certification
of the full Richelot correspondence or its precise convention.
This is an actionable regression gate for a cubic-algebra backend.

Genus-2 counts use exact square characters over F11 and F121 with
F121=F11[s]/(s²-2), and
#J(F11)=11²+1+12*a1+a2, where
a1=#C(F11)-12 and a2=(#C(F121)-122+a1²)/2.
The second elliptic quotient has affine model
w²=6u³+u²+1 and #E_2(F11)=9, consistent with 117.

## Reproduction and gates

- New standalone Rust regression: \`frobenius_partition.rs\`, intended
  command (NOT executed in this environment):
  \`rustc --edition=2021 --test frobenius_partition.rs -o /tmp/ct11-tests && /tmp/ct11-tests\`.
  The actual Rust compiler/toolchain is unavailable here; no Rust test pass
  is claimed. Failures must be preserved and repaired before merge.
- Development cross-checks executed on public constants using Python
  (SymPy factorization, F11³ determinant/bracket arithmetic, Fp/Fp²
  characters) and separately JavaScript (Fp/Fp² character enumeration).
  The two Fp/Fp² implementations agreed on all five model point counts.
  These are two producer-run oracles, not a PCC signature. [pcc:unverified]
- No cloud instance, EKS allocation, benchmark run, or funds used.

**Next implementation:** use cubic-algebra polynomial kernels rather than
requiring each quadratic in Fp[x]. Validate exact target twist and
correspondence against Magma (or independent closed-form Richelot)
and the pre-existing native genus-2 library. For a prime-order E, an
odd-quintic chart tied to a rational Weierstrass point is unavailable;
support a split-infinity even-sextic chart or a genuinely different cover.
Freeze construction+transport+solver+verification cost controls before
any timing.

## Literature and prior work checked first

- Existing project: \`AGENTS.md\`, \`research/chained_jacobian_transfers_20261110/QUEUE.md\`,
  \`SOURCES_20261111.md\`, \`GLOBAL_SUCCESSOR_COLLAPSE.md\`,
  and cryptanalysis #1472 stable nonpointwise-rational 3-kernel.
- Magma Handbook, Richelot Isogenies, specifically kernel description
  via quadratic polynomial over a *cubic algebra*, where the quadratics
  need not individually lie in k[x]:
  https://magma.maths.usyd.edu.au/magma/handbook/text/1615
- Bruin–Doerksen, *The arithmetic of genus two curves with (4,4)-split
  Jacobians*, general Richelot description over non-algebraically-closed
  base fields: https://doi.org/10.4153/CJM-2011-039-3
- Kunzweiler, *Efficient computation of (2^n,2^n)-isogenies* (DCC 2024),
  Propositions 7–8 (correspondence and origin/sign conventions):
  https://doi.org/10.1007/s10623-024-01366-1
- Shared research-KB MCP/CLI, crypto-autoresearcher encrypted lead atlas,
  ecdb and research-scope conductor were unavailable in this runtime.
  No novelty assertion or negative prior-art claim is based on that gap.

**Research standing:** [pcc:unverified], pending Rust compile/tests, immutable
independent replay, correspondence validation, PCC claim record and signed CI
standing. Do not merge this mathematical development into a claimed solver.
