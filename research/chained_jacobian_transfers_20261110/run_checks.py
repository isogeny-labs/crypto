#!/usr/bin/env python3
"""Run reproducible local transfer smoke checks with explicit failure propagation."""
import subprocess,sys,time
for name in ("verify.py","verify_nonisomorphic.py","verify_fibers.py","cantor.py","test_maps.py"):
    start=time.perf_counter()
    result=subprocess.run([sys.executable,name],check=False)
    if result.returncode:
        raise SystemExit(f"FAIL {name} exit={result.returncode}")
    print(f"PASS {name}: {time.perf_counter()-start:.3f}s",flush=True)
print("ALL LOCAL SMOKE CHECKS PASSED; not an end-to-end solver benchmark")
