# ECCp-131 full-parameter index-calculus relation pipeline

- **Status:** `open`
- **Kind:** implementation gap; empirical relation cost unknown
- **Affected work:** [`icx estimate eccp-131`](../src/bin/icx.rs), [`ca-ic prime --curve eccp-131`](https://github.com/aburan28/cryptanalysis/blob/main/suite/src/cli/tools/ic/prime.rs), and the requested ECCp-131 index-calculus ECDLP workflow
- **Next action owner:** index-calculus implementation lane, unassigned
- **Last updated:** 2026-10-20

## Problem

The published Certicom ECCp-131 point does not yet have a validated full-parameter index-calculus path: the public prime-field CLI executes its relation and descent stages on generated smaller curves, and the reference library has no bounded, independently replayed ECCp-131 relation-to-scalar run. The exact group is the short-Weierstrass curve and subgroup in [Certicom's curve list](https://www.certicom.com/en/curves-list), with cofactor one, generator and target bound by their coordinates. The requested end state is a scalar for the published target independently checked as `[x]G = Q` with a complete cold cost record.

## Evidence and uncertainty

- **Observed:** `src/cryptanalysis/ic_engine.rs` classifies prime-field named curves as a scaled regime. The `cryptanalysis` command `ca-ic prime --curve` reports structure on the named curve and runs its relation pipeline on a generated smaller curve; its `suite/src/cli/tools/ic/prime.rs` documents that boundary. The reference `ec_index_calculus_dlp` API accepts arbitrary prime-curve parameters but has no ECCp-131 admission, cost, or replay protocol. `icx run --require-named-curve` refuses a full-parameter run. The existing ECC2K-130 fixed-parameter path is a different field and curve.
- **Derived or proved:** The in-memory BSGS table for a whole subgroup of roughly 131 bits would require on the order of `2^65` entries. This resource cost does not rule out a different algorithm or a bounded interval instance.
- **Suspected explanation:** No efficient, complete relation oracle at this size has been demonstrated in these implementations; its attainable rate and cost remain unknown.
- **Missing capability:** An exact ECCp-131 factor base, verified relation collector, independent-rank solver, target descent, and reconstruction with bounded resource accounting.
- **What this evidence does not establish:** It does not show that ECCp-131 index calculus is impossible, nor does a scaled run establish a full-parameter cost or a logarithm for the published point.

## What would unblock it

- [ ] Freeze an exact ECCp-131 protocol with curve, subgroup, generator, target, factor-base digest, sign convention, relation and rank metrics, stopping rules, raw output paths, and replay controls.
- [ ] Implement and independently replay an exact-curve relation, linear-algebra, target-descent and scalar-reconstruction chain that verifies `[x]G = Q`; account for setup, failed candidates, verification, memory, and wall time.

These conditions would establish a completed run on this point. A speed claim would still require a matched baseline, independent verification, and the repository's progress-fidelity record.

## Next discriminating step

- **Action or proof obligation:** Build an exact-curve relation-oracle admission probe for a fixed, enumerated factor base and a predeclared set of public candidate points, with a separate generated-known-answer positive control.
- **Inputs and controls:** Certicom ECCp-131 parameters and target; same-field random points as null controls; independent point-addition and relation checks; a known-answer construction kept separate from the published target.
- **Execution limits and stop condition:** Freeze candidate counts, solver seconds, memory watchdog, and artifact paths before execution. Stop on the protocol limits or a verified relation corpus, retaining all attempts.
- **Outcomes and next actions:** Verified independent relations permit a rank and full-cost protocol; zero or duplicate rank narrows only the tested factor base and budget; a timeout or invalid witness directs implementation repair and leaves the curve-level question open.
- **Recheck trigger:** A complete exact-curve relation oracle or a new verified transfer with subgroup and cost certificates.

## Resolution and history

Unresolved. This record keeps the full-parameter requirement visible while bounded BSGS and scaled index-calculus entry points are integrated.
