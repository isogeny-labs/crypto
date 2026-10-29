# Checked native PMU integration contract

Frozen before edits, 2026-10-29. Base b15f0908700b0e890641ea0db4e35fe967a5d63c; worktree crypto-ml-pmu-boundaries-20261029. Conductor T847 reserves module registration, telemetry and this design directory; T848 reserves the retained collector subdirectory.

The original corpus needs trustworthy instruction-cost labels. Current legacy HwCounters discards RESET, ENABLE and DISABLE errors; a successful read alone cannot establish a valid phase boundary. Source review found an existing corrected collector, so reuse it rather than introduce a competing lifecycle implementation.

Sources checked: retained collector STATUS.md and ENDIAN_REVIEW.md; instruction-retention-next-step.md; Linux perf_event_open(2), https://man7.org/linux/man-pages/man2/perf_event_open.2.html (events count while enabled, grouped event scheduling and calling-thread scope). This is measurement plumbing, not a cryptanalytic or performance claim.

Step 1 copies exact files from corrected donor runtime 4a57e45ec74ad878a4ed0ce7ccd68abb4f4d7647: src/cryptanalysis/ecbench/isolation/checked.rs and checked/tests.rs, plus its three-line registration. Preserve owner-context/unwind fixes and little-endian 64-bit ABI gate. Existing legacy counter API remains unchanged. No CLI label is repaired by registration alone.

Step 2 must reconcile the retained checked phase adapter and its capture/live/snapshot dependency chain before dispatch changes. No stub binding, duplicate wire schema, or partial producer claim. Preserve exact current root verification, startup/budget gates, and exporter ownership. Independently map the retained consumer before publication of new phase labels.

Validation: source-byte equality for copied collector; native deterministic collector tests including failure/cleanup/ownership cases; scoped formatting and strict release lint; required combined-source full library gate before code push. No new hardware campaign, scientific draw, cloud worker, IAM change, deadline or budget extension. Tests with mocks are software evidence only. Historical donor pass counts do not discharge current-source gates.

Checked lifecycle observations do not prove hardware truth, preexisting pool-thread coverage, descendant lifetime coverage, whole-worker instructions, complete execution traces, or solved ECDLP. Unavailable/censored values remain explicit; preserve old immutable records.
