# Triangular enumeration preserves the n35 first-pair index

The frozen n35 comparison found exact equality of all 8,321,091
key-to-first-pair entries between the original ordered enumeration and a
triangular enumeration. The triangular builder visits 8,390,656 pairs instead
of 16,777,216 and took 66.7–66.9 seconds versus 138.6–138.9 seconds for the
ordered builder in two alternating-order local repeats. The component is an
implementation candidate for reducing cold pair-index preparation.

## Method and result

The exact 4,096-point subgroup base, field model, curve, source hash, and
index key/payload representation come from the [complete n35 pipeline](../n35_full_pipeline_20261120/README.md).
The [protocol](protocol.json) was committed before measurement. Both arms use
`setdefault` so the payload is the first row-major ordered pair producing a
group sum. The triangular arm evaluates only `i ≤ j`, retaining the same
packed payload `i·4096+j`.

| Repeat | Order | Ordered build | Triangular build | Ratio | Exact 8,321,091-entry equality |
| --- | --- | ---: | ---: | ---: | --- |
| 1 | ordered, triangular | 138.853 s | 66.875 s | 2.076 | pass |
| 2 | triangular, ordered | 138.604 s | 66.697 s | 2.078 | pass |

The [result](result.json) retains every timing, entry count, exact comparison
status, and 256 seeded witness replays per repeat. The measured process used
423.120 seconds total, including base construction, both repeats, equality
checks, witness replay, and bookkeeping. Peak RSS was 2,560,212,992 bytes,
below the frozen 8 GiB threshold. These are ordinary-host component times;
the target-online interval and same-point rho measurement from the existing
candidate are unchanged.

## Mathematical reason and proof map

For base size `B`, the packed row-major pair is `iB+j`. If `j<i<B`, then
`jB+i < iB+j`, since the difference is `(i−j)(B−1)>0`. Commutativity gives
`base[i]+base[j] = base[j]+base[i]`. Hence any ordered pair outside the
triangle has an earlier pair inside it that produces the same sum. The first
row-major representative of every sum lies in `i≤j`; preserving row-major
order inside the triangle preserves the original payload.

| Claim | Lean declaration or check | Status |
| --- | --- | --- |
| Swapped packed index is smaller | `TriangularPair.swapped_packed_lt` | Checked |
| Every pair has an equal-key triangular representative with no larger packed index | `TriangularPair.triangular_pair_dominates` | Checked under commutativity |
| Concrete group key and packed-payload bridge | exact dictionary comparison in both run repeats | Passed |
| Independent alternate-traversal replay | `verify.py` | Passed: 8,321,091 keys and 512 sampled payload/point checks |

The Lean source is [TriangularPair.lean](formal/TriangularPair.lean). It has no
`sorry`, `admit`, or custom axiom. `#print axioms` lists only `propext`,
`Classical.choice`, and `Quot.sound`. The pinned toolchain, complete Mathlib
manifest, exact build command, and output are in `formal/`. The checked Sage
runtime is in `sage-runtime-info.json`.
The independent verifier built the triangular index in column-major order
with explicit minimum-payload updates, reproduced the distinct-key count,
and replayed all 512 sampled payloads and group sums. Its receipt is
`verify-output.json`.

The next pipeline step is to define a separate immutable IC candidate whose
manifest selects this builder, then measure cold preparation and one-target
online behavior with independently replayed factor logs. The component
comparison alone does not establish a total pipeline gain.
