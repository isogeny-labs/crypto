# Degree-two local collisions: weighted identity and actual finite graphs

Lean proves the normalized weighted admissible-label identity `1−I2−I2²+I4`, where `I2=Σp_i²` and `I4=Σp_i⁴`. It also checks the actual two-step local collision probability on the complete uniform static-hash/startpair population over Z7, verifies the source's degree condition, and directly disproves deleting that condition. This formalizes and tests a known Bernstein–Lange2013 correction; no novelty or limiting collision constant is claimed. Signed standing remains unverified. [pcc:unverified]

For every finite real label-weight list summing to1, the actual sum of `p_i p_j p_k p_l` over different final labels j,l, excluding opposite orderings of the same two steps, equals `1−I2−I2²+I4`. The proof uses explicit label exclusions and off-diagonal finite sums. This is a generic weighted label identity; its identification with actual graph probabilities is checked separately for the stated finite tables. It is not supplied as a hypothesis. [pcc:unverified]

The actual graph is `F(x)=x+g(h(x))`. The sample space contains all128 maps `h:Z7→Fin2` and all49 ordered starting pairs. A fresh two-step collision requires x,y,F(x),F(y) pairwise distinct and F²(x)=F²(y). A genuine uniform PMF has pointmass equal to seven independent uniform vertex-label factors times two independent uniform-start factors. [pcc:unverified]

| Actual increment table | Source degree condition | Exact population among6272 | Actual event probability |
|:--|:--|--:|--:|
| 1,3; one-step fresh collision | Degree0/1/2 sums distinct modulo commutativity | 448 | 1/14 |
| 1,3; fresh two-step collision | Same condition, kernel verified | 336 | 3/56 |
| 1,2; fresh two-step collision | Distinct/nonzero steps, but degree condition fails | 112 | 1/56 |

All counts are proved over the actual complete sample space by kernel-checked decisions, with no native count oracle or assumed table. For uniform labels the weighted coefficient is3/8, and its value divided by7 is exactly the actual good-table two-step probability3/56. The bad table mixes degrees (`1+1=2`); Lean directly negates the formula after replacing degree genericity by injectivity/nonzero alone. The actual good-table one-step and fresh two-step rates also differ. No qualified source theorem is refuted. [pcc:unverified]

The closed main declaration `TwoStepCollisionAudit.degree_two_collision_certificate` is in [TwoStepCertificate.lean](lean/TwoStepCertificate.lean). See [proof map](proof-map.json), [source plan](sources.json), [source review](evidence/source-review.json), [one-page report](reports/report.pdf) and [diagram](reports/two-step-collision.svg). This supplements the unchanged [static-hash first-collision bridge](../static_hash_collision_audit_20261114/README.md). General actual-group/support coefficient identification, limiting random-increment-table firstcollision penalty/constants and practical fixed/keyedhash refinement remain open. Known source higher-degree asymptotic heuristic corrections are credited, not promoted to proved limiting results. [pcc:unverified]

Whole final gate26/0 failures/0 trusted native; complete1347-character binding`4c1acf0fd9722f21066a7ce0cb3de28321e7e626a7fbb2eaa2efafd2c441c000`, independent native1complete/0prefix/0unresolved. The first bounded header parser failed on scoped ENNReal notation. The mathematically identical explicit type name `ENNReal` rebuilt/regated successfully without editing the frozen parser. Seven build snapshots preserve all four failed builds; both native inputs/protocols and both gates remain:96files/44objects/12128-byte lossless XZ plus five final gate files. Exact local **PCC-20261114-480d25** replay PASS at 2026-11-14T17:30:06Z; signed standing remains unverified. See [verification receipt](verification.json). Native source processing is procedural evidence, not a kernel theorem about parsing, hashing or execution. [pcc:unverified]

Sources checked before implementation: Bernstein–Lange[*Two grumpy giants and a baby*](https://msp.org/obs/2013/1-1/obs-v1-n1-p05-s.pdf), DOI[10.2140/obs.2013.1.87](https://doi.org/10.2140/obs.2013.1.87), full selected printed94–97/PDF9–12 §§3–4 read/visually inspected, with degree and distinct-query qualifications retained; Galbraith[chapter14v2](https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch14.pdf), selected288/296–297; prior actual variance/firstcollision studies; official Mathlib finite sums, PMF and ENNReal APIs. No fullpaper-read or novelty claim. KB/Conductor unavailable127; precise primary/prior/official fallback completed before approach. [pcc:unverified]

Reproduce with Lean4.35.0 and Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679:

```sh
cd research/two_step_collision_audit_20261114/lean
lake build
```

From repository root run `tools/lean/proof-gate.sh research/two_step_collision_audit_20261114/lean`. Inside evidence verify `sha256sum -c attempts.tar.xz.sha256`; from the study directory restore every raw source/config/log/exit/protocol byte with `python restore_attempts.py /tmp/two-step-attempts`. All finite enumerations use kernel decisions, no native proof evaluator. No holes/custom axioms, global elaboration flags, raised limits, linter waivers, new native cryptographic experiment, paid compute or agent storage upload. Only propext/Classical.choice/Quot.sound. Publication/current receipts remain outside immutable pin sets. [pcc:unverified]

Published in existing ready [PR3743](https://github.com/isogeny-labs/crypto/pull/3743) as `786c61653153767fbe75805d2c0a1abc5881210c`, identical to local receipt tree `d3ac94f47ee4fcc8fd94d6c6288f801d50fed8dc`; all40paths/7large-or-binary blobs checked, completed anonymous fetch and fresh PRhead/bodyv102 confirmed. Immutable ledger prefix584 preserved through585entries. No current-head CI pass asserted; parent immutable-ledger fork remains needs-human. Publication metadata is outside frozen pin sets. [pcc:unverified]
