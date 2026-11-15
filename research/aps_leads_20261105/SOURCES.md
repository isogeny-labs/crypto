# Source review — 2026-11-05

## Primary sources

- **APS, revised author manuscript**, full §3 checked against the earlier
  repository review: known labeled combinations, a single signed zero-sum,
  partition constraints and base replacement after an unsuitable system.
  [Revised manuscript](https://iris.unitn.it/retrieve/e3835194-60e8-72ef-e053-3705fe0ad821/Amadori%2C%20Pintore%2C%20Sala%20-%20On%20DLP%20for%20prime-field%20EC%20%28rev%29.pdf).
  This is the starting algorithm for L1/L3/L4.
- **Demin 2026**, full tracing and amortization sections read:
  [Groebner.jl](https://arxiv.org/html/2607.06372v1), §§2–4.
  Learn/apply and batching are established methods. Learning is an upfront
  cost and specialization can fail. L2 proposes their APS integration;
  no published benchmark number is transferred to APS.
- **Milne, Elliptic Curves**, §I.3 pp.25–28:
  [book](https://www.jmilne.org/math/Books/EC2.pdf).
  Standard chord geometry and the two-torsion exception support the
  conditional L1 derivation; the new chart is our proposal.
- **Petit–Kosters–Messeng 2016**, full manuscript checked, especially
  factor-base maps and the concluding root-finding discussion:
  [author PDF](https://christophe.petit.web.ulb.be/files/16PKC_primeECDLP.pdf).
  Cosets and composed low-degree/rational/isogeny maps are prior work.
- **Kudo–Yokota–Takahashi–Yasuda 2018**:
  [official university record](https://kyushu-u.elsevierpure.com/en/publications/acceleration-of-index-calculus-for-solving-ecdlp-over-prime-field/),
  [author bibliography](https://sites.google.com/view/m-kudo-official-website/english/papers).
  Abstract/bibliography checked; full body not obtained. Hybrid and symmetry
  techniques are already attributed there. No novelty or quantitative
  reproduction claim is made against the inaccessible implementation.
- **Müller–McGuire 2018**, full primary slide deck read:
  [Caen slides](https://nitaj.users.lmno.cnrs.fr/Caen2018/Caen2018Slides/Caen2018Mueller.pdf).
  Direct summation checks and group-based joins are existing controls for
  L3, not a new algebraic mechanism.

## Repository review and pins

Main observed for this review:
`2d33e9c068c0f2b3226893017c8ea48548cc0749`.
These Git blob pins match the local source used [pcc:unverified]:

| Path | Git blob | Relevance |
| --- | --- | --- |
| `src/cryptanalysis/aps_single_relation.rs` | `738a300ea27e8908cfa4301548ff418e4cb06200` | Six encodings; certified pruned line uses inverse variable; guaranteed labels and unequal-size sampler already present |
| `src/cryptanalysis/f4_fp.rs` | `e21e8e5e7677035909bd6baac83adadf60bcc67c` | Existing record/replay and mismatch reporting |
| `src/cryptanalysis/ecbench/aps/mod.rs` | `0583c238bffd579f9762533ee82f6c7b0f47ca4d` | Public configuration currently uses a uniform subset size |
| `src/cryptanalysis/identity_certificate.rs` | `114f1f4a3f2a1be027a9b046ef166259df49dd45` | Unchanged issuer/checker reused by the study wrapper |
| `src/hash/sha256.rs` | `9566e3de448e21ff98811cfc4c6992ff3448962e` | Unchanged digest implementation used by the native checker |

Read [earlier source map](../aps_extensions_20261026/SOURCES.md),
[four-lead follow-up](../aps_followup_20261028/README.md),
[measurement draft](../aps_measurement_20261029/PROTOCOL_DRAFT.md),
[complete-cost problem](../../problems/aps-single-relation-complete-cost.md),
and `src/cryptanalysis/polynomial_reuse.rs`.
The existing lead-atlas snapshot `fd387705a71cd2f4f3a896af827d3676e0a07659`
was checked for algebraic membership, representation subset sums,
coincidence harvesting, retained setup, digit presentations and partial
relations. Existing scoped negative results remain in force.

The focused search found no implementation of the proposed nonhorizontal
chart or new-tuple growth frontier in the APS paths reviewed. This is a
bounded source observation, not an absence theorem [pcc:unverified].
Unbalanced sizes, hybrid guessing and tracing each have prior components;
the proposed deltas are the joint policy and APS integration.

Canonical graph review: `docs/index-calculus-scoreboard.html` and
`docs/ic/progress-timeline.json` remain unchanged. These proposals add no
verified curve transport or measured cost to their data [pcc:unverified].
