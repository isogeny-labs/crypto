# R3 archived raw-output manifest

The local evidence archive is delivered with the run response as `ic_pair_schedule_20261118.zip`. GitHub branch contains the exact independently tested **producer, auditor, summarizer and frozen protocol** (all Git blob IDs independently matched). This PR **does not contain** the ~1 MB full raw witness JSON because the sandbox's `github.com` DNS was inaccessible and the connector cannot ingest the local artifact bytes directly; never treat summary claims here as a signed evidence promotion.

| File in artifact archive | SHA-256 (2026-11-18 UTC) |
|---|---|
| `PROTOCOL.md` | `5199afb75054606659da5c7428f0f5e067830aa7b4507ab4758ce2323537464e` |
| `run.py` | `52162b421591330d793df1f7f6c26bb8f6ce662b1c34eabd34241639e3cc89ff` |
| `audit.py` | `c47a94bce9e8fd0bce319467a63774589d6edbf4bded4881920cb1870b737028` |
| `summarize.py` | `54cad2bccb6613eda0519d3e09c58fe6d39ed721f54dc5963263091494e395f3` |
| `raw/B16.json` | `3eb3dc41c2af634c8556d2f0e1b74a1ead6196d23f82b4134574f95150f325c2` |
| `raw/B32.json` | `dfbd2d7b80b281f3a65b5e9d65b4dfe5115068ffba2c759bd84586087aefa263` |
| `SUMMARY.json` | `a2ca336e0125dd6348a7db3011d413f81d02efb9f181895be8f7fb5700779800` |

ZIP SHA-256: `e7fcbab259885acd0c25e6d474ab3f65fc8325281d237784e9a33ad6de35939f`. Full archive also contains failed and corrected independent audit logs, host details, exact commands, R2 seed replay, mutation-control failure, A/A outputs and the next-experiment queue. The R2 comparison in `summarize.py` requires the prior `ic_partial_pairtable_20261118.zip` at `/mnt/data/`; the **run.py/audit.py** reproduction does not depend on that archive.

Verification: `python3 audit.py raw/B16.json raw/B32.json` must output 96 arms / 6 censored / 0 failures and 96 arms / 0 censored / 0 failures, respectively; 48/48 exact rho traces rechecked. Reject any incorrect scalar, incomplete sample, protocol change or mismatched hash. Do not label this PCC verified.
