# 2026-11-19 — Prime31 x-only signed Horner/F3 follow-up (#4812)

## Goal and custody
Continuation of existing Horner x/y PR #4812, not a second algebraic extractor PR. Kept `docs/ACTIVE.md` checked item for this gate. Same ordinary full-prime-order curve E/F_2147483647: a=847514846, b=2087444724, #E=2147446831, G=(625037715,2100627251), cofactor=1; original Rust arithmetic pinned `1df714645aa683cf975bc76fcacb9842cb4780fd`. Existing public-only RR9/F4 extractor owner is #4692.

## Literature and prior lanes (bounded search)
- #4812 original x/y Horner chart; #4809 quadratic Semaev F3 & exact tiny two-factor support; #4810 x-only [16] permutation; #4808 weighted block-scale; #4807 weights; #4801 low-degree interpolation obstruction; #4803 unweighted coverage; #4728 whole Rust grader and #4532 optimized BSGS reference.
- `research/ic_tree_gluing_20261103`, `research/prime31_ic_20261115/native/M8_UNIFORM_RANK_CEILING_AND_F4_PILOT.md`, and old native Rust examples.
- Cross-repository code search: `crypto-autoresearcher/experiments/semaev-alternatives-20261105/SUMMARY.md`; `cryptanalysis/suite/examples/pkm_tower_pilot.rs`. Public primary literature: 2020 Yokoyama et al. complexity analysis (https://doi.org/10.1515/jmc-2019-0029), PKM 2016 (https://doi.org/10.1007/978-3-662-49387-8_1). No novelty claim for standard F3/doubling.
- Research-KB MCP was not in accessible tool roster; large repository KB CLI unavailable outside a Git checkout. GitHub code/PR & web search fallback is not an exhaustive survey.

## Protocol and implementation
1. Committed `research/prime31_ic_20261115/rr9/HORNER_XONLY_SIGN_LIFT_PROTOCOL_20261119.md` **before implementation** with seed `0x584f4e4c595f4348`, four 8-point B31 windows and 256 constructed cases/window, plus falsification gates.
2. Implemented `native/examples/prime31_horner_xonly_sign_lift.rs` reusing original `Arithmetic`, with 22-variable/23-equation x-only F3/doubling chart (21/22 after public target), explicit finite-intermediate scope, poisoned denominator/terminal checks, post-root signed recovery that never reads planted signs. Source-specific CI copies only candidate example into pinned original native crate.
3. Wrote separately implemented `native/verify_prime31_horner_xonly_sign_lift.py`: Python bigint elliptic group and F3 independently regenerate/replay entire cohort, all residues and signed reconstruction. Not a signed independent PCC authority.
4. Updated original `.github/workflows/prime31-horner-xy-chain.yml` to run original and new tests, release 1,024 cases, independent all-case verification and evidence upload. Added full proof, lead audit and next gates in `rr9/HORNER_XONLY_SIGN_LIFT_RESULT_20261119.md`.

## Exact-source result
[Workflow #44578723868](https://github.com/isogeny-labs/crypto/actions/runs/44578723868) **PASS**; [raw artifacts #14556259317](https://github.com/isogeny-labs/crypto/actions/runs/44578723868/artifacts/14556259317).

- Original native library: **40/40 tests**; original x/y example **2/2**; new x-only example **2/2**.
- New original Prime31 release corpus: **1,024/1,024** ordinary finite planted chains, zero exceptions. All x-only polynomial residuals zero; post-root signed leaves reproduce Q; **1,024/1,024** poisoned d and target-x controls rejected.
- Independent big-int Python: **1,024/1,024** cohort exactly replayed; **16/16** saved signed coefficients/certificates matched. Raw JSON sha256 `b6bd2e08e4368bd61ede73c21e74e3f2a688639fb2b8f6967179ebddc921f3b5`; independent report sha256 `8e32a4421b4a92e547d3c38c7ab0ad1841df421c07c5e78e2e1b4a778653bdca`.
- GitHub-wide generic `Workflow files parse` job failed on unavailable historical artifact `research/weak_curve_properties_lean_20261011/root_lib.stdout`, unrelated to source-specific arithmetic outcome. Do not represent the full PR check suite as globally green.

## Classification and handoff
Mathematical reduction/certification passed. Still no blinded public-only extraction and no cheaper complete solver. The y-free system trades 56 quadratics for 14 quartic constraints and lower variable count, which might or might not help F4. Communicated source, exactness scope and 46-positive/18-negative Rust public-only F4 gate to existing #4692 as [comment #7017067328](https://github.com/isogeny-labs/crypto/pull/4692#issuecomment-7017067328). Match strongest prior IC, optimized 2^20 shared BSGS (historical 170.113 ms for 1024 logs), and rho **on a single runner** only after whole-solver integration; separately sourced benchmarks cannot be paired.

PR #4812 updated and ready for review; attempted owner-default enable-auto-merge, blocked by repository GraphQL `UNPROCESSABLE: Merge method squash merging is not allowed`. No merge was performed. Independent signed PCC remains pending.
