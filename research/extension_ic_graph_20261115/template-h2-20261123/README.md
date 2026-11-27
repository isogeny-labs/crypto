# H2: exact degree-7 template constructed; cost comparison open

24 November 2026. Rust, local, exposed saved Nagao systems over F1009.
The degree-7 principal-syzygy template returns a complete 40-polynomial
grevlex basis on saved cases 0 and 1. Both pass original-generator membership,
input containment, all S-pair checks and mutual containment with the saved
unrestricted F4 basis. Each elimination has rank 1652. This is bounded
construction viability, not a measured improvement in the relation finder.
Producer measurement/data claim **PCC-20261124-a42327 is UNVERIFIED**.
[pcc:unverified]

[Report PDF](report.pdf) · [Measured row graph](row-counts.svg) ·
[Native module](../../../src/cryptanalysis/jv_nagao_template.rs) ·
[Original degree-7 summary](degree7-summary.json)

## All construction outcomes

| Construction | Row shape | Outcome |
|---|---|---|
| Generic GB cofactors, original phase 1 | Compiler refuses its witness shape/budget gate | No online application; learning recorded 33,418,328 modular products |
| Principal syzygies, degree 5 | 504 enumerated, 451 kept, 462 columns | Both cases refuse the leading-ideal guard |
| Principal syzygies, degree 6 | 1260 enumerated, 1057 kept, 924 columns | Both cases refuse the leading-ideal guard |
| Principal syzygies, degree 7 | 2772 enumerated, 2184 kept, 1716 columns | Both cases accept complete bases and match saved F4 ideals |

Every listed outcome is retained in its original summary/events. The first
constructor's coarse refusal does not identify the particular failed bound.
It does not establish that generic provenance is impossible. [pcc:unverified]

The saved reference has 40 minimal leading generators, with degree histogram
`2:6, 3:9, 4:10, 5:9, 6:5, 7:1`; its degree-7 minimal generator is `x5^7`.
The degree-5/6 full-basis templates therefore could not satisfy the required
leading ideal. This should have been checked before those runs. Their costs
remain recorded, and they are inadequate-degree controls rather than evidence
against H2 generally. The API now refuses an insufficient degree before
enumerating rows. The read-only Rust structure analysis is in
`saved-reference-shape.json`, claim PCC-20261124-8f3d8b, UNVERIFIED.
[pcc:unverified]

## Recorded arithmetic

| Degree | Case 0 elimination products | Case 1 elimination products | Accepted |
|---|---:|---:|---:|
| 5 | 18,243,997 | 18,240,089 | 0/2 |
| 6 | 157,579,955 | 157,588,350 | 0/2 |
| 7 | 1,004,730,078 | 1,004,673,424 | 2/2 |

These counters include this implementation's explicit provenance-column
arithmetic and inversion exponentiation. Degree-7 exact checking separately
records 11,991,515 and 12,005,619 products; mutual-containment and input
validation counters are separately retained. The two degree-7 eliminations
total 2,009,403,502 products. The row matrix plus provenance uses 8,517,600
coefficient cells. None of these is a runtime measurement or complete machine
cost. They cannot be substituted for F4's differently covered partial
counters, and no speed ratio is asserted. [pcc:unverified]

## Method, integration and checks

`Template::principal_syzygy` enumerates original-generator monomial
multiples up to the degree bound and omits rows reducible by the explicit
principal syzygies `f_i e_j - f_j e_i`. It does not construct a minimal basis
of the full syzygy module. Input leading monomials are frozen; changes refuse.
The development F4 basis supplies only the leading-ideal policy fingerprint.
Its polynomials never fill an online row.

Online modular RREF carries explicit row-operation witnesses. The retained
minimal-leading rows are accepted only after exact membership, original-input
containment and S-pair checks. A matching fingerprint alone cannot accept an
incorrect or incomplete basis. The API returns a checked complete basis or an
explicit refusal with attempted arithmetic; it never converts refusal to a
proved empty root set. [pcc:unverified]

`src/cryptanalysis/mod.rs` exports the opt-in module. Its polynomial type is
the same as `f4_fp::Poly`, and exported terms follow descending grevlex with
monic leading coefficients. This provides the basis boundary for the existing
action/root stage and `jv_nagao_unit::decompose_using`. A fully benchmarked
callback/fallback and whole-IC comparison remain to be completed; no production
default changed. Twelve native source controls, release build and strict
all-target/no-deps Clippy pass. Current-main full-root compilation has separate
inherited syntax/fixture failures, so this is an exact standalone-source
qualification, not a passing full-library claim. [pcc:unverified]

## Frozen calls and custody

All calls had exclusive markers, one thread and 8 GiB virtual-memory limits.
The generic and degree-5/6 calls had 120/180-second outer bounds; degree 7 had
240 seconds. Every native and launching-tool exit was 0, post-worker hash
checks were 0 and stderr was empty. No old H1 or Runpod command was repeated.
These H2 statuses do not repair H1's separately retained launcher discrepancy.
[pcc:unverified]

- Phase 1: plan/source before call at `b6f1c7a3adb65a6dd21bedac932ee32ece822a9c`;
  local root `/workspace/ic-local-runs/template-h2-20261123-01`.
- Degree 5/6: plan/source before call at `5c306fadb079cd4b832b6945abcd4c4a2136f026`;
  local root `/workspace/ic-local-runs/template-h2-koszul-20261124-01`.
- Degree 7: plan/source before call at `8867bf797e58e035c4407ac39a777a5f22365fe6`;
  local root `/workspace/ic-local-runs/template-h2-degree7-20261124-01`.

All use the same hash-pinned H1 saved-bases file; they are exposed same-fixture
case controls, not fresh curves, targets or held-out population evidence.
The saved reference was an offline development oracle. A cold comparison must
price constructing it afresh, all learning, failures, checking, root recovery
and graph materialization.

Original witnesses (6,225,569 bytes for degree 7) and template JSONs exceed the
artifact policy and remain LOCAL with exact hashes in validation/. They have
not been split or claimed uploaded. S3 checkpoint/store credentials and
isolation admission remain unavailable. Producer records and compact summaries
are published on the lane; no signed PCC verdict, leaderboard promotion or
primary rho parity is asserted.

## Next discriminating action

Measure this complete-basis backend against the existing traced six-coordinate
F4 backend with the same root/materialization boundary. Retain full cold setup,
template learning, exact checking and explicit fallback. The present dense
matrix/provenance work is large; neither row pruning nor basis correctness
establishes a speed gain. The broader graph-IC/rho goal remains active.

Concurrent main appends have made the lane's canonical PCC history divergent.
Both original histories are preserved; do not rewrite/rechain an inherited
ledger to obtain green checks. This publication constraint is separate from
the completed native construction controls and still needs reconciliation.

Sources: retained primary Larsson–Åström–Oskarsson 2017 §§2–3 and Larsson et al.
2018 §§1–2, linked in ../hypotheses-20261123/README.md; the existing
certified-relations generic constructor and exact checker; the captured F4
reference and previous H1 packet. KB/Conductor are unavailable as recorded.
No novelty, global optimality or general-template impossibility claim.
