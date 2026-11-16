# Historical receipts inspected during the audit

These are copies of existing producer/reviewer receipts, not fresh replays.
The native checkers in the parent directory are the fresh diagnostics.

| Copy | Source |
| --- | --- |
| `pkm-torus-verify.txt` | crypto `research/pkm_dickson_torus_20261015/verify.txt` |
| `pkm-torus-xcheck-summary.txt` | crypto `research/pkm_dickson_torus_20261015/xcheck/summary.txt` |
| `aux-isogeny-singular-summary.json` | crypto-autoresearcher `experiments/EXP-ECDLP-f49f75/reports/review-DEC-20261015-a08389/summary.json` |
| `aux-isogeny-msolve-summary.json` | crypto-autoresearcher `experiments/EXP-ECDLP-502ae2/reports/review-DEC-20261015-ab6f23/summary.json` |

The main report and `evidence-summary.json` pin the source commits.
`source-sha256.txt` hashes the inspected source files. Local paths in the
hash record identify the checkout used; portable repository links are in
the report. Existing solver timing ratios are not independently admitted
by this correctness audit.
