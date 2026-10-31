# Direct order-check execution attempts

23 October 2026. Source: `transfer_order.gp`, fixed to recompute exactly the published and `b=301` auxiliary curve orders, assert `gcd=1` and `256|#E'`. Intended envelope: 30 s wall and 2 GiB memory. The deterministic `check_transfer_obstruction.py` check succeeded independently of this additional GP verification.

1. `python3 run_bounded.py --wall-seconds 30 --rss-mib 2048 ... -- gp -q transfer_order.gp` raised `PermissionError: [Errno 1] Operation not permitted: 'ps'` in the watchdog before producing a receipt. The empty `runs/transfer-order.stdout` and `.stderr` files are retained. This attempt supplies no bounded result.
2. A Python subprocess with `timeout=30` and `RLIMIT_AS=2 GiB` raised `SubprocessError: Exception occurred in preexec_fn` before a child result; this environment rejected that memory-limit route. It produced no raw child output.
3. `gtimeout 30 /usr/bin/time -l gp -q transfer_order.gp` printed both exact orders, `GCD 1`, and `TRANSFER_ORDER_OK` to `runs/transfer-order-attempt3.stdout`. `/usr/bin/time -l` returned exit 1 after reporting `time: sysctl kern.clockrate: Operation not permitted`; its stderr is retained. The GP assertions succeeded, but the wrapper did not finish cleanly.
4. `gtimeout 30 gp -q transfer_order.gp` exited 0. Its stdout is `runs/transfer-order-attempt4.stdout`, and its stderr is empty in `runs/transfer-order-attempt4.stderr`. It checked both exact orders and `GCD 1` under a 30 s wall timeout. No memory cap or measured peak RSS is asserted for this attempt.

The mathematical deduction uses the exact orders and elementary image-order divisibility. The successful attempt confirms them directly in PARI/GP; the sandbox limitation affects resource accounting for this extra verification only, not the previously retained auxiliary search and screen receipts.
