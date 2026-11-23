# A cyclic F8 genus-two Jacobian forces an original-code BSGS miss

Date: 2026-11-23. Lane: [PR #4977](https://github.com/isogeny-labs/crypto/pull/4977). New algebraic/source audit of the inherited original-root blocker. Exact referenced candidate Git blob: `616422524711c1dd0e54b6bb6c557348b999e262`; malformed production blob: `f001a1b641abd48bb0130c1a9a246e37c28bcc3a`. Input PR head: `0ab8d705183d6f58bfb2679aa944797c6453ba27`.

**Scope:** elementary finite-field proof and exact code-path audit. An independent original-Rust test, separate source review and signed validation have not occurred. All findings [pcc:unverified]. This is a small binary genus-two *correctness control*, not a Prime31 elliptic relation solver, timing improvement, or cryptographic benchmark.

## The curve and its point counts

Work over `F_8`. The smooth genus-two Artin–Schreier curve

$
 C:\q\quad y^2+y=x^5+x+1
$

has one rational point at infinity. It is smooth because the `y` partial derivative is `1`, and has genus two because the right-hand side has odd degree five.

For `F_{2^m}`, `y^2+y=z` has two solutions iff `Tr_{2^m/2}(z)=0`. For any `x` in `F_8^*`, the trace of `x^5+x` is

$
x+x^2+x^3+x^4+x^5+x^6,
$

which equals 0 when `x=1` and equals 1 for the other six nonzero values (use `x^7=1` and the seven-term geometric sum). At `x=0` it is also 0. Since `Tr_{8/2}(1)=1`, exactly the six values `x\in F_8\setminus\{0,1\}` lift, hence

$
N_1=#C(F_8)=1+2(6)=13.
$

For `F_{64}`, select `t` with `t^8=t+1`; write `t^2+t=\delta\in F_8` and `x=a+bt`, `a,b\in F_8`. Such `t` exists because the relative trace `F_{64}\to F_8` is surjective. The absolute trace of the constant 1 is zero. Since

$
t^4=t+\delta+\delta^2,\q\quad
t^5=(1+\delta+\delta^2)t+\delta,
$

taking the relative trace of `x^5+x+1` and then the `F_8/F_2` trace yields

$
Q(a,b)=\operatorname{Tr}_{8/2}
 \bigl(a^4b+ab^4+b^5(1+\delta+\delta^2)+b\bigr).
$

Use `Tr(a^4b)=Tr(ab^2)`, giving the `a` coefficient `b^2+b^4`. At `b=0`, every one of eight `a` values gives trace zero. At `b=1`, the constant is `\operatorname{Tr}(\delta+\delta^2)=0`, so again all eight do. For each of the six remaining nonzero `b`, `b^2+b^4\ne0`; the resulting nonzero linear functional in `a` is balanced, hence four of the eight `a` values give zero. Exactly `8+8+6(4)=40` abscissae lift, proving

$
N_2=#C(F_{64})=1+2(40)=81.
$

The standard genus-two Newton/zeta formula gives

$
#J(F_q)=\frac{N_1^2+N_2}{2}-q,
\q\quad
#J(F_8)=\frac{13^2+81}{2}-8=117=3^2\cdot13.
$

## Prove cyclicity, rather than assuming it

Here `t_1=q+1-N_1=-4`, `t_2=q^2+1-N_2=-16`, and `e_2=(t_1^2-t_2)/2=16`. The Frobenius characteristic polynomial is exactly

$
P(T)=T^4+4T^3+16T^2+32T+64
=(T^2+8)(T^2+4T+8).
$

The two \quadratic factors have resultant `128` and are thus coprime in `Z_3[T]`. The rank-four 3-adic Tate module splits into rank-two Frobenius-stable summands for these factors. On the `T^2+4T+8` summand, `\det(1-F)=13` is a 3-adic unit, so this summand contributes no rational 3-torsion. The other rank-two summand has Frobenius trace zero. Were the order-nine 3-primary rational subgroup `(Z/3)^2`, Frobenius would act as identity on its entire two-dimensional mod-3 summand, forcing trace `2 mod 3`, contradicting trace `0 mod 3`. Hence the 3-primary subgroup is `Z/9`, while the 13-primary subgroup necessarily is `Z/13`. Therefore

$
\boxed{J(F_8)\simeq \mathbf Z/117\mathbf Z}.
$

**There exists a divisor class of exact order 117.** This is an existence theorem, not an executed construction of an explicit Mumford representative. It is also stronger for BSGS than the previous `y^2+y=x^5+x^3` example of Jacobian cardinality 169, which by itself did not determine a divisor-order failure.

## Inspect the exact inherited BSGS scan

The unchanged implementation in `BINARY_HEC_COMPLETE_UNION_CANDIDATE.rs` computes
`root=floor(sqrt(q))`, `lo=(root-1)^(2g)`, `hi=(root+1)^(2g)`.
For `m=3,g=2`, these are `lo=1,hi=81,width=80`.

The `divisor_order_bsgs` implementation sets
`steps=ceil(sqrt(width))+1=10`,
`giant_max=floor(width/steps)+2=10`, and its verified candidate-multiple check ranges over

$
m=\operatorname{lo}+i\cdot\operatorname{steps}+j,\quad
0\le i\le10,\quad 0\le j<10,
$

so **every candidate lies in `[1,110]`**, including a bounded overshoot beyond `hi=81`. The previous 169-order example should not be called a BSGS failure merely because its group order exceeds 81.

For a divisor of exact order **117**, *no* candidate multiple from 1 through 110 annihilates it; the function must return `None` (for `max_baby_steps>=10`; a tighter resource cap returns `None` before searching). The failure follows from the written loop and the proven cyclic group existence, assuming the group law behaves as specified. It does **not** claim that a specific Mumford divisor was passed through compiled native Rust.

A conservative integer upper endpoint uses `ceil(sqrt(q))`, giving `(3+1)^4=256`, instead of 81. Do not silently change `hasse_interval` or install the standalone candidate in production: both-parent source reconciliation and normal root compilation/review gates are still required.

## Reproduction and validation gate

- Mathematical producer check: an independent bounded `GF(64)` bit-polynomial enumerator (mod `z^6+z+1`) gave `N1=13,N2=81,#J=117` for `f=x^5+x+1`; a separate symbolic factorization gave `P(T)=(T^2+8)(T^2+4T+8)`. The analytic proof above does not rely on trusting that enumerator. These are not Rust, Lean or signed tests.
- Source read: candidate `hasse_interval` and `divisor_order_bsgs` at the exact input head, including their bounded overshoot. Keep the 1..110 correction in every report.
- Independent reviewer: challenge the relative-trace count, the integral Tate-module factor splitting, 3-primary cyclicity and the precise BSGS candidate loop. For native validation after authorization, construct the exact genus-two curve with original `F2m` arithmetic, exhaustively identify a rational divisor of order 117 with a separately justified order oracle, confirm old interval and `None` behavior, apply the ceiling fix in an isolated reviewed repair branch, and rerun the original-root regression without suppressing existing tests.
- Source and literature checked: prior `HASSE_Q8_JACOBIAN_COUNTEREXAMPLE.md`, `HASSE_INTEGER_CEILING_REVIEW.md`, both-parent reference, standard finite-abelian-variety Frobenius/Tate-module theory (e.g. [Finite group subschemes of abelian varieties over finite fields](https://doi.org/10.1016/j.ffa.2014.04.001)). Shared research-KB and Rust/native-worker services were not exposed here.
- No new PCC claim, production change, paid experiment, authorized merge, Space mutation or automation change. The Prime31 empty-inclusive sublinear `S3` index remains a distinct open task. [pcc:unverified]
