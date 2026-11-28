# Protocol: pair-trace quotient with a quadratized 2+1 S3 split

**Frozen on 2026-11-25 before implementing the pair-trace arm or observing
any pair-trace solver row.**

Study id: `EXP-C25519-PAIRTRACE-001`. Parent revision: `a41230c232`. The
instrument and analyzer will be native Rust in Isogeny Labs `crypto`. No
JavaScript is used.

## 1. Question and boundary

The active objective is a classical prime-field ECDLP algorithm with expected
total work `r^o(1)`. This study is only a relation-system solver diagnostic.

Ordinary-grevlex permutation did not rescue the selective quotient system:
the primary quotient-first arm used `1.2934390725` times direct F4 work and
`2.3766` times direct peak RSS. The next candidate must change the algebra,
not merely variable names.

The broad two-S3 chain is prior repository work and its H2 root-polynomial
hybrid was cost-unfavourable. The narrower question is whether quotienting the
swap of the first two roots by the exact `(z,c)` pair trace, then making both
S3 links wholly quadratic with auxiliary variables, lowers solving degree
enough to beat the fully symmetric S4/direct-remainder system.

This finite study cannot establish an exponent change. Relation probability,
enough independent relations, setup, filtering, sparse linear algebra,
individual logarithms, and target recovery remain outside it.

## 2. Fixed curve, targets, and baseline

- Curve and subgroup: the existing `curve25519_family_instance(20)` short
  Weierstrass model over `p=1,048,573`, prime subgroup order `r=131,203`, and
  generator `(349,837,122)`.
- Factor base: `x=349,833+3w`, `w in mu_d`, with `d in {7,9,19}`.
- Relation arity: three factor-base abscissae and one fixed target abscissa.
- Fresh seed: `2026112503`; targets are paired exactly across arms.
- Solver: repository `f4_fp`, ordinary grevlex, release mode,
  `RAYON_NUM_THREADS=1`, degree ceiling 24.
- Primary metric: counted F4 `F_p` multiply-adds. Secondary metrics are
  productive degree, rows, columns, steps, exact construction operations,
  peak RSS, and descriptive wall time.

Arm D is the predecessor's fully symmetric S4 plus direct cubic-remainder
membership system in canonical layout. A new D implementation must reproduce
the pinned predecessor D binary on every fresh target's equations metadata,
verdict, work, degrees, matrix extrema, and steps before comparisons count.

## 3. Frozen systems

There are three arms. Only P may qualify.

| arm | system | role |
|:--|:--|:--|
| D | symmetric S4 plus direct cubic-divisor membership | baseline |
| R | raw ordered roots, two quadratized S3 links, three scalar power circuits | split/quadratization control |
| P | swap-quotiented pair trace, two quadratized S3 links, two scalar power circuits | **primary** |

### 3.1 Scalar membership

Each scalar `a in mu_d` uses the following fixed quadratic addition chain.

| d | retained powers | final equation |
|:--|:--|:--|
| 7 | `a2=a^2`, `a3=a2*a`, `a6=a3^2` | `a6*a-1=0` |
| 9 | `a2=a^2`, `a4=a2^2`, `a8=a4^2` | `a8*a-1=0` |
| 19 | `a2=a^2`, `a4=a2^2`, `a8=a4^2`, `a16=a8^2`, `a18=a16*a2` | `a18*a-1=0` |

R applies this to `w1,w2,w3`. P applies it to pair scale `z` and lone root
`w`.

### 3.2 Pair-trace membership

For P, `n=(d-1)/2` and the retained variables evaluate
`U_n(c),U_(n-1)(c)` with the deterministic left-to-right binary circuit,
starting from `(U_1,U_0)=(c,1)`. A doubling step from `(U_k,U_(k-1))` is

```text
U_(2k)   = U_k^2 - U_(k-1)^2,
U_(2k-1) = U_(k-1) * (2*U_k - c*U_(k-1)).
```

For a following one bit, retain `U_(2k+1)=c*U_(2k)-U_(2k-1)`. Constants and
an output identical to an existing variable are substituted, not retained.
The terminal equation is

```text
(c-2) * (U_n(c)+U_(n-1)(c)) = 0.
```

Because `p` does not divide `2d`, its roots are exactly
`q+q^-1` for `q in mu_d`, including `q=1`. P sets
`a=zc`, pair abscissa sum `sigma=2lo+g*a`, and product
`pi=lo^2+lo*g*a+g^2*z^2`, retaining `a` and `pi`.

### 3.3 Quadratized S3 links

For a pair with abscissa sum `sigma`, product `pi`, and intermediate
abscissa `u`, retain

```text
delta = sigma^2 - 4*pi,
beta  = sigma*(pi+A) + 2B,
v     = u^2,
delta*v - 2*beta*u + (pi-A)^2 - 4B*sigma = 0.
```

For the second link set `x3=lo+g*w`, `sigma2=u+x3`, retain `pi2=u*x3`, and
impose

```text
(sigma2^2-4*pi2)*X^2
 - 2*(sigma2*(pi2+A)+2B)*X
 + (pi2-A)^2 - 4B*sigma2 = 0,
```

where target abscissa `X` is constant. Every P and R input equation therefore
has total degree at most two. R obtains `sigma,pi` directly from `w1,w2` and
uses the same retained `delta,beta,v,pi2` variables.

P's frozen variable order is producer-local:
`z,z-powers,c,U-circuit,a,pi,delta,beta,u,v,w,w-powers,pi2`.
R analogously orders `w1,w1-powers,w2,w2-powers,pi,delta,beta,u,v,w3,
w3-powers,pi2`. No post-data layout selection is allowed.

## 4. Correctness gates

Before cost interpretation:

1. enumerate `mu_d` and prove the `(z,c)` map is a bijection from unordered
   pairs, including repeats, for every study order;
2. verify the fast-doubling circuit and terminal trace polynomial against
   direct enumeration of all `q in mu_d`;
3. verify every retained quadratic equation on every planted assignment;
4. compare D/R/P on every unordered distinct-root triple of the main `d=7`
   instance and on the predecessor's exhaustive tiny positive/negative panel;
5. require identical paired targets and identical solver verdicts;
6. reproduce the predecessor D binary exactly on a separate fresh-seed run.

Any mismatch invalidates the instrument. Failed and partial rows are retained;
no seed, equation, layout, threshold, or target may change after a row exists.

## 5. Gated measurement

Run target-major D/R/P. A timeout, allocation failure, memory kill, or degree
ceiling is censored and never assigned a completed work value. Use 10,000
paired bootstrap resamples with seed `2026112503`.

1. **d=7 screen:** four random and four planted targets per arm, 120-second
   cap. Continue only if P has no extra censoring, at least three random pairs,
   P/D upper descriptive 95% work endpoint below `1.00`, productive degree at
   most 4 on at least 3/4 pairs, and every P/D peak-RSS ratio at most `1.50`.
2. **d=9 screen:** the same counts, 180-second cap. Continue only if P/D upper
   endpoint is below `0.85`, P/R upper endpoint below `0.95`, productive degree
   at most 4 on at least 3/4 pairs, and every P/D peak-RSS ratio at most `1.40`.
3. **d=19 qualification:** eight random and four planted targets per arm,
   300-second cap. P qualifies only if its P/D upper endpoint is below `0.50`,
   P/R upper endpoint below `0.75`, productive degree is at most 3 on at least
   6/8 pairs, every P/D peak-RSS ratio is at most `1.25`, and every P/D
   construction-operation ratio is at most `1.25`.

All earlier correctness gates and no-worse-censoring requirements apply at
every stage. A failed stage stops the later stages. R is diagnostic and cannot
replace P after data.

## 6. Scaling authorization and evidence

Only a full d=19 pass opens paired D/R/P cells at `d in {21,27,57}` with the
same counts, seed, equations, layouts, and resource rules. The P/D advantage
must strengthen: the upper two-sided 95% endpoint of the slope of
`ln(P/D)` on `ln(d)` must be negative. A separately frozen `m=4` study is
still required.

Retain exact revisions, repair-overlay provenance, toolchain, commands,
binary hashes, all rows and failures, native analyzer source, JSON, CSV,
editable SVG/PNG, Typst/PDF, complete hashes, and `experiment.export/v1`.
Results remain `[pcc:unverified]` without independent validation.
