---
title: "P-192 genus-3 maps: construction replay and verification closure"
date: "2026-11-27"
geometry: margin=0.7in
fontsize: 10pt
---

The fixed kernel-3 construction rebuilt successfully from its recorded
torsion/module inputs. All five regenerated artifacts are **byte-identical**
to the retained inputs of the separate map verifiers: the plane quartic,
destination relation pool, elliptic backmap, formal series, and global
correspondence. All 31 measured invocations returned their required exit
codes. The construction-chain interval was **1072.613 seconds** on one CPU,
within the original per-invocation limits. [pcc:unverified]

This completes the construction-to-verifier binding for the implemented
map interfaces at the scope below. The initial torsion basis, module records
and selected kernel are **given inputs**. Their historical generation and
selection costs remain unknown. The measured interval includes the original
chain's setup, builds, checks and outputs; initial static file copying and
metadata snapshots, and the final file-hash check, are outside it. This is a
diagnostic construction replay. [pcc:unverified]

## What the maps now support

Let $E$ be the retained P-192 elliptic curve, $H$ its fixed genus-3 cover,
$C$ the reconstructed plane quartic, $F=\phi\pi^*$, and $B=f_*$ for the
actual degree-six morphism $f:C\to E$.

| Component | Established scope |
|---|---|
| Curve-map boundary | All four geometric infinity points; retained affine pole; separate OpenSSL arithmetic |
| Infinity representation | Valid rational degree-12 KM class spaces; multiplicities $0,1,2,3$; actual KM class preservation |
| Actual-map composition | $B\circ F=[6]$ on the stated smooth models |
| Direct backward interface | Valid rational $22\times85$ RR/KM input; equivalent degree-three cycle; actual $f_*$ output |
| Interface controls | Twelve cases including repeated supports, special RR dimension two and an actual triple pole; standalone and class-only calls |
| Construction binding | Five complete regenerated artifacts equal the retained verifier inputs, byte for byte |

The [projective report](../projective_verification_20261126/REPORT.md) checks
the degree-four infinity orbit, its finite elliptic images, pole orders
$-2,-3$, and the principal relation replacing $H_\infty$ by $H_{56}$.
The [composition report](../composition_verification_20261126/REPORT.md)
checks the actual-coordinate differential trace globally. Its difference
endomorphism has zero differential and degree at most $6084<p$, forcing
$f_*\phi\pi^*-[6]=0$. The [callable interface report](../km_pushforward_20261126/REPORT.md)
connects the supplied class space to an equivalent divisor in actual GMP
arithmetic, then checks its full closed-point cycle, multiplicities and
elliptic pushforward separately in OpenSSL. [pcc:unverified]

That interface retains finite RR-chart and factor-splitting schedules;
exhaustion rejects. Its contract concerns valid RR/KM representations on the
fixed model. The mathematical smooth-model and divisor-pushforward hypotheses
remain explicit. The separate checker does not reimplement the entire KM
group law, and no independent signature or new PCC claim is supplied.
[pcc:unverified]

The retained local relation evidence is unchanged: 102 checked relations,
430 transported images and 108 elliptic rows; full/new/closed ranks
$108/102/6$, baseline rank 6 and added old-base gain 0. Its raw relations,
matrices and rank evidence remain in the
[local collection study](../local_collection_20261124/REPORT.md).
[pcc:unverified]

\clearpage

## Every executed construction stage

These are nested intervals inside the single 1072.613-second chain. They
must not be added to that outer interval. All 15 strict builds passed;
the 16 worker calls comprise 15 ordinary successes and the chart checker's
required canary rejection with exit 1. [pcc:unverified]

| Stage | Builds (s) | Workers, including controls (s) |
|---|---:|---:|
| Source derivation | 1.362 | 0.009 |
| Normalized Weil3 | 5.305 | 30.208 |
| Exceptional charts and checker | 7.970 | 308.109 |
| Level2 | 4.995 | 2.210 |
| Quotient theta | 7.294 | 96.590 |
| Riemann | 7.569 | 17.999 |
| Descent | 8.742 | 358.712 |
| Quartic export | 0.833 | 0.005 |
| Destination relations | 3.431 | 0.782 |
| Target Weil | 9.254 | 13.301 |
| Differential | 11.273 | 55.779 |
| Elliptic backmap | 9.673 | 2.734 |
| Formal series | 9.127 | 5.426 |
| Global correspondence | 9.243 | 83.704 |

Nested build wall time totals 96.071 seconds; worker wall time totals
975.568 seconds. The outer interval also includes the original shell setup
and file operations. Outer user/system times are 1063.017/7.299 seconds.
The six subsequent file-custody tests took a separately recorded 0.602 wall,
0.263 user and 0.312 system seconds. [pcc:unverified]

Each compiler and worker inherited one selected allowed CPU and a 1 GiB
virtual-address-space limit. Build timeouts were 300 seconds; worker timeouts
were 600 seconds. The largest observed build was 11.273 seconds and the
largest worker 358.712 seconds. No scientific stage timed out, crashed or
required a limit change. The user's 2026-11-26 coordinate-screen and
matched-rho waiver applies; scalar recovery is outside this task.
[pcc:unverified]

Prior verification costs remain separate, with their original failures and
diagnostics retained: projective/normalization wrappers 76.558 seconds,
composition wrappers 149.116 seconds, and direct-KM-interface wrappers
189.715 seconds. The earlier local collection, correspondence replay and
divisor audit retain their 119.375, 75.269 and 5.720-second intervals in
their original reports. These historical diagnostic rows do not price
torsion-input generation or a complete attack. [pcc:unverified]

## Evidence and controls

The pre-call snapshot contains 211 source/input subjects; all were rehashed
unchanged. The original guarded selector changes only the selected recorded
kernel and its identity strings. Original arithmetic formulas and original
evidence remain unchanged. Each call retains argv, working directory,
executable SHA-256, stdout/stderr, exit status, timeout and wall/user/system
cost. Real ELFs remain local and ignored; their hashes are retained separately
from the temporary execution shims. [pcc:unverified]

The six custody controls passed: the original completed replay is accepted;
coefficient tampering, missing final output, timeout masquerading as canary
rejection, a changed execution pin, and a changed frozen input are rejected.
These tests compare actual replay files and perform no field/group arithmetic.
The arithmetic evidence remains the native construction and separate map
verifiers. Complete receipts and costs are in [summary.json](summary.json),
[attempt-v1](evidence/attempt-v1/), and [checking-v1](evidence/checking-v1/).
[pcc:unverified]

\clearpage

## Reproduce

From the repository root, use a fresh output directory:

```bash
study=research/nist_cover_three_tests_20261116/synthesis_costs_20261126
bash "$study/run.sh" evidence/replay-01
python3 "$study/check_evidence.py" \
  "$study/evidence/replay-01" --require-complete
P192_REPLAY_ATTEMPT="$PWD/$study/evidence/replay-01" \
  python3 "$study/test_evidence.py"
python3 "$study/collect_summary.py" \
  "$study/evidence/replay-01" > replay-summary.json
```

Fresh replay builds are needed to verify executable-byte pins because compiled
binaries are not committed. The checker compares complete exact-decimal JSON,
with type-sensitive equality, and records complete byte equality separately.
It requires exact expected exits, so timeout status 124 cannot count as the
canary's required rejection. Each attempt refuses to overwrite existing
outputs. The archived input-generation Python source was read only; every
curve, field, divisor, matrix and elliptic computation in this replay is
native C++. Python is used for file/JSON/cost bookkeeping. [pcc:unverified]

Methods reuse the existing protocols: Tian,
[arXiv:2007.03172](https://arxiv.org/abs/2007.03172), §4.1; the original
level-2, Riemann and descent implementations and their cited bitangent
reconstruction methods; and the existing backmap/formal/global runners.
The frozen [protocol](PROTOCOL.md) records the prediction, scope, controls,
limits and instrumentation. Its exact pre-call copy is retained; a later
prose-only standing tag in the editable copy changes no acceptance rule.

The accompanying CI repair is a prospective registry-custody successor for an
older aimed-gather benchmark. It preserves every inherited scientific JSON
value and verifies all 211 non-registry raw pins. Only two catalog source-list
fields differ; their complete before/after values are recorded, and complete
target-record equality is explicitly false. Thirteen custody/fidelity tests
and the unchanged targeted evidence-directory validator pass. The full local
91-test attempt had 87 passes, three failures and one error from unrelated
sparse-checkout omissions; full-tree CI remains the publication acceptance
check. See the [custody receipt](../../../benchmarks/20261029-ic-aimed-gather/registry-binding-20261126/README.md).
[pcc:unverified]

Historical torsion/module generation and kernel/cover search costs remain
unpriced. The measured result is the completed selected construction from
given inputs, its exact binding to the separately verified map interfaces,
and the retained checking costs at that explicit scope. [pcc:unverified]
