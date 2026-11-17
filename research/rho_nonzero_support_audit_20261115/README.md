# Two-choice collision lower bound with both increments nonzero

Lean proves that the preceding two-choice result survives conditioning on both
increments being nonzero. Sample an ordered distinct pair uniformly from the
nonzero elements of `ZMod p`, attach independent fair binary labels to vertices,
and start at zero. Let `E_nonzero(p)` be the actual first-repeat mean. For `m≥3`,

\[
 p\ge4m^2(4m+1)\quad\Longrightarrow\quad E_{\rm nonzero}(p)\ge m^2/4.
\]

For every natural `C`, some prime `p>2` satisfies `E_nonzero(p)²>Cp`.
No nonnegative real constant bounds this mean by `K√p` for all primes `p>2`.
This refines the credited two-choice negative calibration; the sources'
qualified `nS≥8` and `nS≥20` conjectures remain open. [pcc:unverified]

The comparison is quantitative. Under the preceding uniform distinct-support
law, zero-containing tables have mass exactly `2/p`: for each nonzero difference
`δ`, the excluded first increments are zero and `−δ`, which differ. The actual
least-repeat index is between zero and `p`. A finite conditional-mean argument
therefore proves `E_all ≤ E_nonzero+2`. Combined with the credited
`E_all≥m²/2` bound, it gives `E_nonzero≥m²/2−2`, then the stated bound.
Bertrand's theorem supplies primes at the required scale. [pcc:unverified]

The support law is derived through an explicit bijection between the actual
conditioned coordinates and embeddings `Fin2 ↪ {x : ZMod p // x≠0}`.
The normalized finite sum is identified with the actual uniform support PMF
and actual product PMF on static vertex labels. No independence of distinct
increments or desired conditional mean is assumed. [pcc:unverified]

Kernel-checked controls give zero admissible supports at `p=2` and exactly two
at `p=3`. Actual witnesses establish a nonempty population for every prime
`p>2`. The empty finite sum at `p=2` is never interpreted as a normalized
probability law. The actual discarded mass and bounded-statistic argument are
proved; the zero-allowed theorem alone was insufficient. [pcc:unverified]

See the [closed certificate](lean/NonzeroCertificate.lean),
[proof map](proof-map.json), [source plan](sources.json),
[protocol](PROTOCOL.md), [source review](evidence/source-review.json) and
[one-page report](reports/report.pdf). The prior
[two-choice packet](../rho_twochoice_collision_audit_20261115/README.md)
supplies the actual maximal/path/counting/deferred-decisions lower bound.
Standard finite conditional-expectation and cardinality arguments are credited;
this is a domain refinement, without a novelty claim. [pcc:unverified]

Primary qualifications are credited from the immediately preceding fulltext
and visual review: Galbraith corrected ch14 PDF4,5,12,13 of38 and
Bernstein–Lange2013 PDF11,12 of28. No broader source reading is claimed; the
original Teske paper remains unread this stage. Four documented KB calls and
Conductor returned127 before processing/code; bounded local and official
fallback was used without an absence claim. Copyright primary files remain
local. [pcc:unverified]

All mathematics is in Lean. Python handles provenance and documents only.
Seventeen frozen custom modules are reused without copying or changes;
all51 predecessor source/artifact pins are checked before and after each
retained build and gate. The first failed simplifier proof is retained.
Default limits and standard foundations only: no holes, custom axioms, native
proof evaluation, global flags or waivers. No solver benchmark, paid compute
or agent experiment upload. The native written-header check is procedural,
without compiler, parser or independent-host refinement. [pcc:unverified]

Restore the exact retained sources/configuration/log history with:

```sh
python3 research/rho_nonzero_support_audit_20261115/restore_attempts.py /tmp/nonzero-support-attempts
```

The result concerns first-repeat expectation, without a Floyd-operation,
cycle-length-alone, scalar-recovery or practical-hash runtime conclusion.
Larger-partition collision penalties and limiting constants, signed standing,
and remaining obligations across all14 items stay open. [pcc:unverified]

Final gate: **20 theorems, zero failures and zero trusted-native exemptions**. Full827-character SHA256`29cb895f96ba2ac7281a8b977ab8fe18e9dadb2d4502b638f4003e4a2bfb51b5`; native1complete/0prefix/0unresolved/empty suffix, after completed gate. Three states/one failure/two successes retained in78files/38objects/18216byte losslessXZ; five gate files. Final one-page47827bytePDF/all4665textcharacters read/all-page visual/inbounds. Valid first draft7files47780byteXZ retained after prose wrapping improvement. All39formal+19artifact=58pins match. Zero final new-source warnings; four inherited module advisories and one frozen auditor deprecation retained. [pcc:unverified]

Exact local **PCC-20261115-f8a8f9** replay PASS at 2026-11-15T16:23:38Z; signed standing unverified. See [verification receipt](verification.json). [pcc:unverified]

Publication: existing ready [PR3743](https://github.com/isogeny-labs/crypto/pull/3743), remote proof `d937e21a29ac4442251d8f7c3b4c6f6fccb52c01` and local receipt `dc34b73effaeebc135730e2e06ed5431a3e971b8` share tree `14d8c582534c45234e460bb06fb4d0ce5d49a191`. Anonymous fetch completed; fresh PR head/body/open/ready matched. Publication metadata is outside the frozen pinset. [pcc:unverified]
