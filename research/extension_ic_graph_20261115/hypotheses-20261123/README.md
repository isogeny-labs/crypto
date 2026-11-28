# Four hypotheses after the graph-algorithm catalog review

23 November 2026. Scope: the existing restricted genus-3 cover over
`F_{p^2}` for the lane's weak `F_{p^6}` elliptic family. These are proposed
applications of established techniques, not novelty, performance, or general
prime-field ECDLP claims. No new target, relation-search run, F4 call, or paid
resource was used in this search round. All proposed gains are unmeasured.
Mathematical derivations and producer checks below are **[pcc:unverified]**.

[Report PDF](report.pdf) · [Editable diagram](hypotheses.dot) ·
[Diagram SVG](hypotheses.svg) · [Native identity check](native/src/main.rs)

## What changed the search

- The catalog correctly separates LP rank metadata from payload construction,
  independent projected equations, and target recovery. A graph change
  must reduce one of those actual costs.
- The November 18 scout is not the latest evidence. The November 19
  `l1-paired/results/README.md` already records 96 fresh-target paired panels
  for anchored quadratic acceptance: 555 full splits and 789 additional
  matched rows. Its prespecified promotion interval was inconclusive. That
  is prior producer evidence, not a new measurement or a revalidated claim.
  Repackaging anchored acceptance as a new hypothesis would duplicate it.
- `l5-kernel-split/README.md` records Krylov plus BM as 87.60% of its counted
  linear-stage multiply-adds; NF reductions perform no multiplications on
  that reconstructed stream. Its rule closed NF-pattern replay. A different
  multiplication operator can change arithmetic work; replaying the same NF
  lookup pattern usually cannot. These are prior, unsigned observations.
- The scout already rejected unconditioned collisions between target partials
  and live unanchored components on the p1009 fixture. Neither that mechanism
  nor collecting more anchors after rank readiness is proposed as new here.

Sources for those facts: `../leads-20261118/{scout-graph-ic/LEADS.md,`
`CRITIC_ROUND2.md,l1-paired/results/README.md,l1-steps-2-3/README.md,`
`l5-kernel-split/README.md}`. All inherited findings remain [pcc:unverified].

## H1 — choose a sparser multiplication operator

**Hypothesis.** Another coordinate, or a small fixed linear form, gives a
cheaper *separating* multiplication operator than the currently fixed last
variable. This reduces the work of every Krylov product, rather than only
avoiding symbolic bookkeeping.

`zero_dim_la::fast_inner` fixes `xn = n - 1`. For the same accepted basis,
build candidate matrices for multiplication by the other coordinates. First
use the staircase's unit-shift counts to screen candidates, then record the
actual sparse/dense layout and arithmetic of the remaining candidates. A
coordinate with more unit columns need not win: its remaining columns may
be denser. Do not choose on unit counts alone.

The grevlex-last-variable choice already has generic structural advantages
in Sparse FGLM. This hypothesis needs a useful asymmetry in these particular
Nagao ideals; it does not assume that the existing choice was arbitrary.

**Cost criterion.** Compare matrix construction, products, projections, BM,
root extraction, coordinate read-out, all nonseparating attempts, and the
original fallback. Learning/selection work belongs to the cold interval.
For a sparse implementation the product term depends on quotient dimension
times operator nonzeros, but the current dense-column path must be priced
using its actual representation. A denser separating linear combination may
lose to a slightly less sparse coordinate with fewer fallbacks.

**First falsification.** On a frozen development cohort, try a bounded list
of coordinates, preserving every root set. Select a policy there, then
freeze it before a disjoint cohort. Reject the policy if total linear-stage
work including failures and selection does not fall. A shorter recurrence
is insufficient: verify separation/read-out and use the general fallback
when needed. Do not reinterpret a missing root as a valid negative answer.

**Rust landing point.** Opt-in action-variable argument in `zero_dim_la`,
with an explicit coordinate-to-read-functional map; keep the F4 basis and
graph collector unchanged. Current `reads[xn] = NF(1)` convention must be
updated coherently. This is different from the closed NF-replay candidate.

**Prior art.** Faugère–Mou, *Sparse FGLM algorithms*, §§2–3, ties work to
multiplication-matrix sparsity and separates shape-position from general
cases. Larsson et al. 2018 studies basis selection for action-matrix solvers.
Their application results do not predict a gain on these Nagao ideals.

## H2 — compile a syzygy-pruned elimination template

**Hypothesis.** The repeated Nagao family admits a smaller fixed elimination
template that constructs the required multiplication operator directly,
without running the full sequence of F4 reductions for every residual.

Existing trace replay selects useful F4 rows. The proposed step is different:
reuse provenance to seek the monomial multiples needed for action-matrix
relations, reduce their representations modulo syzygies, and compile the
resulting coefficient matrix. Online work fills coefficients from the public
residual and performs guarded modular elimination. Select the quotient basis
and action operator together with H1.

**Cost criterion.** Charge template generation and validation, coefficient
evaluation, elimination fill, inversions, action-matrix construction, root
recovery, and fallback. Compare against the existing six-coordinate trace,
not only the original untraced engine. A smaller template can still do more
arithmetic after fill. Full symbolic rational functions in every residual
coefficient are not required, and may cause the parameter explosion warned
about by Joux–Vitse §3.1.

**First falsification.** Use one fixed development template with explicit
input-polynomial provenance. Test ordinary, singular-pivot, degree-drop and
nonseparating inputs against an unrestricted reference. Reject if the
template plus failures costs at least as much as traced F4 on a disjoint
cohort, or if the quotient/completeness obligations cannot be discharged.
Checking returned roots alone establishes neither completeness nor a correct
empty answer; the 64-monomial stopping rule is not a general certificate.

**Rust landing point.** A separate solver callback through
`jv_nagao_unit::decompose_using`; reuse `Poly`, field arithmetic, trace
provenance, materialization and exact group replay. No production default
changes until qualified. This does not duplicate gain-forest PR #4905 or
the graph-catalog integration lane #4929.

**Prior art.** Larsson–Åström–Oskarsson 2017, §§2–3, explicitly constructs
templates over prime fields from polynomial provenance and syzygies. The
2018 basis-selection paper gives additional choices. The numerical
eigenvalue stage in computer-vision applications is replaced here by exact
finite-field root recovery; neither numerical stability nor a generic
training instance is a correctness certificate for exceptional inputs.

## H3 — switch from homogeneous collection to affine descent earlier

**Hypothesis.** Some expensive tail of homogeneous graph collection can be
replaced by a few additional cheap affine descents, especially after H1/H2.
The optimal stopping condition need not be a one-dimensional homogeneous
kernel.

Let the certified homogeneous matrix on the factor-base variables have
nullity `k`, with a complete kernel basis `K`. Write the unknown log vector
as `K t`. Each replayed descent becomes an affine equation in `t` and the
protected target `x`. The current code takes `k = 1` and maintains two unknowns.
A generalization retains all `k` nuisance coordinates and the target.

**Conditional cost model.** Compare `C_collect(k) + C_kernel(k) +`
`C_descent(k) + C_recovery(k)`. If every descent produces an independent
usable row with a stable probability `q_k`, a rough model is `(k+1)c_k/q_k`
for the descent part. This is an explicit modeling assumption: rank
dependence, retries and changed anchor coverage can invalidate it. Do not
substitute a raw success count for independent affine rank.

In the consistent projected system `C t + d x = a`, the target is uniquely
determined when `rank([C|d]) = rank(C) + 1`. Full recovery of every nuisance
coordinate is not required. An equivalent witness has `y^T C = 0` and
`y^T d != 0`, yielding `x = (y^T a)/(y^T d)`. These are ordinary linear
algebra derivations, not a promise that fewer descents suffice.

**First falsification.** Retain certified collection checkpoints with, for
example, `k` in `{1,2,4,8}`. On development data compare the saved tail with
the added descent and kernel costs. Fix one stopping policy before the
disjoint test. Reject when the saved collection cost fails to pay for the
extra target-useful rows. Unlike the old later-stop/extra-anchor proposal,
this deliberately accepts a larger homogeneous nullspace.

**Rust landing point.** Generalize `jv_homogeneous_quotient` only in an opt-in
adapter and use the existing counted affine system with `k+1` columns.
The full kernel span and all retained LP witness substitutions require exact
audit; one nonzero kernel vector is insufficient at `k > 1`. Never pin an
unknown factor-base log to one. The existing `k=1` two-row requirement is
preserved, not bypassed.

## H4 — sieve a larger family for one fixed residual

**Hypothesis.** A residual-specific positive-dimensional family can amortize
algebraic setup over many candidate functions. Its candidates may be much
cheaper than solving a new square Nagao system for each walked residual.
This is the highest construction risk of the four hypotheses.

Use the existing monic degree-seven cover `y^2 = H(x)` and a validated
reduced residual `R = (u,v)` with `deg(u)=3`, `deg(v)<=2`. Set

```
W = (H-v^2)/u,
C = x^2 + c1*x + c0,
L = l2*x^2 + l1*x + l0,
A = u*L - C*v,
S = C^2*W + 2*C*v*L - u*L^2.
```

Then `C^2 H - A^2 = u S`. The certificate in `norm-quotient.identity.json`
and exact integer expansion in the Rust helper concern this polynomial
identity only. With `C` monic, `S` is monic of degree eight. Five free
`F_{p^2}` coefficients give ten `F_p` coordinates; requiring eight lower
coefficients of `S` to have zero imaginary part supplies eight quadrics.
The naive expected dimension is two. **The constraints may be dependent,
singular or have the wrong rational-point behavior; dimension counting
does not establish a rational parametrization, useful fibers, or a cheap sieve.**

Geometrically the functions lie in `L(11 infinity - R)`, rather than the
current degree-nine pole space that leaves a degree-six quotient. When a
valid quotient splits, the principal function supplies a relation involving
the residual and eight factor-base points. Supported quadratic factors can
instead be substituted by replayed graph anchors, using the existing signed
Mumford descriptors. Divisibility, leading coefficient, ramification,
disjointness, invertibility of `C`, principal-divisor identity and projected
subgroup/target replay remain obligations; no new cover transfer is inferred.

**Break-even model.** In a uniform squarefree-polynomial cycle-type model,
full splitting drops from `1/6!` to `1/8!`: the candidate generator would
need to be more than 56 times cheaper per admissible coefficient solution
before setup, for full-only descent. This is a substantial hurdle.
With independent uniform coverage `f` of the quadratic class domain, the
conditional acceptance models are

```
q6(f) = (1 + 15*f + 45*f^2 + 15*f^3)/720
q8(f) = (1 + 28*f + 210*f^2 + 420*f^3 + 105*f^4)/40320.
```

The cost per admissible coefficient solution `g8` must satisfy
`g8/g6 < q8/q6`, with cold family setup, failed fibers and materialization
included. These are asymptotic random-polynomial/independent-coverage
heuristics, not measured probabilities for this family. In particular the
existing p1009 anchor fraction cannot be transferred without a domain and
distribution check. Several outputs of one family may be dependent.

**First falsification.** Construct the coefficient forms and establish the
actual dimension and exceptional strata for fixed public residuals. Seek a
bounded fiber description with cheap specialization. If its generation and
enumeration cannot meet the stated inequality, or does not produce enough
independent target equations, reject it before a large run. Failure to find
one parametrization is not proof none exists. Do not assume the quadratic
one-parameter sieve in Joux–Vitse §3.2 applies: there `C` is constant and
the bilinear structure is stronger.

**Rust landing point.** First a separate form builder beside `weil_forms`,
then a frozen component control using existing `Fq`, polynomial arithmetic,
group replay and graph witness interfaces. H4's builder and sieve are not
implemented by this search; the helper only qualifies the norm identity.

## Evaluation order and requirements

| Priority | Candidate | Immediate information to obtain | What would stop this lead |
|---|---|---|---|
| 1 | H1 operator choice | Actual product work and separation/fallback rate | No reduction after all selection and read-out work |
| 2 | H2 compiled template | Template shape, fill, provenance and cold amortization | Template/fallback work is no cheaper than traced F4 |
| 3 | H3 earlier switch | Certified checkpoint nullities and marginal costs | Added affine equations cost more than the saved tail |
| 4 | H4 fixed-residual sieve | Actual family dimension, fibers and cost per coefficient solution | Family fails the split-probability break-even condition |

Each proposed implementation is Rust. The next component controls are not
runtime evidence; any later performance comparison retains the requested
Runpod backend, complete costs, fresh targets, matched reference and existing
admission gates. This search does not consume or restart an old protocol.

| User requirement | Status in this round |
|---|---|
| Find additional hypotheses | Four conditional proposals with explicit falsifiers |
| Reuse prior results and avoid duplicates | Bounded source/literature review; retrieval gaps below |
| Rust rather than Python science | Rust identity helper and reused native checker |
| Implement/measure all candidate solvers | Not attempted by this hypothesis-search round |
| Paid Runpod comparison | Existing funding/checkpoint prerequisites unchanged |

## Sources and retrieval limits

- Primary Joux–Vitse 2012 §§3.1–3.3, read as full text:
  <https://www-fourier.ujf-grenoble.fr/~viva/research/articles/CoverDecomp.pdf>.
  SHA-256 `7ca88c54ebfcb6fac8d05d313bfe8c26d3cb133081c55c4ab6ea4c1b45729b38`.
  Distinguishes fixed relation families from per-residual systems and warns
  about treating every divisor coefficient as a formal parameter.
- Faugère–Mou, *Sparse FGLM algorithms*, 2013 author preprint §§2–3:
  <https://arxiv.org/pdf/1304.1238>. The report's 2017 journal citation is a
  later version; this search reads the linked preprint, without transferring
  its generic sparsity assumptions to the Nagao family.
- Larsson–Åström–Oskarsson 2017 §§2–3:
  <https://openaccess.thecvf.com/content_cvpr_2017/papers/Larsson_Efficient_Solvers_for_CVPR_2017_paper.pdf>.
  The web reader returned 403; direct native download and pdftotext worked.
- Larsson et al. 2018 §§1–2:
  <https://arxiv.org/pdf/1803.04360>. Quotient-basis choice is an established
  optimization; the source explicitly does not promise a smallest template.
- Source inspection at parent `b68f8e9570`: `zero_dim_la.rs::fast_inner`,
  `jv_cover.rs::weil_forms`, `jv_nagao_unit.rs::decompose_using`,
  `jv_extension_deferred_pair_dlp.rs` rank-readiness/two-column affine path,
  the preceding lead notes and `RANK_STOP_REVIEW.md`.
- **Sources checked (KB): unavailable.** No KB tools exposed; wrapper
  `scripts/research-kb status` returns 127. Requested exact-mechanism,
  negative-form and adjacent-method queries cannot run. Lead-atlas
  `knowledge/leads/ecdlp/README.md` could not be decoded by the connected
  reader. Conductor is absent (127). This is not a repository-wide absence
  or novelty audit. Ownership pointers #4905/#4929 are preserved.

The source-linked hypothesis diagram is explanatory, not a measured speed
plot. Existing canonical scoreboards and historical graphs are unchanged:
this round supplies no new solver measurement or admitted ranking.

## Native identity qualification

The helper includes the existing `identity_certificate.rs` checker and a
SHA-256 adapter. It also expands both sides coefficient-by-coefficient over
the integers with checked arithmetic. The certificate's sampled evaluation
is only over the prescribed Mersenne prime; integer expansion is a separate
producer check. Neither is a signed PCC verdict or a dimension theorem.
The omitted-cross-term negative control must be refused.

```
export CARGO_TARGET_DIR=/tmp/ic-hypotheses-native-target
cargo test --release --offline --locked --manifest-path native/Cargo.toml
cargo run --release --offline --locked --manifest-path native/Cargo.toml -- issue
cargo run --release --offline --locked --manifest-path native/Cargo.toml -- check norm-quotient.identity.json
```

Run from this directory. Use a fresh output path for any newly issued copy;
do not overwrite the committed identity or the old learning-audit evidence.
Validation receipts and the generated certificate are retained beside this
report. No Python scientific implementation is introduced.

Producer qualification: 11 native tests pass, including exact integer
coefficient expansion and refusal of an omitted cross term; strict scoped
Clippy passes. Certificate `IDC1h3a70da000d31ad8a` passes the reused checker
at its 32 sealed points and 16 additional producer-supplied points. This
is not an independent or signed verdict. [pcc:unverified]

The initial helper compile failed because the new caller used incorrect
expression-constructor names and omitted the checker's `hex` dependency.
The original failure log is preserved under `validation/`; the corrected
build reuses the existing API. No solver trial was retried or relabeled.
