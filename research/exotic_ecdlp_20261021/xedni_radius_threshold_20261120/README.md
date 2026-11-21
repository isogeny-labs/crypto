# Exact radius thresholds for two capped rational-lift reductions

The two cells that exhausted their frozen radius caps in the
[preregistered prime panel](../xedni_preregistered_prime_panel_20261120/README.md)
reach complete reduced-point coverage at radii **65** (`p=1531`, group order
1553) and **54** (`p=2287`, group order 2251). Their immediately preceding
radii miss 16 and 24 residues, respectively. The first threshold has a
checked Lean proof: every smaller nonnegative radius misses residue 769,
while every residue has a representation at radius 65 under the observed
`R̄=11Q̄` relation.

## Frozen sweep and exact result

The [protocol](protocol.json) and [runner](run.py) were pushed in
`6f992fe19ca` before execution. They lock the two previously capped cells,
their verified reduced scalars, the source-row digest, and every integer
radius from 0 through 128. Each row enumerates the complete scalar image of
`(a,b)∈[-B,B]²`, including zero. The retained
[258 compressed radius rows](radius-rows.jsonl.gz) include all incomplete widths.

| p | q | `R̄/Q̄` | Prior B and nonidentity coverage | Last incomplete B | Missing at last incomplete B | First full B | Pairs including zero at first full B |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1531 | 1553 | 11 | 41: 984 / 1,552 | 64 | 16 | 65 | 17,161 |
| 2287 | 2251 | 1238 | 49: 2,058 / 2,250 | 53 | 24 | 54 | 11,881 |

The [result](result.json) records the protocol, script, source, compressed-file,
and decompressed-row SHA-256 digests. The deterministic gzip serialization is
an artifact-size amendment after the first run; the 258 rows and their raw
digest `23fdd8855c6d3d427f1807373d2907f37fc1ef979b6b307e91b81c2097e0c48e`
are unchanged. Complete coverage is monotone in the radius because each
box is contained in the next. Thus a full image at B and an incomplete image
at B−1 identify the first full integer radius. The [checked Sage
audit](audit-result.json) independently reduces the supplied rational `Q,R`,
verifies `R̄=λQ̄`, enumerates actual elliptic-curve point images at all four
boundary radii, and compares them with the scalar-row counts and full group.
It reports `PASS` and retains the [accepted runtime receipt](sage-runtime-info.json).

## Proof map

The [Lean module](formal/RadiusThreshold.lean), built using the pinned
[toolchain and Mathlib manifest](formal/build.sh), contains:

* `radius64_misses_769`: every `a,b∈[-64,64]` misses scalar residue 769
  modulo 1553 under `a+11b`.
* `below65_misses_769`: the same exclusion for every integer radius
  `0≤B<65` and coefficients in `[-B,B]`.
* `radius65_covers`: for every integer residue `0≤t<1553`, there exist
  `a,b∈[-65,65]` with `(a+11b)%1553=t`. The proof constructs a centered
  representative and chooses a bounded quotient or endpoint coefficient.

The actual [build output](formal/build-output.txt) shows `#print axioms`
reporting only `propext` and `Quot.sound` for all three declarations. The
elliptic-curve reduction and `R̄=11Q̄` are checked by the Sage audit and
earlier independent rational audit, not formalized as a curve-to-`ZMod`
isomorphism. The `p=2287` first-full claim is an exact finite computation
with independent group-law boundary replay; no Lean theorem for its modular
image has yet been added. `lean4checker --fresh` was unavailable on this host.

This sweep measures coefficient-image thresholds on two selected reductions.
It does not measure construction or arithmetic cost of the rational points
at the expanded radii. The next experiment is a frozen height-order prefix
at B=65 and B=54 that charges exact rational-point construction through
coverage and records coordinate sizes, failed/capped prefixes, and source
runtime.

## Replay

From `/Volumes/SSD990-2/crypto-worktrees/exotic-ecdlp-recovery-20261119`:

```sh
python3 research/exotic_ecdlp_20261021/xedni_radius_threshold_20261120/run.py > research/exotic_ecdlp_20261021/xedni_radius_threshold_20261120/result.json
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_radius_threshold_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_radius_threshold_20261120/audit_sage.py > research/exotic_ecdlp_20261021/xedni_radius_threshold_20261120/audit-result.json
sh research/exotic_ecdlp_20261021/xedni_radius_threshold_20261120/formal/build.sh
```
