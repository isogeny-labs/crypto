# Twist-derived progression bases: frozen bounded protocol v1

Question: can a twist orbit yield source-rational factor bases or redundant
companion equations that reduce actual finite-decomposition work? This is a
new bounded experiment, not completion of E26/E27/E28/E30 or a NIST attack.
Original request: pursue the coordinate/factor-base possibility identified
after the generator/progression discussion. Construction, validity, relation
yield and solving costs are separate obligations. No scalar-preserving map
from the source rational group to the rational twist is assumed.

## Fixed population and constructions

Reuse all ten odd-prime-order original toy fixtures from the committed
execution-v3/decomposition-20261026/PLAN.json, retaining their registered
source/twist identities and generator. No fixture is selected by outcome.
Coordinates are modulo p. Four target-independent constructors:

* twist-orbit: the registered standard twist has coefficients d^2*a,d^3*b.
  Choose its first x with nonzero square ordinate, use the smaller ordinate,
  and enumerate H,2H,... . Convert each X to u=X/d.
* curve-orbit: enumerate the fixture's G,2G,... on the source; u=x(iG).
* field-ap: enumerate u=0,1,...,p-1.
* uniform: SplitMix64 sampling, seed 202610260001 xor p xor (b<<16) xor
  shift; at most 8p draws, with repetitions retained as construction work.
  Here u is the sampled field coordinate.

For each constructor, propose x=u+shift and retain distinct coordinates
whose source ordinate is nonzero and rational. Stop at B accepted values,
the end of one group cycle, p field-AP terms, or 8p uniform draws. Sort the
result. No refill from another constructor, shift search or best-seed choice.
Keep underfilled bases explicitly unsupported. All work, including failed
lifts, repeated coordinates, seed discovery and coordinate conversion, is
charged in each cold worker. Canonical source ordinates in reported witnesses
use the existing native square-root convention.

Geometry panel: four constructors, shifts {0,1,3}, B={8,16}, all ten fixtures:
240 construction records. Census arities 2 and 3 for existing public target
labels 0 through 15, retaining zero yields and duplicate target coordinates.
No final E28 labels 32--63 are opened. The unshifted twist is a negative
control; the unshifted curve orbit exposes known-log progression bias.
Report per-label results and aggregate descriptions only; these are separate
one-query workloads, not a multi-target DLP.

## Fixed cold-cost panel

B=8, shifts {1,3}, all four constructors, arities {2,3}, target label 0:
160 configurations. Five arms each: direct F4 A/A, direct product F4,
eight-branch F4 helper, ordinary group MITM A/A, ordinary group MITM.
Add one companion-curve arm for each curve-orbit/twist-orbit configuration:
80 additional cells, 880 total. Every cell is attempted unless the global
cap is reached. Unsupported construction is recorded rather than refilled.

Companion adds v_i with delta*v_i^2=f(x_i-shift), delta=d for twist-orbit
and 1 for curve-orbit. Every proposed base point is independently checked
to satisfy this condition. The original source membership polynomials and
Semaev chain stay present, so these are redundant constraints on the finite
base, with all auxiliary roots and deduplication charged. This is a finite
encoding experiment; no new universal identity or novelty claim is made.

Use the current native worker with Grevlex, GM=1, D=32 and opt-in linear
roots enabled. One worker at a time, one thread, 2,000 ms cooperative budget,
4-second hard timeout plus 1-second kill grace, 4 GiB virtual memory and
32 MiB/file. The cooperative budget starts after base construction; the
hard limit and Ir interval include construction. Cap the panel at 2,400
seconds and 1 GiB raw output. Preserve
timeouts, solver-incomplete results, unsupported bases and unstarted cells.
Run on the existing local L0 host. No paid host, reservation or funding.

Primary unit is Callgrind simulated guest instructions Ir. No native wall
time, cycles, calibrated field operations or GAE is admitted. The charged
interval includes process startup, parsing, base discovery/conversion/lifting,
equation construction, F4, all finite roots, sign recovery, exceptional
identity-prefix charts, deduplication, source verification and serialization.
It excludes the parent, kernel, independent replay, relation-log/linear
algebra and full scalar recovery. These are not complete IC or rho costs.

## Acceptance and comparison

An independently implemented Rust auditor uses a square table, Euclidean
inversion and affine arithmetic to reproduce every base and exhaust every
signed canonical tuple. It must check exact ordered tuple/minimum-sign rows
for each completed cost worker, reopen input/output/raw-Ir hashes and reject
missing or malformed receipts. PARI independently replays retained positive
signed sums and deliberately wrong-sign controls. Admission requires clean
PARI exit/marker and only documented harmless diagnostics. Direct and group
A/A spreads must be <=5% on each configuration before paired arm ratios are
interpreted. All raw ratios and failed A/A gates remain visible.
Tuple convention sorts the first pair only, matching the earlier worker;
tuple counts are neither relation-matrix rank nor independent-log counts.

Within one base, direct/helper/companion/group return exactly the same
relations and may be compared directly. Across constructors, output sets
differ: cost, yield and coverage are descriptive, not same-output speedups.
Report complete and missing pairs, zero-yield cases, per-fixture/label counts
and ranges. Do not divide by zero or select only successful bases/targets.
The strongest supplied ordinary finite-decomposition control is group MITM;
no fastest-external full-root or matched-rho claim is made.

Freeze protocol, source and plan in Git before scientific runs. Pin binary
hashes and actual commands/exits. Commit deterministic raw archive/member
manifest, native audit, replay receipts, source-linked reports, SVG/150dpi PNG
and inspected PDFs; retain failures. New artifacts use only the user-confirmed
setec-astronomy-artifacts destination if remote publication is necessary.
No validation-gated benchmark row or canonical pipeline gain is promoted.

## Decision and remaining obligations

An exact same-base cost reduction is a stage result requiring broader replay
and full IC costs. Different-base yield changes require a frozen target-blind
selection rule and complete cold/amortized relation collection before they
can establish a pipeline gain. Null/negative results close only this fixed
family and budget. Generator-coupled maps, isogeny quotients, other shifts,
other transforms, arity four, larger fields, full IC/rho and independent-host
replay remain separate unresolved routes. Do not claim all mathematics is
exhausted or that unscheduled work is continuing.
