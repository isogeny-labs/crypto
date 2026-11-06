# F17 exceptional dispatch: source-bound finite witness

This packet checks one valid reduced divisor on the smooth curve
`y²=x⁵+x+2` over `F17`. Its support is `u=x²+6x+5=(x−12)(x−16)` and its
ordinate is `v=0`; hence it is 2-torsion. The [standardized row](ROWS.csv)
records the source-bound public-dispatch result, the separately evaluated
direct doubling subroutine, and the compiled Rust/Cantor outputs. The
[graph](GRAPH.svg) and [PDF report](REPORT.pdf) present the same one-input
comparison for review.

The pinned [author Magma source](https://github.com/ozbayelif/jac-on-jac/blob/ec98e115ed359c8b7facf559eb0d9f240703f6f5/jac-on-jac.m)
(SHA-256 `7416a7fffaafff371738e7498a6795057372e1c477e0079297eebf2347c2e72a`)
checks the inverse predicate before doubling in `ADDDIVISOR` (source lines
582–589). It returns its identity sentinel for this input. The pinned Rust
`add22_weighted` does the same (lines 918–925), returning `Identity` with
label `same_u_inverse`. The compiled probe also checks the independent Cantor
sum is identity.

Evaluating the author `DBL2B0` subroutine directly (lines 400–425) on the
same coordinates produces `(Q,R,S,T,Z,W)=(0,0,0,0,0,0)`. Both weights are
zero, so this is **not a valid weighted divisor representation**; a subsequent
normalization would reject its two required nonzero denominators. The public
author dispatcher never reaches this subroutine for the present input. The
Rust direct `double22_weighted` subroutine separately returns `Identity` with
label `b_zero_identity`. This is a direct-subroutine difference, not a
reachable public-law discrepancy for this fixture. Public undefined inverses:
zero; direct author zero weights: two. No Magma execution was performed: the
author output is an exact evaluation of the pinned source expressions.

The [first probe log](initial-failed-assertion.log) retains an assertion that
incorrectly expected the public Rust label to be `b_zero_identity`; it failed
with actual label `same_u_inverse`. The corrected [compiled log](corrected-cargo.log)
passed, and [run.py](run.py) reproduces both probes. [check.py](check.py)
pins both complete source hashes, checks the inverse-before-double ordering,
recomputes smoothness and support validity over `F17`, evaluates direct
`DBL2B0`, and checks the CSV byte for byte.

From the repository root, retrieve the pinned author source and replay:

```sh
curl --fail --location --silent --show-error \
  https://raw.githubusercontent.com/ozbayelif/jac-on-jac/ec98e115ed359c8b7facf559eb0d9f240703f6f5/jac-on-jac.m \
  --output /tmp/jac-on-jac-ec98.m
sha256sum /tmp/jac-on-jac-ec98.m
python3 research/projective_law_followup_20261022/exceptional_source_bridge/run.py \
  --author-source /tmp/jac-on-jac-ec98.m
bash research/projective_law_followup_20261022/exceptional_source_bridge/render.sh
```

The scope is exactly this one `F17` input and these two pinned sources. It
does not establish formula parity for every exceptional branch, prove the
Picard-group law, or alter the frozen 1,260-target cost ratios.
