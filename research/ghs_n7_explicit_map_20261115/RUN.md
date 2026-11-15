# Magma map and parameter-scan receipt

15 November 2026 UTC. The three successful calls used the public `https://calculator.magma-maths.org/execute` service and Magma 2.30-1. Only the exact public curve equation and bounded scripts in this directory were submitted. No paid worker, credential, private point or scalar was used. Inputs and outputs are retained in full.

| Stage | Command from repository root | HTTP | Magma success/exit | Seed | Magma reported time |
|:---|:---|---:|:---|---:|---:|
| pilot | `python3 research/ghs_n7_explicit_map_20261115/run_pilot.py` | 200 | true/0, untruncated | 2500231478 | 0.13 s |
| full c scan | `python3 research/ghs_n7_explicit_map_20261115/run_scan.py` | 200 | true/0, untruncated | 2500231419 | 0.20 s |
| genus-14 map | `python3 research/ghs_n7_explicit_map_20261115/run_map14.py` | 200 | true/0, untruncated | 2500231358 | 0.75 s |

The raw `pilot_receipt_ssl_failure.json` records the local CA-chain failure before the `certifi` remedy. `scan_receipt_unhandled_c.json` records the first scan's ineligible parameter error before the protocol-preserved `try/catch` correction. `python3 research/ghs_n7_explicit_map_20261115/summarize_scan.py` validated the full 127-row response and generated `scan_summary.json`. `python3 research/ghs_n7_explicit_map_20261115/kernel_bound.py > research/ghs_n7_explicit_map_20261115/kernel_bound.txt` checked the selected c's Frobenius annihilator and kernel-bound inputs.

SHA-256 digests:

- `pilot.magma`: `e910482ee42fb6ece643051f19e4e69fdbb944610ccd16f66b28c6c20628eecc`
- `scan.magma`: `abd6b0f453d3a961b335dd234e963d0d7e00db139535a856a80907b890bd3f88`
- `map14.magma`: `7fb785ae34fcd6c04a314bd51d2933fb3da4999462750592882567800f55ed31`
- `pilot_receipt.json`: `e5ff057030bb7018c8b5ece82e69836ac38fb24e537229c84feb045e9364d0b2`
- `scan_receipt.json`: `7530fa3e15cd4f5d78388e1baf89391e60380efe1fa891bc8dc95aec21ba079e`
- `map14_receipt.json`: `237f5a55ee4f04d46d9ebcd8cf851b3ead040d36ee94a6526e62365d9f56e11b`
- `kernel_bound.py`: `f552908d92d259919a2e933e2bfa013e1bad1d6c7402d4c980f14943fb2fce02`
- `kernel_bound.txt`: `eb2ba31578b17a76bfb61a302e91d4066eebeb63dc5e8c192d3f333e5f37dc3e`
- `scan_summary.json`: `217f3aa3c66c2971a54168bc1c699c8f59af4cd0eee97ca4b11c297af37701e6`

The reported 0.13/0.20/0.75 seconds are Magma service timings for the construction and scan, **not** an index-calculus or rho benchmark. [pcc:unverified]
