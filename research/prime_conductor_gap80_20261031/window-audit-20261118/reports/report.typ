#set page(paper: "a4", margin: 17mm)
#set text(font: "DejaVu Sans", size: 10pt)
= Explicit windows for the unchanged 80-bit target
18 November 2026 · PR 4703 · source-derived counts and development checks.

The evaluator now accepts an explicit baby-window size without changing its
default. For the same frozen degree 729845429276318409687041, the window
B = 414464344064, G = 440234147840 gives zero direct leftovers in the exact
integer schedule model. The default has 722432689208 leftovers. These are
derived counts, not an evaluated target isogeny. Unsigned statement
PCC-20261118-7f57d5 has no independent mathematical standing. [pcc:unverified]

#table(
  columns: (1.7fr, 1.15fr, 1.15fr), inset: 5pt,
  [Target schedule], [Default], [Explicit window],
  [Baby count], [427154957034], [414464344064],
  [Giant count], [427154957034], [440234147840],
  [Direct leftovers], [722432689208], [0],
  [Rectangles at block 6000000], [5068443249], [5068460094],
  [Full baby-array slot bound (bytes)], [6834479312552], [6631429505032],
)

#image("schedules.svg", width: 100%)

The explicit choice slightly increases rectangles at this block size. Tail
removal does not establish a runtime advantage. No throughput conversion,
elapsed-time projection, actual memory peak or total field-operation count is
reported. Stage counts exclude arithmetic inside point/norm operations.

== Scope of the allocation model
The current baby product keeps the count-root fmpz vector alive while the monic
degree-count result has count+1 coefficient slots. The model counts those two
arrays only. Limbs, all other polynomials/jets/norm data, serialization and
allocator overhead are excluded. Live logical storage is distinct from measured
RSS; no swapping or external-memory implementation is modeled.

The example budget is the 128-GiB policy value, 137438953472 bytes, not a live
capacity readback. The older 3417239656272-byte root-slot bound counted only one
array. The new calculation includes the second; it is not an observed memory
regression. Both full-window variants exceed the example allocation budget
before the excluded objects are counted. [pcc:unverified]

#pagebreak()
== Why a small block is not full-schedule feasibility
#table(
  columns: (1fr, 1.2fr, 1.8fr), inset: 5pt,
  [Default-window block], [Rectangles], [Giant contexts regenerated],
  [4096], [10875544558802641], [44546230163497922886],
  [6000000], [5068443249], [30410442856121562],
  [8589934591], [2500], [21357747851700],
  [Full window], [1], [427154957034],
)

The 8589934591 block is only the most generous one allowed by this weak
baby-array slot condition for the fixed default window. It leaves no room for
the excluded data and is not a usable recommendation. The 2500 count does not
bound all window choices or algorithms. A complete external-memory/work/I/O
schedule remains open. The full-window case fails this necessary condition.

== Development checks
Four planner schedules match the stage counters of retained small controls.
The modified default fixture bytes still match the archive, including its 68
known-answer endpoint recoveries. Explicit B=18 at degree 10009 and B=1 at
degree 19 compare 42 point inputs against PARI and the existing default outputs.
The first explicit control has no leftovers. Zero, out-of-range and duplicate
options are rejected; slot-boundary and invalid-degree metadata controls are
retained. These are small development observations. [pcc:unverified]

A first manual candidate, B=412316860416, was rejected as a zero-tail choice:
the exact model gives 549755813888 leftovers. The valid metadata is retained;
it was not a failed construction. Exact native arithmetic gives zero leftovers
for B=274877906944 and the selected near-balanced B=414464344064.

== Implementation and replay
The planner uses GMP exact integers and the actual FLINT fmpz slot size; it
allocates no root vector, performs no curve arithmetic and checks no primality.
It never admits a launch. Arguments: degree, block, memory bytes and optional
baby-window size. The evaluator accepts the existing supplied-case arguments
plus #raw("--baby-window B"). The complete-window guard remains separate
from block size and byte-memory accounting.

See ../README.md for build commands and the result manifest. The post-call
manifest pins the actual source and binaries, not a pre-call checkpoint.
The plotting source lays out only recorded metadata. Existing timing/coverage
figures remain frozen and no benchmark row is added.

Sources: pinned complete evaluator, existing constructor derivation (arbitrary
positive baby window and exact remainder coverage), archived target parameters,
and the installed FLINT 3.1.3 public root-product declaration
(#link("https://flintlib.org/doc/fmpz_mod_poly/index.html")[official reference]).
This is established index-set tuning; no new general identity or isogeny formula
is asserted. KB unavailable: wrapper exit 127; fallback used scoped sources.

== Original goal
The full 80-bit quotient, point images, endpoint/subgroup certificates and
matched scalar/ECDLP timings remain open. So do full-window resource/work
feasibility and cloud publication/budget/capacity admission. IC comparison is
unimplemented. No conductor-gap record or target timing is promoted.
