# Earlier dg/srenew wall blocks, opened after the verdict commit (§8, E7)

Opened after f1e65b1c74 (VERDICT.md) was pushed. Descriptive only: these sessions exited 42 without native A/A admission; they are never pooled and decide nothing. Values are copied from the `wall` block of each `compare-srenew__dg-w0.json` in #2845 (`research/ic_srenew_deferred_guard_20261029/`).

| study | b | `wall.status` | `aa_selection` | `aa_arms` | `corrected_ratio` | `corrected_ci95` | file sha256 |
|:--|--:|:--|:--|:--|:--|:--|:--|
| first-study | 24 | admitted | includes the reference arm | rho-neg, rho-neg-aa | None | None | `e3b0f5e00a72e2ca…` |
| first-study | 28 | admitted | includes arm A | srenew, srenew-aa | 0.8757824971846615 | [0.8721888680376135, 0.8793909329532105] | `f7385a95ac26d86e…` |
| first-study | 32 | admitted | includes the reference arm | rho-neg, rho-neg-aa | 0.8455550671483589 | [0.8414034223103906, 0.8497271969931666] | `0ae3bc4b19c3a28c…` |
| v3 | 24 | admitted | includes the reference arm | rho-neg, rho-neg-aa | None | None | `6d1af160e4f6081c…` |
| v3 | 28 | admitted | includes arm A | srenew, srenew-aa | 0.8721588720909466 | [0.8701333423973179, 0.8741891168901113] | `0ed2531089606455…` |
| v3 | 32 | admitted | includes the reference arm | rho-neg, rho-neg-aa | 0.8257843105605903 | [0.8225521841988255, 0.8290291371996373] | `eda003cf56178881…` |
