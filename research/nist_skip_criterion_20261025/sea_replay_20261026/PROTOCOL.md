# Exact replay of the recorded SEA coverage feature

26 October 2026. This is a source/data replay of an existing exact feature,
with the historical labels already visible. It is not a new blinded test
of a historical selection mechanism, a statistical unusualness claim or a
point-counting performance experiment.

Source inputs are all21 frozen rows in ../inputs/historical.tsv and all21
native feature tables from ../factor_followup_20261026/runs/full21-features-003/features.
The latter are durably bound by the closed-runs artifact manifest. The
independently written native replay must match every recorded
sea_elkies_only_lmax value, including published P384 and P521. No row,
coefficient sign or failed value may be omitted.

The metric reproduces ../native/features.cpp exactly: t=p+1-n,
D=t^2-4p; start product1 and scan odd primes through1000. Include a prime l
when the Kronecker symbol (D/l) is0 or1. The result is the first l at which
the product strictly exceeds2*floor(sqrt(4p)). Thus this source feature
includes ramified primes, omits2 and ignores Atkin information. Its old
name does not make it a strict-Elkies-only measurement or actual SEA cost.
All arithmetic runs in C++/PARI. Python may only plot recorded output and
render or inspect the report.

Before the cohort replay, fixed public integer controls must pass:
- p103,t0: the first included odd primes are7 and13; result13.
- p101,t2: ramified5 is included, then13; result13.
- For the second control, excluding ramified primes changes the result to17.
These are fixed arithmetic controls, not proposed eligible NIST curves or
historical labels. A wrong expected value or mismatched recorded cohort
value stops the replay and retains its failure status.

Run the same21 inputs in two fresh native processes and require byte-equal
output tables and summaries. This A/A check establishes deterministic replay,
not a noise floor, independent-host replication or a statistical variance.
Record source, input and binary hashes, exact arguments, native statuses and
all outputs. Require a successful S3 checkpoint gate before execution.

The only admitted result is exact agreement or a specific counterexample.
No comparison to a random-curve population, new p-value, threshold search,
claim of unusualness, weakness or historical attribution is authorized by
this replay. The shared session log's exploratory Python association is a
lead, not an input result. It stays unadmitted here. The original criterion
and four-factorization requirements remain open where their evidence is
missing. The currently running GNFS014 must continue independently.

References checked: the frozen parent SEARCH_PROTOCOL.md; the exact native
feature contract above; the original Anthropic curve-origins article
https://www.anthropic.com/research/ecdsa-curve-origins (skip criterion remains
unknown); and PARI's Kronecker/ellsea documentation at
https://pari.math.u-bordeaux.fr/dochtml/html-stable/Arithmetic_functions.html
and https://pari.math.u-bordeaux.fr/dochtml/html-stable/Elliptic_curves.html.

The online PARI documentation requests timed out. The official installed
PARI2.17.3 pari.desc entries for kronecker and ellsea were read instead;
REFERENCES.json binds that primary documentation. ellsea's actual early
abort behavior is separate from this coverage proxy. No attempt here
executes a period point counter or claims to recover its original caller.
