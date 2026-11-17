# Does the prime root-finding oracle count as index calculus? A decision memo for the owner

Date: 2026-10-29. Status: decision memo. It is preparation, not measurement. It
reports no new measured result. Each number below is quoted from a file in the
repository, and its path is given. Each statement is labelled **measured**
(with its source), **derived** (arithmetic on measured values, shown), or
**plan**. Background statements about the published literature are labelled
**background**. No number rests on them.

**The decision is the owner's.** The memo sets out the question, the evidence
and the case on each side. It offers three wordings the owner could adopt and
recommends one. It does not edit `AGENTS.md`, `docs/ic/SWEEPER.md`, the
sweeper code or the leaderboard.

Machine-readable classification of every oracle under every wording:
[`classification.json`](classification.json).

> **Status after this memo was written (cross-track review, 2026-10-29).**
> [`../DECISIONS.md`](../DECISIONS.md) records the decision as **R1 = Option C
> (two counted tracks), delegated** by the owner ("use judgment"), to be
> confirmed by merging the PR that carries it. The memo's recommendation
> (Option B) was therefore **not** adopted. The C-column of §5.4 and of
> `classification.json` is the operative classification once that PR merges;
> until then `AGENTS.md` and `docs/ic/SWEEPER.md` still carry the 2026-10-24
> wording (`prime-rootfind` unclassified). `DECISIONS.md` also answers D2
> ("yes, rank the Table B incumbent against the published algebraic
> reference"), which conflicts with Option C's sentence "never ranked against
> each other" (§5.3). That conflict has to be resolved before the
> reference-arm protocol is frozen; see `../review/README.md`.

---

## 0. The question, and the short answer

On 2026-10-24 the owner recorded the following rule
(`docs/ic/SWEEPER.md` "Tracks"; `research/ic_m2_linear_ladder_20261024/REPORT.md` "Criterion"):

> an index-calculus pipeline counts only when its relations come from solving
> summation-polynomial systems over an algebraically defined factor base,
> without table collisions.

The same report lists the excluded table pipelines: the N83 K1120 "PDP4root"
baseline, `engine=rootfind`, the "aimed m = 3" recipe and pair enumeration. The
sweeper put the prime oracle `rootfind-subtract` (search space
`prime-rootfind`) in a third track, `unclassified`. The reason it gives is that
"whether solving the univariate Semaev polynomial per base point counts as
'solving a summation-polynomial system' or as a root table is not decided by
the criterion as recorded" (`docs/ic/SWEEPER.md` lines 150–154;
`src/bin/ic/sweep/space.rs` lines 16–21).

**Recommendation (Option B, §6.1).** Classify `rootfind-subtract` and the other
`rootfind-*` oracles as generic table/enumeration methods, not index calculus.
Adopt a wording that makes the test checkable from code and from a control
experiment. The test: the oracle hands a polynomial system to a solver, the
factor-base constraint is part of that system, and the oracle's relations or
cost would change if the base were replaced by a random set of the same size.
§6 gives the reasons. The main cost of this ruling is that the repository then
has **no counted prime-field index calculus at all**, and so no prime ratchet
baseline, until an algebraic prime oracle (`pkm-semaev` or an FGHR/msolve
arm) completes a verified cold DLP.

---

## 1. What the oracle computes

### 1.1 The algebra

**Background, checked against the code.** For `E: y² = x³ + ax + b` over
`F_p`, Semaev's third summation polynomial, as a quadratic in its last
argument, is

    S₃(x₁, x₂, X) = (x₁ − x₂)²·X² − 2((x₁ + x₂)(x₁x₂ + a) + 2b)·X + (x₁x₂ − a)² − 4b(x₁ + x₂).

These are exactly the coefficients `qa, qb, qc` of `semaev_s3_quadratic` in
`src/cryptanalysis/ic_boundary.rs` (lines 4085–4104). Its two roots are
`x(P + Q)` and `x(P − Q)` when `x₁ = x(P)` and `x₂ = x(Q)`. A unit test in the
same file checks this (around line 10060, "Semaev S₃ roots are the abscissae
of P ± Q").

Fix the target `R` and a base point `F_i = (x_i, y_i)`. The roots in `X` of
`S₃(x_R, x_i, X)` are `x(R − F_i)` and `x(R + F_i)`. By the chord formula,

    λ∓ = (y_R ± y_i) / (x_R − x_i),      x(R ∓ F_i) = λ∓² − x_R − x_i.

So the oracle never has to take the square root of the quadratic's
discriminant: the group law already gives the factorisation, because `y_R` and
`y_i` are known. Both slopes share one inverse, `(x_R − x_i)⁻¹`. Montgomery's
trick turns all `|F_x|` of those inverses into **one field inversion per
target**.

### 1.2 The code: `rootfind-subtract`

`RootfindSubtractOracle`, in `src/cryptanalysis/ic_framework/plugins.rs`
(lines 2238–2521). Per target `R`:

1. Form `x_R − x_i` for every abscissa in the base, and batch-invert them with
   one inversion (`f.batch_inv`; `counters.inversions += 1`;
   `3k` multiplications, lines 2434–2445).
2. For each abscissa, compute both candidates `T = R ∓ F_i` from the shared
   inverse (lines 2457–2477). Each one is **charged as one group addition**
   (`ops.adds += 1`).
3. **m = 2.** Look up each `T` in the factor base by its point key
   (`fb.index_of_key(g.key(&t))`, line 2504). On a hit, re-add `F_i + T` on
   the ledger and check that the sum equals `R` (fail closed, lines 2505–2512).
4. **m = 3.** Probe each `T` against the negation-folded **pair table** of
   `F_j + F_k` (`table.probe`, lines 2514–2517). The table is built in
   `prepare` (lines 2378–2382).

So, as the task card says, **at m = 2 the oracle is one batched inversion plus
`2·|F_x|` hash probes into the base. At m = 3 it is the same inversion plus
`2·|F_x|` probes into a pair-sum table.**

The code describes itself as follows:

- `plugins.rs` lines 2242–2253: "The Semaev `S₃` root in the third variable is
  literally `x(R ∓ F_i)`, so a two-summand sweep over the base needs no
  summation polynomial, no Gröbner basis and no square root … It is the
  `subtract` oracle's search — the same `2·|F_x|` candidate points a target,
  found in the same order — on `prime_fast`'s arithmetic. It is charged
  exactly as `subtract` is."
- `src/cryptanalysis/ic_framework/rootfind.rs` lines 6–7, on `rootfind-mitm`
  and `rootfind-pairs`: "Both decide a target `R` by the group law alone, never
  by an algebraic system."
- `src/cryptanalysis/gpu_rootfind.rs` lines 5–10, the GPU port of the same
  identity: "each `(k, i)` pair is two slope computations and two table
  probes."

### 1.3 The neighbouring oracles

Six oracles find the same relations as `rootfind-subtract` at m = 2, or
something very close to them:

| oracle | where | what it does per target | m |
|:--|:--|:--|:--|
| `rootfind-subtract` | `ic_framework/plugins.rs` 2258 | batched inversion; `x(R ∓ F_i)` for every abscissa; membership probe in the base (m = 2) or the pair table (m = 3) | 2, 3 |
| `subtract` | `ic_framework/plugins.rs` 2003 | `R − P` for every signed base point by an ordinary group addition; membership probe in the base | 2 |
| `semaev_s3_roots` | `ic_boundary.rs` 6350 | for every abscissa `x_i`, form `S₃(x_R, x_i, X)`, solve the quadratic by a square root, test both roots against the base's x-index, lift | 2 |
| `rootfind-mitm` | `ic_framework/rootfind.rs` 161 | m = 2: probe `R` in a pair table of `P_i + P_j`. m = 3: probe `R ∓ P_k` in that table | 2, 3 |
| `rootfind-pairs` | `ic_framework/rootfind.rs` 290 | no table; `U = R ∓ P_i`, then `U ∓ P_j`, then a membership test: enumeration of pairs | 3 |
| `mitm` (Table B's `mitm_m2_negfold_walk*`) | `ic_framework/plugins.rs` 2045; `ic_boundary.rs` | probe `R` (m = 2) or `R − P` (m = 3) in a table of pair sums | 2, 3 |

**Measured: the counted cost is the same.** On the train panel of the
2026-10-29 tuning study
(`research/prime_base_oracle_tuning_20261029/select/band-train-P20.md` and
`band-train-P24.md`; 8/8 verified runs per cell; mean S in GAE/√r, w = 0):

| curve | cell | S (w = 0) | cell | S (w = 0) |
|:--|:--|--:|:--|--:|
| `icv1-fp20-tm1195-26505147` | `rfsub2-n32` | 579.571 | `sub2-n32` | 579.547 |
| `icv1-fp24-tm1013-0c7556ce` | `rfsub2-n162` | 1489.249 | `sub2-n162` | 1489.223 |
| `icv1-fp20-tm1195-26505147` | `rfsub3-n32` | 41.282 | `mitm3-n32` | 41.282 |
| `icv1-fp20-tm1195-26505147` | `rfsub3-n64` | 24.509 | `mitm3-n64` | 24.509 |

At w = 0, `rootfind-subtract` m = 2 costs what `subtract` costs, and m = 3
costs what `mitm` m = 3 costs. The small m = 2 differences (about 0.004%) are
not traced here. (Cross-track review: an earlier draft attributed them to the
native counters, but native work is unpriced at w = 0, where S = G − C in
`ecbench/band.rs`, so that explanation is unverified.) The code says why the
search is the same: it searches "the same candidates in the same
order" and is charged one addition per candidate. The difference shows up only
in work the counted unit does not see, namely the inversions and the wall
time. The planner study found the same thing from the other side: at 24, 26
and 28 bits the `rootfind-subtract` alternative costs **171 [164, 176], 233
[226, 239] and 439 [424, 450]** times the pair-table plan
(`research/ic_planner_validation_20261020/README.md` line 36, **measured**).

**Derived (ratios of train-panel means, unpaired, no interval).** Divide the
cells above by `rho-neg` on the same train curve (1.013 at 20 bits, 1.295 at
24 bits, same files):

- `rfsub2` is about 572× rho-neg at 20 bits (`n32`) and about 1,150× at 24 bits (`n162`).
- The best `mitm2` cell is about 9.9× at 20 bits (`n45`: 10.036) and about 8.1× at 24 bits (`n115`: 10.530).

The `rfsub2` cells were measured at one base size per curve, not tuned, so
they are not the best `rfsub2` can do.

### 1.4 What both sides accept

- **The oracle is exact and complete.** It finds a 2-decomposition `R = ±F_i ±
  F_j` whenever one exists. It returns the same relation set as a Gröbner solve
  of the decomposition ideal `{S₃(x_R, X, Y), ∏(X − x_i), ∏(Y − x_i)}`.
  `RESEARCH_SEMAEV_ROOTFIND_VS_GROEBNER.md` §2 builds exactly that ideal and
  cross-checks it against `groebner_f4::buchberger`, at |F| = 6 because larger
  ideals are out of Buchberger's reach.
- **It moves the constant, never the exponent.** The lead says so
  (`crypto-autoresearcher:knowledge/leads/ecdlp/ic-engine/LEAD-ICE-rootfind-pairtable-oracle.yaml`,
  `mechanism`). So do the generic-table obstruction
  (`…/ic-prime/LEAD-ICP-generic-table-decomposition.yaml`, `obstruction`:
  "Any decomposition oracle that enumerates or table-looks-up factor-base
  elements is a generic time/memory trade-off") and the PKM note's
  linear-algebra floor, which shows no oracle beats rho asymptotically at
  m ≤ 3 (`research/notes/index-calculus/RESEARCH_PKM_TOWER_ORACLE.md` §3.7).
  **The ruling therefore changes bookkeeping and incentives, not what is
  reachable.**
- **Choosing the smallest abscissae buys nothing measurable.** In the
  2026-10-29 study the stride-7919 control base, with the same oracle and size,
  scored 0.809 [0.758, 0.868] of the incumbent at 20 bits (w = 0), against
  0.818 [0.773, 0.866] for the selected small-abscissa base. `ecbench band`
  ranks `stride < sel` in S at every size, i.e. the stride base was no more
  expensive than the small-abscissa base
  (`research/prime_base_oracle_tuning_20261029/README.md`, "Attribution and
  controls", **measured**). That control used `mitm`, not `rootfind-subtract`.
  It shows that the small-x definition of `prime-abscissa` is not something the
  m = 2 search exploits.

---

## 2. The case that the oracle *is* solving a summation-polynomial system (steelmanned)

1. **It computes the complete root set of `S₃(x_R, x_i, X)` for every base
   abscissa.** That is a full solution of the univariate specialisations of
   the summation polynomial. A closed form for a polynomial's roots is still
   root finding. Nobody would say that solving a quadratic with the quadratic
   formula "does not solve the polynomial". The chord formula is a faster
   closed form for this particular quadratic. `semaev_s3_roots`
   (`ic_boundary.rs` 6350) does the same job by the textbook route, with a
   square root (`sqrt_solves`) and an inversion per abscissa. It returns the
   same roots. Calling one of these "solving" and the other "not solving",
   when they differ only by an algebraic identity, is a distinction of
   implementation, not of method.
2. **It decides the same ideal F4 decides.** The repository's own comparison
   frames the root finder as an exact solver of
   `{S₃(x_R, X, Y), ∏(X − x_i), ∏(Y − x_i)}`. It is checked fail-closed by
   requiring that every Gröbner basis element vanish at the returned pair
   (`RESEARCH_SEMAEV_ROOTFIND_VS_GROEBNER.md` §2). A solver is defined by what
   it decides. The only difference here is that it decides much faster than
   the in-house F4. The desk review records that the in-house F4 is itself
   10³–10⁷ worse than a brute grid on PKM systems
   (`research/ic_sweeper_20261024/README.md` §2.5). By that standard the
   in-house F4 is a weak reference, not the definition of "solving".
3. **The factor base is algebraically defined.** It is the zero set of
   `∏(X − x_i)`, and membership is that polynomial evaluated by hashing. The
   binary linear ladder, which the owner's criterion counts, also checks
   membership by computation (a projection onto the subspace) rather than by
   carrying the field equations into a Gröbner basis.
4. **At m = 2 there is no table collision in the excluded sense.** The excluded
   pipelines (PDP4root, `engine=rootfind` on the binary `PairSumTable`, "aimed
   m = 3", pair enumeration) all store or enumerate *sums of base points*. At
   m = 2, `rootfind-subtract` stores nothing but the base itself. Its probe is
   a membership test of one computed root, not a match between the target and
   a precomputed sum. (At m = 3 it does probe a pair table. This argument
   covers only m = 2.)
5. **It is the textbook prime-field decomposition** (background). Semaev's
   original prime-field proposal, and the generic Gaudry/Diem framing, find an
   m = 2 relation by fixing one summand from the base and solving `S₃` for the
   other. That is this oracle.
6. **Excluding it leaves prime fields with nothing to ratchet.** No algebraic
   prime oracle has completed a counted DLP. The sweeper says "no algebraic
   prime space" (`docs/ic/SWEEPER.md` line 168). The leaderboard builder still
   prints "Prime curves have no algebraic oracle in `ecbench`"
   (`scripts/build_ic_leaderboard.py` line 2380), although `pkm-semaev` now
   exists (`ecbench/methods.rs` line 4171). The 2026-10-15 success criterion
   asks for iterative, measured progress. A ruling that counts nothing on
   prime curves removes the prime half of the program's ratchet for as long as
   the algebraic oracles stay unbuilt or blocked. The prime fall-degree study
   is blocked: `research/prime_fall_degree_20261027/RESTART.md` says its raw
   runs are on the owner's Mac and not pushed.

## 3. The case that the oracle is a table collision (steelmanned)

1. **It is `subtract` with a faster inversion.** The code says so
   (`plugins.rs` 2250–2253), charges it as `subtract`, and the measured S
   matches `subtract` (§1.3). Calling the oracle by a new name does not change
   the algorithm. Counting `subtract` as index calculus because its additions
   can be described as roots of `S₃` would be relabelling, or at best
   accounting, in the sense of `AGENTS.md` §3, which says both "are not
   improvements".
2. **The reductio.** If "compute `x(R ∓ F_i)` and look it up in a set"
   solves a summation polynomial, then baby-step giant-step does too: each
   giant step tests whether `x(Q − j·mG)` lies in the baby-step set, which is
   the root of `S₃(x_Q, x_{jmG}, X)` tested against a set. Any collision
   search can be re-described in Semaev's language. A criterion meant to
   separate index calculus from generic methods has to rule this description
   out, or it separates nothing.
3. **The polynomial is never formed and the base's definition is never used.**
   No coefficient of `S₃` appears in `rootfind-subtract`. Only the chord slope
   does. The base enters only as a hash set, so the oracle would run unchanged,
   at the same cost, on any set of `|F|` points. The stride control (§1.4)
   shows that the "smallest abscissae" property does nothing for the
   comparable `mitm` arm. If `∏(X − x_i)` counts as an algebraic definition,
   every finite set is algebraically defined, and the clause "over an
   algebraically defined factor base" becomes vacuous on prime fields. The
   algebraic-membership lead shows why only a tower-style presentation gives a
   prime base low-degree membership. A size-B base defined by one univariate
   polynomial has description degree at least B/3
   (`…/ic-prime/LEAD-ICP-algebraic-membership-factor-base.yaml`, `obstruction`).
4. **The membership probe is a table collision.** The base's x-index is a
   hash table, and a hit is a collision between a computed point and that
   table. The GPU port calls the probes "table probes" (`gpu_rootfind.rs`
   line 10). The criterion says "without table collisions". In the binary
   ladder that counts, a relation comes out of a linear system whose
   *unknowns* range over the subspace. Membership is part of the system, not
   a lookup afterwards.
5. **The cost has a generic shape.** It is `Θ(|F|)` group operations per trial
   at m = 2 (the same 2·|F_x| candidates as `subtract`), and table probes at
   m = 3. That is exactly the shape the generic time/memory obstruction
   bounds. The counted binary method, by contrast, does one linear elimination
   per trial whose size depends on `n`, not on `|F|`
   (`research/ic_m2_linear_ladder_20261024/REPORT.md` "Method").
6. **The name already covers table methods.** `rootfind-mitm` m = 2 is a plain
   pair-table probe of `R`. `rootfind-subtract` m = 3 and `rootfind-mitm` m = 3
   probe pair tables. `rootfind-pairs` is pair enumeration, which the
   2026-10-24 report excludes by name. Only `rootfind-subtract` m = 2 (and its
   GPU port) is borderline. A ruling that rescued it alone would have to
   explain why it counts while `mitm` m = 2, which finds the same relations
   more cheaply, does not.

---

## 4. Consequences of each ruling

The ruling does **not** change the classification of the current Table B prime
rows. All eight are `mitm_m2_negfold_walk` or `mitm_m2_negfold_walk_balanced`,
which is a pair-table probe of `R`, class `accounting` (`docs/ic/leaderboard.json`,
`board[*].factor_base.variant`, **measured**; the table is in
`docs/ic/LEADERBOARD.md` lines 87–110). Under the 2026-10-24 criterion as
recorded, they are already table-track reference rows (`prime-table` is "reference
only", `docs/ic/SWEEPER.md` line 147). The same holds for the 2026-10-29
20-bit result (`prime-abscissa:size=45` + `mitm:negation_folded=1`). §7 raises
this as a separate decision.

| | **Rootfind counts as IC** (Option A, §5.2) | **Rootfind is table/generic** (Option B, §5.1) | **Two counted tracks** (Option C, §5.3) |
|:--|:--|:--|:--|
| Table B prime rows (mitm m = 2) | unchanged: table reference | unchanged: table reference | counted in a *generic-decomposition* track with its own ratchet |
| Leaderboard | needs a new counted prime section of `rootfind-subtract` m = 2 rows. None exist yet: no rootfind row has a `VALIDATION.json`. They would sit far above the reference rows (derived ~572–1,150× rho at 20–24 bits, §1.3) | prime sections relabelled "reference (table/generic)"; one line: "no counted prime-field IC yet"; the stale line at `build_ic_leaderboard.py` 2380 corrected to say `pkm-semaev` exists with no completed row | Table B gets a track label, "generic decomposition: counted in its own track, never ranked against algebraic"; algebraic headline unchanged |
| Prime ratchet baseline | the best verified cold `rootfind-subtract` m = 2 pipeline per size, after a frozen protocol (plan; no such row exists) | **none** until the first verified, cold, complete DLP from an algebraic prime oracle (`pkm-semaev` engine = f4, or an FGHR + msolve arm), however slow; that row is the baseline | generic track: the incumbents (20-bit N = 45 once its second-host replay passes; the planner plans elsewhere); algebraic track: none, as in B |
| Binary ratchet | unchanged: the linear m = 2 ladder is the algebraic incumbent | unchanged | unchanged |
| Sweeper | `prime-rootfind` → `algebraic`. Its space is copied from `prime_table_space` and offers m ∈ {2, 3}, so m = 3 must be removed or kept in `table` | `prime-rootfind` → `table`; `Track::Unclassified` retired | a new `Track::Generic` holding `prime-table`, `prime-rootfind` and `koblitz-table`; `Track::Unclassified` retired |
| What future work counts | rfsub2 base-size tuning, GPU rootfind, batching of the subtraction sweep. **Not** the pair-table constant work in `LEAD-ICP-prime-ic-constant-engineering`, even though it improves the faster oracle on the same relations | prime: only algebraic oracles. `pkm-semaev`, the fall-degree study, an FGHR/msolve reference arm, and possibly small-root lattice decomposition (§7). Rootfind and pair tables become controls and the per-trial cost an algebraic oracle must beat | both lines count, each in its own track. Constant engineering of Table B is a success in the generic track |
| Risk | effort flows into an oracle with a fixed exponent and an Θ(|F|) trial; the BSGS reductio is open; consistency forces `subtract` and `semaev_s3_roots` in too | a long stretch with no prime success to report; pressure to call engineering of reference rows "IC" informally | the recorded 2026-10-24 decision is partly reversed (tables were "reference only"); the headline needs care so that generic-track wins are not read as algebraic progress |

Under every option, **reclassifying a row is accounting, not an improvement.**
No row's `S` or distance to rho changes because of the ruling.

---

## 5. Candidate wordings for `AGENTS.md`

Each wording is drafted to sit under "Success criterion" or "IC measurement
rules" as a short owner rule. The memo does not edit `AGENTS.md`.

### 5.1 Option B — mechanism plus random-base test (recommended)

> **What counts as index calculus** (owner, 2026-10-__). A pipeline counts as
> index calculus only when each relation is produced by a solver applied to a
> polynomial system in which the summation polynomial (or its Weil descent or
> tower presentation) **and the factor-base membership constraint both appear
> as equations or as a parametrisation of the unknowns** (a subspace basis,
> tower variables, a lattice window). The solver may be Gröbner, linear,
> SAT or lattice. A decomposition that enumerates base elements, computes
> group sums or differences, and tests them against a set or a table of
> precomputed sums is a generic table method, **whatever algebraic identity
> describes the computed points**. It is measured and reported as a
> reference, not counted. Audit test: if replacing the factor base by a
> random set of the same size would leave the oracle's relations and counted
> cost unchanged, the oracle is a table method. An algebraic oracle may
> recover solutions by enumeration only after its solver has failed to refute
> the system, and it must report how often that happened.

### 5.2 Option A — univariate root finding at m = 2 counts

> **What counts as index calculus** (owner, 2026-10-__). A pipeline counts as
> index calculus when each relation comes from the roots of a summation
> polynomial over an algebraically defined factor base. The roots may be found
> by a Gröbner, linear, SAT or lattice solve of the system, or, **at m = 2,
> by computing all roots of `S₃(x_R, x_i, X)` in `X` for each base abscissa
> `x_i` and testing each root for membership in the base**. A
> finite set given by its vanishing polynomial is algebraically defined. A
> relation does not count when it comes from probing a table of precomputed
> sums of base points (pair tables, meet in the middle, PDP4root, "aimed
> m = 3") or from enumerating pairs or larger tuples of base points.

Its weak points: it makes `subtract` count, because `subtract` finds the same
roots by ordinary additions. It counts that while `mitm` m = 2, which finds the
same relations more cheaply, does not. And the m = 2 cut-off is not a
principle; it has to be stated by fiat, or `rootfind-pairs` (enumerate one
summand, then root-find the next) would count as well.

### 5.3 Option C — two counted tracks

> **Tracks of index calculus** (owner, 2026-10-__). Index-calculus pipelines
> are reported in two tracks. Each has its own baseline and ratchet, and they
> are never ranked against each other. **Algebraic**: relations come from a
> solver applied to a system that contains both the summation polynomial and
> the factor-base constraint, as in Option B; this is the headline track.
> **Generic decomposition**: relations come from enumerating the base,
> computing group sums or differences and testing them against the base or a
> table of precomputed sums (`subtract`, `rootfind-*`, `mitm`, pair tables,
> large primes on a pair table). Improvements in this track are successes
> under the 2026-10-15 criterion and are labelled "generic decomposition".
> They are never described as algebraic progress, and each row reports its
> distance to matched rho.

### 5.4 Every existing oracle under each wording

**IC** means counted as index calculus. **table** means a reference, not
counted. **control** means kept as a cross-check or cost floor. **generic**
means counted in Option C's generic track.

| oracle (source) | what it does | B (5.1) | A (5.2) | C (5.3) |
|:--|:--|:--|:--|:--|
| `rootfind-subtract` m = 2 (`ic_framework/plugins.rs` 2258) | batched inversion, `x(R ∓ F_i)`, base membership probe | table | **IC** | generic |
| `rootfind-subtract` m = 3 (same) | same, probe in the pair table | table | table | generic |
| GPU rootfind (`gpu_rootfind.rs`, `gpu/rootfind/`) | the m = 2 identity on a GPU, two table probes per (target, slice) | table | **IC** | generic |
| `subtract` (`plugins.rs` 2003) | `R − P` for each signed `P`, membership probe | table | **IC** (same roots as rootfind m = 2) | generic |
| `semaev_s3_roots` m = 2 (`ic_boundary.rs` 6350) | forms `S₃`, solves the quadratic by a square root, membership probe | table (enumerates the base; fails the random-base test) | **IC** | generic |
| `rootfind-mitm` m = 2 (`ic_framework/rootfind.rs` 161) | probe `R` in a pair table | table | table | generic |
| `rootfind-mitm` m = 3 (same) | probe `R ∓ P_k` in a pair table | table | table | generic |
| `rootfind-pairs` m = 3 (`rootfind.rs` 290) | enumerate pairs, membership probe on the third summand | table | table (pair enumeration) | generic |
| `mitm`, `mitm-prime-batch`, `mitm-prime-blocked`, `mitm-fiber`; Table B `mitm_m2_negfold_walk*` | pair-sum table, probe `R` or `R − P` | table | table | generic |
| binary pair tables: `pdp3-koblitz` `engine=rootfind`, `rootfind-negfold`, `rootfind-window*`; `mitm-frobenius*` (with large primes) | `PairSumTable` or `PairTable` lookups | table | table | generic |
| linear S₃ Weil descent, m = 2 (`examples/ic_m2_linear.rs`; `research/ic_m2_linear_ladder_20261024`) | linear system in `(x₁+x₂, x₁x₂)` over the subspace, linear split | **IC** | **IC** | **IC** (algebraic) |
| `descent-algebraic` with F4-family or SAT engines (`plugins.rs` 2829); `pdp3-koblitz` `engine=inherited-f4` / `f6-ic` | Weil descent of S₃/S₄ over a subspace, handed to a solver | **IC** | **IC** | **IC** (algebraic) |
| `pkm-semaev` `engine=f4` on `prime-kummer` (`ic_framework/pkm_semaev.rs`) | `S_{m+1}` with tower variables `y_{i,j}² = y_{i,j+1}`, F4 over the tower ring; grid recovery only after a non-refutation | **IC**, provided `budget_exceeded` (grid-decided) systems are reported, and a row where the grid decides most systems is read as a control | **IC** | **IC** (algebraic) |
| `pkm-semaev` `engine=exhaustive` (same) | every tuple of `V^m` tested against `S_{m+1}` | control (enumeration) | table (tuple enumeration) | control |
| PKM tower oracle `pkm-tower` (planned; `RESEARCH_PKM_TOWER_ORACLE.md` §4.3) | as `pkm-semaev`, with `prime-tower` bases (Kummer, Dickson, 2-isogeny) | **IC** (plan) | **IC** (plan) | **IC** (plan) |
| small-root lattice decomposition (lead `LEAD-ICP-coppersmith-smallroot-semaev`, not implemented) | Coppersmith-style small roots of `S_{m+1}` over a window base | **IC** if the owner accepts a window as an algebraic parametrisation (the wording above does) | **IC** | **IC** (algebraic) |

---

## 6. Recommendation

### 6.1 Adopt Option B

The recommendation is Option B (§5.1). Rule `rootfind-subtract`, together with
`subtract`, `semaev_s3_roots` and the GPU port, a generic table method: measured
and reported as a reference, not counted as index calculus.

### 6.2 Why

1. **It matches what the code says and does.** The oracle's own documentation
   says it "needs no summation polynomial, no Gröbner basis", is "the `subtract`
   oracle's search", and is "charged exactly as `subtract` is"
   (`plugins.rs` 2242–2253). Its sibling's documentation says the `rootfind-*`
   oracles decide "by the group law alone, never by an algebraic system"
   (`rootfind.rs` 6–7). The measured S equals `subtract` at m = 2 and `mitm` at
   m = 3 (§1.3). Counting it would add a counted prime row without changing the
   algorithm. That is the accounting move `AGENTS.md` §3 says not to claim.
2. **It keeps the 2026-10-24 criterion meaningful.** Under Option A the clause
   "algebraically defined factor base" constrains nothing on prime fields,
   because any finite set has a vanishing polynomial. The BSGS reductio (§3,
   point 2) also stays unanswered. Option B's random-base test gives the clause
   content. It can be checked from code and by a control the program already
   runs (the 2026-10-29 study ran a stride control, though not a random-base
   control; its README records "Random-base control: absent").
3. **It treats the same relations the same way.** Under B, `subtract`,
   `rootfind-subtract`, `semaev_s3_roots` and `mitm` (which find the same m = 2
   relations) all land in one class. Option A has to split them.
4. **Nothing reachable is lost.** All sides agree the oracle cannot change the
   exponent, and no m ≤ 3 oracle can beat rho asymptotically (§1.4). The ruling
   only decides which work the program calls index-calculus progress. Option B
   points that label at the only lines that could move an exponent: tower
   presentations, solving-degree behaviour, and lattice small roots.
5. **The rule is auditable.** A reviewer can classify a new oracle by reading
   its `decompose`. Does a solver receive a system containing the base
   constraint? If in doubt, run it on a random base of the same size.

### 6.3 What Option B costs

The prime half of the program has no counted baseline today. Its first counted
row will probably be far worse than the reference rows, given the desk review's
finding on the in-house F4 (§2, point 2). That is the honest position under the
recorded criterion, not an artefact of this ruling.

### 6.4 If the owner wants countable prime progress now

Choose Option C, not Option A. C keeps the generic oracles together and gives
them a ratchet without calling them algebraic. A splits oracles that find the
same relations.

---

## 7. Decisions this memo raises but does not settle

1. **Table B and the 2026-10-29 result under the 2026-10-24 criterion.** As
   recorded, the criterion already makes the eight Table B prime rows, and the
   new 20-bit `mitm` baseline, table-track reference rows (§4). Two documents
   still read pair-table gains as IC successes:
   `LEAD-ICP-prime-ic-constant-engineering.yaml` ("Each verified gain is an
   improvement over the current IC baseline") and
   `research/prime_base_oracle_tuning_20261029/README.md` ("new prime-field IC
   baseline at 20 bits"). The owner should confirm one of two readings: those
   are reference-track improvements (Options A and B), or they count in a
   generic track (Option C). This is independent of the rootfind question.
2. **Small-root lattice decomposition.** Is a window base an "algebraically
   defined" base? Option B's wording says yes, through "a lattice window" in
   the parametrisation clause. Strike that phrase if the owner disagrees.
3. **`pkm-semaev` grid recovery.** After F4 fails to refute a system, the
   oracle reads the solutions off the grid (`pkm_semaev.rs` header). Option B
   allows this, provided the rate of systems decided on the grid
   (`budget_exceeded`) is reported. The owner may prefer a stricter threshold.
4. **Two corrections for the owner or the atlas maintainer (not made here).**
   (Cross-track review: the first one has since been made in
   `crypto-autoresearcher:knowledge/leads/ecdlp/ic-engine/LEAD-ICE-rootfind-pairtable-oracle.yaml`,
   `mechanism`. The second, in `scripts/build_ic_leaderboard.py`, is outside
   the program's write scope and is still open.)
   - `LEAD-ICE-rootfind-pairtable-oracle.yaml` says root finding "is now the
     default oracle in every Table A/B row". The Table B prime and
     random-binary rows record `mitm_m2_negfold_walk*` variants, which are pair
     tables (`docs/ic/leaderboard.json`), so the sentence reads as root finding
     and pair tables taken together.
   - `scripts/build_ic_leaderboard.py` line 2380 says "Prime curves have no
     algebraic oracle in `ecbench`", but `pkm-semaev` is wired in
     `ecbench/methods.rs` line 4171. The accurate statement is that it has no
     completed row.

---

## 8. Next actions after each ruling (plan)

- **After B.**
  - Move `prime-rootfind` to `Track::Table` and retire `Track::Unclassified` (`src/bin/ic/sweep/space.rs`).
  - Add the wording to `AGENTS.md`.
  - Relabel the prime sections of the leaderboard as reference, and add the "no counted prime IC yet" line.
  - Freeze a protocol for the first counted prime row: `pkm-semaev:m=2,engine=f4` on `prime-kummer` at the smallest sizes where it completes. Use matched rho-neg, the §5a band, and the grid-decided rate reported. That row becomes the prime ratchet baseline.
  - Unblock the prime fall-degree study: upload the Mac runs (`research/prime_fall_degree_20261027/RESTART.md`) and add `isolab` support for its non-ecbench spec.
  - Add a random-base arm to the next prime oracle study, so the audit test has measured support.
- **After A.**
  - Move `prime-rootfind` m = 2 to `Track::Algebraic` and drop m = 3 from that space.
  - Freeze a protocol that tunes the base size for `rfsub2`, which has not been tuned (§1.3).
  - Put its rows through the validation gate.
  - State in the leaderboard that `subtract` and `semaev_s3_roots` count on the same terms.
- **After C.**
  - Add `Track::Generic`, holding `prime-table`, `prime-rootfind` and `koblitz-table`.
  - Label Table B as generic.
  - Keep `LEAD-ICP-prime-ic-constant-engineering` as a generic-track lead.
  - Take the 2026-10-29 20-bit row through second-host replay as the generic-track baseline at 20 bits.
  - Start the algebraic prime track as in B.

---

## Sources read

- `src/cryptanalysis/ic_framework/plugins.rs` (lines 1991–2521: `subtract`, `mitm`, `mitm-prime-*`, `rootfind-subtract`; 2802–2915: `descent-algebraic`)
- `src/cryptanalysis/ic_framework/rootfind.rs` (`rootfind-mitm`, `rootfind-pairs`)
- `src/cryptanalysis/ic_framework/pkm_semaev.rs` (header)
- `src/cryptanalysis/gpu_rootfind.rs` (header)
- `src/cryptanalysis/ic_boundary.rs` (module table, lines 60–99; `semaev_s3_quadratic`, 4085; `decompose_prime`, 6340)
- `src/cryptanalysis/ecbench/methods.rs` (prime oracle dispatch, 4160–4200)
- `src/bin/ic/sweep/space.rs` (tracks, `prime_table_space`, `prime_rootfind_space`)
- `docs/ic/SWEEPER.md` ("Tracks", "What it cannot do yet")
- `docs/ic/LEADERBOARD.md` (Table B, planner validation, bitop/v1 ladder); `docs/ic/leaderboard.json` (Table B variants)
- `research/ic_m2_linear_ladder_20261024/REPORT.md`
- `research/ic_sweeper_20261024/README.md` (§2.4–2.5, §6 D1)
- `research/notes/index-calculus/RESEARCH_PKM_TOWER_ORACLE.md` (§0, §1, §2.4, §3.7)
- `RESEARCH_SEMAEV_ROOTFIND_VS_GROEBNER.md` §2
- `research/prime_base_oracle_tuning_20261029/README.md`; `select/band-train-P20.md`, `band-train-P24.md`
- `research/ic_planner_validation_20261020/README.md`
- `research/prime_fall_degree_20261027/RESTART.md`
- `AGENTS.md` ("Success criterion", §3 classes)
- `crypto-autoresearcher:knowledge/leads/ecdlp/ic-engine/LEAD-ICE-rootfind-pairtable-oracle.yaml`
- `crypto-autoresearcher:knowledge/leads/ecdlp/ic-prime/LEAD-ICP-prime-ic-constant-engineering.yaml`, `LEAD-ICP-generic-table-decomposition.yaml`, `LEAD-ICP-algebraic-membership-factor-base.yaml`, `LEAD-ICP-coppersmith-smallroot-semaev.yaml`

Not done here, because this is a desk memo with no run: the `research-visuals`
graph and PDF. If the memo lands in a PR, the classification table in §5.4 is
the natural figure.
