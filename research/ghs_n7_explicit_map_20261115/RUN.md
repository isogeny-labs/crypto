# Magma map and parameter-scan receipt

15 November 2026 UTC. The calls used the public `https://calculator.magma-maths.org/execute` service and Magma 2.30-1. Only the exact public curve equation and bounded scripts in this directory were submitted. No paid worker, credential, private point or scalar was used. Inputs and outputs, including the failed class-normalization attempt, are retained in full.

| Stage | Command from repository root | HTTP | Magma success/exit | Seed | Magma reported time |
|:---|:---|---:|:---|---:|---:|
| pilot | `python3 research/ghs_n7_explicit_map_20261115/run_pilot.py` | 200 | true/0, untruncated | 2500231478 | 0.13 s |
| full c scan | `python3 research/ghs_n7_explicit_map_20261115/run_scan.py` | 200 | true/0, untruncated | 2500231419 | 0.20 s |
| genus-14 map | `python3 research/ghs_n7_explicit_map_20261115/run_map14.py` | 200 | true/0, untruncated | 2500231358 | 0.75 s |
| initial class check | `python3 research/ghs_n7_explicit_map_20261115/run_map14_class.py` with `map14_class_origin.magma` | 200 | false/0, degree 56 | 1811450755 | 3.96 s |
| finite-point class check | `python3 research/ghs_n7_explicit_map_20261115/run_map14_class.py` | 200 | true/0, class order 23 | 1811450818 | 4.129 s |

The raw `pilot_receipt_ssl_failure.json` records the local CA-chain failure before the `certifi` remedy. `scan_receipt_unhandled_c.json` records the first scan's ineligible parameter error before the protocol-preserved `try/catch` correction. `python3 research/ghs_n7_explicit_map_20261115/summarize_scan.py` validated the full 127-row response and generated `scan_summary.json`. `python3 research/ghs_n7_explicit_map_20261115/kernel_bound.py > research/ghs_n7_explicit_map_20261115/kernel_bound.txt` checked the selected c's Frobenius annihilator and kernel-bound inputs.

The first class check used `h(P)-h(O)` and produced a divisor of degree 56, leaving a free degree coordinate. Its input is preserved as `map14_class_origin.magma` and its failed response as `map14_class_receipt.json`. The second check used `h(2P)-h(P)`, produced degree zero and an exact order-23 class in a class group with torsion order 16,560. The response is `map14_class_finite_receipt.json`. These calculator timings cover class-group construction and map evaluation; they are not a full index-calculus versus rho comparison.

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
- `map14_class_origin.magma`: `a436caac53f586b4f61f9cc5a9a87320a984ce5d357b95d30bdc5f659058bc0f`
- `map14_class.magma`: `f6e7e839c2fa9d956e236866b5777fe0884f18c18d68238da7b4c19f2c899ade`
- `map14_class_receipt.json`: `9f74cb4571f388d7e3615d1745e2c720bfefcb67657bcf9bad4ad971a7f4661c`
- `map14_class_finite_receipt.json`: `7edea010b610d612c0025e7cacba249b5d1b99bd48a518cc2c1cc58ad099292e`
- `run_map14_class.py`: `1f5118862bb2091058af898a722bb0badb84aebacad81442bfceb6c48b24a42d`

The reported 0.13/0.20/0.75 seconds are Magma service timings for the construction and scan, **not** an index-calculus or rho benchmark. [pcc:unverified]
