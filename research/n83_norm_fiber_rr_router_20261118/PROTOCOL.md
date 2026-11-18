# Frozen protocol: norm-fiber Riemann--Roch router

Frozen on 2026-11-18 before implementation and before any relation-yield
measurement.  The registration hashes are in `REGISTRATION.sha256`.

## Question and claim boundary

Test whether effective degree-four representatives of classes in, or in fixed
kernel cosets of, the transported elliptic image produce a useful
image-compatible relation matrix.  The experiment is a structural and cost
gate.  It is not a speedup claim and it does not run the n=83 DLP.
[pcc:unverified]

The exact n=9 gate exhausts the registered one-step family for every element of
the target prime-order subgroup in two arms: the transported arm `K=0` and one
fixed, certified nonzero `K` in `ker(phi_*)`.  It does **not** exhaust all
roughly `q^2` kernel classes.  The ordinary-ambient and exceptional fixtures
are controls, not exhaustive ambient-Jacobian samples.

## Literature and prior-result boundary

- Enge--Gaudry, *An L(1/3+epsilon) Algorithm for Discrete Logarithm for Low
  Degree Curves*, arXiv:cs/0703032, PDF SHA-256
  `1545437aac0012c5eb8c4304e7032d272c3f6aa6743741d7aeb4ec581df399ab`:
  factor bases and smooth effective divisors are classical; its random-divisor
  smoothness assumptions are not imported as facts about this image family.
- `research/n83_genus3_image_solver_20261031/DESTINATION_CALCULUS_SCREEN.md`:
  ordinary full-Jacobian genus-three collection is already screened out.
- `research/n83_jacobian_projector_20261031/RESULTS.md` and
  `research/n83_image_closed_degree2_20261101/RESULTS.md`: the projector is
  certified, while rational degree-two cover fibers yield only scalar
  multiplication relations.
- `research/chained_transfer_next_20261115/NOTE.md`: higher-weight relations
  inside the image remain a hypothesis and require measured rank and complete
  cost accounting.

The shared research KB was unavailable; this protocol uses the repository
record and pinned primary paper above.

## Registered construction

Let the genus-three source be

```text
C: v^2 + h(u)v = f(u),   h(u)=u^2,   f(u)=u^7+u,
```

with cover `phi(u,v)=(u^3,uv+1)` to
`E: y^2+xy=x^3+1`.  For a reduced class `X=(u0,v0)` and each constant
`c in F_q`, form

```text
V = h + v0 + c*u0,
U = (f + h*V + V^2) / u0.
```

A candidate is admitted only when the division is exact, `U` is monic of
degree four, and native Mumford reduction certifies
`[U,V mod U]-4*infinity = X`.  Every admitted candidate is therefore an
effective-divisor/Riemann--Roch representation with an executable
reconstruction certificate.  No candidate that fails a condition is silently
discarded; failure counts are emitted by reason.

The B=1 smoothness oracle accepts exactly when `U` splits completely over the
base field.  Multiplicities are retained.  For every root `alpha`, use the
source point `(alpha,V(alpha))`, map it by `phi`, and project it by the target
cofactor four.  Repeated projected labels are merged into one sparse matrix
entry with their multiplicity.  Identity labels are allowed and create no
column.  The sum of the four unprojected labels must equal `phi_*(X)` and the
sum of projected labels must equal `[4]phi_*(X)`.

## Exact n=9 instance

Use the deterministic sparse irreducible modulus returned by
`find_irreducible_sparse(9)`.  For the target curve the trace recurrence
`s_0=2`, `s_1=-1`, `s_i=-s_(i-1)-2s_(i-2)` gives
`#E(F_(2^9))=2^9+1-s_9=508=4*127`.  Select the first canonical nonidentity
point of exact order `r=127` as `G`; independently enumerate its 127 multiples
and verify closure and exact order.

Registered exhaustive arms:

1. `transported_k0`: for every `a=0..126`, set `X=phi^*([a]G)` and enumerate
   all 512 constants `c`.
2. `kernel_coset`: deterministically scan rational source points `R`, set
   `D=[R]-[infinity]` and `K=[3]D-phi^*phi_*(D)`, and select the first `K`
   which is nonzero and has certified zero pushforward.  For every `a`, set
   `X=phi^*([a]G)+K` and enumerate all `c`.

Registered controls:

- `ordinary_ambient`: 127 deterministic sums of three rational point classes,
  selected by fixed xorshift64 seed `0x83_09_2026_1118_0001`; enumerate all
  `c` for each class and compute the RHS through the general pushforward.
- `exceptional`: the identity, the pullback of the target two-torsion point,
  the chosen `K`, and the seed divisor `D`; enumerate all `c` and classify
  every degree drop or rejection.
- Every `K=0`, nonzero-kernel and ordinary-ambient class receives a direct
  general-pushforward replay.  `K` must be nonidentity, `phi_*(K)=O`, and the
  seed `D` must not be reported as a zero-kernel class unless its direct
  pushforward is actually zero.

## Factor-base and rank grid

Sort all 126 nonidentity points of `<G>` by canonical encoded `(x,y)` and take
nested prefixes

```text
B in {8, 16, 32, 64, 96, 126}.
```

A split candidate contributes a row at bound `B` only when every nonidentity
projected component label is in that prefix.  Row coefficients are merged
multiplicities modulo 127.  For an input whose direct pushforward is `[t]G`,
the RHS is `4*t mod 127`; for `X=phi^*([a]G)+K`, this is `12*a mod 127`.

Compute deterministic Gaussian rank over `F_127` after each accepted row.
Also solve the full-rank systems without consulting the enumerated label-log
table, then compare the solved values with that table as a verification only.
[pcc:unverified]

For each arm and `B`, emit:

- candidates, exact degree-four representations and rejection counts;
- split candidates and factor-base-contained relations;
- relation probability per candidate and per exact representation;
- raw projected components, distinct merged labels, global distinct labels,
  collision count and collision rate;
- collected rows, nonzeros, independently replayed rank [pcc:unverified], rank/row and
  rank/candidate;
- matrix dimensions and exact CSR byte estimate using 32-bit column indices
  and 16-bit residues, plus row offsets;
- consistency failures and wall-clock time as diagnostic fields only.

## Blind individual logarithms

Only a full-rank `B` is eligible.  Use 16 targets
`Q=[k]G`, where `k` is drawn from xorshift64 seed
`0x83_09_2026_1118_10a1` and is withheld from the relation routine.  For each
target, scan masks `s=0..126` and constants `c=0..511` in order on
`phi^*(Q+[s]G)+K`, stopping at the first factor-base-contained
representation.  Recover

```text
k = inverse(12) * sum_i(m_i*log_G(L_i)) - s  (mod 127)
```

for the nonzero-kernel arm, with the same equation for `K=0`.  Verify every
answer by scalar multiplication.  Emit success count, candidates tried and
all failure reasons.  The planted `k` is used only to construct and finally
verify `Q`.
[pcc:unverified]

## Registered predictions and cost gate

For unstructured degree-four labels, a size-`B` projected factor base gives
approximately `(B/q)^4/4!` useful relations per candidate.  Collecting `B`
rows therefore costs on the order of `q^4/B^3`; sparse linear algebra is at
least `B^2` in this conservative screen.  Balancing gives
`B about q^(4/5)` and work about `q^(8/5)`, which has no crossover with
`r^(1/2)` when `r about q`.

The exact n=9 gate still runs because it can falsify the unstructured-label
model and expose rank defects or unexpected closure.  Advancement to bounded
n=13,19,23 requires all of:

1. nonzero independent rank in an image-compatible arm;
2. at least one full-rank factor-base system and at least 15/16 replayed blind
   individual logs;
3. no reconstruction, pushforward, subgroup or RHS failure;
4. a complete fitted collection + matrix + individual-log model which, with
   conservative confidence bounds and all setup included, predicts a finite
   crossover below the strongest registered source rho.

If condition 4 fails, stop after n=9 and record the structural measurements;
do not run n=13,19,23 or n=83 timings.  If it passes, freeze a dated addendum
with bounded grids and budgets before those runs.  No n=83 run is permitted
without a later frozen finite-crossover prediction.

## Reproducibility and validation

The implementation is a native Rust binary.  Its deterministic JSON output is
checked by a separately invoked validator which recomputes totals, ranks,
matrix byte counts, subgroup equations and recovered logs from emitted sparse
rows.  Required pre-push checks are the binary unit tests, validator tests,
`cargo test --release --lib`, formatting, clippy for touched targets, prose
lint and artifact/size checks.  Scientific fields remain `[pcc:unverified]`
until an independent signed PCC verifier exists.
