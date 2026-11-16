---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# Finite repository rho counts and their accounting

The actual repository benchmark completes768/768 known-answer calls:
576 measured and192 warmups, with no failure or interruption. Every
measured deterministic replay matches. The192 measured and64 warmup
same-seed A/A pairs have identical counts, phases, native counters and
answers. A separately copied-index one-counter corruption is detected.
SessionECBS1h99ad02ca65c0. [pcc:unverified]

![Native recorded means and descriptive95% intervals, rounded as printed by ecbench table.48measured runs percurve/arm:16targets and3seeded rounds. All costs include setup, search and internal verification. The dashed0.886 line is a standard-negation model reference, not a universal lower bound. [pcc:unverified]](means.pdf){width=97%}

All four curves are registered generic prime-roster inputs with automorphism
count2 and cofactor1. The A/A arm repeats negation rho on identical seeds;
it tests determinism, not independent-seed variability. Main and preflight
populations are separate. All scientific clocks are invalid; only counted
operations are reported. [pcc:unverified]

The observed means lie above sqrt(pi/4). These small fixed inputs do not
refute or establish the qualified asymptotic average. Every size is below
the harness's2^25 calibration range, and fixed setup is explicit. Correct
source hypotheses and arithmetic cost models remain intact. [pcc:unverified]

\newpage

# Cold totals, emitted counters and honest comparisons

S is counted group-addition equivalents divided by sqrt(r), with an
addition and doubling charged1 each and scalar multiplication charged by
its actual additions/doublings. The interval covers method setup through
recovery and internal verification. All rows use48measured calls percurve/
arm. The A/A arm equals the negation arm in every paired record.
[pcc:unverified]

| Registered curve | r | Negation S0 | Plain S0 | Negation S0.1 | Negation S1 |
|:--|--:|--:|--:|--:|--:|
| icv1-fp14-tm43-158b6914 | 16231 | 1.715 | 2.178 | 1.829 | 2.857 |
| icv1-fp16-t351-d433f24e | 65003 | 1.839 | 1.892 | 1.962 | 3.075 |
| icv1-fp18-t175-45770ebc | 261673 | 1.349 | 1.532 | 1.453 | 2.381 |
| icv1-fp20-t1293-3c2999ba | 1046999 | 1.245 | 1.631 | 1.353 | 2.328 |

The band prices every emitted unpriced operation at0/0.1/1GAE. Negation
emits canonicalisation counts; plain lists none, so its displayed band is
constant. This is the emitted-counter band, not a calibrated complete
bit-operation model. The ordering reverses at weight1 on every curve.
No ranking holds throughout the declared band. Classification: accounting,
since changing the price changes numbers while the algorithms are fixed.
No isolated causal effect of negation is claimed: the two registered
methods also have their documented walk/cycle policies. [pcc:unverified]

| r | Negation setup | Search | Internal check | Total | Emitted unpriced |
|--:|--:|--:|--:|--:|--:|
| 16231 | 2669 | 6935 | 882 | 10486 | 6983 |
| 65003 | 6401 | 15088 | 1011 | 22500 | 15136 |
| 261673 | 6644 | 25283 | 1206 | 33133 | 25331 |
| 1046999 | 6744 | 53123 | 1293 | 61160 | 53171 |

The r column joins the registered curves above. These are exact integer
sums over48measured negation calls. Every plain/A/A phase and counter is
retained in analysis/main/native.jsonl and the canonical session.
[pcc:unverified]

The separate12-call preflight verifies and replays completely. Its seeds/
targets are distinct; it is not pooled into the main figures. The original
adapter export omitted analysis-script metadata and was retained as an
unvalidated draft; a new completed pilot record passes. Main metadata was
completed before its first export write. No solver outcome was retried.
[pcc:unverified]

\newpage

# Formal arithmetic, provenance and scope

The reporting Lean project proves arithmetic on the explicitly recorded
totals. It does not prove native execution authenticity, compiler refinement,
curve cardinalities or a limiting collision law. The native and formal
assurance scopes are separate. Full local gate11/0failing/0trusted-native.
[pcc:unverified]

| RhoConstantReportAudit declaration | Exact checked obligation |
|:--|:--|
| normalized_interval | Exact squared inequalities imply positive-denominator mean bounds |
| recorded_mean_intervals | Four rational intervals of width0.0001 for the recorded means |
| exact_phase_totals | Setup + search + internal verification equals each recorded total |
| emitted_weight_order_flips | Weight0/1 ordering reverses for all four sets of recorded integers |
| recorded_means_above_reference | Fixed recorded means exceed sqrt(pi/4), with no expectation assertion |
| finite_report_arithmetic_certificate | Combined finite report arithmetic |

Lean4.35.0 / Mathlib56a00585; only propext, Classical.choice and Quot.sound.
No holes, custom axioms, native_decide, trusted-native exceptions, waived
linters or new elaboration options. Local replay is distinct from independently
signed standing. Proof/source/packet receipts accompany the study.
[pcc:unverified]

Main protocol045a59cc was frozen and committed01eb80f3 before data;
preflight protocol3892337a was frozen and committed1214cbda before its
separate data. Main64targets,3rounds,1warmup,3arms. Binary61030c23 is
pinned exactly, with3,098 selected source/input files matching baseline
1d2ad3fd. Actual optimized release build succeeded after retained source-
coverage errors and connection interruptions. Full hashes are in protocols
and evidence/build-provenance.json. [pcc:unverified]

The executable's Git metadata is runtime-derived, not a compile-time revision
receipt. Actual main session Git commit is null and is not backfilled. Exact
compiled-input and binary hashes remain authoritative. Native SQLite3.46.1
analysis source/binary/header/runtime/query hashes are frozen. Flow768planned,
attempted,completed;0notcompleted/interrupted/resumed/excluded. Every
scientific clock has wall_validfalse. No independent-machine or signed-
execution claim. No paid compute or storage. [pcc:unverified]

Canonical session files remain loose for native replay paths under the13a
exception and approved zero-paid local checkpoint. Both exec/ directories
are empty. Generated SQLite index is rebuildable derived data, not canonical
evidence; no .db is committed. All measurements, warmups, controls and failures
are retained. Reproduction commands and exact files: study README.
[pcc:unverified]

Primary reading: Bernstein–Lange SafeCurves survey, selected4/4.1–4.5/5intro/
B.1–B.2. It distinguishes batched-affine additions and cost models; sqrt(pi/4)
is not claimed universally optimal. Web PDF text succeeded after bounded
page/PDF failures; no full-paper read invented. KB unavailable, bounded
repository/primary/Mathlib fallback. [pcc:unverified]

This is requirement11's finite benchmark component. Generic p-adic/complexity/
native-refinement, limiting-law proofs and the remaining fourteen-item
requirements stay open. No novelty, new security conclusion or refutation
of a correctly qualified source statement. [pcc:unverified]
