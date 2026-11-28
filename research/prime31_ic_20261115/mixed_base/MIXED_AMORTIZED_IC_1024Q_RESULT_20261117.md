# Prime31 reusable mixed-base index calculus: 1024 complete unknown scalar logs, fully charged reuse

**Executed 2026-11-17 UTC.** Source-specific native Rust 1.100.0 [Actions #44024919023](https://github.com/isogeny-labs/crypto/actions/runs/44024919023) passed at source commit `52b977cd3bb62ca78038a9d88479051b64ee8be2`. All **40/40** native library tests and **3/3** new example tests passed. The actual production run recovered **1024/1024 full unknown-target ECDLP scalars**, including 61/61 original-curve base logarithms, signed original-group relation witnesses, and `[k]G=Q` checks for every solver output; the caller compared every answer to its deliberately withheld generator scalar. Zero caller target misses/caps. Producer evidence is **[pcc:unverified]**, no separately signed independent certification. Source-specific [raw attempts/artifact #14302631901](https://github.com/isogeny-labs/crypto/actions/runs/44024919023/artifacts/14302631901), ZIP digest `sha256:707d78965e3df25ede8a4ac6e4b107ba4558fca42a0a372341707fd790d73a58` (as reported by GitHub).

## Frozen protocol and exact method

The prospective [protocol](MIXED_AMORTIZED_IC_PROTOCOL_20261117.md) was committed at `9c0a8fa1` **before coding or timing**. Only the existing cofactor-one (p=2147483647), (n=2147446831) curve and original native Rust `PRIME31`, `Arithmetic`, `Base::build`, `HalfTable`, `Rank`, original signed point replays were used; no weak curve. Both actual B31 bases share one point, so the deduplicated unknown logarithm vector has dimension 61.

Setup finds 61 independent `aG=sum(c_i P_i)` relations by the existing four-plus-four signed group half-tables. The 61×61 modular system yields all `ell_i=log_G(P_i)`, independently verified via `[ell_i]G=P_i`. For every new public `Q` the existing MITM oracle finds `aG+bQ=sum(c_iP_i)`, with nonzero b; solve `log_G(Q)=b^(-1)(sum c_i ell_i-a)`, verify `[log_G(Q)]G=Q`. The caller's target scalar is **never** supplied to `descend_unknown_q`. Setup paid once for all targets; each target's failed attempts, group witness, modular row, and verification are charged to its descent.

## Source-specific fully charged cold cost (one 1024-Q batch)

| Scope | Exact wall time |
|---|---:|
| Once-per-curve setup, both tables + 61 logs + checks | **1733.484 ms** |
| All 1024 complete descents, each including verification | **1029.611 ms** |
| **Complete 1024-target batch, setup + all descents** | **2763.095 ms** |
| Mean descent only, **excluding** setup | 1.00548 ms/Q |
| Mean fully charged across 1024, **including** setup | **2.69833 ms/Q** |

Frozen prefixes from CI:

| Number of unknown Q | Completed / attempted | Entire setup + descent wall |
|---|---:|---:|
| 1 | 1/1 | 1734.081472 ms |
| 64 | 64/64 | 1813.225204 ms |
| 256 | 256/256 | 2008.244089 ms |
| 512 | 512/512 | 2246.585790 ms |
| 1024 | 1024/1024 | 2763.094679 ms |

**Historical unmatched context ONLY**: #4486 observed 85.208 ms for 24 complete unfolded rho32 Q (3.55 ms/Q) on another workflow and another target roster. Numerically comparing its 3.55 ms/Q to this 2.70 ms/Q is **not a matched-rho result, not an isolated-CPU result, not a statistically valid win, and not a faster-than-best claim**. This approach is prohibitively expensive for one cold Q (~1.734 s); its possible usefulness requires many logs on the **same curve with reusable precomputation**. The table construction also implies a memory/multi-target tradeoff, which this run did not independently price.

## Critical novelty and competitor qualification

This is a fully solving **index-calculus-derived multi-target** method with actual factor-base relations and rank inversion, but the current **4+4 group table is generic meet-in-the-middle equivalent**. Reuse across targets is classical multi-target preprocessing, **not** a novel non-generic index-calculus asymptotic advance. It cannot yet claim an algorithm beating an optimized multi-target BSGS or WZ rho, or a published prime-field IC implementation. The central open original-ideal nonenumerative RR9/S9 factor extractor from public R remains untouched.

The source-specific optimized WZ benchmark #43994897230 failed *before solving* on an unrelated pinned `sat.rs` delimiter; do not interpret as a rho loss. The strongest **next gate** is a matched 1024-Q run on the same isolated CPU (including a no-precompute rho, tuned WZ negation/Brent, an explicitly **precomputed** generic BSGS sharing its table, and published APS/PKM implementation); compare full 1024 setup+all solve time, peak memory, paired target ratios, A/A drift, correctness, misses, and compute-equivalent cost. Even a matched win over unfolded rho would not settle optimized precomputed BSGS.

## Previous negative leads and reproducibility

- Source: [Rust implementation](../native/examples/prime31_mixed_base_amortized_ic.rs), [pinned CI](../../../.github/workflows/prime31-mixed-amortized-ic.yml), full immutable source commit above, [protocol](MIXED_AMORTIZED_IC_PROTOCOL_20261117.md).
- Prior #4503 and #4506 measured complete **cold** mixed 4+4 at ~2.12 s/Q, ~2.05 s paid table setup each time; #4505 exact mixed-support theorem has no cost claim.
- #4454/#4486 improved classic APS, but its per-Q S3 method still relies on generic MITM pair enumeration; #4466 provided correct given-leaf RR9 Hermite x-norm certificate, **not** unknown-target polynomial extraction.
- Crypto-autoresearcher #3301/#3305 and `experiments/semaev-alternatives-20261105/SUMMARY.md`/`ERRATA.md` include Vieta, norm-lattice, m6 F4, tower F4, and alternative attack negative controls. SA-19 twisted-Edwards rational 2-torsion coordinate model is inapplicable to the particular odd-prime-order cofactor-one curve without rational 2-torsion.
- Primary papers: PKM (2016), DOI 10.1007/978-3-662-49387-8_1; APS (2018), DOI 10.1016/j.ffa.2018.01.009. KB and Conductor MCP were unavailable; explicit GitHub and literature fallback, no absence/novelty claim.

## Failed-source evidence preserved

Initial [CI #44024263180](https://github.com/isogeny-labs/crypto/actions/runs/44024263180) compiled and passed 40 existing native tests but failed the new setup test because the old single-target row generator represents `-b` as `n-b`; the new setup deliberately passes `b=0` and thus got noncanonical integer `n` instead of `0`. The exact source correction `row[basis.len()] %= PRIME31.n` (commit `07ba170`) fixed only that representation and preserved protocol parameters. A subsequent overly sparse immutable checkout missed required original `src/cryptanalysis/f4_fp_tower.rs`; follow-up workflow `52b977c` hydrates both original tower modules without modifying arithmetic. No data-dependent seeds, caps or positive-verdict changes were made. Successful experiment is exact source `52b977c`, not these failed precursors.

**Decision:** preserve this credible positive *amortized full scalar recovery* result, do not promote a frontier speed/novelty claim until matched best-rho, cached BSGS and independently signed PCC checks are complete. No paid compute or leaderboard changes.
