# 2026-11-15 public PR sweep follow-up (16:54 UTC capture)

Public-only, non-atomic update to the earlier [08:00 PT snapshot](./sweep-0800-PST.json); original snapshot and survey receipts are untouched. [pcc:unverified]

## Complete public PR discovery (as returned by created-date search partitions)

[Machine-readable frozen public inventory](./sweep-followup-public.json) retains **310 open PRs** across the nine public repositories (including the two with no open PR): crypto 189, cryptanalysis 57, crypto-autoresearcher 56, lfd-summation-systems 4, prototype 2, ecbench 1 and ecc-challenges 1, autoresearcher 0 and experiments 0. Every search partition returned fewer than the connector's 100-result cap. This is search-index enumeration, not an atomic GitHub transaction; the connector does not expose a native PR-list pagination cursor. [pcc:unverified]

Against the frozen [16:16 UTC inventory](./sweep-0800-PST.json), **newly open** are [crypto #4306](https://github.com/isogeny-labs/crypto/pull/4306), [#4307](https://github.com/isogeny-labs/crypto/pull/4307) and [#4308](https://github.com/isogeny-labs/crypto/pull/4308); [#4305](https://github.com/isogeny-labs/crypto/pull/4305) is no longer open (**merged**, verified from PR metadata), giving a net increase of two. Newly discovered heads are pinned in the inventory. Sampled changed heads include #4295, #4188, #4098 and #3877. The other head rows retain earlier exact-SHA captures; **not every current head, check context or review was reprobed**. [pcc:unverified]

## Substantive updates grouped with their parent leads

- **PKM algebraic fall degree — #4306 joins #4098/#4173.** Source claims two m=2,t=14 cap-6 re-runs completed, both with no refutation, hence d_F at least 7. This defeats the flat-at-six prediction but **cap 7 was only reported launched** at 16:45 UTC, not evidenced complete or live. No m>=3 growth law is proven; m=2..6 fall-degree/FFD differences are observed at tested cells only. Next: retain the cap-7 terminal receipt, verify the actual native F4 computation independently and charge relation yield and full IC cost. [pcc:unverified]
- **Cross-characteristic transfer — #4308 belongs to #3877's transfer lead.** A source-reported Lean example identifies **order-three** subgroups on curves over F5 and F7, with a scalar congruence and classical-choice additive equivalence. Its Lean 4.24.0 / Mathlib 4.24.0 proof gate was reported passing; **this is not an efficiently computable encoded-point map** or a general ECDLP transfer. Next: define and price a scalar-blind evaluator and preserve field/subgroup restrictions. [pcc:unverified]
- **IC parity / generic-floor analysis — #4307** is a session-log-only scope memo, not a newly executed experiment. Its F=1 and F=2 cost statements rely on a named generic-collision model; do not treat those generic decomposition bounds as an obstruction to bona fide algebraic IC. [pcc:unverified]
- **Auxiliary setup — #4305 merged**, but its source expressly says the 14,754-preimage numerical qualification was **not run** because the existing checkpoint upload gate failed. Merge is not scientific qualification. [pcc:unverified]

## Sources and gates

Current [root leads.log](../../../leads.log) groups proof stacks, transfer continuations, algebraic/non-generic IC, N83, HPC and scientific gates as research leads. Earlier source links and exact-head evidence remain frozen. The research-KB MCP was not loaded in this runtime and the prior wrapper failed with missing executable; vault/private content is deliberately excluded from this public snapshot. Search results do not establish current cloud-worker process status. Signed PCC standing and independent replay were not verified by this follow-up. No runtime/cloud action, leaderboard promotion, merge, infrastructure change or ledger rewrite was made. [pcc:unverified]

The existing [survey PR #4249](https://github.com/isogeny-labs/crypto/pull/4249) remains open and conflicted; its [Copilot merge-resolution instruction](https://github.com/isogeny-labs/crypto/pull/4249#issuecomment-6889535511) was inspected. Preserve complete current-main PCC ledger prefix and both sides of concurrent session logs before any merge, then rerun repository checks and review. Squash auto-merge connector error must not be mistaken for a permission rejection. [pcc:unverified]
