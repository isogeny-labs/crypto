# RR/F signed-output normalization — November 12, 2026

**Implemented:** extraction of canonical signed point triples from one supplied
rational RR or F solver assignment, including reduced RR assignments. The
CLI validates the exact emitted input hash and every equation before extraction;
the reduced RR path additionally validates its reconstruction map and original
equations. Nine native development tests pass. Claim `PCC-20261112-699e09`
remains unverified. [pcc:unverified]

This closes an implementation dependency from the November 10 protocol. It does
not establish solver-enumeration completeness or a F4 cost improvement. [pcc:unverified]

## Declared common chart

- m = 3, 2 <= t <= 10, prime 3 < p < 2^32; the curve is nonsingular.
- R is a finite on-curve point with nonzero y_R.
- Three distinct rational tower roots give distinct support abscissae; the
  support excludes x_R and points with y = 0.
- The RR function has monic quadratic A and a nonzero constant C. Its norm
  identity is checked coefficient by coefficient after interpolation.

Roots are found by division over the full declared tower domain, reconstructed
from public coset metadata (or the full Kummer subgroup and scale). Stored
planted points are never used by extraction. F's cubic support polynomial is
W^3-e1 W^2+e2 W-e3; RR uses its U coefficients. The extractor checks full
splitting, then scales abscissae for Kummer. RR fixes each sign via -A/C;
F enumerates all eight sign choices and retains exact sums to R on the common
chart. Outputs are sorted and deduplicated point triples.

Unsupported chart cases return an error and must remain visible to the harness.
They must not be relabeled UNSAT or silently discarded from a complete-domain
comparison. Missing solver assignments remain unknown, not empty output sets.

## Interface

```sh
p1tool extract --metadata C.ms.plant.json --system-file C.ms --values values.json
```

`values.json` is a JSON array in the variable order of C.ms. A supplied metadata
file and source-built input must be independently pinned by the execution
manifest: producer hashes and metadata are not signed evidence. The output
records the exact chart and explicitly disclaims solver-enumeration completeness.
The same path handles original RR, reduced RR and F inputs. T is not supported
by this extraction command yet.

## Development evidence and limits

[Qualified test log](checks/extraction-tests-qualified-20261112.txt) records
the native release test command and outcome. The bounded Dickson fixture uses
p = 1151, a = 294, b = 772 and t = 4 from existing P1 smoke controls. A separately
written finite reference enumerates all distinct on-curve support triples and
all signs, checks sums using a separate affine-addition implementation, and
groups them by finite target. Field primitives are shared. Positive supports
are encoded through the existing RR/F emitters and their normalized outputs
are compared with the reference, including targets with multiple decompositions.
A target with no reference decomposition is separately checked against every
distinct F support. This is a bounded development fixture, not an independent
signed audit or exhaustive enumeration of the ambient solver variety.

Other fixtures check negative-target orientation, reduced-to-original RR lifts,
repeated-root/C=0 rejection, modified tower metadata, modified input hashes,
wrong assignments, inconsistent modulus metadata and reduced-map digests.
A scaled Kummer control checks coordinate conversion. The original emitter's
22-group selftest also passes in the dated log. [pcc:unverified]

## Remaining experiment requirements

The benchmark harness must obtain every rational solver assignment, normalize
each one, union the outputs, retain excluded-chart/failed cases explicitly and
check those complete sets against an independent reference before timing is
admitted. The broader domain including repeated abscissae and chart exceptions
is still unqualified. No m5 extraction or solver claim is made.

The S3 checkpoint transport, live shared budget/pool readback and source-built
experiment image remain missing. No scientific F4 run, speed comparison or
new matrix-width result occurred. Main has also advanced into a frozen-ledger
merge conflict; the attempted merge was aborted with both histories intact.
PR #3835 stays open for evidence-preserving reconciliation.

Sources: the existing P1 [emitter description](../split_prime_ecdlp_20261107/followups/p1_rr_dickson/p1tool/README.md),
[P1 results](../split_prime_ecdlp_20261107/followups/p1_rr_dickson/RESULTS.md),
[round7_rr](../ic_tree_gluing_20261103/round7_rr/README.md),
and the [PKM primary paper](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf).
KB remains unavailable (no loaded MCP, repository wrapper has no executable).
This implements the previously specified output contract; no novelty is claimed.

[Diagram](extraction-flow.svg), [PDF](REPORT-20261112.pdf),
[editable PDF source](REPORT-20261112.typ).
