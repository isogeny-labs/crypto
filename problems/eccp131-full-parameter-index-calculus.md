# ECCp-131 full-parameter index-calculus relation pipeline

- **Status:** `open`
- **Kind:** exact-curve implementation and validation gap; method-specific two-sum coverage obstruction
- **Affected work:** [`ic prime --curve eccp-131`](../src/bin/ic/prime.rs), [`icx estimate eccp-131`](../src/bin/icx.rs), [`ca-ic prime --curve eccp-131`](https://github.com/aburan28/cryptanalysis/blob/main/suite/src/cli/tools/ic/prime.rs), and the requested ECCp-131 index-calculus ECDLP workflow
- **Next action owner:** index-calculus implementation lane, unassigned
- **Last updated:** 2026-10-20

## Problem

The published Certicom ECCp-131 point does not yet have a validated full-parameter index-calculus solution. The `ic prime` command now has a bounded exact-curve two-sum relation/rank/descent probe, while the `ca-ic prime` command in `cryptanalysis` still executes its relation pipeline on a generated smaller curve. The exact group is the short-Weierstrass curve and subgroup in [Certicom's curve list](https://www.certicom.com/en/curves-list), with cofactor one, generator and target bound by their coordinates. The requested end state is a scalar for the published target independently checked as `[x]G = Q` with a complete cold cost record.

## Evidence and uncertainty

- **Observed:** `src/cryptanalysis/ic_engine.rs` classifies prime-field named curves as a scaled regime. The `cryptanalysis` command `ca-ic prime --curve` reports structure on the named curve and runs its relation pipeline on a generated smaller curve; its `suite/src/cli/tools/ic/prime.rs` documents that boundary. The `crypto` command `ic prime --curve eccp-131` now admits exact-curve two-sum enumeration and an optional bounded natural-relation/rank/target probe, with a [predeclared diagnostic protocol](../research/eccp131_bsgs_ic_20261020/EXACT_IC_PROBE_PROTOCOL.md). The generic `ec_index_calculus_dlp` API and `icx` workflow do not have a completed ECCp-131 relation corpus. `icx run --require-named-curve` refuses a full-parameter run. The existing ECC2K-130 fixed-parameter path is a different field and curve.
- **Derived or proved:** The in-memory BSGS table for a whole subgroup of roughly 131 bits would require on the order of `2^65` entries. For the current two-sum method with `B` distinct factor-base abscissae, there are at most `B` doublings and `2·C(B,2)` sums, hence at most `B²` distinct sum abscissae and `2B²` covered nonzero group points. A uniform nonzero candidate therefore hits with probability at most `2B²/(n−1)`. With the published `n = 0x048E1D43F293469E317F7ED728F6B8E6F1` and the CLI's largest admitted `B = 8192`, that upper bound is `2^−103.1875` per candidate. Even if every successful row were independent, collecting `B+1` rows would require at least `2^116.188` candidate trials in expectation under independent uniform sampling. This is a coverage bound for this factor base and two-sum oracle, not a lower bound for all index-calculus methods.
- **Suspected explanation:** No efficient, complete relation oracle at this size has been demonstrated in these implementations; its attainable rate and cost remain unknown.
- **Missing capability:** A relation method that produces enough natural, independent exact-curve rows and a target descent within an executable resource envelope, followed by independent replay and complete cold cost. The bounded two-sum collector and modular-rank path now have code, but code presence does not satisfy the published-point scalar criterion.
- **What this evidence does not establish:** It does not show that ECCp-131 index calculus is impossible, nor does a scaled run establish a full-parameter cost or a logarithm for the published point.

## What would unblock it

- [ ] Freeze an exact ECCp-131 protocol with curve, subgroup, generator, target, factor-base digest, sign convention, relation and rank metrics, stopping rules, raw output paths, and replay controls. A bounded two-sum diagnostic is recorded separately; it does not replace a complete relation corpus.
- [ ] Implement and independently replay an exact-curve relation, linear-algebra, target-descent and scalar-reconstruction chain that verifies `[x]G = Q`; account for setup, failed candidates, verification, memory, and wall time.

These conditions would establish a completed run on this point. A speed claim would still require a matched baseline, independent verification, and the repository's progress-fidelity record.

## Next discriminating step

- **Action or proof obligation:** After recording the bounded exact two-sum probe, specify and test a different relation oracle or factor-base construction that can produce natural rows beyond the `2B²` support limit at an executable cost. Retain the existing two-sum base as a negative control.
- **Inputs and controls:** Certicom ECCp-131 parameters and target; same-field random points as null controls; independent point-addition and relation checks; a known-answer construction kept separate from the published target.
- **Execution limits and stop condition:** Freeze candidate counts, solver seconds, memory watchdog, and artifact paths before execution. Stop on the protocol limits or a verified relation corpus, retaining all attempts.
- **Outcomes and next actions:** Verified independent relations permit a rank and full-cost protocol; zero or duplicate rank narrows only the tested factor base and budget; a timeout or invalid witness directs implementation repair and leaves the curve-level question open.
- **Recheck trigger:** A complete exact-curve relation oracle or a new verified transfer with subgroup and cost certificates.

## Resolution and history

Unresolved. This record keeps the full-parameter requirement visible while bounded BSGS and scaled index-calculus entry points are integrated.
