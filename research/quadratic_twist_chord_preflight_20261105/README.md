# Next experiment: complete target-chord decomposition

This is [PR3450](https://github.com/isogeny-labs/crypto/pull/3450), the execution-design follow-on to merged [PR3438](https://github.com/isogeny-labs/crypto/pull/3438). It specifies a falsifiable component experiment, including the diagonal branch missing from the initial remainder-only proposal. It is not an experiment result, an implemented solver or a checked theorem. The canonical `protocol.json` is DRAFT, not frozen or launch-ready. [pcc:unverified]

PCC statement record: `PCC-20261105-7179e4`, with no proof or signed verification. The earlier three ideas remain selected: chord completeness first, a shared collector after admissible rows exist, then the trace-variable encoding. No novelty claim is made. [pcc:unverified]

## Concrete next steps

1. Implement a pinned Lean proof of the curve/line factorization, cubic remainder, distinct/diagonal coverage and identity handling in [ALGORITHM.md](ALGORITHM.md). Preserve failed sources and use the unmodified proof gate and axiom audit. A new polynomial identity claim also needs source-bound IDC controls. No proof build has run.
2. Implement separate native producer and arithmetic auditor for the declared F37 fixture. They must reconstruct complete ordered signed coefficient rows independently, rather than accept a hash or a producer success flag. Implement the canonical export and meaningful corruptions before data. Neither program or runner exists yet.
3. After the approved official checkpoint, run a labelled engineering preflight on a fixed first case per model, retaining its data and any changes. Freeze a new version with actual source/analysis hashes and the observed resource estimate before the full census. This known finite fixture is exploratory, not a blinded confirmatory sample.
4. Run the full query census: 84 size-three policies in each of four roles (main, native twist, transported main base, pulled-back twist base), with every one of the 19 subgroup queries including identity. That is 6,384 PLANNED query records per instrument; 6,048 are nonidentity. These counts include repeated queries/controls, not distinct DLP challenges. No query is executed now. [pcc:unverified]
5. Only after row completeness passes, freeze and execute separate cold, one-target setup/recovery workloads for all 18 original source points per policy/role. Public base logs must come from admitted setup relations; record any rank-deficient setup as blocked. Then test fixed collector schedules against the entire source-policy frontier under the charged work contract. These later workloads are pending and are not discharged by the query census. [pcc:unverified]

## What this first experiment decides

The first primary outcome is exact equality of complete ordered signed rows with the independent point-enumeration reference, including diagonal and identity cases. Any omitted/extra row, wrong original-source transport, or missed planted branch invalidates the instrument; a timeout is incomplete coverage. [pcc:unverified]

A complete-or-unknown solver cannot establish strict per-query row complementarity merely by unioning two identical complete sets. Collector benefit must be tested across charged query prefixes, or with a separately specified genuinely partial-certificate solver. This design does not claim that the new chart improves yield or that its displayed degree is a lower intrinsic regularity. [pcc:unverified]

## Status and launch

No native scientific/proof command, official checkpoint, upload, scalar study or measurement has run on this branch. Read [LAUNCH_GATES.md](LAUNCH_GATES.md) for the exact new checkpoint-only scope. The previously pending prefix belongs to the now-merged proposal branch and is not reused as a disguise for this branch. Existing local resources only, two workers maximum, 600-second stage caps and one scientific window no longer than 3,600 seconds.

The three existing `XP-20261105-twist-*` backlog cards remain draft; this follow-on does not create a duplicate researcher, automation or task. No lease, funding, reservation or main-ancestry promotion is claimed. The same single automation, disabled after the proposal PR merged, is now rescheduled to follow this active PR at the spaced cadence. It may review CI and genuinely independent work, but may not launch dependent science or upload without the exact approval. All old research and merged worktrees remain unchanged.

The exact algorithm, query population, controls, invalidation rules and degree/cost distinctions are in [ALGORITHM.md](ALGORITHM.md), [PROTOCOL.md](PROTOCOL.md), and the draft canonical [protocol.json](protocol.json). The [two-page report](reports/design/report.pdf) is a design diagram, with no numerical results. Old quantitative scoreboards, benchmarks and graphs remain unchanged because there is no new measured data. [pcc:unverified]
