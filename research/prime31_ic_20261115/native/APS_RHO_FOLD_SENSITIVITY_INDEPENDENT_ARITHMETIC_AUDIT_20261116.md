# Independent arithmetic replay of the frozen 48-Q APS versus folded/unfolded rho receipts

## Scope (2026-11-16; producer data independently recomputed in this session)

Input: unmodified [240-case raw Rust JSON](APS_RHO_FOLD_SENSITIVITY_48Q_EXACT_CI_20261116.json) at SHA-1 blob recorded with the GitHub source-specific program. GitHub Actions source: [run 43921120382](https://github.com/isogeny-labs/crypto/actions/runs/43921120382), frozen source `aed18cf91172a74d76bcab2a7530aa656dc61ee2`.

An **independent arithmetic replay** was executed in a separate tool/runtime without invoking the Rust example's scalar multiplication, Semaev verifier or result summaries: JavaScript BigInt short-Weierstrass affine arithmetic over p=2147483647 with a=847514846, b=2087444724, G=(625037715,2100627251), subgroup order n=2147446831. Each of 48 caller-known k values reproduced the archived Q by scalar multiplication, and all five solver-recovered scalar values equaled k. For every APS case, the recorded final signed relation satisfied (alpha + beta*k) mod n = 0. Every arm's cold nanosecond total was recomputed from its 48 individual receipts; paired APS wins were recounted.

| Independent check | Result |
|---|---:|
| Public Q=[k]G from separate BigInt group formulas | 48 / 48 |
| All five arms' reported complete scalar recoveries equal caller k | 240 / 240 |
| APS signed congruences (alpha+beta*k) mod n = 0 | 48 / 48 |
| Total cold wall recomputed from individual case rows | APS 109734280 ns; folded32 213126270 ns; unfolded32 101453752 ns; unfolded64 99473873 ns; unfolded32 A/A 101399922 ns |
| Paired APS wins vs folded32/unfolded32/unfolded64 | 37 / 21 / 23 out of 48 |
| Mismatched data fields, outputs, or congruences | **0** |

The source-exact JSON includes its own success counts, so a summary copied uncritically would not be independent verification. Here the totals and identities were recomputed from **all cases** with an independent implementation of field inversion, group addition and double-and-add. The replay is separate arithmetic checking, **not** independent cryptographic custody, isolated-host timing reproduction, proof of honest random target selection or a keyring-signed PCC verdict. It does not convert the unisolated producer timing into an externally validated performance ranking. [pcc:unverified]

Interpretation remains unchanged: 48-Q APS is slower than both unfolded rho controls in aggregate; bootstrap uncertainty overlaps parity. This audit validates the recorded arithmetic and aggregation, not algorithmic novelty, optimized rho supremacy or general ECDLP security claims.
