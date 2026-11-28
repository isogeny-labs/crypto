# Direct constraints on shared-RNG explanations

Question fixed before the check: can pure candidate visitation reproduce the
recorded published/skipped indices with either (A) fixed extra draws per
candidate, or (B) extra draws after retaining a curve that grow monotonically
with its published base-point search attempts? This is exact analysis of
already recorded integer observations, not a curve-generating experiment.

## Observations and sources

historical.tsv is an exact copy of the existing21row input at crypto commit
81f0308c75204f3a1c0ec52fdf78396eee0898ed. Only bits,role,run,index are used;
the checker never computes with p,b,n or seed values. The six existing
point-attempt counts in point-attempts.tsv come from the primary reconstruction:
https://www.anthropic.com/research/ecdsa-curve-origins (selected-base-point code
and its printed output). See also https://words.filippo.io/seeds-solved/ for
the candidate enumeration. [pcc:unverified] Existing SEARCH_PROTOCOL.md, REPORT.md,
results/rng-models.json and native/rng_models.cpp were read before choosing
this approach. That earlier finite search tested52520models without an exact
fit; this check does not rerun or extend that parameter scan.

The source-derived task concerns unknown caller behavior. No link between
the two distinct RNGs is asserted. ModelB deliberately allows a hypothetical
coupling of the base-point cost to the lrand48 seed stream. Its assumptions
are stated so a failed model is not mistaken for an exhaustive explanation.

## Model assumptions

Each candidate consumes20lrand48calls. A known index i therefore begins at
offset20(i−1). All eligible candidates actually visited are retained; there
are no hidden discarded acceptances, additional filters, jumps backwards,
state resets or restarts. Initial warmup may be arbitrary and field-specific.

A. Every candidate is followed by d>=0 extra calls, constant within a field.
Visited starts have spacing s=20+d. Differences between published starts
must all be multiples of s, whatever the initial warmup. If their gcd is20,
the only possible s>=20 is20, so d=0; eligible positions between publications
cannot then be missed. This eliminates all d/warmup values, not a finite scan.

B. Extra calls occur only after retaining a curve, with none after rejected
candidates. To reach another published index in the same run, the intervening
extra count D must be a multiple of20. Write q=D/20. After retained index a,
to miss skipped index s before the next retained index b requires q>=s−a;
to reach b requires q<=b−a−1. The checker retains the tightest bounds from
all recorded skipped positions in every publication gap.

For the base-point-cost version, D=f(r), where r is the existing reconstructed
attempt count and f is any nonnegative nondecreasing function within a field.
Fixed costs plus a fixed number of calls per point attempt are a special case.
An affine f(r)=C+L*r, C,L>=0, is also checked via an explicit P192 inequality.
No f is fitted; the checker emits incompatible bounds as witnesses.

## Checks, limits and controls

Strict native C++17,10second arithmetic watchdog, all21input rows and all six
point-attempt rows required. Emit all publication gaps, gcds, and contradiction
witnesses. A planted stride60case and a feasible monotone-cost interval case
must be accepted as controls; impossible inequalities must be rejected.
These are producer self-checks, not signed independent verification. No curve,
primality test, new point count, random sampling, historical runtime replay,
factorization, DLP, paid job, checkpoint or artifact-store upload is involved.
Conductor/KB are unavailable; current documented fallback and primary sources
apply. No credential, keyring, old evidence or old proof subject is changed.

Models with calls after rejected candidates, state-dependent costs beyond r,
hidden accepted outputs, reseeding or caller point-count errors remain outside
these constraints. Their examination needs a separately specified model; this
note does not establish an explanation of the historical skips.
