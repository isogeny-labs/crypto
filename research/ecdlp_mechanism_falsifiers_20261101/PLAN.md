# ECDLP mechanism audit campaign

Status: **planning record — no experiment has run and no closure is claimed**

Date: 2026-11-01

Target repository: `isogeny-labs/crypto`

Backlog handoff: `backlog/XP-20261101-ecdlp-mechanism-falsifiers/`

## 1. Requested outcome and evidence contract

The campaign tests the proposed Holant/GAG, elliptic-code decoding, implicit
`k`-SUM, low-degree algebraic-query, interpolation, modular-curve,
stabilizer-rank, digit-lift, ordering-oracle, and quadratic-twist claims.

Experiments may:

- refute a universal statement by returning a verified counterexample;
- prove an identity or finite-boundary result with a symbolic or exhaustive
  certificate;
- support or refute a stated finite-range scaling model.

Experiments may not promote a finite null result to an asymptotic lower bound,
prove a complexity conjecture from fitted exponents, or close a mechanism
family outside the declared parameter and algorithm boundary.

Before any scientific row, split the relevant item into a focused branch and
freeze `PROTOCOL.md` plus an `experiment.protocol/v1` `protocol.json`. Implement
cryptographic research instruments in native Rust. Supported x86 measurement
jobs run through Iso Lab on EKS Spot; counted searches use the queue and its
checkpoint contract. Preserve the exact command, repository revision, source
and binary hashes, manifest, seed list, dependency lock, hardware manifest,
stdout/stderr, exit status, wall/CPU time, peak RSS, raw observations, unified
`experiment.export/v1`, and independent review receipt. This plan and its draft
card do not reserve hardware, create a lease, or authorize a scientific run.

Suggested common manifest fields:

```yaml
experiment_id: EXP-...
claim_ids: []
hypothesis: ""
finite_scope: ""
instances: []
seeds: []
budget: {}
controls: []
measurements: []
correctness_checks: []
invalidation_rules: []
falsification_rule: ""
support_rule: ""
promotion_rule: "coordinator review required"
```

Use at least 32 independent curves per scaling bucket when the bucket is not
an exhaustive finite family. Split curve classes and seeds 60/20/20 into
development, validation, and held-out sets before tuning.

## 2. Atlas reconciliation and adversarial corrections

The supplied note was checked against a 154-lead snapshot. The current
crypto-autoresearcher checkout reports 159 leads, and two proposed “missing”
mechanisms now have direct rows. This PR does not edit programme state; it
records the current homes so a later evidence PR advances them instead of
creating duplicates.

| Mechanism | Current atlas home or nearest row | Consequence for this campaign |
|---|---|---|
| GAG/Holant presentation | `LEAD-ICE-gag-contraction-planner`; tensor neighbor `LEAD-EXO-hashed-factor-base-tensor-ecdl` | Run semantic and presentation tests; do not file `SO-HOLANT` from a toy null. |
| Elliptic-code decoding | no exact title match; nearest decomposition and hint-family rows | Preserve as a candidate experiment; atlas placement is a later crypto-autoresearcher review. |
| Implicit EC `k`-SUM | `LEAD-ICP-generic-table-decomposition`, `LEAD-EXO-dl-character-sum-pseudorandom` | Extend the existing native `k=4/5` evidence and the merged prime-field base/null census rather than rebuilding either baseline. |
| Bounded algebraic queries and interpolation | `LEAD-EXO-coordinate-statistic-leakage`, `LEAD-EXO-invertible-hint-subset`, `LEAD-EXO-augmented-oracle-ggm` | Search for counterexamples and state exact model boundaries. |
| `X_1(N)` / diamond operators | `LEAD-STR-level-structure-torsor` | Execute that row's finite checks; genus growth alone is not a closure proof. |
| Stabilizer-rank simulation | `LEAD-EXO-quantum-ecdlp-resources` | Price a named circuit and simulator only. |
| Number-ring digit lift | `LEAD-TRF-global-lift-small-height`, `LEAD-TRF-kernel-of-reduction-lemma` | Measure compatibility defects and height; do not restate the existing global-lift theorem as a new result. |
| Actual `x` ordering | nearest row `LEAD-EXO-augmented-oracle-ggm` | Test model separation because actual coordinate order is not an opaque random ordering. |
| Quadratic-twist sharing | existing `tools/twist-relations` studies and curve-model rows | Reuse the native instrument and separate polynomial identity from rational relations and cost. |

The experiment design deliberately attacks these overclaims in the supplied
falsifiers:

1. GAG completeness is semantic completeness. Its Gröbner/Nullstellensatz
   normalization and #P-hard worst case do not lower-bound this Semaev family.
2. Projection from points to abscissae changes multiplicities and exceptional
   points. Characters diagonalize full group convolution; they need not be
   eigenvectors of the stated `x`-indexed matrix.
3. Cai--Chen's complex-weight #CSP dichotomy uses Block Orthogonality,
   Mal'tsev, and Type Partition conditions. “Support is not an affine
   subspace” is not the paper's complete tractability test.
4. Cheng proves the subset-sum/codeword correspondence and hardness results;
   it does not prove that Guruswami--Sudan returns empty lists on the proposed
   window base. The list-size formula and decoder radius must be derived and
   checked under the exact divisor convention.
5. A worst-case or explicit-set `k`-SUM conjecture does not automatically
   apply to one fixed, implicitly generated EC window set. Conversely, flat
   additive statistics do not prove worst-case hardness.
6. A degree bound must charge the pullback through scalar multiplication,
   coefficients/advice, adaptivity, and evaluation cost. Syntactic coordinate
   degree alone is not the oracle's algebraic complexity.
7. Lange--Winterhof's exact theorem text is a prerequisite. The accessible
   Kiltz--Winterhof paper gives interpolation bounds for named ECDH maps and
   cites the COCOON result; neither abstract licenses a closure for every bit,
   statistic, rational function, or advice table.
8. Explicit `X_1(N)` model size and a Bravyi--Gosset simulation formula are
   costs of named methods, not lower bounds on every representation or
   classical simulator.
9. A digit lift failing one curve equation is an exact compatibility defect,
   not by itself the height obstruction. Twist-invariant `S_m` over the
   algebraic closure likewise does not identify the two rational factor bases.

The six collision items named in the supplied note remain out of scope for new
runs. Their existing rows are referenced, not re-reviewed: CM transport,
window-base additive energy, spectral/Jennrich tensors, x-only isogeny walks,
split-Jacobian projection, and the secp256k1 GLV lattice. In particular,
`EXP-KSUM-STRUCTURE-02` imports the frozen global energy/second-moment evidence;
it measures only missing local-clustering diagnostics and their correlation
with algorithmic cost.

## 3. Requirement-to-evidence matrix

| Claim family | Primary experiment | Strongest possible output |
|---|---|---|
| Semaev tensor is a projected Cayley operator | `EXP-HOLANT-SEMANTICS-01` | Exact identity or explicit counterexample |
| Tensor has exploitable low rank/basis | `EXP-HOLANT-RANK-02` | Exact finite ranks and bounded transform search |
| GAG/Holant gives a proof closure | `EXP-HOLANT-REWRITE-03` | Representation comparison only; no general lower bound |
| GS/list decoding yields no usable relations | `EXP-ECODE-LISTS-01`, `-DECODER-02` | Exact small list census and scoped scaling evidence |
| EC factor base is implicit hard `k`-SUM | `EXP-KSUM-BAKEOFF-01`, `-STRUCTURE-02` | Algorithmic counterexample or scoped empirical frontier |
| Algebraic query advantage is bounded | `EXP-ALGQ-FIXED-01`, `-ADAPTIVE-02` | Counterexample or exhaustive finite-family certificate |
| Lange–Winterhof closes bounded-degree leakage | `EXP-INTERP-REPRO-01` | Exact theorem reproduction plus scope counterexamples |
| `X_1(N)` is dead by representation size | `EXP-X1-COST-01` | Verified finite construction/cost curve |
| Stabilizer rank makes Shor simulation dead | `EXP-ZX-COST-01` | Cost of named simulator/circuit only |
| Digit lifts fail as a transfer | `EXP-LIFT-COMPAT-01` | Exact defect and bounded-height nonexistence certificate |
| Actual `x` ordering is generic | `EXP-ORDER-ABLATION-01` | Direct model-separation counterexample |
| Twist invariance has no single-target value | `EXP-TWIST-SHARING-01` | Exact identity plus matched end-to-end cost |

## 4. Holant/GAG/Semaev experiments

### EXP-HOLANT-SEMANTICS-01 — exact operator identity audit

**Claim tested.** The matrix

\[
M[y,y'] = \#\{x\in F_B:S_3(x,y,y')=0\}
\]

is the factor-base Cayley adjacency projected to `x`-coordinates and is
diagonalized by group characters.

**Instances.** Ordinary short-Weierstrass curves over prime fields with
8–16-bit primes. Include cyclic prime-order groups, cyclic composite-order
groups, noncyclic groups, curves with rational 2-torsion, and curves without
rational 2-torsion. For each curve use thresholds
`tau=floor(p^beta)` for `beta in {0.20, 0.25, 1/3, 0.50, 0.75}` and record the
actual factor-base cardinality.

**Construct and compare four operators.**

1. Full group convolution on `E(F_p)`, including `O` and point multiplicity.
2. Its exact restriction to even functions under `P -> -P`.
3. The Kummer-orbit operator with explicit exceptional-point weights.
4. The matrix `M` exactly as stated, both on valid `x`-coordinates and on all
   field elements.

**Measurements.** Entrywise differences, exceptional rows, exact rational
rank, determinant when square, characteristic polynomial, eigenvalue
multiplicity, and residuals for characters and symmetrized characters. Verify
every `S_3=0` witness by lifting signs and checking the group relation.

**Controls.** Full convolution must diagonalize under the finite Fourier
transform. A random symmetric matrix must not. A Kummer operator with the
derived multiplicity and infinity corrections must match the even restriction.

**Decisive outcomes.** One verified mismatch refutes the literal identity.
Exact equality for all exhaustively enumerated curves in a finite range proves
only that finite range. Character residuals below exact-arithmetic zero are
required; floating-point closeness is not a certificate.

### EXP-HOLANT-RANK-02 — exact and approximate rank frontier

**Claim tested.** The Semaev-chain tensors or transfer matrices have a hidden
rank/bond-dimension reduction usable by contraction.

**Protocol.** For `m in {3,4,5}` and the same curves:

- compute exact matrix-flattening ranks over `Q` and over at least three
  unrelated certificate primes;
- compute tensor-train ranks for each variable ordering;
- report singular spectra and numerical ranks under preregistered relative
  thresholds `2^-20`, `2^-30`, and `2^-40`;
- measure exact contraction time, arithmetic-operation count, peak memory,
  and reconstruction error;
- compare direct sparse contraction, variable elimination, FFT on the full
  group, Kummer/Fourier contraction, and the repository's best Boolean/#SAT
  tensor baseline.

Do not report CP-rank nonexistence from optimizer failure. Flattening ranks are
lower bounds; decomposition output must reconstruct the tensor exactly or at
the declared approximation tolerance.

**Controls.** Affine/parity tensors with known Fourier tractability, random
tensors with matched sparsity, and the existing hashed-factor-base tensor.

**Falsification rule.** A verified basis/decomposition with total relation cost
below the matched baseline on held-out curves refutes the blanket “no tensor
escape” claim. Full ranks support only the tested flattenings and sizes.

### EXP-HOLANT-REWRITE-03 — bounded holographic and rewrite search

**Claim tested.** No holographic basis makes the tensor tractable, and GAG
rewriting reduces to the measured solving degree.

**Search boundary.** Exhaust all invertible basis changes for domains of size
2 and 3. For larger domains search only named families: character/Fourier,
Kummer cosine, monomial/permutation, sparse block, and seeded random unitary or
finite-field bases. Record the family and budget; never call this complete.

For each finite signature, evaluate Cai--Chen's actual three tractability
conditions where their fixed-domain complex-weight model applies. Record any
domain-size or uniform-language mismatch that prevents use of the dichotomy;
do not replace those checks by the weaker assertion that the support is not an
affine subspace.

For each representation—Semaev ideal, GAG-style polynomial network, direct
tensor, full group convolution—measure normalization size, treewidth,
elimination order, maximum intermediate degree, solving degree, proof/rewrite
length when available, contraction work, and memory.

**Controls.** Known affine, product-type, matchgate/Holant, and deliberately
hard signatures. The classifier must recognize its positive controls before a
negative Semaev result is valid.

**Outcome labels.** `transform_found`, `not_found_within_family_and_budget`,
or `classifier_invalid`. No result from this experiment alone is a lower bound.

## 5. Elliptic-code list-decoding experiments

### EXP-ECODE-LISTS-01 — exact list-size and first-moment census

**Claims tested.** The expected-hit formula, the Johnson/GS parameter squeeze,
and “empty lists everywhere except at full rate.”

**Instances.** Prime fields from 8–16 bits; ordinary curves with known group
structure; `D={P:x(P)<tau}` with actual `B=|D|` between 8 and 48 for exhaustive
runs. Sweep `k=2,...,B-1` and

\[
M=\lfloor\sqrt{Bk}\rfloor+1.
\]

Use `L(kO)` and `L(MO)` bases from the host's native curve/function-field
implementation. Keep divisor degree, code dimension, evaluation injectivity,
and strict-versus-nonstrict decoder radius explicit.

**Measurements.**

- exact `N_M`, the number of distinct `M`-subsets of `D` summing to `O`;
- exact or exhaustive average list size over `a in L(MO)` where feasible;
- observed list-size distribution for uniform `a` otherwise;
- comparison with `(q-1) N_M q^(k-M)` and with the originally claimed formula;
- zero, duplicate, and nonzero divisor-function counts;
- verified relations per received word and per field operation.

Use subset-sum dynamic programming as a counting cross-check, not as the
claimed decoder. For every decoded pair verify `g=a-f`, its divisor, all
evaluation agreements, and the resulting group sum.

The displayed first moment is a hypothesis to check, not an input to the
verdict. The implementation must first derive the exact count under its divisor
and quotient-by-scalars convention; exhaustive enumeration decides any missing
automorphism, zero-function, or injectivity factor.

**Controls.**

- planted `a=f+c*g_S` from a known relation `S`;
- a received word with no relation, established by exhaustive enumeration;
- genus-zero Reed–Solomon fixtures for the decoder;
- a random linear code with matched `(q,B,k)`;
- a dense evaluation set where Li–Wan–Zhang predicts existence.

**Decisive outcomes.** A planted relation that is not returned invalidates the
decoder implementation. A random-list rate exceeding the corrected first
moment outside preregistered confidence intervals refutes the model. An empty
finite census does not prove asymptotic emptiness.

### EXP-ECODE-DECODER-02 — end-to-end GS/list-recovery/ISD cost

Scale to 16–24-bit fields and `B` up to 512, subject to the host budget. Compare:

1. Guruswami–Sudan with all valid multiplicity parameters;
2. the Zhang–Liu construction exactly as published;
3. list recovery if implemented;
4. information-set decoding with setup, unsuccessful trials, and verification;
5. direct meet-in-the-middle subset sum.

Report setup, per-word decoding, relation verification, duplicates, independent
rank contribution, memory, and downstream linear algebra separately. Use
random, planted, and adversarially selected received words. Count only new
verified independent relations.

**Attack-revival gate.** The method must beat the matched relation-generation
baseline after setup amortization and verification on held-out curves. A stage
speedup alone is not sufficient.

## 6. Implicit EC `k`-SUM experiments

### EXP-KSUM-BAKEOFF-01 — algorithm and representation bake-off

**Claim tested.** The implicit `x`-window set behaves like an unstructured
explicit set and offers no sub-meet-in-the-middle algorithm.

Generate actual `x`-window sets, uniformly random subsets of the same group,
known-scalar arithmetic progressions, and planted mixtures. Do not expose the
random-subset scalar labels to the tested algorithms.

Sweep group sizes from 16–40 bits and factor-base sizes `2^6` through the
largest feasible size. Include `beta in {0.20,0.25,0.30,1/3,0.40}` and record
the expected number of target representations so empty-instance effects are
not mistaken for algorithmic failure.

Compare pair-table meet-in-the-middle, sorted/streamed pairs,
Schroeppel–Shamir-style low memory, Pollard-style collision search,
repository-native relation search, character/FFT methods where their input is
legitimate, and clustered methods only after their prerequisites are verified.
The `ic.ksum` `k=4/5` arms and their frozen native sessions are the baseline;
new code must replay those controls rather than substitute a fresh prototype.

Report group operations, field operations, table probes, bytes, setup, online
time, target success probability, and cost per verified relation. Fit exponents
only after fixing the size interval and algorithm; provide bootstrap intervals
and raw points.

**Falsification rule.** Reproducible scaling below the conjectured exponent on
held-out EC window sets, without hidden scalar labels or uncharged
preprocessing, refutes the stated EC-specific conjecture in the measured
range. It does not refute or prove ordinary worst-case `k`-SUM.

### EXP-KSUM-STRUCTURE-02 — multiscale structure challenge

Begin with a coverage table for the frozen window-base additive-energy,
second-moment, character-spectrum, and coincidence-harvest evidence. Treat
those values as imported covariates, not new measurements. Measure only the
uncovered diagnostics: local energy across scales, pair-sum bucket tails,
BSG-style dense subgraphs, `x`-interval clusterability, and the same quantities
after stratifying by quadratic character, sign, endomorphism orbit, and
`x`-prefix. A representation-compatibility replay of a frozen global statistic
must reproduce the original fixture and is not reported as a new result.

**Controls.** Uniform random sets; full arithmetic progressions; generalized
arithmetic progressions; random sets with a 1%, 5%, or 10% planted structured
subset; matched-energy sets with different local clustering. Diagnostics that
cannot detect the planted controls are invalid for “no structure” claims.

Correlate each statistic with actual algorithmic cost from
`EXP-KSUM-BAKEOFF-01`. A null correlation is reported as such, not as an
obstruction proof.

## 7. Low-degree algebraic-query theorem program

### EXP-ALGQ-FIXED-01 — exact fixed-test census

For a prime-order subgroup generated by `P`, enumerate every secret `k` for
groups up to roughly `2^20`, set `Q=[k]P`, and form

\[
X_i=[a_i]P+[b_i]Q=[a_i+b_i k]P.
\]

Test preregistered polynomial families of syntactic total degree
`D in {1,2,3,4,6,8}` in coordinates of one to four `X_i`. Stratify scalar
coefficients `b_i` as small, `sqrt(N)` scale, and uniform modulo `N`.

For each test record:

- syntactic degree and number of monomials;
- coefficient/advice size and evaluation operations;
- degree of the pullback through each multiplication map, using division
  polynomials or a certified upper bound;
- exact zero set and probability;
- bias against each explicitly named baseline;
- maximum bias selected on development curves and replicated on held-out
  curves.

**Controls.** Generic equality tests with known root counts; identically zero
group-law relations; planted high-bias tests; random polynomials; and tests
whose syntactic degree is small but pullback degree is deliberately large.

Predefine the big-O constant to be tested. “Bias exceeds `D/sqrt(N)`” is not a
valid falsifier of an unspecified asymptotic `O` bound.

### EXP-ALGQ-ADAPTIVE-02 — bounded adaptive decision trees

Build a finite query library validated by `EXP-ALGQ-FIXED-01`. For small
groups, exhaust all decision trees up to the feasible `(T,D)` boundary; for
larger groups use a preregistered beam search and label it incomplete.

Each leaf stores the set of consistent secrets. For a uniform secret, report
the optimal one-guess success probability, transcript entropy, leaf-size
distribution, construction/evaluation cost, and comparison with

\[
\min(1,C_1T^2/N+C_2TD/\sqrt N)
\]

for fixed registered constants and for both syntactic and pullback degree.

**Decisive outcomes.** One verified tree above the bound refutes that precise
finite statement. Exhaustion below the bound proves only the finite query
library and `(T,D,N)` range. A heuristic tree search cannot prove the bound.

## 8. Interpolation theorem scope

### EXP-INTERP-REPRO-01 — exact ECDL/ECDH interpolation frontier

**Precondition.** Archive the complete Lange–Winterhof theorem statements,
including subgroup, coordinate, degree, agreement, and characteristic
hypotheses. Do not start from abstracts. The identified source is Lange and
Winterhof, COCOON 2002, DOI `10.1007/3-540-45655-4_16`; only a paywalled
preview was accessible during this design review, so the card cannot become
`ready` until a lawful complete copy is read and the theorem is transcribed
with page/theorem numbers.

For small curves and prime-order subgroups, construct monomial evaluation
matrices and solve exactly for the minimum degree/weight of polynomials that:

1. interpolate the complete ECDL map in the paper's representation;
2. interpolate the complete ECDH map;
3. agree on only a specified fraction of inputs;
4. predict one registered bit/statistic of the logarithm;
5. use rational functions or a short adaptive list of polynomials, if those
   variants are in scope.

Sweep 7–16-bit fields, with exhaustive exact linear algebra. Use Lagrange
interpolation as a positive existence control and degrees immediately below
the published bound as negative controls. Reproduce the theorem's numerical
fixtures before exploring variants.

**Output.** Exact minimal degrees in the tested monomial model, nullspaces,
rank certificates, agreement curves, and explicit counterexample polynomials
when found. A low-degree statistic refutes the proposed extension from exact
interpolation to all leakage; it does not contradict the original theorem.

## 9. Quick-kill experiments

### EXP-X1-COST-01 — modular-curve representation audit

For prime `N` from 5 through the largest constructible value, use checked Magma
scripts or a native Rust implementation to construct `X_1(N)`, its map to
`X_0(N)`, and diamond actions. A native verifier independently checks exported
equations, genera, maps, and action identities; no Python/Sage research path is
introduced.
Verify

\[
g(X_1(N))=(N-5)(N-7)/24
\]

in the valid prime range. Record model construction, coefficient size,
equation count/degree, memory, diamond evaluation, fiber enumeration, and
recovery of the scalar up to sign. Include the correct base point
`(E,<P>) in X_0(N)`.

Controls: known genus-zero and genus-one levels, plus direct ECDLP enumeration.
This yields a cost curve for explicit models, not a lower bound on every
implicit modular representation.

### EXP-ZX-COST-01 — named stabilizer-rank simulator accounting

Compile the same toy EC-Shor circuits to Clifford+T under exact seven-T and
ancilla/postselection four-T Toffoli decompositions. Verify the quantum output
for the largest directly simulable instances. Record qubits, Toffoli count,
T-count, Clifford count, output width, approximation error, and observed
Bravyi–Gosset simulation cost.

Use resource-estimator outputs at 32, 64, 128, 224, and 256 bits to compute the
published simulator's modeled cost without calling it a lower bound. Keep the
`2^30` and `2^38.98` Toffoli circuit estimates separate and identify which
circuit each belongs to.

Controls: a same-size Clifford circuit and a published hidden-shift fixture.
The conclusion is restricted to the named compilation and simulator.

### EXP-LIFT-COMPAT-01 — pseudo-Mersenne digit-lift compatibility

For secp256k1 and scaled toy analogues, define the number ring and reduction
map explicitly. For every toy point and sampled production points, compute the
canonical digit lifts and the exact defect

\[
delta_P=\tilde y_P^2-\tilde x_P^3-\tilde A\tilde x_P-\tilde B.
\]

Verify reduction of `delta_P` to zero; record whether it is zero in the ring,
its coefficient height, norm/ideal valuation, and expression in the reduction
ideal. Compare lift/add/reduce with reduce/add/lift, including denominators and
height growth.

On toys, exhaust corrections `(Delta A,Delta B)` up to a registered height
`H` and return a certificate when no single corrected curve contains the
selected lifted points. Scale `H` before seeing results. On production
parameters, solve the linear compatibility equations and report residuals and
modeled heights; do not claim universal nonexistence.

Controls: points obtained by reducing one genuine characteristic-zero curve;
independently lifted points; coordinate-isomorphic models; and random digit
representatives. Complete transfer cost includes construction, correction,
group operations, recovery, verification, and failed searches.

### EXP-ORDER-ABLATION-01 — actual `x` order versus random encodings

Run identical bounded-query algorithms under three interfaces:

1. opaque uniformly permuted encodings with lexicographic comparison;
2. actual affine `x`-coordinates with equality only;
3. actual affine `x`-coordinates with order/comparison.

Repeat the random encoding over at least 100 independent permutations per
curve. Record transcript distributions, information gain, best-guess success,
and full cost. Use the same affine-combination query library as
`EXP-ALGQ-ADAPTIVE-02`.

A reproducible distinction between interfaces 1 and 3 refutes “actual
`x`-ordering is GGM-simulable.” Failure to find an advantage does not make the
oracles equivalent.

### EXP-TWIST-SHARING-01 — twist identity and reusable-work audit

Extend `tools/twist-relations`; do not create a second twist arithmetic or
runner. First replay its exhaustive `S_3` group-law control and preserve the
existing failed checkpoint-gate attempt as infrastructure evidence, not a
scientific row.

For paired models `E:y^2=f(x)` and `E^d:d*y^2=f(x)` with nonsquare `d`:

- verify `S_m` identity symbolically for `m=3,4` and by independently generated
  evaluation tests for `m=5`;
- enumerate rational `x`-sets and verify their quadratic-character partition,
  including 2-torsion exceptions;
- count polynomial-system solutions, rational lifted relations, duplicates,
  and independent relation-matrix rank on each side;
- benchmark shared Gröbner preprocessing, separate solves, sign/rationality
  filtering, linear algebra, and logarithm recovery.

Compare single-target `E`, single-target twist, and dual-target workloads.
Amortize setup only when the identical artifact is reused. Controls: square
`d` (isomorphic/full-transfer positive control), nonsquare `d`, and an unrelated
curve with matched size.

Any verified end-to-end single-target advantage refutes “only useful when both
logs are wanted.” Polynomial identity alone is not an algorithmic gain.

## 10. Execution order and stop rules

### Phase A — exact counterexample/correctness gates

Run `HOLANT-SEMANTICS-01`, the small boundary of `ECODE-LISTS-01`,
`ALGQ-FIXED-01`, `INTERP-REPRO-01`, and the identity portions of
`LIFT-COMPAT-01` and `TWIST-SHARING-01`.

Stop a dependent experiment if its positive control fails, an arithmetic
identity is not independently verified, the divisor/code parameters violate
injectivity, or a transcript uses hidden scalar labels unavailable to the
algorithm. Preserve the failed receipt.

### Phase B — bounded searches and algorithm bake-offs

Run `HOLANT-RANK-02`, `HOLANT-REWRITE-03`, `ECODE-DECODER-02`, both `KSUM`
experiments, and `ALGQ-ADAPTIVE-02`. Freeze algorithms and tuning before the
held-out set.

### Phase C — representation and end-to-end cost

Run `X1-COST-01`, `ZX-COST-01`, the full lift transfer audit, ordering ablation,
and twist sharing. Report construction, online work, recovery, and verification
separately.

## 11. Promotion gates

| Proposed status | Required evidence |
|---|---|
| `refuted` for an exact claim | Independently verified counterexample and reproduction receipt |
| `proved_finite_scope` | Exhaustive scope declaration plus checkable certificate |
| `supported_cost_model` | Held-out scaling, controls, uncertainty, and complete cost accounting |
| `not_found` | Search family, parameter boundary, seeds, and budget explicitly recorded |
| `closed_obstructed` | External theorem or new proof with exact hypotheses; experiments alone are insufficient |
| `beats_rho` | Correct end-to-end matched comparison including setup, memory, verification, linear algebra, and recovery |

No toy result changes a canonical scoreboard automatically. Every recommended
status change requires the host's independent review and Coordinator workflow.

## 12. Delivery status and sources checked

| Deliverable | Status in this PR | Remaining gate |
|---|---|---|
| Adversarial claim audit | complete as a design review | Any changed atlas disposition belongs in crypto-autoresearcher after evidence. |
| Fifteen experiment specifications | planned, not attempted | Split into focused branches; implement native producer and independent verifier. |
| Controlled protocols | not frozen | Preflight each instrument, justify power/sample size, write `protocol.json`, and freeze before data. |
| Scientific execution and export | not attempted | Source-matched EKS image/route, checkpoint gate, raw rows, `experiment.export/v1`. |
| Proof or closure | not established | Exact theorem with matching hypotheses or a new checked proof; experiments alone do not suffice. |

Primary sources read for this design:

- Gao, Shaikh and Kissinger, [*Graphical Algebraic Geometry*](https://arxiv.org/abs/2605.13993), especially semantic completeness, #CSP encoding, and Theorem 7.2's worst-case hardness. Repository reading: crypto-autoresearcher `knowledge/literature/KN-LIT-c9e59d.md`.
- Cheng, [*Hard Problems of Algebraic Geometry Codes*](https://arxiv.org/abs/cs/0507026), especially the elliptic-code subset-sum correspondence, Theorems 1--3, and its reduction scope.
- Li, Wan and Zhang, [*On the Minimum Distance of Elliptic Curve Codes*](https://arxiv.org/abs/1501.01138), especially Proposition 4.1 and the dense-evaluation-set existence regime.
- Cai and Chen, [*Complexity of Counting CSP with Complex Weights*](https://arxiv.org/abs/1111.2384), especially its three tractability conditions; it does not supply the proposed affine-support shortcut.
- Chan and Lewenstein, [*Clustered Integer 3SUM via Additive Combinatorics*](https://arxiv.org/abs/1502.05204), especially the explicit clustering/BSG prerequisites.
- Lange and Winterhof, [*Polynomial Interpolation of the Elliptic Curve and XTR Discrete Logarithm*](https://doi.org/10.1007/3-540-45655-4_16). Bibliographic identity was verified; full text was inaccessible here and remains a recorded gate.
- Kiltz and Winterhof, [*On the Interpolation of Bivariate Polynomials Related to the Diffie--Hellman Mapping*](https://doi.org/10.1017/S0004972700036042), especially Theorems 5--6 and its statement that they extend Lange--Winterhof's Theorem 3.
- Kohel and Shparlinski, [*On Exponential Sums and Group Generators for Elliptic Curves over Finite Fields*](https://doi.org/10.1007/10722028_24), as a scoped exponential-sum input rather than an adaptive-query theorem.
- Bravyi and Gosset, [*Improved Classical Simulation of Quantum Circuits Dominated by Clifford Gates*](https://arxiv.org/abs/1601.07601), including the named sampling upper bound with exponent about `0.23 t`.

Repository evidence read before designing extensions:

- [`research/prime_fourier_flatness_exp7_20261007/RESULT.md`](../prime_fourier_flatness_exp7_20261007/RESULT.md): six predicate cells on 8--14-bit curves; no adjusted candidate and no universal flatness conclusion.
- [`research/prime_ksum_k5_20261028/README.md`](../prime_ksum_k5_20261028/README.md) and [`research/prime_ksum_pooled_20261029/README.md`](../prime_ksum_pooled_20261029/README.md): native `k=4/5` generic-decomposition baselines and their validation limits.
- [`research/ic_prime_eth_leads_20261031/SYNTHESIS.md`](../ic_prime_eth_leads_20261031/SYNTHESIS.md): merged skeptic reconciliation for twelve prime-field leads, including size-matched base/null and shared-window second-moment evidence; it reports no exponent movement and preserves the PKM/contractor frontiers.
- [`problems/hashed-ecdl-tensor-backend.md`](../../problems/hashed-ecdl-tensor-backend.md): general tensor correctness/complexity audit remains open despite bounded native controls.
- [`quantum_circuit_secp256k1/README.md`](../../quantum_circuit_secp256k1/README.md): current named circuit resource counts and their finite test-island scope.
- [`tools/twist-relations/README.md`](../../tools/twist-relations/README.md): native `S_3` controls, experiment accounting, and the failed checkpoint-gate attempt that produced no scientific data.

No result, curve identity, benchmark, leaderboard row, progress point, or
canonical dashboard changes in this planning PR.
