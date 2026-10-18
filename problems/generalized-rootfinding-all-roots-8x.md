# Generalized rootfinding: complete all-roots8x qualification

- **Status:** open
- **Kind:** implementation and empirical qualification gaps
- **Affected work:** [native bounded solver](../tools/generalized-rootfinder/README.md), [Boolean comparison](../research/f4_f5_boolean_perf_20261018/REPORT.md), [theory cycle](../research/quintic_cached_frobenius_20261017_v1/README.md), [next experiment](../backlog/XP-20261017-quintic-support-frobenius/README.md)
- **Next action owner:** generalized-rootfinding research program
- **Last updated:**2026-10-18

## Problem
A novel exact generalized solver with qualified8x end-to-end gain over the
fastest matched genuine all-roots F4/F5/F6 has not been established over the
requested prime and extension fields, including GF83 and five NIST primes.

## Evidence and uncertainty
Observed primary source: rootfind-mitm stops at a first group-law hit.
Generic wide-prime F4 arithmetic now exists, but full matching original-root
extraction and field/domain adapters remain obligations. This differs from
the existing rational IC pipeline complete-cost-calibration problem.

Derived: the cached characteristic-two operator preserves quotient squares
even for repeated H. A GF4 coefficient counterexample rejects a K-linear
implementation. The specified dense operation model predicts at most34/21
from that local change, not a measured gain or universal runtime bound.

Missing: native integration and independent replay, fastest-external
comparisons, complete hard all-roots matched panel, novelty review and formal
bridge. The managed executor being offline additionally prevents execution
and recovery of the unfinished native checkpoint upload in this cycle.
No bounded failure proves the research program impossible.

## What would unblock it
- Complete canonical original-root output, including exceptional and auxiliary
  fibers, on exactly matching equations, domain and field representation.
- Original immutable GATE:20preregistered hard completed matching cases,
  fastest baseline>=1second, three cold repetitions, A/A<=10percent,
  qualified one-thread isolation and worst slowdown<=1.10.
- Requested8x gain against the fastest matched genuine F4/F5/F6, strongest
  available external configurations, and requested field-family coverage.
- Independent correctness replay and defensible primary-source novelty review.

A component predicate or a first-hit decomposition does not satisfy these
conditions. Restoring compute enables experiments, not scientific promotion.

## Next discriminating step
Use the linked three-shard blocked card after implementation and hash freeze.
It prices complete support setup/recurrence/extraction instead of the predicate
alone. Mismatch refutes implementation; regression rejects universal dispatch;
a null result redirects work toward upstream reuse and hard-family search.

## Resolution and history

2026-10-18: Completed a native bounded generalized solver with certified fields,
exact polynomial, affine-linearized and rational-composition domains, complete
system enumeration/component MITM/Boolean dispatch, and the complete original
eight-coordinate S3 separator. Ten independent native tests pass, including
literal all-coordinate separator enumeration, poles, repeated and inseparable
roots, a wide prime certificate and GF(2^83) extraction. The linked study
compares matching 16/22-variable Boolean root sets and same-target F6/rootfind
decomposition calls. This closes the bounded native execution gap for these
contracts. It does not close hard implicit-domain search, the original
20-case external-comparator/runtime gate, independent-host replay, novelty
or the requested 8x qualification. Those conditions remain unchanged.
Unresolved. This record and its PR keep the original requirement visible;
they do not stop the continuing program or close the goal.
