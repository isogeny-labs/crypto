# NIST candidate selection: weakness hypotheses remain unresolved

- **Status:** `investigating`
- **Kind:** unknown selection cause; missing applicable attack construction and validation
- **Affected work:** [selected/skipped comparison and protocol](../research/curve_candidates_dissect_20261021/hypotheses/REPORT.md)
- **Next action owner:** unassigned
- **Last updated:** 2026-10-30
- **Latest search:** [30 October exact label-allocation audit](../research/nist_skip_criterion_20261025/exact_pair_audit_20261030/RESULTS.md)

## Problem

**Latest result, 30 October, exact allocation audit:** using the already
verified scores, the fixed comparison of42additional candidates with eight
publications gives55higher additional scores,7ties and40lower scores across
102within-field pairs. Independent subset enumeration and rank-DP methods
agree on all20037600label assignments:5869345are at least as extreme as the
observed117/204score, an exact allocation fraction0.292916567. All68external
score observations match within the stated seed-ID/multiset scope. This is
retrospective existing-results arithmetic, not new random-curve sampling or
repair of the external study's disclosed procedural deviations. All ten
phases pass, including three deliberate corrupt-output refusals. Public
G=[s]Qrecipes with invertible known s merely convert the DLP between bases;
they supply no solver. Historical cause and both weakness hypotheses stay open.

**Earlier result, 30 October, broader scope:** the fixed strict-splitting rule
fails throughout100--5000on the recorded69position prime-order census. All82
branches are retained, and the independent PARI verifier proves all76distinct
field/order primes, checks the point certificates and reconstructs six output
files. At691there are22unpublished positions at or below their field's published
maximum:14other-root-only and8width-only. P192,p239,P256have no positive-margin
cutoff individually. Original21subset reproduces691--718and769--772exactly.
The broader result limits splitting alone. Applying the additional root/width
screens first still permits the narrow fitted criterion, whose historical
use and motive remain unverified. No actual original visits are inferred.
All ten phases match expectations, including three corruption refusals.
No new curves, point counts, DLP or performance measurement. Full random
baseline and authenticated historical explanation remain open.

**Earlier result, 30 October, narrow scope:** the complete100--5000integer cutoff reanalysis
retains all21rows and independently reconstructs all five native output files.
Joint strict separation occurs only at691--718and769--772:32of4901integer
positions (0.6529%), with no successful cutoff above772. Including ramified
primes produces zero joint-success cutoffs. P224has positive margin at3957
positions (80.7386%); the other paired fields have positive coverage7.6311%
to20.7713%. These are descriptive within-cohort grid fractions, not random-
curve probabilities or comparisons of statistical significance between fields.

The requested100--5000label family allows29of4480assignments; the old2--2000
family allows23. They share22, with7new-only and1old-only assignment because
the requested range removes cutoffs below100. Their supplementary2--5000
union has30successful assignments. All calibration remains retrospective and
restricted to these declared families. Three corrupted outputs are correctly
refused. The local exact reanalysis uses the existing-results development
exception documented before execution; no S3 checkpoint or remote custody is
claimed. The raw176member archive is delivered locally with full hash readback.

This establishes cutoff sensitivity, not the historical selector or an ECDLP
advantage. At that stage the60000accepted random-curve baseline and broader
69position classification remained open; the later scope check above addresses
the latter. The random baseline remains unrun, and original decision logs absent.
No new curve, point count, DLP or performance measurement was run.

**Prior result, 29 October afternoon:** an externally reported empirical
criterion reproduces exactly. Count odd primes ell≤691 with strictly positive
Frobenius-discriminant character. Within each paired field, every publication
has a smaller count than every omission. The published/omitted ranges are
P-192 57–60/61–62, P-224 50/53–59, P-239 42–57/61–71 and P-256 57/58.
All 21 curves remain, including unpaired P-384/P-521 controls. Separate
native verification checks 6,342 characters, 303 cutoff positions,
4,480 label assignments and all six output files. The restricted-family
count 23/4,480 reproduces. This is not a new discovery
or independent historical holdout. Including ramification makes cutoff691
fail in P-192; the exact definition matters. See the current report/PDF above.
The historical explanation and both vulnerability hypotheses remain unresolved.

**Current status, 29 October:** the full declared generator/coefficient
comparison has now run and passed separate native verification on all 21
choices. It checks 100,548 point-trait cells, 19,152 single envelopes,
147,875 same-field difference cells and 880 scalar checks. No exact single
envelope separates all labels and no tested scalar recipe matches. The
canonical P-224 x pattern 0/1/1 versus 3 holds in both views but retains eight
other omissions. All eleven unknown historical skipped generators remain NA.
The original four factor targets remain proved complete (84/84 records).
The prior access blocker is resolved by testing the already-authorized
client; earlier dated blocked/unrun notes below are historical. See the
[current report/PDF](../research/nist_skip_criterion_20261025/generator_b_execution_20261029/REPORT.md).

We cannot yet establish that the published P-224 or P-256 curve is vulnerable
to an ordinary-input attack, or that weaknesses in earlier eligible curves
explain why they were skipped. The historical skip rule and any applicable
hidden-attack witness remain unknown.

The initial six exact models, generators and full ICV1 identities remain
frozen in the study's `hypotheses/inputs.tsv`. The latest linked study expands
the recorded population to 21 eligible prime curves: ten published and eleven
skipped across six streams. Its source and normalized parameter tables remain
separate. Eligibility uses the recorded known checks; it is not a historical
decision log.

## Evidence and uncertainty

- **Observed:** the earlier eligible indices match the four supplied curves.
  The native diagnostic rechecks seeds, orders, twists, nearby-order factors
  and 3-division polynomials on the six exact models.
- **Observed:** P-224 index 1485 has a larger largest twist prime than the
  published P-224 curve. Published P-256 has a stronger twist but a smoother
  `n-1` than its skipped counterpart. Published P-224 has Frobenius-order
  conductor 3 and a fully split 3-division polynomial, unlike its three
  skipped counterparts. Exact values and scope are in the linked report.
- **Observed:** the [native factorization follow-up](../research/nist_skip_criterion_20261025/factor_followup_20261026/REPORT.md)
  has proved all four requested integers, including P-521 `n+1`, completely.
  There are 84/84 complete records and 1,127 features per row in the replay.
  All four complete-factor families are admitted over all 21 controls.
  The shortest structural conjunction still has three tests; generation
  features retain the two-test fit. Neither establishes historical use.
- **Observed:** an [exact native SEA-feature replay](../research/nist_skip_criterion_20261025/sea_replay_20261026/REPORT.md)
  matches all 21 recorded bounds. A uniform preference for a larger bound
  fails on P-256 (published191, skipped211); the reverse fails on P-224
  (published257, skipped139/179/233). The proxy includes ramified primes
  and omits Atkin information. No new p-value, measured SEA cost or
  historical interpretation is admitted.
- **Observed:** [exact embedding degrees](https://github.com/isogeny-labs/crypto/pull/2648)
  were certified for all21 orders. Maximal degree accepts0/10 published and
  2/11 skipped; the modern RFC7748 bound accepts10/10 published and10/11
  skipped. Opposite-label P256 orders and several P224 orders share
  embedding index0x3. These fixed criteria do not explain all skips.
- **Observed:** the [period ECPP artifact trial](../research/nist_skip_criterion_20261025/period_ecpp_20261026/REPORT.md)
  executed all21 known-prime orders with one fixed full-catalog configuration.
  All21 raw CLI labels were prime, with no composite label. Corrected saved
  certificate admission accepts3 published and6 skipped, retaining7 published
  and5 skipped as indeterminate because of actual failed legacy theorem rows.
  All three originally supplied skipped P224 curves pass admission. The
  twelve unknowns are not weak/composite/rejected labels. This configuration
  cannot provide a universal rejection explanation; historical use is unknown.
- **Derived or proved:** primality and product checks certify the recorded
  factorizations; generator and Hasse checks certify the six prime full
  orders. These are computational checks, without Lean formalization.
- **Observed:** [recovered FreeLIP 1.0/1.1](../research/nist_skip_criterion_20261025/freelip_replay_20261027/REPORT.md)
  completed all84 fixed calls (two routines in both versions on all21 original
  orders). Every configuration accepts all10 published and all11 skipped
  orders, with no rejection or unknown after source-faithful saved-output
  validation. The original wrapping outputs and initial observer unknowns
  remain preserved; no legacy rerun was used for that correction. These
  routines do not explain the skips when fed the correct group orders.
- **Proposed source connection:** zprobprime(n,7) performs a base2 test then
  seven random-base trials, consistent with a possible cause of the published
  seven-call FreeLIP warmup. Rejection sampling and absent original-caller
  evidence prevent equating this with seven state advances or claiming that
  the historical generator used that call. No new base-point reconstruction
  was executed.
- **Observed:** the [exact embedding-degree test](../research/nist_skip_criterion_20261025/exact_embedding_20261026/REPORT.md)
  computed and separately certified all21 minimal degrees. None of ten
  published curves has maximal degree; two skipped curves do. The fixed
  modern RFC7748 bound accepts ten published and ten skipped curves,
  rejecting only skipped p239 index1562. Published/skipped P-256 both have
  `(n-1)/k=0x3`, precluding separation by a filter of that normalized degree
  and their common field alone. These are exact counterexamples to those
  filters, not historical attribution or a practical pairing attack.
- **Suspected explanation:** unknown. Deliberate weak-curve selection,
  deliberate avoidance of weak candidates, a family-wide unpublished
  method, and engineering or bookkeeping filters remain distinct hypotheses.
- **Observed and conditionally proved:** [saved ECPP work analysis](../research/nist_skip_criterion_20261025/period_work_20261027/REPORT.md)
  retains all21 records and independently checks all210 cells of ten fixed
  source counts. None of40 single cutoff envelopes separates the labels.
  Even the conjunction of per-field upper envelopes retains skipped p239
  indices1562 and2682. Each is no larger componentwise than published
  indices2185 and3048. Eight axiom-free Lean theorems certify those literal
  inequalities and the implication for any monotone cost with transitive
  comparison: accepting2185 under an upper budget also accepts both omissions.
  This excludes those counter-based budgets, not unrecorded point-counting
  work or arbitrary acceptance rules. The dominance diagnostic was posthoc;
  no held-out prediction or historical-use attribution is claimed.
- **Missing capability:** an explicit applicable attack or retained-witness
  route, its full cost, and a prospective selection-rule test.
- **What this evidence does not establish:** a trait difference is not an
  ECDLP attack. Conditional auxiliary-input or twist attacks need actual
  protocol preconditions. Routine checks and bounded negative searches do
  not prove security or exclude an expensive intermediate advantage.

## What would unblock it

- [ ] Freeze the attack contract: exact subgroup, public and auxiliary inputs,
      oracle access, any retained witness, construction and recovery costs.
- [ ] Supply and verify an executable route, including subgroup preservation
      and independent recovery, against the matched strongest reference.
- [ ] For a selection explanation, specify a rule and confirm it on held-out
      matched candidates after conditioning on all known acceptance filters;
      retain failures and apply the declared multiple-testing policy.

An attack can establish vulnerability without establishing anyone's intent.
A predictive filter can explain selection without establishing an attack or
proving historical use. Neither hypothesis is disproved by failure to find
a route within the present diagnostic.

## Next discriminating step

The [latest source check](../research/nist_skip_criterion_20261025/source_check_20261027/REPORT.md)
found no original decision record in the checked public and connected sources;
none was supplied. This access gap remains explicit. The prior November-draft
and broader-cohort reports were retrieved at pinned commits and read without
recomputing their arithmetic. This does not prove that no archive or criterion
exists. Eligibility in a reconstructed stream does not establish original
visitation and rejection.

For the selection question, obtain an original generator/acceptance log,
additional genuine historical choices outside the fitted dataset, or a
specified mathematical mechanism with independent predictions. Freeze the
rule before examining new labels, and replay any recovered code against the
entire recorded stream. Random new streams alone cannot validate a claim
about historical choices. Group-order-dependent RNG schedules and an
unrecovered old SEA implementation remain outside the tested finite family.

The 21-curve diagnostic, short-rule calibration and 52,520-model generation
search, full84 factorization replay, exact-degree checks and fixed period-ECPP
trial ran. The ECPP source mirror is not authenticated as the original
generator's bytes, and no original caller/acceptance record links it to those
choices. Further artifact/configuration variants are untested; this bounded
negative trial does not exclude all certificate or historical resource filters.
No attack, explicit map or new historical holdout campaign has been scheduled.
For the vulnerability tracks, the original explicit-map and
auxiliary-input obligations remain. New runs retain their own protocol,
limits, failures and exact witness obligations.

## Resolution and history

2026-10-21: connected the supplied curves to the pinned earlier-candidate
census and added a native six-curve diagnostic. Both weakness hypotheses
and the historical skip explanation remain unresolved.

2026-10-26: expanded to all 21 eligible prime curves and found independently
verified empirical two-test and three-test separators. No historical
mechanism is established: 4,385 of 4,480 permitted shuffled labelings admit
an exact one- or two-test separator somewhere in the compared search families.
None of 52,520 defined RNG/generation models fits all labels. All 21 twist
orders are fully factored and proved. The raw records, finite search scope,
root-convention counterexample and remaining evidence obligations are retained
in the linked report. An exact fit does not resolve the selection explanation
or establish a vulnerability. Unresolved.

2026-10-26 follow-up: all four remaining factorization targets passed
independent native prime/product proofs. The unchanged full-21 search replay
has 1,127 features per row and retains the existing conjunction lengths.
Generation-family as-good calibration is 1,973/4,480 for an exact fit of
at most two tests; the earlier 15-family union remains frozen separately.
The [factorization obstruction](nist-p521-nplus1-composite-cofactor.md) is
resolved. The historical explanation and applicable attacks remain unproved.

2026-10-26 continuation: all21 embedding degrees were checked exactly, and
all21 period ECPP prover/checker pairs actually ran. Six admitted skipped
certificates contradict the fixed artifact's reject-all-skips hypothesis.
The native CM metadata reader was corrected from original source and every
saved packet revalidated without a legacy rerun. Twelve genuine failed
theorem transcripts remain indeterminate. All failed preparation/runtime
attempts and the initial restrictive-reader results are retained. This is
additional negative evidence, not completion of the historical selector goal.

2026-10-27: tested actual recovered FreeLIP source because its RNG is linked
to six published May1997 base points. Both original versions and both routes
accept the complete census. This excludes these fixed correct-order primality
paths as a universal skip explanation; a caller feeding incorrect point
counts, a different implementation/configuration, or another acceptance step
remains outside the test. A source-derived seven-call mechanism is proposed,
not historical attribution. The original criterion remains unresolved.

2026-10-27 saved-work follow-up: native extraction and a distinct token scanner
check ten work features for every row without changing twelve old certificate
unknowns. All40 declared cutoffs fail, and two same-field omissions dominate
published controls in the required direction. Lean4.34.1 and the unchanged
repository axiom auditor check eight conditional-budget theorems with empty
axiom lists. All failed proof attempts and144 raw archive members are retained.
Prior sibling FreeLIP prefix reconstruction was found and cited rather than
repeated or claimed as new. Original-caller, point-count and visit records
remain the missing bridge. The historical criterion and both weakness
hypotheses remain unresolved.

2026-10-29: the user confirmed that original generation/point-count/acceptance
records are unavailable and expanded the requested comparison to a wider
difference inventory, especially a possible base-point/coefficient relation.
The [generator/b study](../research/nist_skip_criterion_20261025/generator_b_20261029/REPORT.md)
separates the assigned smallest-x points from ten known standardized generators
and eleven unknown historical skipped generators. Existing measurements show
b is square for 9/10 published and 9/11 skipped curves. The x=0 and x=1
relations follow from the equation; five conditional Lean proofs and their
transitive audit pass. These facts supply no historical selector or attack.

The finite extension implements 171 point/coefficient traits, model and
generator controls, all same-field pairwise differences and public-scalar
equality recipes. The native producer and separate GMP verifier pass a tiny
synthetic integration and nine corruption controls. **The actual new cohort
comparison has zero attempts.** The restored runtime reports no checkpoint
identity, and the mandatory uploaded-checkpoint gate prevents launch. No paid
resource or DLP campaign ran. This is an access dependency, not a proved
mathematical obstruction, and tests do not satisfy the measurement request.

To unblock that extension, restore the existing lab checkpoint identity through
runtime credential settings, verify the complete preparation upload/readback,
freeze the supported execution route and admit all 21 rows under the stated
protocol with independent checking. Preserve all unknowns and failures.
Completion would characterize the declared finite family; historical use would
still require a decision trace or independent historical predictions.
The current preparation/report/PDF is an additive partial deliverable.

2026-10-29 actual execution: the existing authorized client authenticates,
preparation remote GET/member verification passes and checkpoints upload.
The earlier failure to test that access was corrected explicitly. Local exact
analysis was declared after the supported EKS contract could not express
the native job and project inspection was denied. No paid job, policy change
or DLP campaign was substituted.

Attempt001 is preserved: positive0, production124 at300seconds under768MiB,
verification not attempted and zero point-trait rows. Resource diagnostics
support an explicit frozen version2 with only the uniform address ceiling
raised to1536MiB. Original binaries/prover and all inputs/families are unchanged.
Attempt002 completes all three phases0 and all21choices. A separate GMP
affine verifier checks the full finite outputs; underlying GMP integers and
OpenSSL SHA are shared. Full raw objects pass remote GET and member checks.
Canonical version2 export passes file verification with zero findings.
Version1's partial export remains refused, with its missing phase disclosed.

The wider finite inventory is now a verified deliverable, superseding the
preparation's unrun status additively. No supported historical skip rule or
applicable attack is established. The user's absence of original logs is
recorded; do not ask for them again. New evidence or an independently
predictive mathematical mechanism is needed for the historical claim.
The prior six-basepoint FreeLIP RNG reconstruction remains relevant and is
not refuted by zero matches in this different scalar-recipe family.

2026-10-29 strict-count follow-up: the asanso17October authored post was read
through a named mirror (direct X endpoint403) and its concrete claim was
frozen before a new native replay. The full21replay and separate PARI
verifier pass, including three deliberately corrupted-output refusals.
Strict separating cutoffs are691,701,709,769;23of4480stream-count-preserving
labelings admit some separator in the303-position family. The fixed691
count1/4480cannot be presented as confirmatory because that cutoff was
selected retrospectively in the earlier work. Other hypotheses examined
are outside this calibration. The two singleton fields do not supply skip
controls or independent predictions.

This p,n-dependent profile does not change when the base point changes;
the eleven original skipped generators are still unavailable. It matches
the recorded paired labels but does not authenticate the original caller,
acceptance rule or visit history. The user has no original records. Do not
declare the historical objective complete or claim exhaustive measurement
over all mathematical parameter functions. The exact finite inventories
are complete; broader coverage and a supported historical mechanism remain
open. No DLP, actual point-counting cost or attack was measured.

2026-10-29 publication obstacle: result commit713202779 and normal main merge
ce2cd12c8 are local; remote head was last confirmed as1238726d9. The PR title
and body are updated, but these new commits are not yet pushed. Local canonical
verification, bound evidence and size checks pass. The artifact check requires
the85071-byte strict-count plot to move to the artifact store. The exact upload
was stopped on three attempts before execution because automatic approval review reported
its selected model at capacity. This is a review-service resource failure,
not failed authentication or a determination that the upload is unsafe.
The [publication note](../research/nist_skip_criterion_20261025/PUBLICATION-20261029-STRICT.md)
tracks the concrete remaining steps. Unblock by restoring automatic review,
executing the same authorized upload, verifying a full remote GET, passing
artifact admission, and normally pushing the existing branch. Those steps
publish the empirical result; they do not resolve historical attribution.

2026-10-29T15:45Z storage update: automatic review recovered; the identical tools/artifacts
plot upload succeeds and a full remote GET verifies every original byte.
The new pointer/receipt are bound separately, preserving all 51 result hashes
and 77 previous parent lines. The temporary artifact-storage obstacle above
is resolved. Normal push and current-head checks remain delivery steps; the
historical and vulnerability obstructions are unchanged.

2026-10-29T15:51Z delivery update: normal push of728caf607 succeeds; the connector
confirms that exact open, non-draft, mergeable PR head. Local artifact/size
checks pass and earlier approval-service obstacles are resolved. Hosted
gate/size/artifacts are running and PR review has not returned; no merge or
current-head hosted approval is claimed. The finite criterion and PDF are
published deliverables. Historical attribution and weakness remain open.

2026-10-29 validation expansion: the user's requested controls now include
10,000 accepted prime-order curves per field, each publication's percentile,
the full100--5000cutoff sweep, a written fixed criterion before new data,
and an ECDLP mechanism assessment. The [new design and native preparation](../research/nist_skip_criterion_20261025/random_curve_validation_20261029/README.md)
retain all four requirements. Primary generation, exact sweep production,
independent sweep verification and tied-percentile/Holm analysis have native
preparation implementations with passing bounded development tests. They
have not generated a new baseline or extended cutoff result. Full-journal
verification, production recovery/scheduling and an approved PARI worker
route remain gaps; the portable EKS contracts inspected support ecbench.

The fixed primary statistic is strict+1count at691 in the lower direction;
new uniform-seed calibration would use the pinned SHA1 curve map and explicit
acceptance conventions. Already examined P384/P521 cannot be unseen holdouts.
A robust calibrated deviation would still not establish a historical
selector, a security motive or a DLP advantage. The [mechanism note](../research/nist_skip_criterion_20261025/random_curve_validation_20261029/MECHANISM.md)
states the missing transfer/solver/cost obligations. The first discriminating
execution remains a separately labelled bounded pilot followed by the full
60000accepted-curve primary cohort, not a substitute small run. Exit requires
complete verified journals, matched percentile/uncertainty output and the
full cutoff report under a committed execution freeze and checkpoint.
