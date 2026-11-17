# N83 four-family hosted replay audit

The independent [GitHub Actions run 39096459511](https://github.com/aburan28/crypto/actions/runs/39096459511), attempt 1, completed all four strata successfully. The [sealed audit](receipts/run_39096459511/audit.json) reports `PASS_FOUR_INDEPENDENT_HOSTED_REPLAYS`. Each stratum ran on a distinct hosted runner, separate from all four original provider runners, and re-executed the exact source verifier at commit `577c8cbe9ddfa20a2b6a62d8c39eb1dd339497ed` over its archived provider results.

The [dispatch](dispatch.json), SHA-256 `7a9addb02606f8b3f3202c50900bd0c001ff6d034adc6a1e06bb30e85f15f894`, freezes provider run `39075872898` attempt 1, the four provider artifact IDs and ZIP hashes, member coverage, and raw analysis hashes. The hosted workflow ran at head `7f3a0bf3c7feaa54ecb3c5e497dd2022de8e1ab2`. The audit binds that head and completed GitHub run to [job metadata](receipts/run_39096459511/jobs.json), [artifact metadata](receipts/run_39096459511/artifacts.json), the four archived [replay ZIPs](receipts/run_39096459511/), and each receipt's source, runner, provider, and dispatch fields. All four verifier exits are zero, stderr is empty, and each full replayed JSON equals both the ZIP-bound provider `analysis.json` and the prior local independent replay output. The committed [run metadata](receipts/run_39096459511/run.json) records the completed success state.

| Stratum | Hosted runner | Hosted artifact ID | Hosted ZIP SHA-256 |
| ---: | --- | ---: | --- |
| 0 | GitHub Actions 1000338504 | 12125970452 | `5239efd5229be133448a0249df3be0817f0ce88d420de7d94d424fa5689ff491` |
| 1 | GitHub Actions 1000338489 | 12125353610 | `59d8004e3cf2368c21f197b013b0dec3cfc4da36af78b9cc03585c9b4c88f217` |
| 2 | GitHub Actions 1000338611 | 12125054614 | `c138e5e2c81fb31cc112e50dca5bd33912264d14ab5aef4960352c73935792a6` |
| 3 | GitHub Actions 1000338522 | 12126050163 | `e75450edcae9df2c4d3c531a2d62858bde6461f4f8097d72b80de3bd88e039dc` |

The [audit controls](validation/hosted-audit-controls.json) reject a forged successful run state, a one-bit artifact mutation, and a forged replay status even when its artifact size and SHA-256 metadata are updated consistently. Before the final stratum completed, the audit rejected the incomplete run state. `py_compile` passed for the audit program, and the N83 Python discovery suite passed 63 tests. The replay adds provenance validation, not timing samples; the original [24-point runtime selection](https://github.com/aburan28/crypto/pull/2123) remains the source of the within-panel K2816 cold-process verdict. Its online verdict is `BOUNDED_UNKNOWN`, and `selected_best_total_runtime` for the broader factor-base sweep remains null.
