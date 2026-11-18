# All-window bounds for the pinned evaluator

For the unchanged 80-bit target, the exact minimum of the two-phase necessary
coefficient-slot model over every single-rectangle baby window is
**9,665,413,335,568 bytes**. It excludes GMP payloads, other jets, prepared
cache, scratch and allocator overhead. For the example 128 GiB cap, every
fitting tiling of the pinned source requires at least **21,241,297,635,754
coordinate contexts** and **2,473 rectangles**. These are exact conditional
model bounds, not RSS observations, timings or limits on all isogeny methods.
PCC-20261118-61e7ed binds the derivation and native evidence, unsigned and
unverified/not_covered. [pcc:unverified]

The optimizer matches exhaustive enumeration on 1,048,576 window configurations
covering n=2..2048. The context/rectangle inequalities pass 6,291,456 blocked
configurations. Nine native layout checks verify simultaneous monic h and
zeroth-giant-jet lengths on the existing degree-5/19/10009 fixtures. Seven
prior pipeline metadata records match the modeled B/G/coverage/block counts.
All qualifiers exit 0 with empty stderr. Five CLI rejections pass.
[pcc:unverified]

## What the added phase changes

For a baby chunk b and giant chunk g, the necessary logical live slots are:

| Phase | Coexisting data | Necessary slots |
|---|---|---:|
| Baby product | RootVector and monic h | 2b+1 |
| Giant norm | h and zeroth giant jet | b+2g+2 |

The zeroth giant jet is the product of g monic quadratics (Z-x_j)² and thus
has exactly 2g+1 coefficients. h is retained through the giant loop.
The older baby-only model remains a valid necessary bound; it was never
sufficient for admission. This audit adds another mandatory phase and an
all-window optimization, with a separate scope/metric identity.

A retained counterexample is the degree-10009 full schedule: baby-phase 101
slots fit a 101-slot test cap, while the simultaneous h/giant phase needs
152 slots and fails it. Conversely, a small bound fitting five slots still
sets launch_admitted=false. [pcc:unverified]

## Derivation and exact integer optimizer

Let n=(ell-1)/2, G=floor(n/(2B)), L=n-2BG, and i/j be the numbers of baby/giant
blocks. Then 0<=L<2B. With C whole slots in the cap and M=floor((C-1)/2),
necessary fit bounds each actual chunk by M. The current traversal processes
W=B+iG+L contexts. From iM>=B follows

    2MW >= n+2M,       W >= 1+ceil(n/(2M)).

Likewise n<2B(G+1)<=2ijM(M+1), so

    rectangles >= floor(n/(2M(M+1)))+1.

For one rectangle, minimize max(2B+1,B+2G+2). At trial cap C, existence is
equivalent to n<2B floor((C-B)/2) on the permitted B interval. This product
increases separately on each parity; its two rightmost B values suffice.
Binary search in C gives an exact model optimum, with a rechecked witness.
The complete argument, source assumptions and exclusions are in
[DERIVATION.md](DERIVATION.md); no formal Lean theorem is asserted.

The minimum-model witness has B=604088333472, G=302044166736 and
L=1180929268736. It is not a runtime recommendation. Its direct leftovers
differ from both the balanced schedule and the original zero-leftover choice.
The integer optimization relaxes native index-width guards; this target
witness fits the pinned 64-bit native indices. Bounds on the relaxed set
remain necessary for the actual subset. Neither this witness nor a lower
bound that fits a cap establishes physical memory fit. [pcc:unverified]

## Existing schedules and scope

At block 6000000 both retained windows have a necessary phase peak of
144000016 bytes. This excludes the other live objects and does not establish
fit. The balanced schedule still processes 30411592443767804 contexts and
the zero-leftover schedule 30410908928835584 contexts in these loops.
No context count is converted into group/field operations, seconds or cost.
[pcc:unverified]

The all-window bounds use the example 137438953472-byte cap, not a current
quota/capacity readback. They apply to the pinned repeated-giant traversal
with its in-memory h/zeroth-jet objects. External-memory/global polynomial
processing, cached traversals and different isogeny algorithms need their
own models. This result does not rule them out or prove wall-time infeasibility.
[pcc:unverified]

## Evidence and reproduction

- [Summary](summary.json), [frozen scope](README.md), [native integer model](audit.cpp)
  and [actual-layout checker](check_layout.cpp).
- [Native exhaustive/layout output](proof/qualification.stdout),
  [prior-record equality](proof/recorded.stdout), source/input/binary manifests
  and passing post-call checks.
- [Illustrated PDF](reports/report.pdf), editable Typst and recorded-value plots.

Compile audit.cpp with C++17/GMP and installed FLINT headers; compile the layout
checker as the prior native evaluator with FLINT/PARI/GMP and its unchanged
helper object. The frozen shell qualifiers take the repository, native binary
paths and fresh output paths. Python only joins serialized metadata; it does
not calculate a bound, window or curve point. No production source is changed.

The first audit build failed a misleading-indentation warning; the first
layout build called get_ui on a GMP expression. Corrected builds pass with
warnings denied. Their sources/errors remain retained, and no arithmetic
attempt failed. Old build manifests resolve the preliminary source through
source-build-a/source-build-b; current pre-call manifests name corrected code.
[pcc:unverified]

Raw outputs are archived deterministically. Named PCC/manifest subjects stay
loose under the path-resolution exception. Seven older prepared-evaluator
records newly named as inputs also stay loose, with exact bytes retained from
their unchanged original archive. The original 80-bit source/kernel, prior
receipts and source snapshots remain unchanged.

## Original requirement status and next routes

| Requirement or route | Status |
|---|---|
| At least 80-bit evaluated prime endomorphism-conductor crossing | Open; no lower endpoint or full quotient/images |
| Single-rectangle pinned evaluator under example 128 GiB | Fails the necessary model for every baby window |
| Bounded tiling of pinned loops | Necessary context/rectangle bounds above; actual time, memory and budget still unmeasured |
| Prepared norm/cache changes | Correctness controls retained; window/loop obligations unchanged |
| External-memory or different traversal | Not implemented or excluded by this derivation |
| Endpoint scalar/ECDLP/IC measurements | Not executed; source transport, reference/IC/cloud admission still open |
| GitHub/S3 | Existing draft updated; prior archive/store blocker retained |

The next construction route must change the constrained data/traversal plan
or establish a fully priced admitted tiled execution; window tuning alone
cannot make the single-rectangle route fit this example cap. This is not a
universal impossibility statement. All construction, transport, verification
and endpoint costs remain part of the original experiment. No job is active.

The source-specific bounds have neutral labelled progress metrics, separate
from conductor-bit and measured-crossing series. Prior immutable graphs are
unchanged because their narrower source/phase scope remains valid. No
benchmark/leaderboard frontier moves. Primary source locations and unavailable
KB/Lean status are in the scope/derivation records.


## Committed exporter and publication check

The scoped committed-state exporter emits all three neutral rows from
41a9134f54e4 with their exact native values and explicit scope labels. The
full command exits 1 on unchanged docs/ic/progress-timeline.json (line 565,
column 6); its partial stdout is not a completed export. The timeline blob
matches the parent commit. Development/committed metric checks do not claim
a passing full exporter. [pcc:unverified]

The committed-head store check flags only the earlier 318420-byte wide-harness
archive. This audit's 2984-byte deterministic archive, required SVG/PDF and
named evidence satisfy their configured thresholds. No signed store retry,
new upload or live experiment has been attempted. PR remains draft.
[pcc:unverified]
