# Exact local cutoff reanalysis, 30 October 2026

User instruction: "Run expanded loop". Interpret this as the already specified
100--5000 cutoff expansion on the fixed21curve cohort. The separate request
for10000accepted random curves in each field remains open. No new seed,
curve, point count, factorization, scalar search or DLP is generated here.

## Execution classification and custody

This is deterministic reanalysis of already certified results, with all target
labels visible in advance. AGENTS.md section9a explicitly says local "analysis
of existing results remain development work". Use that local-analysis exception;
this is not an empirical random-curve experiment or a performance benchmark.
CHECKPOINT_SKIP=exact_reanalysis_of_existing_certified_results records this
classification. The experiment-only EKS and S3-launch gates are not claimed
to have passed. No S3 checkpoint or remote raw-artifact receipt is invented.
Preserve the previously denied paper archive locally and do not export it.

This classification is an explicit interpretation of the repository's existing-
results exception, not a claim that the random-curve experiment's gates have
been removed. Its execution freeze, checkpoint and worker requirements remain.
The former producer acknowledgement string implied a checkpoint; its only
source change here is renaming that string to ACK_LOCAL_EXACT_REANALYSIS.
Mathematical computation, cohort requirements and all verification assertions
are byte-identical after reversing that string substitution. The separate PARI
verifier is byte-identical to the prepared source. Preserve the originals.

Commit this plan, sources, input and build bindings to the existing PR before
executing the real cohort. That durable source record is a Git record, not an
S3 checkpoint. Retain all raw outputs and logs in a fresh local directory and
provide a complete downloadable archive. Remote artifact custody remains a
separate status if the proxy continues to prevent access.

## Fixed mathematics and scope

For every original h=1curve set t=p+1-n and Delta=t^2-4p. For each integer
L=100,...,5000 record strict S=#odd primes ell<=L with (Delta/ell)=+1,
ramified R=#with character0, and sensitivity S+R. Prime2has no assigned
Legendre character and is recorded as NA_ell2. Keep all21rows:10publications,
11named omissions. Only four fields have both labels; P384/P521 remain
singleton controls with no discriminatory within-field margin.

For each paired field record margin=min_omitted(S)-max_published(S).
The strict rule succeeds if every paired margin is positive. Record every
one of4901integer cutoff outcomes, all contiguous success intervals and
every field's positive-margin coverage. Report the complete range even if
the original pattern disappears. Do not choose a new headline cutoff.
The previously specified primary cutoff remains decimal691.

Enumerate all4480stream-publication-count-preserving label assignments over
the645effective cutoff positions representing100--5000. Report the number
allowing any strict separator, the fixed691number and the separately counted
inclusive sensitivity family. The earlier2--2000calibration must reproduce
23strict search successes and the fixed691case1. Expanded-family results
must not be labelled23/4480unless that is the actual recomputed value.
Fractions are finite enumeration proportions, not probabilities of a backdoor,
proof of historical exchangeability, or correction for every explored trait.

Source producer and verifier reconstruct all five files independently:
characters.tsv, profiles.tsv, cutoffs.tsv, calibration.tsv and summary.json.
Producer uses GMP arithmetic/residue enumeration and recursive label choices;
verifier uses PARI characters/primes and a brute-force label-mask inventory.
They can share GMP integer primitives; do not claim library independence.
Require exact byte agreement and the separate verification receipt.

## Bounded execution and controls

Use one native process at a time and a1536MiBaddress-space limit, clean
LANG/LC_ALL=Cenvironment, no native measurement clock and no paid resources.
Phases: producer self-test30seconds; verifier self-test30seconds; actual
producer180seconds; actual verifier300seconds. No automatic retries.
Keep all exit statuses and unsuccessful attempts. A failed invariant or
timeout is a failure, never an exclusion of a curve/cutoff or permission to
change the criterion. The two self-tests must exit0before actual extraction.

Then make three fresh output copies and deliberately corrupt one character
cell, a calibration flag, and a summary count. Each independent-verifier
invocation has300seconds and must refuse with native exit1. Preserve the
altered files and expected-versus-observed outcome. These are integrity
controls, not new curves and not successful verification of bad results.
Each of the three native corruption-preparation commands has30seconds and
must exit0. Thus the record contains ten native phase outcomes in total.

Derive report tables and graph input from the verified raw files with native
code. Any renderer only lays out those recorded numbers. Keep figure source,
plot data and inspected report PDF. Source inputs and counts are exact for
this finite cohort: sampling uncertainty bars are inapplicable. Any plotted
subset of cutoff positions must be explicitly identified; the full grid
remains in the raw archive. Preserve the earlier paper and its frozen figures.

## Interpretation limits and sources already reviewed

This is a retrospective robustness check. It neither explains an authenticated
historical decision nor establishes an ECDLP advantage. The eleven omissions
are a narrower retrospective subset of the69position draft-compatible census;
this calculation does not classify the wider59unpublished positions.
S depends only on p,n and cannot be altered by choosing G. No hidden G/b
relationship follows from this count. The full random-curve baseline and
independent full-journal verification are not discharged by this reanalysis.

Method and literature are unchanged from the committed version2design:
../random_curve_validation_20261029/PROTOCOL-v2.md and MECHANISM.md;
../strict_elkies_20261029/PROTOCOL.md and its native outputs;
../source_check_20261027/sources/acceptance-report.md;
Shparlinski--Sutherland arXiv:1112.3390v2, pp1--3;
Jao--Miller--Venkatesan arXiv:0811.0647v2, Theorems1.5--1.6.
No new distributional or security theorem is assumed.
