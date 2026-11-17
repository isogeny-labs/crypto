# Prime31 B31 eight-signed-leaf: actual Dickson-base misses, independent replay and the multiplicity gate

2026-11-16. **Producer source-specific GitHub Actions passed; independent arithmetic readback completed; independent signed PCC not available** [pcc:unverified]. This is an exact generic 4+4 membership oracle and a design correction for a **future** target-conditioned algebraic index calculus, not a cheaper non-generic ECDLP solver.

## Reused Rust source and immutable evidence

- Native Rust: [prime31_s9_b31_half_sum_oracle.rs](examples/prime31_s9_b31_half_sum_oracle.rs), source-specific SHA **ec891c47b1a0de025b61f9ae6676d538c8848e39**.
- Successful Rust 1.100.0 CI **[43952234526](https://github.com/isogeny-labs/crypto/actions/runs/43952234526)**; **40/40** native library tests, **2/2** half-sum example tests, and both B31 base panels with original-group replay assertions. Producer artifact: [14269144740](https://github.com/isogeny-labs/crypto/actions/runs/43952234526/artifacts/14269144740), ZIP digest **sha256:771612053124f331c66145f49298884e9e17f9e8911b01c04c751c2c47ded633**. Frozen protocol is [S9_B31_FOUR_PLUS_FOUR_ORACLE_PROTOCOL_20261116.md](S9_B31_FOUR_PLUS_FOUR_ORACLE_PROTOCOL_20261116.md).
- Curve: **p=2147483647**, E: y²=x³+847514846x+2087444724, full group **n=2147446831 prime**, cofactor one. Same fixed 24 distinct caller-known but solver-hidden Q=[k]G for both factor bases; 16 blinded constructed-positive Q and identity/cancellation control per base.
- Measured generic source work in each panel: **677,040** unordered four-token words, **618,885** distinct integer four-net signatures and **618,885** distinct four-sum group points, zero extra collisions of distinct net signatures; **678,993** group additions constructing the four-sum index. The reported setup diagnostic was 303.448 ms first-x and 305.449 ms Dickson; these are neither full ECDLP solves nor matched rho timings.

## Important cross-base observation

| B31 factor base (same 24 public natural Q) | Natural 8A positives | Blinded planted positive | O cancellation | Missing natural Q |
| --- | ---: | ---: | ---: | ---: |
| First 31 ascending rational abscissae | **24/24** | 16/16 | yes | 0 |
| Actual depth-six Dickson 31-point fibre used for K961/S5 | **22/24** | 16/16 | yes | **2** |

The two **explicit** natural nonrepresentability records for the actual special base, each scanned to **618,885** distinct four-sum entries without a complement hit, are:

| Case | Public Q=(x,y) | Caller-side k, never supplied to lookup |
| --- | --- | --- |
| 4 | (854640086,1374782344) | 1610011808 |
| 21 | (1146958692,202234633) | 1264623843 |

Both Q are representable on the *first-x* base but not on the *Dickson* base in this exhaustive producer experiment. They are **retrospective controls**, not new holdouts. Their generator multiplication coordinates were separately reproduced with an independently coded prime-field EC implementation.

A separate arithmetic readback of the immutable ZIP reran all **48** natural caller-point checks (24 same Q in each panel) and all **80** positive eight-token signed witness sums (41 first-x, 39 special) using independent modular point addition/scalar multiplication; all agreed. The two negatives additionally have full-exhaustion counters, but a separately designed exhaustive negative verifier or PCC signature has not been run; the source itself and source-specific CI supply the executable exhaustive search.

**Conclusion at the tested scope:** equal four-sum cardinality and a 24/24 first-x success do **not** imply all natural Q are representable on the actual special 31-point algebraic base. Direct S9(Q) extraction, even if perfect, cannot always succeed without moving to other public combinations R=aG+bQ. This says nothing yet about full index-calculus complexity.

## New mathematical normalization: why many generic 4+4 hits can mean few S9 algebraic roots

Let D_m(B)={d in Z^B: ||d||_1<=m and ||d||_1≡m (mod 2)}. Net signatures are exact cancellations of ±P_i; the equality follows by padding opposite-sign pairs. At B31:

- |D4|=**618,885**; |D8|=**5,654,193,417**.
- Every d in D8 has at least one ordered split d=u+v with u,v in D4; the number of these **integer** splits across every d is |D4|²=**383,018,643,225**.
- Thus the **exact combinatorial mean per d** is **67.74063336** ordered 4+4 splits for one integer eight-signature.
- Averaged across **all n elliptic-curve group points Q**, the exact mean number of *eight-net-signatures* mapping to Q is |D8|/n=**2.632984126**, while the mean number of ordered integer four-half pairs mapping to Q is |D4|²/n=**178.3600123**. These are exact counting means, not per-target measurements; they remain valid when distinct eight-net-signatures collide in the group.
- A Poisson map with mean 2.633 would suggest ~92.8% coverage; **Poisson occupancy is only a heuristic**, not a proof or empirical estimate. The observed Dickson 22/24 and first-x 24/24 are not precise population-coverage measurements.

An algorithm that reports 178 'solutions' per random Q by counting generic 4+4 pairs can overcount distinct signed roots by a factor of roughly 68 on global average. Moreover different eight-leaf words and factor polynomials f can share a net signature if opposite-sign cancellations are inserted. The actual S9/K–S5 algebraic solver needs original-ideal-certified **rational f and orientation**, not merely one generic group witness.

## New Rust experiment registered and checked in (execution status separate)

Prospective [protocol](S9_B31_MULTIPLICITY_PROTOCOL_20261116.md), [native Rust source](examples/prime31_s9_b31_signature_multiplicity.rs), and [pinned CI workflow](../../../.github/workflows/prime31-s9-net-multiplicity.yml) implement the missing diagnostic. It exhausts each 4A complement scan, handles noninjective four-signature/group maps without silently dropping any integer signature, deduplicates exact eight-signature vectors for **32 fresh** natural Q plus **12 blinded** planted Q and identity on each of the **two** bases, and re-adds every net signed witness to the original prime31 curve. The two explicit earlier misses are **separate retrospective regressions** and do not contaminate the fresh coverage denominator.

The new workflow [run 43957151566](https://github.com/isogeny-labs/crypto/actions/runs/43957151566) was queued at the time of this handoff; **no pass, new distribution, runtime, or improved solver claim** is assumed from the source alone. Exact source/pinned CI completion, per-Q multiplicities, collision accounting and independent PCC remain to be read before accepting a new experimental finding.

## Lead decision and next experiments

1. **Priority 1: algebraic f-from-Q extraction** in the *actual* Dickson31 base, not first-x31. Use [M8_RATIONAL_QUOTIENT_BRIDGE_20261116.md](M8_RATIONAL_QUOTIENT_BRIDGE_20261116.md) and Rust certified quotient/root machinery of [crypto #4228](https://github.com/isogeny-labs/crypto/pull/4228) / [#4349](https://github.com/isogeny-labs/crypto/pull/4349). Require original-ideal, y/sign/infinity/repeated-root correctness and recovery without scanning the 4A group table or 48,903,492 unordered degree-eight factor multisets. On the two known Dickson misses it must correctly return no rational eight-leaf solution; on blinded positives and fresh natural positive Q it must find at least one certified solution.
2. **Priority 2: algebraic cost and rank.** If f extraction works, use verified signed relations and incremental rank-32 Rust modular elimination to solve unknown log_G(Q) across 12+ blinded cases, recording all failed walks, base/quotient construction, solver and certification costs. Direct Q failures demand trial/retry logic; no hidden generator scalar or generic table.
3. **Priority 3: strongest-generic and external index-calculus controls.** Existing [48-Q prior result](APS_RHO_FOLD_SENSITIVITY_48Q_RESULT_20261116.md) measured APS cold **109.734280 ms**, unfolded rho64 **99.473873 ms** on one unisolated producer sample, with paired uncertainty crossing parity. The optimized native Wiener–Zuccherato look-ahead rho comparator and mature PKM/APS/F4/CAS same-host frontier **have not been completed**. Compare identical public Q, cold time, memory and the exact verified recovered k; include A/A controls, independence and PCC before ranking.
4. **Deprioritize unchanged negative leads:** anchored m6 F4 (sparse positive coverage and censored planted solving), generic S3 root pair-index retuning (only engineering), SA-02 Vieta-conic sieve (incorrect quadratic assumption; quartic/genus-one condition), SA-14b PKM-tower F4 degree growth (negative), SA-24c genus-two transfer (no advantage over rho and corrected linear-incidence elimination). SA-01b and SA-13b remain *inconclusive*, not kills. Sources: [crypto-autoresearcher summary](https://github.com/isogeny-labs/crypto-autoresearcher/blob/main/experiments/semaev-alternatives-20261105/SUMMARY.md) and [errata](https://github.com/isogeny-labs/crypto-autoresearcher/blob/main/experiments/semaev-alternatives-20261105/ERRATA.md), [cryptanalysis #1536](https://github.com/isogeny-labs/cryptanalysis/pull/1536).
5. **Published-prior-work rule:** PKM 2016 [DOI 10.1007/978-3-662-49387-8_1](https://doi.org/10.1007/978-3-662-49387-8_1), APS 2018 [DOI 10.1016/j.ffa.2018.01.009](https://doi.org/10.1016/j.ffa.2018.01.009). The currently complete APS \(S_3\) explicit pair enumerator remains generic MITM equivalent, not a novel non-generic attack.

Sources checked (KB): no research-KB MCP catalog or wrapper loaded in this execution; used GitHub original source, exact Actions logs/artifact, independent readback, two sibling repository research reports and publisher records. This is bounded prior-work review, not an exhaustive novelty claim. All new producer measurements remain [pcc:unverified] pending signed standing. No merge, paid compute or leaderboard promotion.
