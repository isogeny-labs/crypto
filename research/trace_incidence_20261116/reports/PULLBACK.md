# Source-coordinate Sylow splitting

This is a derived, unsigned adaptation of the supplied paper's root-finding
algorithm. It keeps the input coordinate instead of reconstructing a canonical
translated divisor between recursion levels. The native implementation and
separate small-field checks support the formulation; a formal proof, signed
standing, exhaustive priority review and admitted performance comparison are
not supplied. [pcc:unverified]

## Representation and invariant

After Algorithm2, a source factor has a reduced ordinate and represents
S(t)=(t,v(t)) on either the auxiliary curve or its twist. Work in the reduced
split algebra A=F_p[t]/u(t). For one prime ell dividing N, write
N_ell=N/ell^val_ell(N). A recursive node stores(u,v,C,m), where every supported
point satisfies S(t)-C in E[N_ell ell^m]. The offset C is a known rational
ell-primary point. Start C=O for each new prime. [pcc:unverified]

Take the same representatives J as Algorithm5. For P in J, set T=C+P and
M=N_ell ell^(m-1). Then compute

```
g_P(t) = gcd(u(t), psi_M(S(t)-T))
child  = (g_P, v mod g_P, T, m-1).
```

The torsion test is Algorithm4 pulled back directly to the original source
parameter. For finite T=(x_T,y_T), use

```
s  = (v+y_T)/(t-x_T) mod u
x' = s^2-t-x_T mod u
y' = y_T+s*(x_T-x') mod u.
```

The plus sign in y' is the paper's correct subtraction formula. For T=O use
the identity map. Apply the division-free normalized psi recurrence of
Remark5.4, including even indices and characteristic-p indices; never invert
2y. All operations stay in the source quotient. [pcc:unverified]

## Exceptions and collisions

Before any finite-T inversion, enumerate the known source points T and -T
for all T=C+P. Test u(x_T)=0 and the original ordinate sheet v(x_T)=±y_T.
Emit matching roots, deduplicate their abscissae, remove one balanced product
of linear factors and reduce v once. At most2|J| point candidates and|J|
distinct abscissae are considered. The remaining t-x_T denominators are
units. A translated infinity is exactly S=T and is handled by this pass.
[pcc:unverified]

Different source roots remain different components of A even if their
translated x coordinates collide. The pair(x',y') is evaluated componentwise;
there is no need to collapse the parameter to x', compute an image minimal
polynomial, interpolate an inverse, or lift an opposite-point collision.
This changes the representation needed for root finding. The separate
canonical divisor-translation API retains its reconstruction work.
[pcc:unverified]

## Separation and termination

The representative classes partition the node's allowed group, so the GCD
factors partition its nonexceptional source roots. Child ordinate reduction
uses the parent v, and its new offset gives the same invariant at m-1.
At m=0 retain the original-coordinate factor for the next prime; a linear
factor can be returned immediately. [pcc:unverified]

If two roots survived together after all primes, their lifted point difference
would be annihilated by every N_ell. These cofactors have gcd1, so the lifted
points would coincide. Original source abscissae were distinct; every final
factor is therefore linear. The twist branch uses the paper's consistent
scaling u_tw(z)=beta^-deg(u1) u1(beta*z), v_tw(z)=v1(beta*z)/beta and returns
the original root beta*z. [pcc:unverified]

## Cost chain

Let d=deg(f), Xi be the largest prime dividing N*Ntw, and M(s) a fast
polynomial multiplication cost. For a node of degree s and j=|J|, charge:

- all j offsets and exceptional source evaluations, O(j*s) scalar field work
  plus the known-point arithmetic;
- one balanced removed-root product and exact division, source reductions,
  and all failed representatives;
- j modular translations, division-polynomial tests and GCDs, with cost
  soft O(j*M(s)*log p);
- reduced source ordinates passed from parents, rather than repeated
  reductions of a global degree-d ordinate into every tiny factor.

At each depth the factors have disjoint degree mass at most d. Summing over
prime levels uses sum_ell m_ell=O(log p), and j<=Xi². Balanced CRT in the
embedding and a balanced final root-product check retain soft-linear degree
work. A conservative derived online allowance is

```
soft O(Xi^2 * d * log^2(p) + d * log^3(p)),
```

with fast polynomial arithmetic and a valid datum supplied. The extra
logarithmic allowance conservatively covers known-point arithmetic and this
implementation's exponentiation-based scalar inverses. There is no d^omega
canonical-reconstruction term in this root-finding formulation. If both
groups are cyclic, replace Xi² by Xi in the subgroup-test contribution.
This is an unverified derivation, not a fitted exponent or signed theorem.
[pcc:unverified]

The implementation's conservative live storage is logarithmic recursion/memo
depth times degree mass, plus cached representative points: soft O(d+Xi²)
field words, with runtime/backend overhead separate. No empirical memory or
wall-time advantage is asserted. [pcc:unverified]

The cold computation still costs

```
T_construct(D) + T_validate(D) + T_online(f,D).
```

The paper's general efficient datum-construction problem remains open; the
supplied toy builder is explicitly exhaustive. An isogeny preserves the
rational orders and Xi. A cyclic codomain can alter representative counts,
but its complete basis and additional levels must be charged.
[pcc:unverified]

## Connection to prime-field relation recovery

For three random labelled pools of size B, the implemented pipeline is

```
public generation + datum setup/validation + norm/GCD
 + roots of selected GCD + signed witness recovery + kP=Q verification.
```

The native Horner norm still uses O(B²) quotient products of dimension B.
Its cost is O(B² M(B)+B M(B)log B), before roots and witness recovery.
If g is the selected GCD degree, witness queries cost O(B*g) group operations
in the worst case, plus the final scalar check. Constant/linear GCDs share
direct shortcuts across all four root backends. The new root-finding bound
does not supply a better bound for the norm. [pcc:unverified]

`incidence.cpp` invokes this backend and recovers actual informative labels
on the original target curve. `integration_audit.cpp` checks the whole GCD
support and recovered scalar using independent arithmetic, retaining natural
misses and an uninformative-first control. It records construction, validation,
public draws/rejections and solving separately, without timing or GAE pricing.
[pcc:unverified]

## Evidence and prior art

- [Native interface and reproduction](../native/README.md).
- [Independent root audit](../native/audit-data/002-result.json).
- [Integrated witness/isogeny check](../native/audit-data/004-result.json).
- [Independent derivation and limitations](../native/pullback_review.json).
- Supplied Spaenlehauer arXiv:2610.32508v1, Algorithms2,4,5,6, Remark5.4,
  Theorem5.8 and§8.1. Its general-isogeny discussion in§1.3 is prior work with
  an external root-inversion dependency; the implemented core uses its
  multiplication-isogeny tests.
- [Grenet–van der Hoeven–Lecerf2015](https://www.texmacs.org/joris/rmodroots/rmodroots.pdf),
  §2.2 Algorithm2: fixed-source factors with coset labels are an established
  design pattern. No priority claim for this exact elliptic formulation.
- `src/isogeny/velu.rs` and `src/bin/isoclass_rho/isogeny.rs`: established Velu
  codomain and point-map controls; the new code specializes degree2.

Sources checked(KB unavailable): no loaded KB MCP connection and the repository
wrapper exits127. These bounded primary sources were read; neither missing
search coverage nor successful toy checks establish novelty. [pcc:unverified]
