# Written three-adic contraction argument

Status: written derivation with independent packet, precision and
[global proof review](reviews/ternary-global.md). Full formal verification and
a general recursive implementation are open. This is an adaptation of the
pinned October 25 manuscript's Appendix D.4, not a distinct ECDLP reduction.
The review contains the complete parameter, arithmetic and computational-model
argument; this document records the central construction and claim boundary.

## T1. Exact claimed scope

Fix constants `C,A,B>0`. A bipartite multigraph has at most `Cs` Boolean
edges, polynomially many vertices, maximum degree `ceil(s^(9/10))`, and total
description length `Dlen`. Entries belong to
`(Z/3^P0 Z)[V]/(V^ell)`, with `1<=P0,ell<=Cs`. Each specified entry is uniformly
evaluable in `(s+Dlen)^B 2^(A s^(9/10))` bit time and polynomial space.
The ternary adaptation computes its contraction in

```
(s+Dlen)^O(1) 2^O(s/sqrt(log s)) bit time; (s+Dlen)^O(1) bit space.
```

This is the three-adic counterpart of the source theorem, under the explicit
input model. It is not an assertion about arbitrary mod-two input entries.
For 0/1 entries the nonnegative integer contraction is at most `2^M`, where
`M` is the number of edges. Taking `P0=M+1` recovers it exactly since
`2^M < 3^(M+1)`. A fixed adjustment of `C` accommodates that precision.

## T2. Labels, orbits, packing and locality

The label field is `F_67`. It is separate from the coefficient ring of
residue characteristic 3. A vertex has `2a` label coordinates, written as
two components for each `i`. A row/column pair class has `4a` coordinates.
Define

```
S(A,B)=(-A-B,A),   S^-1(A,B)=(B,-A-B),   S^3=I.
```

At coordinate `i`, act by `S` on component 1 and `S^-1` on component 2.
Actions at different coordinates commute. Since there is no three-torsion
in the additive group of `F_67`, a coordinate is fixed exactly when its
four entries vanish. An orbit has size `3^k`, with `k` its nonfixed-coordinate
count. Set `L=3^a`. Slot digits apply these actions and give each class
`L/3^k` preimages. Lean checks the group action, fixed-point condition,
translation bijections between reached fibers and conditional locality;
the entire combinatorial packing is not thereby formalized.

There are `Darr=67^(4a)` pair classes. Uniform vertex labels and conditional
expectation give
`sum_gamma(X_gamma-M/Darr)^2 <= 2M Delta`. If

```
M >= 2 Darr^2 Delta,    M >= L Darr,    2L <= Delta,
```

each load is at most `2M/Darr`. Round a common padded load up to a multiple
of `L`; it is at most `3M/Darr`. Uniform orbit fibers then produce at most
`3M/L` packets. Every coordinate/component/symbol has at most `3M/67` real
ports on either side. The first two inequalities establish packing; the
third maintains the new-tensor degree invariant in recursion.

If `t!=0`, put `i=v3(t)` and `c=(t/3^i) mod 3` in `{1,2}`. For slots with
`y-x=t`, their lower digits agree and `y_i=x_i+c mod 3`. Component `c`
therefore has `row(x)=column(y)`. The versioned
[integer-slot formalization](lean-digits/MATHEMATICAL_PROOF.md) now proves this
bridge using actual signed-integer division and `padicValInt 3`. It also proves
`v3(y-x)<a` for distinct slots in `[0,3^a)` and constructs the required `Fin a`
coordinate. The Cauchy-support-to-difference theorem and graph-port bridge
remain separate open formal obligations.

## T3. Deformation and integral charts

Use the integer grid `G={-L,...,2L-1}`, `alpha=3^(a+1)` and

```
C(T)=product_(t in G)(T-t),
W_xy(T)=C(T)/(C'(0)(T-(y-x))).
```

All slot differences lie in `G`; cancellation makes `W` polynomial of degree
`3L-1`. Entries across packets are zero. On real ports, `W(0)=I`.
Let `A(Y),B(X)` be products of lifted row/column tensor entries on independent
Boolean port words. With `l_0(z)=1-z`, `l_1(z)=z`, deform by

```
F(T)=sum_(X,Y) A(Y)B(X) product_x l_(Y_x)((W(T)X)_x).
```

Then `F(0)` is the contraction and `deg F<=M(3L-1)`. This uses the full
Boolean Lagrange factors, including the negative sign in `l_0`; replacing
them with positive-only row factors would be a different expression.

Take a monic degree `e=4 ceil(log2 s)` polynomial irreducible modulo 3 and
work in `O=Z_(3)[U]/(f)`, with nilpotent `V^ell=0`. The grid is a complete
residue system modulo `3^(a+1)`. At each `t`, the number of nonzero differences
of valuation `k` is `2*3^(a-k)`, so all `C'(t)` have equal valuation.
Cancel the known three-parts coefficientwise before inverting the remaining
unit denominator. This proves `W(t+alpha z)` is integral, including at roots.
The extension supplies at least `s^4` pairwise unit-difference points for
polynomial interpolation. Constructing the extension by finite enumeration
is polynomial in `s` because `e=O(log s)`; it is charged work.

## T4. Tapered Hermite precision

For `1<=u<=a`, `M>=u`, set

```
h=ceil(2M/u), q_i=h max(u-i,0), D_t=L/3^i,
p_t=p+4hD_t+8M(a+1), i=v3(t).
```

The `2L` unit nodes give total multiplicity at least `4ML`, exceeding
`deg F`; also `q_i<=3M`. The normalized Hermite factor from source D.9 obeys

```
v3 E_t(0)=2D_t sum_(j=1..a-i) (j/3^j)(q_(i+j)-q_i)
           >= -3D_t h.
```

Here `sum j^2/3^j=3/2`. Nodes where both relevant valuations equal `i` do
exist in characteristic 3; they contribute zero. The boundary nodes of
valuation `a` have zero multiplicity, so no nonexistent label coordinate
is selected. Dividing the local chart by the integral unit series
`E_t(t+alpha z)` loses no digits. Evaluation of its truncated quotient at
`-t/alpha` loses at most `3M(a+1)` more. Thus an input error at precision
`p_t` contributes at valuation at least

```
p_t - 3D_t h - 3M(a+1) >= p.
```

Recover by exact coordinatewise rational Hermite interpolation, summing
before cancelling the common denominator's three-parts. All multiplier
numerators/denominators have polynomial bit length. This is a bit-cost
argument, not unit-cost exact rational arithmetic. At each node the full
chart needs `M(3L-1)+1<s^4` evaluations for sufficiently large `s`.

## T5. The necessary target split

At an active node group vertices by coordinate `i`, component `c`. Locality
makes `W(t)` group-local. Tile columns by `y=y0+3^(i+1)j`. A tile has one
source group. Its geometric crossing expansion has source moments
`sum_y j^b X_y` and coefficients divisible by `3^(b+1)`.

**The initial proposal omitted a target index.** A ternary packet can have
the three distinct symbols `A,B,-A-B`; one source tile can cross to two
targets. Assign one formal variable per packet/tile/weight/**target**, even
when two variables later substitute the same numerical source moment.
Each formal variable then has exactly one source and target, restoring
coefficient-extraction factorization over groups. There are at most

```
2 * (3M/L) * 3^(i+1) = 18M/D_t
```

modes of each weight. Keep only tuples with `sum_Z (b_Z+1) beta_Z < p_t`;
the omitted terms vanish. The colored-partition estimate bounds tuple count
by `exp(O(sqrt(M p_t/D_t)))`. A child has at most `6M/67<M/8` edges; new
packet degree is at most `2L`. Implicit packet entries expand at most `L`
affine factors and take `exp(O(L log s))` time with polynomial space.

The native unsplit regression gives `0` where direct and corrected results
give `9` modulo 27. Its frozen example, both formulas and every case row
are retained; the failed idea was corrected rather than suppressed.

## T6. Recursion and complete cost

Set `lambda=ceil(log2 s)`, `epsilon=2^-20`, leaf threshold
`H=floor(s/floor(sqrt(lambda)))`, and at depth `d` use

```
u_d=floor(epsilon lambda/2^d), a_d=2 sum_(j<=d)u_j, a_(-1)=0.
```

Shrinkage gives active depth at most `log2(lambda)/6+O_C(1)`, hence
`u>=lambda^(4/5)` eventually. Both `L` and `Darr` are at most `s^(1/1000)`;
all packing/degree inequalities hold at active calls for sufficiently
large `s`. Explicit parameter tests and direct summation cover the finite
remainder without uncomputable threshold advice.

Each tensor retains its immutable creation precision `pc`: compute its
**complete final entry** modulo `3^pc`, then lift those coordinates. Lifting
intermediate factors or recomputing a formula at a larger precision is
invalid in general. Descriptions are shared, not recursively composed.
Inherited precision is at most `s lambda^O(1) 3^(a_(d-1))`. For `i<u`,
`D_t>=3^(a_(d-1)+u)`, so the extra `3^u` absorbs inherited precision and
evaluation loss. Consequently `p_t=O_C(MD_t/u)` and mode enumeration costs
`exp(O_C(M/sqrt(u)))` at every active level. Precisions stay polynomial.

The depth-`d` branching exponent is bounded by a constant times
`(s/sqrt(lambda)) (sqrt(2)/8)^d`. Summing these exponents and polynomial
overheads across depths, including direct leaves and every entry evaluation,
gives T1. Depth-first traversal stores polynomial graphs, descriptions,
indices, Taylor data and rational recovery workspace. No exponential table
or free entry oracle is hidden in the space/cost claim.

## T7. ECDLP implication and novelty boundary

Boolean counting can be used in the already published compact hashed-factor-
base reduction. It retains expected exponent
`O(b/m+mb/sqrt(log b))`, balanced at `m~(log b)^(1/4)`. Thus a correct full
implementation of this internal variant would have the **same exponent
class** as that existing claimed prime-field algorithm, under its exact
curve/order hypotheses. No cryptographic performance measurement or complete
ternary ECDLP implementation is supplied here. This transfer is not an
essentially new reduction and cannot discharge the user's novelty criterion.
