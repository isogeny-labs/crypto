# Red-team review: IC claim-tier definitions (draft)

Reviewer: Red Team (red-team scope). Date: 2026-11-15.

## What was reviewed, and its standing

- `docs/ic/SUCCESS_CRITERIA.md`, SHA-256 `83879186a0ffa446dd73cf6ddb003711f01fbf0c924daec3a1d35e6b3aaff19c`
- `docs/ic/success_criteria.json`, SHA-256 `de4a16209ba03868bc1d074fe2fc70af0fca1c643c03cf625aee05016951b204`
- Plus the uncommitted pointer edits in `AGENTS.md` (lines 303-316) and `CLAUDE.md`.
- Base commit of the checkout: `121e62e45795cc2f9ec6ff575e3a85ff31c1bb28`.

These are working-tree-only, uncommitted drafts. They are reviewed here as a
**proposed rule text**, not as durable evidence. Nothing below refers to a
committed artifact for the draft itself. The findings apply to the bytes with
the hashes above. If the text changes, the findings must be re-checked.

Context read: AGENTS.md "Success criterion" (267-301), "Tracks of index
calculus" (318-343), "Intermediate-cost..." (487-548), §1-§8b (1024-1608),
§11, §12, §15 (1811-2037), "IC measurement rules" (2156-2203), "Sabotage is
assumed" (222-265); `docs/VALIDATION_GATE.md`; `docs/bounds/README.md`;
`docs/ecbench/README.md` (pricing table); `research/band_wcal_20261113/NOTE.md`;
`research/ic_positive_control_20261112/RESULTS.md`;
`problems/section5a-band-cannot-show-a-known-ic-win.md`; `docs/curves/ic/curves.yaml`.

Labels used: **wrong** means the text states something false or contradicts a
governing rule. **unsupported** means a rule is accepted without the evidence
it needs. **unpriced** means a cost goes uncharged or is transferred to
another arm. **loophole** means a producer can claim a tier without the
substance. **impossible** means the criterion cannot be met as written.
**missing control** means a needed control is absent.
[V] = verified here by reading the repository or by computation (scripts were
run in this session; numbers are reproduced below). [B] = believed from
literature memory; there was no literature database in this session. Confirm
these with `research-kb` before relying on them.

---

## Ranked summary (most severe first)

| # | sev | label | one line |
|--:|:--|:--|:--|
| F1 | critical | wrong + loophole | The precedence clause plus "IC measurement rules" (online clock *after* reusable setup) lets T1 be claimed with the precomputation excluded. |
| F2 | critical | unsupported + loophole | T2 is decided on the slope of the total cost. A hidden steep phase or a fixed cost passes the fit and the +8-bit held-out. Verified: fitted α = 0.250 for a pipeline whose 1%-share phase grows as r^0.68; the held-out misses by only 10%. |
| F3 | critical | missing control | There is no preregistration mechanism and no multiplicity control over paths, variants, instances, targets, weights and held-out intervals. The repository's own commitment+beacon and Holm machinery is not invoked. |
| F4 | critical | loophole | "Every automorphism" is not the same as "cheapest rho at each weight". Frobenius can be read out (11.4x, 3.5 bits on ECC2K-130). At w=1 an automorphism rho can cost more than plain rho, and set-up-heavy rho variants inflate the reference. |
| F5 | high | wrong (vs §12) | Planted targets are allowed. AGENTS.md §12 requires public targets and a cross-host-class audit for vs-rho claims. |
| F6 | high | wrong + impossible | The positive control is cited as the T1 precedent, but under the draft's own rule 2 it is "unresolved". T1 (band and w_cal) cannot pass the known win. `w_cal` exists only for prime fields and is biased by the GAE implementation. |
| F7 | high | unpriced | "Every phase priced" has no mechanism. Algebraic-solver work is uncounted in `ic.pipeline`, and transfer arms record `total_gae: 0`. Lower-bound records can pass the band. |
| F8 | high | ambiguous: impossible or loophole | "All of T1's evidence rules at each size": read literally, T2 requires beating rho at all six sizes. |
| F9 | high | wrong (vs AGENTS.md) | T1 at one size contradicts "growing-size checks" (Success criterion, §2, §15). |
| F10 | high | impossible + loophole | T2 is infeasible on the ECC2K-130 reference family: only m = 41, 83, 97 fall in 32 <= b < 100 [V]. The relaxations are unstated. Cofactor drift and γ(b) along the path bias α. |
| F11 | high | unsupported + definition | T3 model selection is powerless [V], and the T3 definition excludes Diem-type and quasi-polynomial bounds [B]. |
| F12 | medium | wrong | The size bands conflict with ecbench bound tiers (field-bit tiers; fits spanning tiers are refused). T0 is missing from medium and challenge. "b >= 48" is inconsistent. JSON `max_size_b:48` is inverted. |
| F13 | medium | loophole/unpriced | Transfer charging: "charged once to the first curve" amortizes across panels; field-only precomputation; a single conversion factor; weak-class and known-literature reproductions are counted as tiers. |
| F14 | medium | wrong/overstated | Transfer table: undefined q and missing genus caveat in the GHS row; MOV row omits quasi-polynomial small characteristic; anomalous condition; genus-3 row attribution and coverage [B]. |
| F15 | medium | missing control | Statistics: estimator unspecified; pseudo-replication of the IC set-up; optional stopping; failures and timeouts; heavy tails; A/A, positive and negative controls. |
| F16 | medium | loophole | T2 held-out: interval width is free, power is low, the weight of the fit is unspecified, and the held-out instance can be pre-measured. |
| F17 | medium | loophole | Rho extrapolation: the threshold is subjective ("b >= 64"; measured rho is cheap there), extrapolation crosses tiers, which T1 rules it replaces is unstated, and 32 targets at the challenge is unreasonable. |
| F18 | medium | missing control | Tracks are not mentioned. A generic-decomposition T2/T3 would contradict the generic-group lower bound and should be treated as an accounting error. |
| F19 | medium | unpriced | Memory is ignored, but the bounds frontier ranks ops and memory together (a "trade"). |
| F20 | medium | loophole | "Family" is undefined. Instance selection and weak-instance checks for direct IC are missing, and §15 calls for a known faster attack as a reference arm. |
| F21 | low | gap | pcc standing (verified, not merely cited) is not required. T1 has no independent review. |
| F22 | low | gap | T3 heuristic tests have no pre-declared numeric criteria. Prime-field "T3 scrutiny" is undefined and understated. |
| F23 | low | consistency | Rho S: AGENTS §2 "S ≈ 1.3" against the draft's formula (0.886 with negation). |
| F24 | low | form | The JSON is not machine-checkable. pcc lint-prose gives 8 hits. Literature statements are unverified. |

---

## Findings in detail

### F1 (critical; wrong + loophole). Precomputation can be excluded through the precedence clause

**Location.** SUCCESS_CRITERIA.md lines 8-11 ("Where this file and those
disagree, they win") and 30-33 ("One target, cold (IC measurement rules): the
primary cost covers everything ... including all precomputation"). JSON line 5
`targets`. Against AGENTS.md "IC measurement rules" lines 2167-2175 and
2184-2187.

**Problem [V].** The draft attributes the cold, all-precomputation interval to
"IC measurement rules". Those rules say the opposite. They read: "Start the IC
online clock when target-dependent computation begins, after reusable
target-independent base, index, relation-log, or solver setup is ready ...
Report reusable setup separately". Their primary ratio is
`rho_online_ms / ic_online_ms` (wall time, which conflicts with §6 too). The
cold reading comes from the Success criterion ("end-to-end `S` (every phase,
cold)"), §8 and docs/bounds/README.md line 28-30. AGENTS.md is therefore
internally split. The draft's precedence clause hands the choice to the
producer.

**Failure scenario.** Take an IC with `r^{2/3}` target-independent relation
collection and linear algebra and an `r^{1/3}` descent. The producer cites
"IC measurement rules" as the primary interval and the precedence clause
("they win"). At `b = 48` the online interval reads about `2^8` below rho,
while the cold cost is about `2^8` above it. This is the classic
precomputation transfer: the cost moves to a phase the headline does not see.

**Minimal fix.** Replace lines 30-33 with:

> - **One target, cold.** Every tier claim (T1 to T3) uses the cold
>   end-to-end interval: from the instance to a verified `[k]P = Q`, with all
>   target-independent setup (factor base, relation collection, linear
>   algebra, solver set-up, transfer construction) charged in full to that one
>   target. This is the end-to-end `S` of AGENTS.md "Success criterion" and §8.
>   The online interval of AGENTS.md "IC measurement rules" (setup reported
>   separately) is reported as its own row labelled `online-only` and never
>   supports T1 to T3. A cost amortized over `t` targets is a separate row that
>   names `t` and is never the headline.

Replace lines 10-11 with:

> ... and changes none of them. For tier claims the cold end-to-end interval of
> section 1 governs; another reading in those documents (for example the online
> interval of "IC measurement rules") is reported beside it, never instead of
> it. Any other disagreement is fixed in this file in the PR that finds it, and
> no claim relies on the disagreement meanwhile.

AGENTS.md "IC measurement rules" itself needs an owner ruling to say the same.
That is out of the Red Team's scope; it is flagged here for the Coordinator.

**Cheapest test.** Have the Coordinator answer one question: "Is a T1 claim
with setup excluded admissible under AGENTS.md as written?" Today the answer
is yes, by the text cited above.

---

### F2 (critical; unsupported + loophole). T2 passes a hidden steep phase or a fixed cost

**Location.** Lines 91-95 (fit) and 100-103 (held-out). JSON T2
`alpha_ci95_below_rho_alpha_ci95` and `per_phase_fits`.

**Problem.** The tier is decided by the slope of the **total** cost. The
per-phase fits are required, but their results have no consequence. A phase
that is small at the measured sizes but steep is invisible to the total
slope. A size-independent set-up flattens the total slope; docs/bounds/README.md
§4 says so itself: "Fixed costs pull `α` below the law at small sizes". The
+8-bit held-out has little power against either effect.

**Verified numerically [V]** (python script run in this session; the model is
an assumption, the arithmetic is exact). Cost `C(b) = 2^{0.25 b} + K·2^{0.68 b}`,
with `K` set so that the steep phase is 1% of the total at `b = 56`. Six sizes
`b = 32 ... 56`, noiseless:

| quantity | value |
|:--|:--|
| total-fit α over 32-56 | **0.250** (interval would lie well below 1/2) |
| held-out `b = 64`: actual / predicted | **1.099** (passes any plausible interval) |
| steep phase's share at 64 / 72 / 80 | 0.098 / 0.541 / 0.927 |
| steep phase overtakes at | `b ≈ 71.5` |
| actual / predicted at 80 / 96 | 13.6 / 1477 |

This is the AGENTS.md §5 residual-walk failure again: relations grew as
`n^{1/3}` while the linear algebra grew as `n^{0.68}`. The draft would grant
T2 to a method whose asymptotic exponent is 0.68 > 1/2.

**Minimal fix.** Replace item 3 with:

> 3. **The fit**: `ln C = ln K + α · ln r` by `ecbench bound fit` (fit
>    version 2), for the total and for every phase separately, at every weight
>    of the §5a band and at `w_cal`. The claimed exponent is `α*`, the largest
>    of the phases' upper 95% limits. A phase with work on fewer than four
>    sizes has no interval and blocks the claim. T2 requires
>    `α* < 1/2 − δ` at every weight, with `δ` fixed in the preregistration.
>    For every phase, the size at which its fit projects it to reach half the
>    total cost is computed and reported. The held-out size (item 5) lies at or
>    beyond the largest such projected crossover below `b = 128`; otherwise the
>    result is reported as a local slope over the measured range, not as T2.

Add to item 4:

> A fitted exponent below the method's own derived exponent, by more than the
> fit's interval, is unexplained (a fixed cost, a phase not yet dominant, a
> counting gap). The claim is held until it is accounted for.

**Cheapest test.** Feed the T2 checker the synthetic two-phase data above, and
rho records with an added constant set-up. It must refuse both.

---

### F3 (critical; missing control). No preregistration mechanism, no multiplicity control

**Location.** Line 82 ("before any data"), lines 59-60 ("at least 32"),
101-102 ("with an interval"), section 5, and JSON T2
`family_path_declared_before_data`.

**Problems.**
- "Before any data" names no mechanism. Exploratory data on the same curves
  already exist in the repository.
- Nothing limits how many family paths, variants (factor-base sizes, oracles,
  parameter rules), instances or weights are tried before the one that passes
  is reported (garden of forking paths).
- The held-out interval width is the producer's choice (see F16).
- "At least 32" targets allows optional stopping: add targets until the
  interval clears 1.
- The repository already has the needed machinery and the draft does not use
  it:
  - `ecbench challenge commit` plus a later beacon, so the epoch is derived
    and not chosen (docs/bounds/README.md §3.4, lines 226-243);
  - Holm correction (§5, lines 373-378);
  - the append-only pcc ledger.

**Arithmetic [V].** With `K` independent null variants (true ratio about 1),
each tested one-sided at 2.5%, the chance that at least one shows a "T1" is
`1 − 0.975^K`: 22% at `K = 10` and 40% at `K = 20`. Requiring the interval to
clear 1 at every band weight is an intersection-union test. It does not
inflate the error rate across weights, but it does nothing across variants.

**Minimal fix.** Add a section "Preregistration and multiplicity":

> Every T1 to T3 claim is preceded by a preregistration, committed and recorded
> as a pcc claim before the first measured run, naming:
> - the family rule, path, sizes and instance-generation rule;
> - the method and its parameters as functions of size;
> - targets per size, the estimator, its interval and the weights;
> - `δ`, the held-out gap, and the acceptance and abandonment conditions
>   (AGENTS.md §4).
>
> Instances and targets are derived from an `ecbench challenge commit`
> commitment and a later beacon (docs/bounds/README.md §3.4). The held-out
> instance is derived from the hash of the frozen fit and a beacon later than
> it, so it cannot have been measured before.
>
> The preregistration names its family of attempts: every path, variant and
> instance set the thread has preregistered for the same tier. Intervals are
> Holm-corrected over that family (family-wise 0.05), and every attempt,
> failed or abandoned, is listed with its outcome. The sample size is fixed in
> advance. Adding targets after seeing an interval starts a new attempt.

**Cheapest test.** Check that the preregistration's commit precedes, in git
history, the first run archive it covers. Count the thread's attempts in the
pcc ledger.

---

### F4 (critical; loophole). The rho reference is not "the cheapest rho"

**Location.** Lines 34-39. Against VALIDATION_GATE.md lines 10-13
("strongest available ... negation, Frobenius or the automorphism fold") and
docs/bounds/README.md lines 553-560 and 562-580.

**Problems.**
1. "Every automorphism of `E`". The Frobenius of a Koblitz curve is an
   inseparable endomorphism. It acts on `E(F_{2^m})` as a group automorphism,
   but a strict reader can leave it out, giving `γ = 2` instead of `2m`.
   For ECC2K-130 [V]: `√(π r/4) = 2^{64.33}` against
   `√(π r/(2·262)) = 2^{60.81}`, a factor of `√131 ≈ 11.4` (3.5 bits).
   The validation gate names Frobenius; the draft does not.
2. "With every automorphism" is not "cheapest". The band charges the class
   canonicalisation per native operation, so at `w = 1` an automorphism rho can
   cost more than plain rho. The repository has measured this flip
   [V, from docs/bounds/README.md]: on the prime toy domain the order of
   `rho.plain` and `rho.negation` flips at 0.34-0.55 GAE per canonicalisation.
   A Frobenius-orbit canonicalisation over `m = 131` squarings, each charged
   1 GAE, would erase the √131 gain. A producer chooses whichever rho variant
   is most expensive at the binding weight.
3. At small sizes, set-up-heavy rho variants inflate the reference. Per
   docs/bounds/README.md: the strong lockstep rho's set-up is most of its total
   at 2^15-2^21, with α fitting at 0.13; signed-Frobenius runs show tails of
   11.7x and 54x the median.

**Minimal fix.** Replace lines 34-39 with:

> - **The rho reference** is, at each weight of the band and at `w_cal`, the
>   cheapest measured configuration among the registered rho variants on the
>   same instance. The variants are `rho.*` in the ecbench registry: plain,
>   negation, negation with the Frobenius or other automorphism fold, each with
>   and without its set-up variants. Each is run cold on the same targets, in
>   the same binary or process, with its unpriced work priced the same way
>   (§5a).
>
>   An automorphism here is any efficiently computable map that acts on `⟨P⟩`
>   as multiplication by a scalar of small multiplicative order: negation, the
>   Frobenius on Koblitz and other subfield curves, and the order-4 and order-6
>   maps on `j = 1728` and `j = 0` curves. `γ` is the order of the group they
>   generate.
>
>   The idealized expected cost is `√(π r / (2γ))` group operations. Measured
>   rho constants have been 1.06x to 2.22x that floor at lab sizes
>   (docs/bounds/README.md §5, §9), so a measured rho is required wherever it
>   can run (section 3).

**Cheapest test.** At the claimed size, run `rho.plain`, `rho.negation` and
`rho.signed_frobenius(_strong)` through `ecbench band`. Confirm that the
reference used is the minimum at each weight.

---

### F5 (high; wrong vs AGENTS.md §12). Planted targets allowed

**Location.** Line 22 ("`Q ∈ ⟨P⟩` a target not seen before the online clock
starts"). Against AGENTS.md §12 lines 1869-1875: "A `vs_rho` claim is one IC
run against one strong-rho run on one **public** target ... passes only with an
audit receipt from **another host class** that replayed both runs". Every
domain in docs/bounds/FRONTIER.md is `planted targets`.

**Problem.** Under the sabotage contract, a planted target (`Q = [k]P` with `k`
held by the harness) can leak `k` to the solver. It can also be built to
decompose over the factor base, which makes the descent trivial.

**Minimal fix.** Replace line 22 with:

> `Q ∈ ⟨P⟩` a public target (`target_kind: public`, AGENTS.md §12): derived by
> hashing a beacon-derived seed to the curve and clearing the cofactor, so no
> party knows `log_P Q` before the solve. `P` is the instance's published
> generator or is derived the same way. Planted targets serve controls and T0
> only.

Add to T1 item 3: "and an audit receipt from another host class that replayed
both arms (AGENTS.md §12)".

**Cheapest test.** Read `target_kind` in the claim session's spec.

---

### F6 (high; wrong + impossible). T1 band and w_cal, and the misread precedent

**Location.** Lines 62-65 and 71-73. Against AGENTS.md §5a lines 1189-1211;
the problem file; RESULTS.md lines 13-19, 36 and 39-42; band_wcal NOTE lines
32 and 55.

**Problems.**
1. **The precedent is misrepresented [V].** The draft cites the positive
   control as a T1 precedent. Under the draft's own rule 2 it is "unresolved",
   as its own frozen verdict says (RESULTS.md line 36):
   - C/R is 5.82 [3.84, 9.37] at w = 0.1 and 31.5 [20.8, 52.7] at w = 1.
   - It is a genus-3 Jacobian DLP, not an `(E/F_q, P, Q)` instance.
   - It has one curve with 4 seeds at `p = 65,537`, not 32 targets.
   - Its `w_cal` is "informational, not part of the frozen verdict", a best-of-5
     timing on a shared host.
   - Every number in it is `[pcc:unverified]`.
2. **The rule cannot pass the known win.** T1 needs ratio < 1 across the band.
   The measured mulmod cost is 0.005-0.013 GAE, 8-20x below the band's lower
   edge. The repository's only known IC win therefore cannot pass T1 at lab
   sizes. As written, T1 has no demonstrated power for mulmod-heavy IC.
   §5a already separates the band verdict (which governs the leaderboard) from
   "ranked at calibrated price". The draft merges them into one gate and calls
   the calibrated-only case "unresolved".
3. **`w_cal` exists only for prime fields [V].** `ecbench wcal` times a mulmod
   and an affine addition on `y^2 = x^3 + ax + b` over `F_p`, `p < 2^63`.
   There is no binary, Koblitz or extension-field helper. §5a says "If a
   calibration exists for only some arms or sizes, use the band alone". So on
   the ECC2K-130 reference family T1 can only be band-only, or nothing.
4. **The GAE denominator is biased [B on magnitude].** The helper's addition
   includes its own inversion. Fast rho implementations batch inversions
   (Montgomery's trick), so the addition rho actually pays is cheaper,
   I believe several times cheaper. A slow denominator makes `w_cal` small,
   which favours IC. The `10·w_cal` point covers some of this.
5. **Pricing of other counters is unstated.** At the calibrated reading, how
   non-field counters are priced is not said. band.rs prices them at every band
   weight, a 3x3 grid (NOTE line 55). Without that rule a producer could price
   everything at `w_cal`.

**Minimal fix.** Replace T1 item 2 with:

> 2. The ratio `C_IC / C_rho` (the ratio of summed costs over the targets,
>    two-stage bootstrap 95% interval, docs/bounds/README.md §5) lies wholly
>    below 1 at every weight of the §5a band. This is **T1 (band)**, and it is
>    the reading that governs the leaderboard (§5a).
>
>    Separately, the claim is **T1 (calibrated)**, reported as "ranked at
>    calibrated price" (§5a) next to the band verdict, when all of these hold:
>    - both arms carry a calibration for the same host: `w_cal` measured with
>      `perf-ipc` on an isolated host, with the addition time taken from the
>      rho reference's own per-step time on that host;
>    - the interval lies below 1 at `w = 0`, `w_cal` and `10·w_cal`;
>    - every non-field counter is priced at each band weight, as
>      `ecbench band --w-cal` does.
>
>    T1 (calibrated) without T1 (band) is reported with both verdicts and is
>    never written as plain T1.

Replace lines 71-73 with:

> The IC positive control (`research/ic_positive_control_20261112/`, genus-3
> Jacobians, `[pcc:unverified]`) is why both readings exist. It is below rho in
> group operations from `p = 257`, unresolved across the band, and below 1 at
> an informational calibrated price only at `p = 65,537`. It is a Jacobian
> DLP, not an instance of section 1, and these rules would make it T1 at
> neither reading. It is cited as the reason for the calibrated reading, not as
> a T1 result.

**Cheapest test.** Apply rule 2 as drafted to `control.json`; expect
"unresolved". Read the `ecbench/wcal.rs` input types for binary-field support.

---

### F7 (high; unpriced). Uncounted work passes the band

**Location.** Lines 26-29 ("every phase priced") and T1 item 1. Against
docs/ecbench/README.md line 365 ("an algebraic solver's wall-priced work is
taken back out"), line 535 (`ic.pipeline` leaves "algebraic-solver
operations" unpriced), line 536 (`ic.pipeline_counted`: "these runs remain
lower bounds") and line 506 (`jacobian_image.*`: "`total_gae` is 0") [V].

**Problem.** The band prices only counted work, so uncounted work is free at
every weight. The headline track is the algebraic track (msolve and F4 on
`⟨S_{m+1}, F_V⟩`), and that is exactly where solver operations go uncounted.

**Failure scenario.** A T1 claim for a solver-based decomposition: the total is
group operations plus the counted native work, and the band reads it below rho
at all three weights. The solver's work was never in the total.

**Minimal fix.** Add to section 1:

> - **No uncounted work.** A tier claim lists every process and library the run
>   invoked, phase by phase. Each has an operation count, priced by the band or
>   by a pinned, versioned conversion (`docs/ic/calibration.json`). A record
>   that is a lower bound cannot support T1 to T3 at any weight; it is reported
>   as a lower bound. Lower bounds include: a record marked `bounded` with an
>   uncounted or unconverted component, an algebraic solver whose work was
>   taken out, and a transfer arm with `total_gae: 0`.

Add to section 5: "A record with an uncounted phase, at any weight."

**Cheapest test.** Run `ecbench band` on the claim session and list what is
`bounded` or unrankable. Grep the records for solver phases that have no count.

---

### F8 (high; ambiguous, so either impossible or a loophole). "All of T1's evidence rules at each size"

**Location.** Line 80. JSON `T1_rules_each_size`.

**Problem.** Read literally, this includes T1 rule 2 (ratio < 1), so T2 would
require beating rho at all six sizes. A genuine smaller-exponent method with a
large constant could then never reach T2. The positive control is exactly that
case (`p^{4/3}` against `p^{3/2}`, losing across the band at every `p`). T2
would collapse into "T1 at six sizes", which defeats its purpose. The other
reading (rule 2 not required) is not written down, so a validator cannot apply
it.

**Minimal fix.** Replace with:

> Required: T1 evidence rules 1 and 3 at each size (cold, one target, the
> preregistered number of public targets, verified and replayed, the
> validation gate). T1 rule 2 (ratio below 1) is not required: T2 is a
> statement about the exponent, and the ratio at each size is reported as the
> distance remaining.

In the JSON, use `"T1_rules_1_and_3_each_size"`.

---

### F9 (high; wrong vs AGENTS.md). T1 at one size

**Location.** Lines 68-69 ("It says nothing about other sizes"). Against
AGENTS.md lines 297-299 ("A claim to beat rho uses the complete accounting,
**growing-size checks**, and independent replay"), lines 1076-1077 (§2: "An
explicit comparison against rho includes the full method at growing sizes")
and lines 2006-2009 (§15: report growth "so a constant-factor win at small
sizes is not mistaken for a better exponent") [V].

**Minimal fix.** Replace item 4 with:

> 4. The claim names the size, the family and the instance. It reports the
>    ratio at that size and at no fewer than two smaller sizes on the same
>    preregistered path, with the ratio's trend (AGENTS.md "Success criterion",
>    §2, §15). The ratio need not be below 1 at the smaller sizes. The claim
>    says nothing about larger sizes.

---

### F10 (high; impossible + loophole). T2 on the reference family; cofactor and γ along the path

**Verified [V]** (Lucas-sequence point counts for `E_a` over `F_{2^m}`, with
Miller-Rabin primality, run in this session):
- `E_0` with prime `m` and `#E = 4·prime`: `m ∈ {5, 7, 13, 19, 23, 41, 83, 97, 103, 107, 131}`.
  The sizes are `b = log2 r ∈ {..., 21.0, 39.0, 81.0, 95.0, 101.0, 105.0, 129.0}`.
  In `32 <= b < 100` that leaves only **m = 41, 83, 97**.
- Allowing a smooth cofactor (smooth part up to `14156`, prime `m` in 29-100):
  m = 41 (b = 39.0), 53 (`h = 428`, b = 44.3), 61 (`h = 14156`, b = 47.2),
  83 (b = 81.0), 97 (b = 95.0). That is five sizes, with a 34-bit gap.
- `E_1` with `#E = 2·prime`: nothing between `b = 22` (m = 23) and `b = 100` (m = 101).
- Also confirmed: ECC2K-130 `#E = 4r`, with `r` prime and `log2 r = 129.00000`
  (`r` just above `2^129`). The m = 83 order `4·2417851639230796216685689`
  matches §8a, with the second factor prime and `log2 = 81.0`.

**Problems.**
- Six sizes over 24 bits on the exact reference family cannot be had. The
  producer must relax the family (any cofactor, `E_1`, composite `m`, which
  §8b says loses fidelity, or generic binary curves). The draft does not say
  which relaxations are admissible, so the family can be chosen after the
  data.
- **Cofactor drift.** IC cost follows the field size `q^m`; rho follows `r`.
  If `h` shrinks along the path, `r` grows faster than `q^m`, and the IC slope
  in `ln r` falls without any algorithmic change.
- **γ(b).** On a Koblitz path `γ = 2m` grows with size. Rho's local exponent in
  `ln r` is then `≈ 1/2 − 1/(2 m ln 2)`: 0.487 at `m = 56` [V]. That is about
  as large as the 95% half-width of α at 32 targets per size over six sizes
  (`≈ 0.013` for CV 0.52 [V]). Comparing with a fixed `α = 1/2` hands IC that
  margin.

**Minimal fix.** Add to T2 item 1:

> The path fixes the cofactor, or bounds it by a declared constant and reports
> `log2 h` per size. The fit is reported against both `ln r` and `ln q^m`, and
> the claim holds only if both satisfy item 3. The rho arm is compared with its
> own model along the path, `√(π r / (2γ(b)))`, not with a constant `α = 1/2`.
>
> For the ECC2K-130 reference family (`E_0`, prime `m`, `#E = 4r`), only
> `m = 41, 83, 97` fall in `32 <= b < 100`. A T2 claim toward ECC2K-130 states
> which relaxation it uses: Koblitz `E_0` or `E_1` with prime `m` and a declared
> cofactor bound, or another stated family. It is a result for that relaxed
> family (§8b).

---

### F11 (high; unsupported + definition). T3 model selection cannot discriminate, and the definition is too narrow

**Location.** Lines 109-110 (definition), 134-137 (model selection), and JSON
T3 `fit_L_and_power_models`.

**Verified [V].** Take a noiseless `L_N[a, c]` curve scaled to `2^24` at 64
bits, sampled at eight sizes `b = 48 ... 80`:

| competitor | a | max in-sample error (ln) | error at +8 bits (ln) |
|:--|:--|:--|:--|
| pure `K·r^α` | 1/3 | 0.095 | −0.248 |
| pure `K·r^α` | 1/2 | 0.085 | −0.226 |
| `K·r^α (ln r)^k` | 1/3 | **0.0014** (α 0.081, k 4.9) | **−0.0097** |
| `K·r^α (ln r)^k` | 1/2 | **0.0018** (α 0.138, k 4.4) | **−0.0123** |

The draft's competitor, the pure power law, is a straw model. Any `Õ(r^α)`
method has polylog factors, and with them it matches the L-curve to 1% at the
held-out size even without noise. Rule 4 can only report that "the held-out
measurement falsifies the power law", and that carries no information about T3.

**Definition [B].** As I recall, Diem's subexponential results for `E/F_{q^n}`
are stated as `(q^n)^{o(1)}` (`n → ∞`, `n / log q → 0`) and as
`exp(O((log q^n)^{2/3}))` when `n ≈ √log q`. Neither has the
`(ln ln N)^{1−a}` factor of `L_N[a, c]`. Quasi-polynomial small-characteristic
finite-field DLP (reachable through pairing transfers) corresponds to `a = 0`.
The strict "`L_N[a, c]`, `0 < a < 1`" form would exclude the very result the
draft cites.

**Minimal fix.** Definition:

> The cost is subexponential in `N` along the family: `N^{o(1)}`, stated in the
> form the analysis proves. Examples are `exp(O((ln N)^a (ln ln N)^{1-a}))` or
> `exp(O((ln N)^a))` with `a < 1`, and quasi-polynomial
> `exp(O((ln ln N)^2))`. The common case is `L_N[a, c]` with `0 < a < 1`.

Item 4:

> 4. **Data**: at least eight sizes on the path. Data can only falsify. Report
>    the measured cost against the analysis's prediction, with its explicit
>    treatment of the `o(1)` term, at every size and at a held-out size at
>    least 8 bits above the largest fitted. Beside it, report a fit of the
>    exponential competitor `K · r^α (ln r)^k`. Agreement with the
>    subexponential prediction is consistency, not evidence of T3; T3 rests on
>    item 2.

---

### F12 (medium; wrong). Size bands, ecbench tiers, T0 coverage, JSON inversion

**Location.** Lines 146-151; JSON lines 7-10 and line 15 (`max_size_b:48`).
Against docs/bounds/README.md lines 107-111 ("`tier` is derived from the
**field size** — `toy` <= 32 bits, `medium` <= 96, `crypto` above — and a fit
whose sizes span tiers is refused") [V].

**Problems.**
1. **Same words, different scales.** "toy" and "medium" mean `b = log2 r`
   bands in the draft and field-bit tiers in the bounds tool.
   docs/bounds/FRONTIER.md's "Koblitz, medium" covers `r` from `2^27.8` to
   `2^47.2`. That invites citing a FRONTIER "medium" bound as draft-medium
   evidence (`b >= 64`).
2. **The mandated fit tool refuses the draft's band.** The draft's medium band
   (`64 <= b < 100`) includes field sizes above 96 bits (Koblitz `m = 97`).
   `ecbench bound fit`, which the draft requires, refuses to fit across that
   tier edge. A prime 32-bit field (`b ≈ 32`, draft "small") is a bounds "toy"
   size and cannot sit in the same fit as larger sizes.
3. **T0 is missing above the small band.** The medium and challenge rows (and
   the JSON) leave out T0. The m = 83 gate (§8a, `b = 81`) is precisely a T0
   activity, and the Success criterion sets a baseline "for each curve and
   size". A validator reading the JSON "field by field" would reject an m = 83
   T0 row.
4. **No band for `b >= 100` outside the challenge.** ECCp-131, ECC2-131,
   sect163k1 and NIST curves fall in no band.
5. **"Largest at `b >= 48`" is inconsistent.** Toy sizes are "never cited as
   evidence of scaling", so the smallest fitted size is at least 32 and six
   sizes over 24 bits put the largest at 56 or more. The held-out size is then
   64 or more.
6. **JSON inversion.** `"max_size_b:48"` reads as an upper limit, the opposite
   of the prose.

**Minimal fix.** Rename the draft's bands (for example `B0 <32`, `B1 32-64`,
`B2 64-100`, `B3 >=100`, plus the challenge) and give the field bits and the
bounds tier beside `b`. State that each T2/T3 fit lies inside one bounds tier.
Add T0 to every row and T1 to the challenge row. Replace "the largest at
`b >= 48`" with "the smallest at `b >= 32` and the largest at `b >= 56`". In
the JSON, use `"min_largest_size_b": 56` with numeric values throughout.

---

### F13 (medium; loophole/unpriced). Transfer charging rules

**Location.** Lines 187-196 and 171-174.

**Problems.**
1. **Amortization through a panel.** "A construction that is reusable across
   curves is still charged once to the first curve". In a T1 panel of `n`
   instances, the reusable construction lands on one instance and is divided by
   `n` in the panel mean. That is amortization by the back door, against the
   draft's own line 31.
2. **Precomputation charged to another run.** Precomputation that depends only
   on the field (a factor-base table for `F_{2^131}`, a symbolic summation
   polynomial) can be declared "reusable" and charged to a historical first
   curve, so it never appears in this run.
3. **One conversion factor.** "Converted ... by a measured factor" uses a single
   time ratio against a possibly slow GAE implementation (see F6, item 4).
   NFS-like or Jacobian destinations have heterogeneous operations, so one
   factor is not meaningful.
4. **Weak-class reproductions counted as tiers.** "A transfer ... counts exactly
   like a direct IC win" at "every tier". Implementing MOV on
   small-embedding-degree curves would give a "T3", and Gaudry's algorithm on
   `F_{p^3}` would give a "T2" [B: both known]. Both are reproductions of known
   weak-class results; labelling the subfamily does not stop a tier headline.
5. **Missing pre-declaration and coverage.** The applicability check is not
   required before the run, and the fraction of the family it covers is not
   reported.

**Minimal fix.** Replace item 1's last sentence with:

> Any construction or precomputation that depends on the field, the curve, the
> subgroup or the base point is charged in full in every cold row, on every
> instance of a panel. Method-only artifacts (symbolic polynomials in generic
> coefficients) are regenerated in the run or charged at their measured
> generation cost. Reuse savings appear only in a separate amortized row that
> names the number of curves.

Item 3:

> Each destination-group operation type is converted by its own measured
> factor, against the rho reference's own per-step time on the same host, and
> the conversion is read at 0.5x, 1x and 2x of each factor. Ranking requires
> the order to hold at all three.

Add item 6:

> 6. **Literature status.** The claim states the best known complexity for that
>    subfamily. A transfer whose subfamily is already known to be weak at that
>    tier (MOV-weak, anomalous, GHS-weak, a fixed-`n` Weil restriction) is
>    reported as a reproduction for that subfamily. The applicability check and
>    the subfamily's density among the curves of that size are preregistered.

---

### F14 (medium; wrong or overstated). Transfer table mathematics [B unless marked]

**Location.** Lines 177-183.

- **GHS row.** The row says "IC on the Jacobian: `Õ(q^{2-2/g})`" but leaves
  `q` undefined. Here `q` is the **subfield** size and `E` is over `F_{q^n}`;
  the cost must be compared with rho on `E`, about `q^{n/2}`. GHS covers
  typically have genus about `2^{n-1}` or more (genus `n` only for special
  curves). The `Õ` hides factors exponential in `g`, so "fixed small `g`"
  carries the whole row. "`L[1/2]`-type for large genus" should say
  `L_{q^g}[1/2, √2]` when `g` grows faster than `log q` (Enge-Gaudry).
  GTTD double large prime gives `Õ(q^{2-2/g})`; I am fairly confident of this.
- **Genus-3 row.** This is not a transfer from `E`; it is a second stage, after
  a cover to a genus-3 hyperelliptic curve. The isogeny to a plane quartic is
  Smith's (2008). I believe it applies to roughly 18-19% of genus-3
  hyperelliptic curves in odd characteristic, not to all. `Õ(q)` for plane
  quartics is Diem / Diem-Thomé (`Õ(q^{2-2/(d-2)})` with `d = 4`).
- **Pairing row.** "`L_{q^k}[1/3]`" is wrong for small characteristic, where the
  finite-field DLP is quasi-polynomial (Barbulescu-Gaudry-Joux-Thomé 2014, and
  related work). For large and medium characteristic the variant matters (NFS,
  TNFS or exTNFS when `k` is composite), and all of these are heuristic.
- **Anomalous row.** The condition is `r = char(F_q)` (trace 1 over a prime
  field), which is correct as `#E(F_p) = p` for prime fields. Add a warning:
  Koblitz curves are called "anomalous binary curves", yet Smart's attack does
  not apply to ECC2K-130 (`r ≠ 2`) [V: `#E = 4r`].
- **Table header.** "cost on the target group" is mislabelled for the rows that
  are not transfers from `E`.

**Fix.** Restate each row with the variable it is in, the compared rho cost
on `E`, the coverage fraction, and "(heuristic)" where it applies. Keep the
"to be checked with research-kb" note.

---

### F15 (medium; missing control). Statistics of the T1 interval

**Location.** Lines 59-63 and section 6.

**Problems.**
- **Estimator.** The estimator is unspecified: mean of ratios, ratio of means,
  median or geometric mean. A producer can pick the best one.
- **"Paired".** Pairing on the same target gives no variance reduction, because
  rho's cost does not depend on `Q`. The interval is in effect a ratio of two
  independent means.
- **Pseudo-replication.** If one IC precomputation is shared across the 32
  targets and each target's "cold" cost is reported as setup plus descent,
  the setup's variance is sampled once and the interval is too narrow.
- **Failures.** Failures, timeouts and restarts are not covered. AGENTS.md §8
  (1518-1520) and "IC measurement rules" (2188-2189) require them to be kept
  and charged.
- **Heavy tails.** These are not addressed. docs/bounds/README.md §3a shows one
  run at 54x the median, and rho's CV was 0.37-1.10 in the positive control
  [V].
- **Controls.** AGENTS.md lines 284-286 require "A/A and positive controls".
  The bounds challenge's A/A arm shares seeds and is only a determinism check
  (line 251) [V]. The draft requires none of these.

**Minimal fix.** Add to T1:

> **Estimator and controls.** The ratio is `Σ C_IC / Σ C_rho` over the
> preregistered targets, with the two-stage bootstrap of docs/bounds/README.md
> (2,000 resamples, recorded seed).
> - Each target is a separate cold IC run with its own independent
>   precomputation seed.
> - Every launched target counts: failures, timeouts and restarts are charged
>   at their consumed cost, and the success probability is reported.
> - `tail.large` is reported for both arms; a large tail requires a
>   replication session.
> - Controls, each in the same session:
>   - an A/A comparison of the rho reference against itself with independent
>     seeds (its interval must contain 1);
>   - a positive control whose expected win is known;
>   - a negative control expected not to win (a fixed-`n = 2` Weil
>     restriction, or a decomposition oracle on a random base).

---

### F16 (medium; loophole). The T2 held-out test

**Location.** Lines 100-103.

**Problems.**
- **Interval width.** The producer chooses the interval width; a wide one cannot
  be missed.
- **Power [V arithmetic].** Over 8 bits, an exponent difference `Δα` changes
  the predicted cost by `2^{8Δα}`: 1.057x for 0.01, 1.117x for 0.02 and 1.320x
  for 0.05. At realistic noise the test cannot tell `α_IC` from 1/2.
- **Weight.** The weight at which α is fitted and predicted is unspecified. At
  `w = 0` the IC's native work disappears.
- **Pre-measurement.** The held-out size may already have been measured in
  exploratory runs.

**Minimal fix.** Replace item 5 with:

> 5. **Held-out size.** After the fit is frozen, an instance is derived from
>    the hash of the frozen fit and a later beacon, at a size at least 8 bits
>    above the largest fitted size and beyond the crossovers of item 3.
>    - The fit tool's own 95% prediction interval is used, without widening,
>      at every band weight and at `w_cal`.
>    - The size is chosen so that this interval excludes the prediction of the
>      rho-exponent model (`α = 1/2`, refitted to the same data). If no
>      feasible size achieves that, the held-out test cannot test the claim,
>      and T2 is not reached.
>    - A measurement outside the interval falsifies the exponent claim on that
>      path.

---

### F17 (medium; loophole). Rho extrapolation

**Location.** Lines 159-164 and line 150 ("rho may be infeasible").

**Problems.**
- **The threshold is subjective.** "Typically `b >= 64` within the session's
  budget" leaves the switch point to the producer. Expected rho cost with
  negation [V arithmetic]: `2^{31.8}` GAE at `b = 64`, `2^{39.8}` at 80 and
  `2^{43.8}` at 88. At an assumed `10^7` GAE/s per core [B on rate], that is
  about 6 minutes per target at 64 and about 27 hours at 80. Measured rho is
  cheap at 64, and extrapolation would come in where measurement is
  affordable.
- **Extrapolation crosses tiers.** It runs from medium-tier bounds (`r` up to
  `2^47.2` on the frontier) into `b >= 64`, and on to field sizes above 96
  bits ("crypto" tier). docs/bounds/README.md line 44-46 says "Toy stays toy
  ... the frontier never mixes tiers".
- **Replaced rules are unstated.** With an extrapolated rho, T1 rule 1 ("both
  arms ... 32 targets") and the paired interval cannot be met; the draft does
  not say what replaces them.
- **γ mismatch.** The sealed bound's `γ` must be the instance's own.
- **The challenge.** 32 cold IC solves of ECC2K-130 is not a reasonable
  requirement.

**Minimal fix.**

> Rho is measured at every size whose expected rho cost per target is at most
> `2^{40}` GAE (about `b <= 80` with negation), with at least 8 targets above
> `b = 64`. Above that, the reference is the rho model `√(π r/(2γ))` times the
> low end of the measured constant's 95% interval at the largest measured
> size on the same family and `γ`, labelled "rho model (extrapolated, bound
> ECBND…)". In that case T1 rule 1 applies to the IC arm only, and the ratio
> interval comes from the IC interval alone. At the challenge instance, one
> verified IC solve, checked against a preregistered cost prediction with
> its interval, replaces the 32-target rule.

---

### F18 (medium; missing control). Tracks and the generic-group lower bound

**Location.** The draft does not mention AGENTS.md "Tracks of index calculus"
(318-343).

**Problem.** Generic-decomposition oracles "run identically on a random base".
In the generic group model (Shoup and Nechaev [B, standard]), any algorithm
needs `Ω(√r)` group operations for constant success probability, and a cold
single target charges any precomputation. A T2 or T3 result in that track
therefore implies uncounted group work. The draft also does not say which
track's baseline a transfer ratchets.

**Minimal fix.**

> Tier claims name their track. A T2 or T3 candidate in the
> generic-decomposition track contradicts the generic-group lower bound
> `Ω(√r)`; it is treated as an accounting error until the uncounted group work
> is found. Transfers are reported in the track of their destination solver.

---

### F19 (medium; unpriced). Memory

**Location.** Silent throughout the draft. Against docs/bounds/README.md lines
293-296 and 386-389 (default axes `ops` and `memory`; "better on some, worse
on others" is a `trade`), and AGENTS.md lines 504-507 (account for memory).

**Problem.** A T1 with fewer operations but more memory than rho would be a
`trade` under the bounds verdict and a T1 under the draft. IC memory is
"unknown" in the records (bounds README line 304), and "unknown is not zero".

**Minimal fix.** Every T1 to T3 claim reports peak memory per size (T2/T3 also
its fitted exponent), or says "memory unknown". A claim that is better on
operations and worse on memory is written "T1 (ops), trade on memory".

---

### F20 (medium; loophole). "Family" is undefined; instance selection

**Location.** Lines 60-61 ("or a panel of instances of the same family and
size"), line 68 and lines 197-200.

**Problems.**
- **Post-hoc family.** A family defined after the data ("curves on which our
  factor base splits well") is a weak subfamily in disguise.
- **Instance selection.** A hand-picked T1 instance can be screened from many
  candidates.
- **Weak-instance checks.** Embedding degree, anomalous order, subfield or GHS
  weakness are required only for transfers, not for direct IC.
- **Reference arm.** §15 asks for the fastest known method; on a weak instance
  that is the known attack, not rho.

**Minimal fix.**

> A family is a rule that produces instances from a size and a beacon seed,
> declared in the preregistration, with its density among all curves of that
> field size reported. Instances are drawn by that rule, and every drawn
> instance is reported. Each instance passes the weakness checks (embedding
> degree above a declared bound, not anomalous, subfield and cover structure
> disclosed per §8b). If a known non-generic attack applies, it is an
> additional reference arm (§15).

---

### F21 (low; gap). pcc standing and review for T1

Section 6 asks the claim to *state* the pcc id and standing. A claim at
standing "unverified" therefore still qualifies. `VALIDATION.json`'s
`independent: true` flags are written by the producer (VALIDATION_GATE.md lines
67-71) [V]. T1, unlike T2 and T3, has no independent review.

**Fix:**

> T1 to T3 require pcc standing `verified` (CI-signed). T1 also requires
> `validator-breakthrough` on the frozen evidence.

---

### F22 (low; gap). T3 heuristic tests; prime-field scrutiny

**Problems.**
- **Unstated criteria.** Item 3 gives no numeric pass/fail criteria. "Withdraws
  the claim at that size and above" is ambiguous for an asymptotic heuristic.
- **"T3 scrutiny".** Line 157-158 uses the phrase without defining it.
- **Understated [B].** For general prime-field curves, and for prime-degree
  binary fields, not even an exponent below 1/2 is known, not only no
  subexponential algorithm. Lean "where feasible" is optional in effect.

**Fix:**

> Each heuristic test has a numeric acceptance criterion in the
> preregistration (for example, observed solving degree <= forecast; relation
> yield within a factor 2 of the predicted smoothness probability, with an
> interval; matrix rank deficit <= a declared bound).
>
> A T2 claim on any family where no algorithm with exponent below 1/2 is known
> (general prime-field curves, prime-degree binary fields) also meets T3 items
> 2 and 3.

---

### F23 (low; consistency). Rho's headline S

AGENTS.md §2 (line 1063) says rho is "a flat `S ≈ 1.3`", which is plain rho
(`√(π/2) ≈ 1.25`). The draft's formula with `γ = 2` gives
`√(π/4) ≈ 0.886` [V]. A producer could tabulate "rho = 1.3".

**Fix.** Add one sentence: "with negation, rho's idealized `S` is `0.886`;
`S ≈ 1.3` in §2 is plain rho and is never the reference".

---

### F24 (low; form). JSON, prose lint, literature

**JSON.**
- Thresholds are strings (`"min_sizes:6"`).
- `"calibrated": ["0", ...]` holds strings.
- There are no fields for the preregistration id, public targets, estimator,
  fit weights, the Holm family, growing-size checks, cross-host-class audit or
  uncounted work.
- A validator cannot check "field by field" as section 6 promises.

**Prose lint.** `python3 tools/pcc/pcc.py lint-prose docs/ic/SUCCESS_CRITERIA.md`
gives 8 findings [V]. Most are definitional false positives, but the
positive-control sentence (lines 71-73) states a lab result without
`[pcc:unverified]`.

**Literature.** The section 4 complexities, the Diem and first-fall-degree
remarks, and the absence claim "no subexponential algorithm for general
prime-field curves is known" are unverified (research-kb was unavailable to
both the author and this reviewer).

**What I believe [B].** First-fall-degree arguments for binary curves
(Petit-Quisquater 2012; Semaev 2015) were challenged by Huang-Kosters-Yeo
(2015) and Kosters-Yeo (2015), so "disputed" is fair. Gaudry's `Õ(q^{2-2/n})`
for fixed `n` is right, with constants that grow very fast in `n`, and `n = 2`
gives exponent 1/2, the same as rho [V arithmetic: `(2 − 2/n)/n = 0.5, 0.444, 0.375, 0.32` for `n = 2..5`].

---

## What was checked and found correct [V]

- **L-notation.** `L_N[a, c] = exp((c + o(1)) (ln N)^a (ln ln N)^{1-a})` is the
  standard form (but see F11 on scope).
- **Rho formula.** `√(π r / (2γ))` is correct with `γ` counting `±1`. It equals
  the bounds floor `√(π/2A)·√r`.
- **ECC2K-130.** `E_0` over `GF(2^131)`, `#E = 4r`, `r` prime,
  `log2 r = 129.0000`. With `γ = 262`, rho costs `2^{60.81}`.
- **Rho bound side.** The low end of the rho bound's interval errs in rho's
  favour, as claimed.
- **§8b subfields.** "No proper intermediate subfields" for `m = 131` is
  consistent with §8b.
- **Bounds README.** "four sizes minimum" is cited correctly.
- **Skills and agents.** All named skills and agents exist:
  `red-team-breakthrough`, `validator-breakthrough`, `dreg-triage`,
  `lean-instance-gate`, `ecdlp-surfaces`, `curve-algebra`, `isogeny-volcano`,
  `perf-ipc`.

## Reproduction of the numeric checks

All numbers marked [V] come from short stdlib-only Python scripts run in this
session (`python3 -I`): ECC2K-130 order and primality, rho costs, Koblitz
order enumeration by Lucas sequences, the two-phase T2 simulation, the
fixed-cost simulation, and the L-curve versus power-law and polylog fits. The
scripts were not saved. Re-running them requires only the formulas stated in
each finding.
