# Random-curve calibration and cutoff robustness — design version 2

Recorded 29 October 2026; source-audit amendment at18:23UTC, before any new data. Supersedes version1 as explained in AMENDMENT-001.md. Status: written design; new sampling and extended
cutoff execution have not started. Commit and bind the native analysis before
collecting affected data. This file does not assert a completed canonical
experiment.protocol/v1 freeze, checkpoint upload, cloud launch, or result.

## Requested scope

1. At least 10,000 accepted, proved prime-order curves in each of the six
   existing prime fields: 192, 224, 239, 256, 384 and 521 bits. This means
   60,000 accepted curves, not 60,000 attempted seeds. Report all ten published
   curves' percentiles in their respective fields.
2. Sweep every integer cutoff from 100 through 5000; publish every outcome,
   field-level margins and normalized count profiles, including failures.
3. Fix the statistic, direction, cutoff, sampling and decision rules in
   writing before the new data. Never describe already examined P-384/P-521
   as unseen holdouts. Reserve new samples for prospective null calibration.
4. Explain what the statistic measures and what an actual ECDLP mechanism
   would additionally require. No DLP, attack or weak-curve claim is requested.

## What was known before this design

All 21 original rows, labels and counts through 2000 were visible. The earlier
strict separating prime cutoffs are 691, 701, 709 and 769. The original
303-position, 4480-label calibration is retrospective. P-384 has strict
count 54 and P-521 has count 52 at 691, and both lack an omitted control in
that 21-row cohort. The earlier draft-compatible census contains 69 seed
positions (10 published, 59 unpublished); the named eleven omissions are a
retrospectively restricted subset. None of these facts is prospective evidence.

## Fixed primary statistic and question

For h=1, set t=p+1-n and Delta=t^2-4p. Count

    S(E,L) = number of odd primes ell <= L with (Delta/ell) = +1.

The primary cutoff is decimal 691, with the lower-count direction fixed.
Keep zero characters in a separate R(E,L) column. Prime 2 is excluded, not
assigned a numerical character zero. S+R is a separately labelled sensitivity
analysis and cannot replace S if it gives a better result.

Primary question: where does each published S(E,691) lie in the distribution
produced by the matched seed-to-curve procedure, conditional on the fixed
acceptance predicates? Do not assume a binomial distribution for S or replace
curve point counts by random primes in the Hasse interval.

## Population and sampling

Use the exact six field moduli in ../strict_elkies_20261029/inputs.tsv and a=-3.
Use SHA-1 seed expansion from the pinned seeds.go.txt (SP 800-186 C.3.1/C.3.2):
v=floor((bits(p)-1)/160), w=bits(p)-160*v-1; h0 is the w low bits of SHA-1(seed),
followed by SHA-1((seed+i) mod 2^160) for i=1,...,v. Interpret the concatenation
as c. Reject c=0, 4c+27=0 modulo p, or nonsquare -27/c. For p=3 modulo 4 use
the exponentiation root (-27/c)^((p+1)/4). For P-224 choose the smaller root;
the two signs are isomorphic over this field and give the same primary count.

The primary sampling source is a fresh 32-byte nonce drawn and recorded AFTER
the committed design/analysis freeze. For field bits B and attempt index i>=0,
take the first 20 bytes of

    SHA256(ASCII("ICV1-random-curve-calibration/v1") || 00 || nonce[32]
           || uint16_be(B) || uint64_be(i)).

This is a pseudorandom seed baseline using the same seed-to-curve map; it is
not the original timestamp-seeded drand48 process and is not uniform over
isomorphism classes. Its Monte Carlo interpretation assumes the seed source
behaves as independent uniform 160-bit draws. State that assumption.

A separately named matched-drand48 sensitivity cohort is also requested if a
supported worker route is available: freeze an additional independent nonce,
derive one 32-bit initializer per field by SHA256(domain || nonce || B), reject
the six already known historical initializers, emit the top byte of each new
48-bit state, and group 20 consecutive bytes into a seed. Use the same curve
map and acceptance predicates. Target 10,000 accepted per field separately;
do not pool its serially correlated samples with the primary IID-model cohort
or attach IID binomial intervals to it. This remains a distinct, unimplemented
sensitivity arm until its source, seed derivation and dependence-aware analysis
are frozen. It must not be silently substituted for the primary cohort.

## Acceptance and exact order verification

Match the pinned reconstruction's root, order-width and embedding-degree
conventions, and require nonsingularity, proved prime full order n,
bit_length(n) <= bit_length(p), and p^k != 1 modulo n for 1<=k<=1023.
Also explicitly require n != p as an additional nonanomalous-curve condition.
The reference suitableOrder uses ProbablyPrime(20) and has no explicit n!=p
branch; its comment that the width check excludes anomalous order is not a
logical implication. This study strengthens primality to a proof and adds
the stated anomalous-order exclusion; do not claim literal equivalence to
the reference program. These include retrospective conventions, not a verified
1997 acceptance policy. Report every conditioning choice and rejection count
in each percentile caption; the native sampler already implements them.

PARI ellsea(E,1) may abort when it proves a small prime factor of the order;
such an abort is a composite-order rejection, not a point count of zero.
Survivors need a proved primality test, not only BPSW/probable primality.
For every accepted curve store p,a,b,seed,c,n and a nonzero point P, check the
Hasse inequality, n^2>16p and [n]P=O with separate GMP point arithmetic.
Together with prime n these checks certify the exact full order. Re-run the
primality proof and point certificate in a separate verifier; disclose shared
PARI/GMP dependencies. A claimed accepted sample without those checks is invalid.

Record every attempted seed and its deterministic disposition. Errors,
timeouts, OOMs and missing records are NOT mathematical rejections. Never
discard a slow candidate or stop after a favourable percentile. Complete the
contiguous seed prefix ending at the 10,000th accepted candidate in each field.
Retain duplicates as draws and report them; do not redraw to improve diversity.

## Percentiles, uncertainty and multiplicity

For each of the ten published targets, report k_lt, k_eq, k_le and N, with
the tied percentile interval [k_lt/N, k_le/N] and descriptive midrank. The
primary lower-tail Monte Carlo probability is (1+k_le)/(N+1), conservative
at ties; never report zero. Use exact 95% Clopper--Pearson intervals for the
two CDF endpoints under the stated IID seed model. Show counts as well as
rounded percentages. N=10,000 resolves the empirical CDF in steps of 0.01
percentage point; this is resolution, not a 0.01-point confidence interval.
The widest ordinary 95% marginal CDF interval is about one percentage point
on either side at this N. No retrospective effect-size power claim is made.

Treat all ten published-curve primary tests as one family and apply Holm's
procedure at familywise alpha=0.05. Predeclare a descriptive lower-decile
flag (k_le/N<=0.10), reported separately from corrected significance. Shared
same-field controls induce dependence, which Holm tolerates. There is no
pooling of fields, target selection after results, upper-tail switch, or
claim that calibration proves historical selection or a backdoor.

## Extended cutoff sweep and retrospective calibration

For each integer L=100,...,5000, report S,R,S+R for every original row. For
each paired field report margin min_omitted(S)-max_published(S); the original
within-field rule succeeds exactly when every paired margin is positive.
Retain the 384-/521-bit singleton rows without assigning a separation margin.
Cutoffs with no new prime repeat the previous counts; report contiguous
integer success intervals, not just a selectively chosen prime list.

Enumerate all 4480 original stream-publication-count-preserving assignments
over THIS expanded cutoff family and report the fraction allowing any
lower-count separator. Also report the fixed-691 fraction and the separate
inclusive family. All of this remains retrospective. Expanding to 5000
changes the search family; do not reuse 23/4480 as its adjusted fraction.

After a complete primary baseline, publish all ten percentile trajectories
through 5000. Only 691 is primary. The trajectories are descriptive unless
a simultaneous procedure is separately frozen before the new data; pointwise
bands must not be described as simultaneous significance across the range.
An effect that fails across most of the range must be reported as such.

## Controls and analysis discipline

The null arm is actual seeded, counted, accepted curves with the stated
conditioning. Positive controls are toy fields with exhaustive point counts,
planted ordered/misordered count vectors and known quadratic residues,
nonresidues and zeros, all through the same instrument. A/A controls rerun
an identical fixed toy seed batch and compare complete output bytes; this
checks determinism, not the null sampling variance. An independently seeded
secondary batch, if run, is recorded separately and cannot replace the fixed N.
Corrupt a seed, order, point and a character/table cell and require refusal.

Fix arithmetic, source/build hashes, libraries, seed encoding, order predicates,
cutoff, ties, multiplicity and analysis. Measure host/runtime versions, seed
attempt counts, duplicates, errors, provisioning and interruptions. Interleave
the six fields in the scheduler. Give temporary field-neutral IDs to analysis
tables; no analyst blinding or independence is claimed because these targets
and labels were already inspected. No timing or speedup is a primary metric.

## Execution, stopping and current launch gates

Default backend is EKS Spot, counted work only. The currently inspected portable
request and recovery contracts execute ecbench; they cannot execute this PARI
worker. The source MCP catalog describes an arbitrary-command run method, but
no such MCP connection is installed in this session. A matching tested image
and supported authenticated submission route remain required. Do not launch
a fixture, modify deployment permissions, or silently substitute another host.

Before any affected experiment run, AGENTS.md requires a successful uploaded
checkpoint and checkpoint.sh gate. An earlier paper archive's S3 export was
explicitly rejected by automatic approval. Do not bypass that decision by
including the paper payload in this study's checkpoint or using another route.
Prepare and inspect a concrete checkpoint payload and obtain authorization
covering any previously denied material before export. No new upload is made
by writing this protocol.

First launch is a cost/verification pilot, not a replacement for 60,000 accepted
curves: ten accepted curves per field or the fixed deadline, at most one worker
per field, two vCPUs each, 60 minutes each, no automatic
retry. Store all attempts and use a distinct nonce. Do not reuse pilot samples
in the production cohort. A pilot can establish measured attempt/acceptance
costs and whether the worker/certificates function, but cannot produce the
requested 10k percentiles. No production spend or completion-time estimate is
claimed without that measurement. Freeze production resources and an absolute
deadline in a version-2 execution amendment before production data; retain this
version. At deadline, report incomplete N and leave percentiles unestablished.

No job is launched by this design. Native builds and bounded toy unit tests are
development work, not the requested random-curve study. Record the canonical
protocol/export and independent replay before promoting measured conclusions.

## Interpretation and falsification

If the published curves have ordinary percentiles, the observed finite separator
does not establish unusualness against this null. If they are unusually low
after correction, that establishes an association under the frozen null, not
who selected it or why. If the sweep is narrow, describe cutoff sensitivity.
Additional historical controls must include the broader draft-compatible
cohort, with a root/seed-level rule frozen first. Such controls are not claimed
to be unseen after the previous census inspection.

S depends only on p and n, and cannot be changed by choosing G. The identities
y^2=b at x=0 and y^2=b-2 at x=1 are forced by a=-3 and are not independent
evidence of a secret scalar relationship. MECHANISM.md states the remaining
map, subgroup-preservation, destination-solver and measured-cost obligations.

## Sources reviewed before this design

- Pinned source: ../sources/seeds.go.txt, generation, hashSeed and suitableOrder.
- Pinned source audit: ../source_check_20261027/sources/acceptance-report.md.
- Existing frozen ../strict_elkies_20261029/PROTOCOL.md and results/summary.json.
- Shparlinski--Sutherland, On the Distribution of Atkin and Elkies Primes,
  arXiv:1112.3390v2, pp.1--3 and section 5. Its generation theorem varies p;
  it is not a finite-field percentile model for these six fixed p values.
  https://arxiv.org/pdf/1112.3390
- PARI official ellsea documentation: tors=1 proves some composite rejections
  early; aborts are not permission to omit slow candidates.
  https://pari.math.u-bordeaux.fr/dochtml/html/Elliptic_curves.html#ellsea
- Jao--Miller--Venkatesan, Expander graphs based on GRH with an application
  to elliptic curve cryptography, arXiv:0811.0647v2, Theorems 1.5--1.6 and
  sections 4--6. Conditional reductions within isogeny/endomorphism levels
  require a destination solver; the count alone supplies no ECDLP attack.
  https://arxiv.org/pdf/0811.0647
