# Reproduction and measured scope

Run `sage -python research/ghs_n7_traceone_genus8_20261115/screen.py` from the repository root. It writes `screen.json` after exhaustive q=2 and q=4 finite-field checks. The first output, before the absolute-subfield sieve amendment, remains in `screen_before_field_sieve.json`.

Run `python3 research/ghs_n7_traceone_genus8_20261115/run_map8.py` to submit the one public curve to the Magma calculator. The successful [receipt](map8_receipt.json) reports HTTP 200, Magma 2.30-1, exit code zero, no truncation, seed 2500227443, 0.35-second Magma service time, and an exact order-71 image. The input is `map8.magma` with SHA-256 `df6116d140fe77b5001f89268dbee146fdf46ea27b3114ca8d67ddbf15177923`. Only public curve parameters were sent. The receipt checks the map and a bounded target-subgroup scalar recovery; it does not time index calculus or rho.
