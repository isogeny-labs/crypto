# Acceptance-predicate clarification before any affected data

29 October 2026,18:23UTC. No production or pilot seed has been drawn and
no extended-cutoff replay has run. PROTOCOL-v2.md supersedes the original
PROTOCOL.md for the proposed study. Preserve version1 and its11preparation
bindings unchanged; this amendment does not change the statistic or cutoff.

The pinned seeds.go.txt suitableOrder function (lines215--238) uses
ProbablyPrime(20), an order-width bound and the1023embedding checks. It has
no explicit n!=p branch. Its width-check comment does not establish that
condition: n=p has the same bit length as p, and p^k modulo n is then zero,
so neither of these checks rules it out. This is a source/logical audit;
no such seed or anomalous curve is exhibited or asserted to have occurred.

The proposed native sampler deliberately requires a proved prime order
and explicitly excludes n=p. Version1's phrase "the pinned reconstruction's
predicates" could imply exact equivalence and is corrected in version2.
These extra conditions are now explicitly part of the prospective baseline,
not attributed to undocumented historical software. The sampler's source
already implements them, so no binary or development-test result changes.
All primary count, tie, cutoff, lower-direction and multiplicity rules are
unchanged. No affected data preceded the correction.
