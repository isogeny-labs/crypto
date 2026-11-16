#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= RR/F output normalization: qualification continuation
12 November 2026 · Native development checks · Producer-unverified

The native extractor now checks one rational assignment against the emitted
input digest and every equation, reconstructs reduced RR variables if needed,
and returns canonical signed point triples summing to the public R. It rebuilds
the full declared tower root set from public metadata; stored planted supports
are not used. Claim PCC-20261112-699e09 is unverified.

#image("extraction-flow.svg", width: 100%)

== Exact scope
This is the m3 common monic-RR chart: distinct rational support abscissae,
nonzero support y and constant C, support disjoint from x_R, monic quadratic A,
and finite R with nonzero y_R. RR signs come from -A/C; F enumerates signs and
orients them to R. The reconstructed norm identity and group sum are checked.
Unsupported charts remain errors, never silent empty answers. Extraction is
implemented for RR and F; T and m5 are not covered by this addition.

== Checks and limits
Nine native release tests pass. The bounded Dickson fixture enumerates all
distinct support triples and signs through a separately written finite reference
and compares normalized RR/F outputs, including multiple-output targets. A
zero-output target is checked across every distinct support. Other checks cover
negative-target orientation, Kummer scaling, reduced lifts, repeated roots and
C=0, modified metadata, mismatched input digests, invalid assignments and map
digests. The original emitter's 22-group selftest also passes.

The reference shares field primitives, and positive supports are encoded through
the existing emitter. This is not an independent signed audit or exhaustive
enumeration of the ambient solver variety. A runner must still obtain every
rational solver assignment, union normalized outputs and qualify completeness
against an independent reference before equal-output timing is admitted.

== Current dependency and next action
The S3 checkpoint upload transport and current shared budget/pool readback are
still unavailable. No F4 comparison or paid run started. Updating PR 3835 from
main also conflicted with the append-only claim ledger; the merge was aborted
without choosing a ledger side. The implementation remains reviewable on the
existing branch, with frozen evidence preserved.

Next: evidence-preserving ledger reconciliation, complete solver-output
collection and admitted matched m3 measurements. The original objective stays
open: retain RR's lower solving degree while reducing matrices enough to lower
complete cost against the strongest equal-output baseline.

== Sources and artifacts
EXTRACTION-20261112.md defines the contract and interface. Raw native output:
checks/extraction-tests-qualified-20261112.txt and
checks/upstream-selftest-20261112.txt. Code: p1tool/src/extraction.rs.
Sources: the P1 emitter/protocol/results and round7_rr, linked in the Markdown
report; #link("https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf")[Petit–Kosters–Messeng, PKC 2016].
KB remains unavailable. No new mechanism, F4 width reduction, solving-degree
bound or performance improvement is asserted. Historical reports and scoreboards
are unchanged. #link("https://github.com/isogeny-labs/crypto/pull/3835")[PR 3835].
