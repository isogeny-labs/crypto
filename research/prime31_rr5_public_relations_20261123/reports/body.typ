= Result and scope
<result-and-scope>
#strong[Development09 completes the fixed Prime31 B8 positive and empty
controls with the Grevlex cofactor presentation, and recovers the F11
public target through the reusable public-query loop.] Its regression
invocation passes 22 cases with three filtered out. The B8 positive, B8
empty and F11 Grevlex controls then each pass in separate one-case
invocations. These are four recorded invocations, not one 25-case test
run.

This additive study preserves the published native checkpoint
#link("https://github.com/isogeny-labs/crypto/commit/269cd7b552bd3abcc88c88a6eb5771b59cceb0bf")[`269cd7b552bd3abcc88c88a6eb5771b59cceb0bf`];.
Development01–08 are historical, unchanged observations from that
checkpoint. Development09 was executed against main
#link("https://github.com/isogeny-labs/crypto/commit/e51060051c6963b8852f86844f1ec53dc61f895f")[`e51060051c6963b8852f86844f1ec53dc61f895f`];,
which already contains a separate RR5 qualification study. Its source,
evidence and registrations remain in their existing directory. The new
#link("evidence/development-09/source-manifest.json")[development09 receipts]
bind the relocated source and changed production F4; their qualification
is separate from the unchanged development01–08 records.

The checkpoint implements the existing RR5 monic-quartic construction in
native Rust. It extracts signed four-point relations, checks their
actual group equations, and feeds authentic public relations into exact
modular rank and target-scalar recovery. Its local Prime31 B4 extraction
controls pass. An integrated small-field public-target control recovers
its scalar using three independent #strong[augmented] rows and verifies
the answer by scalar multiplication. The implementation refuses
enumeration fallbacks before they execute.

Both historical B8 presentations use Lex ordering and remain
inconclusive under their frozen elimination caps. The separate study on
main first supplies the completed Grevlex B8 controls credited below;
development09 now qualifies that presentation in this public-relations
integration. These successes retain the scope of two fixed B8 targets.
The original B677 expanded input reaches its symbolic-size cap. An exact
cofactor lift removes that construction obstacle: all 677 equations are
built and a test-only known-answer assignment satisfies them. #strong[No
B677 relation extraction, complete Prime31 recovery, or matched cost
comparison is reported.] \[pcc:unverified\]

The principal fixture is unchanged:

$ E \/ bb(F)_p & : y^2 = x^3 + 847514846 x + 2087444724 ,\
p & = 2147483647 , #h(2em) n = 2147446831 ,\
G & = (625037715 , 2100627251) . $

Its source is the original `prime31_ic_20261115/native/lib.rs`. B4 and
B8 controls use the first eligible public abscissae, explicitly for
correctness. The B677 control uses the original 509-point Dickson base
plus the actual frozen 168-point RUN66 CSV. These populations are not
interchangeable.

#figure([#box(width: 100%, image("reports/figures/stages.svg"));],
  caption: [
    Current development09 qualification stages: completed fixed B8
    controls and F11 public recovery; original B677 remains input
    construction only. The figure describes correctness stages, not
    algorithm speed. Sources: development-09 raw receipts.
  ]
)

== Requirement-to-evidence map
<requirement-to-evidence-map>
#figure(
align(center)[#table(
  columns: 3,
  align: (col, row) => (auto,auto,auto,).at(col),
  inset: 6pt,
  [Requested result], [Current evidence], [Status],
  [Nonenumerative four-summand extraction],
  [Development09 Grevlex B8 positive `Complete` with one signed row and
  generator `Empty`; no field/domain scan branches],
  [Two fixed B8 controls qualified locally; omitted charts remain],
  [Signed, independent public relations],
  [Shared group-checked adapter for actual `aG+bQ`; exact rank and
  duplicate/dependency dispositions],
  [Retained regression and current F11 integration pass],
  [Complete scalar-recovery integration],
  [Development09 F11 Grevlex public loop: nine queries, three
  independent augmented rows, scalar 3, final replay],
  [Small-field control complete; Prime31 open],
  [Requested-size factor base],
  [Original B677 cofactor equations and known-answer equation check],
  [Input construction only],
  [Fully charged best-IC and optimized-rho comparison],
  [Source audit and phase-by-phase gate; no admitted panel],
  [Open],
  [Growing-family complexity claim],
  [No family or admitted measurements],
  [Not attempted],
)]
)

Passing a unit test that checks correct handling of an inconclusive
result is not a successful extraction. The result tables below preserve
that distinction. Main’s separate B8 receipts remain credited as prior
qualification. The table’s current B8 and Grevlex F11 entries refer to
this directory’s development09 calls.

= Construction and what makes extraction nonenumerative
<construction-and-what-makes-extraction-nonenumerative>
Let the supplied finite public target be $Q = (r , s)$, and let the
chosen base have distinct abscissae and nonzero ordinates. Set

$ h (X) = product_i (X - x_i) , quad V (X) = X + b , quad L (X) = c X + d , $

where $b , c , d$ are unknown coefficients. The function

$ ell (X , y) = (X - r) L (X) + (s + y) V (X) $

vanishes at $- Q$ and has pole order five at the identity. The residual
norm identity gives the monic quartic

$ U (X) = (X^2 + r X + r^2 + A) V (X)^2 - (X - r) L (X)^2 - 2 s L (X) V (X) . $

The four coefficients of $h med mod med U$ are the equations in the
expanded presentation. Solving these equations over the field determines
the quartic; factoring it determines the four abscissae. Each ordinate
is recovered uniquely:

$ y_i = - s - frac((x_i - r) L (x_i), V (x_i)) . $

The implementation looks each root up in the original base, recovers its
sign, and checks that the four points sum to the supplied target. It
does not scan base pairs, four-tuples, multisets or the entire field to
discover a row. Tiny exhaustive signed-tuple oracles occur only in test
code for validation; their outputs are never supplied to the candidate.
An independently written integer group law supplies separate replay
controls.

The native traced visitor retains the original generators. Before any
fallback branch, its trace sink requires an actual nonconstant
univariate or an exact inconsistency. Without a usable eliminant, it
stops as inconclusive. It admits only branches labelled
`univariate_roots`; finite-domain and full-field branches are rejected
before specialization. Field factoring uses the original counted
Frobenius/root routine. A completed traversal is meaningful only with
all original equations and signed group replays checked.

The chart excludes the identity target, a target abscissa in the base,
repeated leaf abscissae and rational two-torsion leaves. Those omitted
cases are reported explicitly; they are not silently accepted or treated
as global absence proofs. The denominator lemma and converse are proved
in #link("MATHEMATICS.md");. Under the stated assumptions, every quartic
dividing the squarefree split selector has an invertible denominator,
and every distinct-abscissa signed decomposition yields its unique
normalized function. The written proof is not a Lean or signed-PCC
proof.

== Prior work and exact contribution
<prior-work-and-exact-contribution>
This is a specialization and integration of the existing RR9 method. A
concurrent merged implementation in `examples/prime31_rr5_quartic.rs`
\(\#4942) uses the same quartic and remainder ideal under the exact
parameter change $u = r$, $v = - s$, $t = b$, $beta = c$, $alpha = d$.
Horner remainders versus long division do not constitute a different
mathematical construction.

The
#link("../prime31_four_summand_20261122/REPORT.md")[separate study now on main]
also contains the traced visitor, strict chart/failure states and the
exact same signed-row adapter: its `relation_adapter.rs` Git blob is
byte-identical to this checkpoint. It already exercises extracted rows
through a test-only public-label F11 recovery schedule and qualifies the
cofactor B8 presentation with Grevlex. Those shared components and
results are credited to the existing work; none is a new contribution
relative to current main.

This continuation preserves a reusable public `aG+bQ` query/recovery
loop with retained attempts, deadlines and target-functional stopping;
the original-B677 cofactor input check; the compact-selector argument
and source-data setup certificate; and native metadata export. Its F11
SplitMix schedule is a different control from main’s public-label
schedule. Neither schedule is a Prime31 solving-cost experiment.
#link("SOURCE_AUDIT.md") records the revision boundaries and exact
shared source. No first-extractor, novel-index-calculus or asymptotic
claim is made. Development09 adds an explicit Grevlex cofactor form to
that reusable extractor/consumer path while retaining the historical Lex
forms and their incomplete outcomes.

= Signed rows, independence and scalar recovery
<signed-rows-independence-and-scalar-recovery>
For a public query generated as $R = a G + b Q$, a verified signed
decomposition supplies

$ sum_i c_i P_i = a G + b Q . $

Writing $P_i = x_i G$ and $Q = d G$ gives the exact mod-$n$ equation

$ sum_i c_i x_i - b d = a . $

The consumer therefore inserts the augmented coefficient row
$[c_0 , dots.h , c_(B - 1) , - b divides a]$. It consolidates signs,
checks leaf indices, checks the original group equation, deduplicates
scalar-equivalent labelled rows, and performs exact modular elimination.
Duplicate and dependent rows are distinguished from rank increases. A
false RHS cannot create useful rank.

The target is recoverable when its coordinate functional is determined
by the row space; all factor-base logarithms need not be determined
first. The result is accepted only after a final $d G = Q$ check. Query
generation accepts public curve data, public base points, public target,
limits and a coefficient-stream seed. It does not accept the target’s
logarithm or the leaf witness.

The integrated control uses $y^2 = x^3 + x + 6$ over $bb(F)_11$,
$n = 13$, $G = (2 , 7)$, base $(2 , 4) , (3 , 5) , (5 , 2) , (7 , 2)$
and public target $(8 , 3)$. The original SplitMix64 coefficient
mapping, seed `0x525235`, generates the actual RHS points. All nine
attempts are retained. Six queries hit an explicitly unsupported chart;
the other three supply accepted rank increases and recover $d = 3$.
Development09’s separately invoked Grevlex cofactor control reproduces
that public schedule, with `status=solved`, `final_replay_passed=true`
and `deadline_exceeded=false`. It accepts neither the target logarithm
nor base logs as candidate input.

#figure(
align(center)[#table(
  columns: 4,
  align: (col, row) => (right,auto,auto,right,).at(col),
  inset: 6pt,
  [Accepted query], [Canonical coefficients $c$], [Canonical $a , b$],
  [Augmented rank],
  [1],
  [$(1 , - 1 , - 1 , 1)$],
  [$(1 , 7)$],
  [1],
  [4],
  [$(1 , 1 , 1 , 1)$],
  [$(3 , 0)$],
  [2],
  [9],
  [$(1 , 1 , 1 , 1)$],
  [$(12 , 10)$],
  [3],
)]
)

The last two rows have the same factor-base coefficients and different
labelled RHS. Their difference determines the target coordinate. Thus
the stated rank three is rank of the augmented system, not three
independent factor-base coefficient vectors. This is a small-group
integration control with a known-answer fixture; it establishes neither
natural Prime31 yield nor cost. Sources: the historical
`RR5_INTEGRATED_F11` object in development-06 and the current
`RR5_INTEGRATED_F11_GREVLEX_COFACTOR` object in
#link("evidence/development-09/public-recovery.stdout")[development09 public recovery];.

Main’s separate test-only F11 schedule accepts the target row and visits
public labels 1–12. It recovers the same scalar at augmented rank two,
then finishes its fixed schedule at rank three. The two schedules
therefore have different stopping points and transcripts; their rank
counts are not competing claims about a single run. See its
#link("https://github.com/isogeny-labs/crypto/blob/e51060051c6963b8852f86844f1ec53dc61f895f/research/prime31_four_summand_20261122/native/independent_tests.rs")[integration control source]
and
#link("../prime31_four_summand_20261122/REPORT.md")[qualification report];.

Static review also corrected transcript-accounting defects: a query is
retained even if point construction consumes the remaining budget; row
consumption stops after target determination; relation failures retain
the partial transcript; and the absolute cooperative deadline includes
consumer setup and validation. Native timeout and output-limit states
take precedence over fallback or replay diagnostics. A late verified
recovery is explicitly `solved_after_deadline`.

= Larger inputs and retained development outcomes
<larger-inputs-and-retained-development-outcomes>
The coefficient lift introduces a monic $W$ of degree $B - 4$ and
imposes

$ U (X) W (X) = h (X) . $

The quotient is unique because $U$ is monic. Eliminating its
coefficients gives exactly the original remainder equations, so this
lift does not add a solution or choose a tuple. It has $B - 1$
variables, $B$ equations and total degree at most three. This removes
expanded symbolic remainders at the price of more variables; elimination
can still be expensive.

#figure([#box(width: 100%, image("reports/figures/b8-inputs.svg"));],
  caption: [
    Historical Lex input counts for the same B8 positive fixture.
    Expanded F4 degree cap 10; cofactor F4 degree cap 5. Neither
    checkpoint call completed extraction. These are separate from main’s
    completed Grevlex controls; no cost ratio follows.
  ]
)

#figure(
align(center)[#table(
  columns: 3,
  align: (col, row) => (auto,auto,auto,).at(col),
  inset: 6pt,
  [Development-06 control], [Input], [Observed extraction outcome],
  [Prime31 B4, expanded],
  [3 variables; 4 quadratic equations],
  [Signed positive and empty controls complete],
  [Prime31 B4, cofactor],
  [3 variables; 4 quadratic equations],
  [Same positive signed row recovered],
  [Prime31 B8, expanded],
  [3 variables; 4 equations; 319 terms; degree 10],
  [Positive and empty queries inconclusive],
  [Prime31 B8, cofactor],
  [7 variables; 8 equations; 99 terms; degree 3],
  [Positive query inconclusive],
  [Original B677, expanded],
  [20,000-term per-polynomial cap],
  [Construction limit, before F4],
  [Original B677, cofactor],
  [676 variables; 677 equations; 13,480 terms; degree 3],
  [Known-answer input check passes; no extraction call],
)]
)

For the two B8 positive calls, the observed native maxima are 208 rows /
3,752 columns \(expanded) and 9,414 rows / 14,106 columns \(cofactor).
These are traversal diagnostics, not peak RSS or a matched runtime
comparison. Both calls preserve zero full-field branches and zero
finite-domain evaluations. Their failure to produce a univariate is
bounded evidence, not a proof that this ideal cannot be solved
algebraically. \[pcc:unverified\]

== Separate completed B8 controls on main
<separate-completed-b8-controls-on-main>
At main `e510600…`, the existing study’s quotient-product presentation
uses Grevlex, degree cap ten, a thirty-second deadline, a 20,000-term
construction cap and 4,096 assignments. Both retained native controls
complete:

#figure(
align(center)[#table(
  columns: 3,
  align: (col, row) => (auto,auto,auto,).at(col),
  inset: 6pt,
  [Existing main control], [Observed outcome], [Native traversal],
  [#link("../prime31_four_summand_20261122/evidence/b8-positive-02/stdout.txt")[Positive target];],
  [`Complete`; exactly one signed row],
  [Seven F4 nodes; solving degree four],
  [#link("../prime31_four_summand_20261122/evidence/b8-empty-02/stdout.txt")[Public generator];],
  [`Empty`; zero signed rows],
  [One F4 node; solving degree four],
)]
)

Both have seven variables, eight equations and 99 input terms; the
recorded maxima are 1,085 rows and 1,121 columns, with zero full-field
branches and finite-domain evaluations. These are existing main results,
not a new run in this directory. This checkpoint’s cofactor control used
Lex, degree cap five and five seconds. Presentation details, ordering
and bounds differ, so the records neither isolate an ordering effect nor
establish a speed comparison. No B677 extraction or complete Prime31
recovery follows from either B8 control.

== Current development09 Grevlex integration
<current-development09-grevlex-integration>
The new `CofactorGrevlex` form feeds the same monic-cofactor equations
into the original traced visitor and retains the signed-row consumer and
public-query interface. The new source inventory binds this directory
and main’s production F4. The two fixed B8 calls use the same public
natural-x base and targets as the separate main controls, with degree
cap ten, thirty seconds, a 20,000-term construction cap and at most
4,096 assignments.

#figure(
align(center)[#table(
  columns: 5,
  align: (col, row) => (auto,auto,right,right,right,).at(col),
  inset: 6pt,
  [Development09 control], [Outcome / signed rows], [F4 runs], [Factored
  univariates], [Reported `field_ops`],
  [#link("evidence/development-09/b8-positive.stdout")[B8 positive];],
  [`Complete` / 1],
  [7],
  [36],
  [54,099,486],
  [#link("evidence/development-09/b8-empty.stdout")[B8 generator];],
  [`Empty` / 0],
  [1],
  [0],
  [51,100,692],
)]
)

The positive row is `[(0,1),(1,-1),(2,1),(3,1)]`; its public target is
$(92816593 , 1965281801)$. The empty target is the public generator.
Both inputs have seven variables, eight equations, 99 terms and degree
three. Both traversals finish with solving degree four and recorded
maxima of 1,085 rows and 1,121 columns. Neither times out, stops its
trace, executes a full-field branch, evaluates a finite domain, refuses
a fallback or rejects an assignment. The independent test oracle
evaluates 1,120 signed subsets per target; its rows never enter the
candidate.

The separately invoked
#link("evidence/development-09/public-recovery.stdout")[F11 Grevlex control]
recovers scalar 3 after nine public attempts and augmented rank three,
with successful final group replay and no deadline overrun. Its six
unsupported queries remain in the transcript. This establishes the
small-field consumer integration for the new form; it supplies no
complete Prime31 recovery.

The `field_ops` entries and other counters remain partial native
diagnostics. They omit material setup, symbolic, memory and native work
described below; they are not calibrated complete costs. No test
duration, ratio, performance ranking or random-target yield estimate is
derived from these fixed controls. Both accounting-completeness flags
remain false. \[pcc:unverified\]

== Frozen attempts and failures
<frozen-attempts-and-failures>
Development01–08 below retain their original source and executable
bindings. Development09 is a separate completed component qualification
against the new namespace and current production source; its four
invocations are listed separately.

#figure(
align(center)[#table(
  columns: 3,
  align: (col, row) => (auto,auto,auto,).at(col),
  inset: 6pt,
  [Attempt], [Outcome], [Interpretation],
  [Development 01],
  [Build failed],
  [Workspace metadata problem; no tests],
  [Development 02],
  [Build failed],
  [Existing selector rejected changed whole-file source pin; no tests],
  [Development 03],
  [14 tests passed],
  [Original B4, chart, independent replay and consumer checks],
  [Development 04],
  [17 tests passed],
  [Adds B8/B677 bounded outcomes and integrated F11 recovery],
  [Development 05],
  [Build failed],
  [Missing `hex` dependency metadata for the unchanged identity
  checker],
  [Development 06],
  [21 tests passed],
  [Corrected accounting, cofactor input, identity checks and deadline
  control],
  [Development 07],
  [One setup test failed],
  [Inherited eighth coset constant disagreed with its seventh-power
  witness],
  [Development 08],
  [One setup test passed],
  [Corrected RUN52 constant; complete CSV/fibre/union setup certificate,
  independently frozen],
  [Development 09 regression],
  [22 cases passed; 3 filtered],
  [Fresh scoped regression against the relocated source and current F4],
  [Development 09 B8 positive],
  [1 case passed; 24 filtered],
  [Grevlex traversal complete; one exact signed row],
  [Development 09 B8 empty],
  [1 case passed; 24 filtered],
  [Grevlex traversal complete; no signed rows],
  [Development 09 F11 Grevlex],
  [1 case passed; 24 filtered],
  [Public-query recovery and final replay pass],
)]
)

Build, pre-call and post-call manifests are separate. Test executables
are copied to immutable paths before calls. Failed builds have no
fabricated execution receipt. The early manifests also list generated
source copies; those historical manifests are preserved unchanged. Later
snapshots explicitly exclude build directories. All recorded test
timings are ordinary development-run output and are not promoted into
performance values.

Development07 also records a pre-call current-source mismatch in the
separate metadata-only CLI module, which was hardened after compilation.
Its original bytes are preserved and the library executable was pre/post
hashed; the failed test is not admitted as a passing certificate.
Development08 required every source check to pass, and used a new
immutable library executable and a separately frozen metadata CLI. Its
one passing setup test does not turn development06 into a new 22-test
run.

Development09’s library and metadata builds each exit zero. Its four
test invocations each exit zero, with separate commands, pre-call
inventories, source checks, raw output and post-call hashes. The
regression filtered the three focused controls, which were each run
separately; no case was ignored.

Two original native identity records accompany the norm and denominator
identities: `IDC1h140ede2d54279ca7` and `IDC1h660312477c7469f3`. Each
uses 32 issuer points plus 32 additional fixed checker-stream points
over $2^61 - 1$, with `proof=null`. These are deterministic
producer-local sampled identity checks; distinct streams do not
establish independent execution or a formal proof over the integers. The
raw certificate and checker verdict occur in development-06 and are
exported without changing the statement.

= Exact compact-selector route
<exact-compact-selector-route>
The original B677 base has more structure than its expanded degree-677
product shows. Let $f (X) = X^3 + A X + B$ and let $c_1 , dots.h , c_8$
be the existing public seventh-power coset constants. Define

$ H_D (X) = D_1024 (X , 1) - 1616683971 , #h(2em) H_Y (X) = product_(j = 1)^8 #scale(x: 120%, y: 120%)[\(] f (X)^7 - c_j^2 #scale(x: 120%, y: 120%)[\)] . $

The development08 source-data certificate checks the actual CSV, the
entire 1024-element Dickson fibre, all eight 21-element extra fibres,
disjointness and canonical union. The product $H = H_D H_Y$ has 1192
distinct rational abscissa roots. Exactly 677 lift to the original base;
the remaining 515 have nonsquare curve RHS. The setup check runs neither
circuit solving nor relation extraction.

The initial check exposed an inherited constant error in the later
cubic-factor example. The original RUN52 source and the actual CSV agree
on $c_8 = 1852957275$, with anchor $(227959902 , 136769428)$. This study
uses that corrected value and preserves both the historical source and
development07’s failed receipt. The exact census above is bound to
development08’s `COMPACT_SELECTOR_SETUP` object, not inferred from the
earlier failed check.

There is a useful four-leaf argument for removing those extra geometric
possibilities. For a nonzero rational target with $h (r) eq.not 0$,
outside both signed lifts of every base point, on an odd-order curve,
write a four-leaf relation as $Q = T + Z$, where $T$ sums rational
leaves and $Z$ sums leaves whose ordinates are negated by Frobenius.
Frobenius gives $Q = T - Z$, hence $2 Z = O$ and $Z$ is rational. Odd
rational group order forces $Z = O$. One such leaf cannot sum to zero;
two require identical abscissae; three leave a rational base point equal
to $Q$; four force $Q = O$. All are excluded by the chart assumptions.
Thus every geometric RR5 solution of $U divides H$ uses only the
original 677 abscissae.

#link("COMPACT_SELECTOR.md") gives the full hypotheses and proof. It
does not extend automatically to five or more leaves, repeated
abscissae, identity or base targets, or even rational group order.

The selector can be evaluated modulo the quartic with 22 quotient-ring
multiplication gates: ten for the Dickson ladder, four for $f^7$, seven
for the eight-factor product, and one final multiplication. Encode each
gate exactly by $A B = U Q + R$, with $deg Q lt.eq 2$ and
$deg R lt.eq 3$. A conservative lifted description has 157 variables and
158 quadratic equations. Those counts are derived from the written
circuit, not a measured solver result. Its root extractor remains to be
implemented and qualified; compact input alone gives no small
elimination bound.

= What the requested comparison still requires
<what-the-requested-comparison-still-requires>
The primary comparison remains #strong[one unknown public target from a
cold start];. It must charge base selection and construction, selector
and equation setup, every unsuccessful and successful query,
elimination, roots, signs, exceptions, row handling, useful rank, final
linear algebra, recovery, verification and peak memory. Multi-target
amortization is a separately labelled secondary result.

The source audit identifies the original algebraic solver and production
`PkmSemaevOracle` family as IC candidates, the optimized generic IC
controls in `prime_fast`, and `prime_fast::rho_parallel` with
`RhoConfig::tuned(n)` as the optimized negation-map wall reference. The
existing `rho.negation` counted arm must also be retained. The best
applicable IC configuration must be selected from reproducible evidence
before the matched experiment; this report does not declare whichever
implementation is easiest to call to be the incumbent.

The present counters are explicitly incomplete. In particular, symbolic
map work and allocation, some coefficient evaluations and powers, F4
memory, input setup and complete native-word costs are not fully priced.
Consumer setup arithmetic is already included in cumulative consumer
arithmetic and must not be counted twice. The source flags
`phase_accounting_complete=false` and `complete_cost_accounting=false`
remain correct.

There are also observed execution dependencies. The documented
main-branch EKS route builds the root ecbench executables; its
source-checkpoint allowlist does not include this standalone
RR5/arithmetic source closure. A supported image/command integration is
required. The available runtime has no authenticated EKS/AWS client or
exposed workflow-dispatch operation, and the actual checkpoint preflight
failed for lack of upload authentication. No paid job was launched.
These are the checkpoint’s observed execution constraints, distinct from
its historical Lex B8 outcomes. Exact source checks and revision
boundaries are in #link("SOURCE_AUDIT.md");.

The inherited root manifest syntax was repaired on the earlier main
snapshot while this checkpoint was developed. Its initial failure is
historical. The checkpoint’s no-run build reached its 60-second cap; a
separate invocation of the required
`cargo test --release --locked --offline --lib -- --test-threads=1`
reached its 300-second process cap while compiling dependencies, both
with exit124. No root-library test result or new source compilation
defect was established.

A separate integration root-library attempt against `e510600…` also
reached its 60-second process cap during dependency compilation, with
exit 124. Its new
#link("evidence/root-library-integration-gate/exit")[raw receipt] and
#link("evidence/root-library-integration-gate/stderr")[compiler output]
belong to this integration, not either historical `f0ca664…` attempt. It
produced no passing root-library test result. Development09’s passing
component qualification is a separate gate and does not change this
full-library outcome.

The unchanged-source closure qualification is a narrower gate. No
full-library pass or signed standing validation is inferred from the
component tests.

= Reproduction, evidence and next technical step
<reproduction-evidence-and-next-technical-step>
The preserved sources, protocol, raw output and exact hashes are in this
study. The read-only source selector copies the actual full F4
parent/child modules and byte-identical dependency ranges; it does not
replace their algorithms. Original curve arithmetic is linked from its
existing Rust source. See #link("README.md") for the build entry point
and the protocol for immutable execution details. The native metadata
exporter preserves raw pins, marker records and sampled identity
certificates; unsigned registration is an attestation, not an execution
proof or trusted verdict.

The historical
#link("reports/development-export/observation.json")[native export] pins
116 raw files and preserves 23 parsed stdout records and 2 sampled
identity certificates. It records failed builds, failed setup, capped
library gates and the upload preflight alongside successes. Its standing
is unverified; no independent executor or trusted verifier is claimed.

That observation retains the original `prime31_four_summand_20261122`
paths and belongs to checkpoint `269cd7b…`. Its bytes and historical
manifests are not rewritten to describe this directory. The completed
#link("reports/integration-export/observation.json")[integration export]
instead pins 160 raw files under the relocated namespace and preserves
41 parsed marker objects and two sampled identity certificates. Its
SHA-256 is
`688e4c7dd5ca6be980ecf1c5c29dcfa355a4e994724f0e5a7bb951a1f736a507`. It
includes development09 and the separate integration root-library
timeout. The actual new source inventory, rather than an old executable
or rewritten manifest, binds current main’s changed F4 to the scoped
qualification. The export remains unsigned and unverified; it does not
issue a verdict.

The next technical step is public-target relation extraction at the
original B677 base: implement and qualify the source-certified
compact-selector circuit, or another exact presentation, through the
already integrated signed consumer. The compact circuit has not been
implemented, and the existing B677 cofactor check still supplies input
construction only. A successful input check does not satisfy the
extraction requirement. After actual public relations at the requested
base size can drive scalar recovery, the frozen cold comparison can be
admitted on matched resources.

The canonical IC scoreboard, progress timeline and performance-gain
graphs remain unchanged: this checkpoint contributes correctness and
retained cap evidence, with no newly admitted cost value or curve
ranking. The first figure describes current development09 stages; the
second retains the historical development06 input comparison at its
original scope. \[pcc:unverified\]
