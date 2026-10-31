# Recorded ECPP work limits and historical curve omissions

27 October 2026. This is a prospective analysis plan for already completed
replay004 transcripts, written after the labels and the first transcript's
format were inspected. It is not blinded, a new historical dataset, or an
authenticated replay of the original generator. No legacy program runs here.

Question: can a fixed upper limit on recorded ECPP work accept all ten
published controls and exclude all eleven named omissions? Keep all 21 rows,
including P-384/P-521 and the twelve indeterminate certificate admissions.
Certificate-admission status is retained as text and is never a numeric feature.

## Sources and scope

The prior period_ecpp_20261026 trial supplies exact input/status/stdout packets,
its complete catalog, and its immutable archive and receipts. Original source
ecpp.c prints N_i, returned discriminants, chosen routes and factor records;
findd.c scans the catalog from rank zero at each depth. Catalog rank is one
based. With no retry/exhaustion, the last returned discriminant's unique rank
is the number of catalog records consumed at that depth. Printed returned
discriminants alone do not count all records: genus/norm failures are silent.
The terminal build-chain level can omit the End-of-depth line, and the final
Pratt record is added separately. Count explicit N_i/D[[i]] pairs, not timing
lines or final certificate records. No elapsed-time field is admitted.

New prior-work review found a completed portable FreeLIP prefix reconstruction
in crypto-autoresearcher at351c9c021c7be044ade54dc7e352b33d3a772244:
research/nist_seed_freelip_prefix_20261024/report.md. It reports24/24 matching
call paths, six manual-seven matches and24 nonmatching controls. Do not repeat
it or claim the seven-call idea as new. Its original invocation is unknown.
The corresponding period-NTL source audit distinguishes a different default
call path. Both RNGs are separate from lrand48 unless a caller couples them.
The Anthropic/Val­sorda reports and Mattsson's18October TLS message still leave
the omissions unexplained. Broader draft-based69/99-position censuses are
separate cohorts; this analysis does not relabel those positions or claim to
explain choices outside the fixed21.

## Fixed features

Extract exactly these ten integer quantities from the phase-one text:

1. root_catalog_records: last root-level catalog rank;
2. peak_catalog_records: maximum last rank over build-chain levels;
3. total_catalog_records: sum of those last ranks;
4. root_returned_forms: root-level next-D messages;
5. total_returned_forms: all next-D messages;
6. chain_levels: explicit N_i/D[[i]] pairs;
7. elliptic_levels: chosen D>1 pairs;
8. factor_records: phase-one Factor= records, including repeated prints;
9. factor_parameter_records: phase-one itmax= parameter records;
10. forced_retries: phase-one Forced-to-retry messages.

These are source-defined counts, not equivalent CPU costs. Catalog quantities
are unavailable for the affected root/chain if an exhaustion or retry prevents
exact recovery; never substitute zero. Other printed counts remain counts of
the retained transcript. A feature enters a cutoff comparison only if every
one of its21 values is known. No row is dropped. An incomplete ten-feature
joint test is inconclusive, not silently reduced to fewer features.

## Fixed cutoff comparisons

For every complete feature, evaluate four envelopes: upper/lower, each global
and per field. The tight upper bound that admits all published controls is
their maximum; the tight lower bound is their minimum. A skipped row inside
that envelope is a counterexample to every cutoff of that orientation which
keeps all the controls. Retain every such row and a published extremum witness.
These bounds use visible labels; they are fitted feasibility bounds, not
historical limits or independent predictions. Lower bounds are diagnostic,
not a claim that more expensive proofs were preferred.

Also evaluate the conjunction of all ten global upper bounds and of all ten
per-field upper bounds, only if all ten features are complete. A skipped row
inside every tight upper bound defeats every conjunction of these upper-limit
features that accepts all published controls. If a single or joint envelope
separates the labels, call it a fitted candidate only; source attribution and
an independently specified historical prediction are still required. No
unlisted feature, weight, subset search, p-value or new threshold is added.

## Admission and execution

Copy only the old inventory and final certificate-admission table. Freeze
SHA256/byte bindings for those inputs, the original catalog and all21 original
input/prover.status/prover.stdout packets. Retain raw sources in the existing
immutable archive. Strict C++17/Wall/Wextra/Werror analysis uses exact text and
integers, with no prime/curve arithmetic or scientific Python. Synthetic
positive/negative parser controls precede the census. Require ordered depth
pairs, a strictly descending decimal chain, exact requested root, catalog
membership, process0 and exactly one final prime CLI label. A parser/input
failure stops this analysis and is retained; no automatic retry or weakened
binding. All old certificate unknowns stay unknown.

Local analysis of existing results is development under AGENTS9a. No EKS or
paid worker is launched. Use a successful checkpoint gate, a fresh output,
768MiB address cap, disabled cores and a60second watchdog. This is a protection
ceiling, not a measured runtime. Capture the source, command, status and native
outputs; one analysis invocation processes21 packets. No clock/rusage or
performance isolation is admitted. Do not rerun ECPP or FreeLIP.

Preserve every failure. Publish native results and an experiment.export/v1
for this analysis invocation, source/data bindings, a graph/diagram, inspected
PDF and an update to existing ready PR2666. Store and fully download any new
raw archive before ignoring its run directory. Earlier studies/checksums stay
frozen. The original historical criterion and both weakness hypotheses remain
open unless an independently justified mechanism is actually established.
