# Four-summand algebraic extraction: development protocol

Status: frozen before the first new native solver invocation. This is a bounded
correctness qualification, not a performance experiment. Parent tracking PR: #4938.

## Objective and source reuse

Implement the degree-five Riemann–Roch specialization of the existing RR9
construction. Extract four distinct factor-base abscissae from a public target
using polynomial remainder equations and the existing F4/root-finding code.
Recover signed points, verify their sum, and feed only genuine public labelled
relations into the original authentic-RHS linear algebra contract.

The principal curve remains PRIME31 in
`research/prime31_ic_20261115/native/lib.rs`. Its original Arithmetic is reused.
The existing production-source integration qualifier supplies F4 and finite-field
root factoring; it does not qualify the full repository library. No changes to
the merged #4901 or #4924 source/evidence are part of this development.

## Hypotheses and falsifiers

1. The RR5 remainder system has exactly the signed, distinct-x four-leaf
   decompositions for finite Q outside the factor base. Check exact point sums
   and a separate small-field signed-tuple oracle in test-only code.
2. Original F4 can extract positive roots or refute empty B=4 cases without
   factor-base tuple enumeration. Deny every non-univariate branch before child
   traversal. No available univariate means inconclusive, never empty.
3. Higher B can reuse the same interface. B=8 is the next correctness control;
   a degree/time/size limit is a retained incomplete outcome. It is not a
   statement of mathematical impossibility or evidence at B=677.
4. Signed relation insertion must distinguish duplicate, dependent and new-rank
   rows. A recovered target scalar is accepted only after original-curve dG=Q.

## Fixed development controls

- PRIME31 B=4 points chosen from a deterministic public x scan. Public targets
  include a positive sum with a nontrivial sign, its negative, and an ordinary
  point checked against the independent B4 oracle. Witnesses stay test-only.
- Ordinary small-field control: p=11, A=1, B=6, n=13, G=(2,7), and base
  [(2,4),(3,5),(5,2),(7,2)]. Compare all 12 finite public targets to an
  independently implemented integer group law and test-only signed-four oracle.
  Exactly (8,3),(8,8),(10,2),(10,9) are outside the base; the other eight
  finite points and the identity must have the explicit chart status.
- B=8 bounded positive and empty cases, if the B4 extraction gate passes.
- After the B4 gate: original hybrid B677 (509 Dickson points plus the frozen
  RUN66_Y7_EXTRA168.csv), one explicitly constructed positive public point,
  five-second construction/extraction deadline and 20,000-term symbolic cap.
  Retain the exact incomplete status; this is a scale/cap correctness control,
  not a yield or performance sample. No tuple search on this base is allowed.
- Integrated one-public-target F11 loop: the same B4 fixture, Q=(8,3), original
  native SplitMix64 public coefficient stream seeded 0x525235, at most 64
  queries and 30 seconds. Retain every (a,b), public point, extractor outcome,
  signed row, consumer receipt, rank change and final scalar replay.
- Reject duplicate abscissae, zero-y base points, invalid points, identity Q and
  target-in-base with explicit chart status. Preserve repeated-x scope as open.
- Stop before any forbidden full-field/finite-domain fallback. Exercise the
  fail-closed path deliberately. Preserve budget/interruption outcomes.
- Relation mutations, sign changes, false RHS, duplicate/scalar-dependent rows
  and exact target recovery. No base-point secret logs enter the adapter.

## Resources and evidence

Local development only: at most two compiler workers and one test worker;
per solver call deadline <= 5 seconds, initial degree cap 5 for B4; later
bounded controls may use degree <= 10, 30 seconds and a documented matrix cap.
Limit each test process to 180 seconds and retain nonzero exits. Use an immutable
copied test executable with a pre-call SHA-256. Keep build, pre-call and post-call
manifests distinct; never overwrite a prior attempt. Capture raw stdout/stderr.
No benchmark clock result, speed ratio, GAE calibration or fitted exponent is
derived from these tests. All failures and caps remain part of the record.

## Original completion gate, still required

After an extractor passes the public correctness/rank gate at the requested
factor-base size, preregister an admitted cold single-target panel on at least
three eligible ordinary curves. Compare the strongest reproducible applicable
algebraic IC incumbent and optimized negation-map rho on identical public
targets/resources. Include base/selector construction, symbolic setup, all
successful and unsuccessful queries, roots, signs/exceptions, useful rank,
linear algebra, target recovery, replay, peak memory, failures and interruptions.
Report amortized work separately. Parameterize growing sizes before interpreting
an asymptotic exponent. The existing generic pair tables are controls only.

## Sources checked

KB unavailable: no loaded research-KB tools; `scripts/research-kb status`
returned exit 127. Fallback: current AGENTS, ACTIVE, November 22/23 logs, merged
PRs #4901/#4924, run69/70B/71/72, trace-skew and monic-root sources, and
`research/prime31_ic_20261115/rr9/RR9_PROJECTIVE_D9_THEOREM_20261117.md`.
Semaev 2004/031 and Faugere–Huot–Joux–Renault–Vitse EUROCRYPT 2014 were located;
direct IACR PDF retrieval returned 403 in this runtime. Existing internal
source-linked discussion supplies the scoped prior-art context. No novelty
claim follows from this incomplete literature access.

## Coordination

Conductor is unavailable on PATH. Use a new isolated checkout and disjoint
new source paths, as documented by `docs/CONDUCTOR.md`. Root owns extraction,
protocol/evidence and publication. A separate worker owns the signed-row adapter;
a mathematical reviewer owns the written equivalence and source review.

## Development 05 increment, frozen before build or execution

Development 04 passed its 17 correctness/cap tests. The B8 positive and empty
queries stopped Inconclusive without an eliminant; the expanded original-B677
input stopped at the 20,000-term construction cap. Those frozen outcomes remain
unchanged. The integrated F11 public loop recovered the target after nine public
queries and three independent accepted rows. No cost comparison was run.

Independent static review found cooperative-deadline accounting gaps. This
increment retains a public query even if point construction exhausts its budget,
stops row consumption when the target functional is known, preserves partial
records on relation rejection, starts the deadline before consumer setup,
preserves native time/output-limit status, and records an over-budget recovery
separately. Input-validation arithmetic on early chart returns is retained.

Add the exact coefficient lift U*W=h, with W monic of degree B-4. It has B-1
variables, B equations, and degree at most three. This changes the algebraic
presentation, not the solution set. Its larger variable count does not promise
faster extraction. Freeze these additional development controls:

- Prime31 B4 and B8 positive queries, same public x-scan bases and signed test
  fixtures as development04, coefficient-lift F4 degree cap5, five seconds per
  extraction. Require exact B4 recovery; retain B8 caps/inconclusive outcomes.
- Original B677 coefficient-lift construction only: construct all equations
  within five seconds; test a known-answer assignment obtained by test-only
  interpolation and numeric polynomial division. This is an input-identity
  check, not an extracted relation or a B677 solve. No B677 F4 call is included.
- A deterministically expired absolute-deadline control must return Deadline
  before native extraction. The earlier signed-row and recovery checks remain.
- Original native identity-certificate issuer/checker, two RR5 polynomial
  statements: 32 issuer points and 32 extra checker points from the fixed seeds
  recorded in identity_record.rs. proof=null; this is a sampled finite-field
  identity receipt, not a formal proof or signed standing validation.

Keep the previous two-compiler/one-test-worker, 2 GiB process virtual-memory and
180-second test-process caps. Preserve source, immutable executable, pre-call,
raw output, exit and post-call manifests under a new development05 directory.

Development05 stopped at compilation: the original identity checker also uses
the existing hex crate. Added that metadata dependency, retained its compiler
failure, resolved the lock offline, and reran the unchanged controls under
development06. All21 controls passed. The lifted B8 call remained inconclusive;
the B677 input-only check constructed676 variables,677 equations and13480 terms
of degree at most3. No new extraction or performance interpretation follows.

## Development07 setup certificate, frozen before execution

Execute only `selector_certificate::compact_selector_source_data_setup_certificate`
from a separately copied binary. It binds the actual RUN66 CSV and the existing
Base::build fixture to the compact-selector note: terminal1616683971,
1024 distinct Dickson roots,509 nonzero rational lifts,515 Euler-certified
nonsquare RHS values,no zero RHS,eight frozen cosets of21 distinct roots,
no overlap with the full Dickson fibre,and exact canonical677-point union.
Use original curve arithmetic and the unchanged original modular power routine.
This is a source-data setup certificate only; it runs no compact circuit solver,
no RR5 root extraction,and no benchmark. Keep one worker,2GiB,180seconds and
separate build/pre-call/raw/exit/post-call evidence. Source additions for native
artifact export are metadata tools and are not a cryptographic experiment.

## Development08 corrected setup certificate, frozen before execution

Development07 compiled successfully but its one setup test failed with exit101:
the last public seventh-power witness evaluated to1852957275, whereas the
historical cubic-factor example supplied1852955275. The original RUN52
`examples/prime31_coset_xlift_f4.rs` and the actual RUN66 CSV agree on the
corrected anchor `(1852957275,227959902,136769428)`. The first seven coset pairs
agree between the two source examples. Preserve the original sources and all
development07 receipts; correct only this study's copied final constant and
source attribution. Bind the RUN52 source in the new build inventory.

Repeat only the same selector setup test from a new immutable executable,
with the same one-worker,2GiB,180-second process limits. Require the complete
pre-call source check to pass before execution. No extraction, scalar solve,
compact circuit, or performance experiment is added. This separate check does
not relabel the21-test development06 suite as a new22-test run.

Also rebuild the metadata-only CLI after its independent static review:
serialized-size bounds, durable new-file directory entries, and explicit
post-mutation retained-state errors were tightened. Its export and unsigned
PCC schemas are unchanged. Registration must name an actual published producer
commit and preserve the current canonical ledger prefix.

Development08 outcome: both native builds exited0, every pre-call source check
passed, and the separately frozen one-test selector control passed. The raw
`COMPACT_SELECTOR_SETUP` object records the full predicted root/lift census and
exact original677-point union. It also explicitly records no circuit evaluation,
relation extraction or performance measurement. The development06 suite remains
its own21-test observation. Failed development07 and its binary-only metadata
source drift remain preserved separately.
