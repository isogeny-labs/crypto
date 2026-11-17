#!/usr/bin/env python3
"""Run one cell of lfd-summation-systems' split-m closure and count its
float64 matrix-product operations, without editing that repository.

    closure_wrapper.py LFD_ROOT OUT_DIR -- cell --m 4 --k 2 --primes 7681 --seeds 1

What is counted.  The engine (csd/ic_bounds_m3_batched.py BatchedClosure,
csd/ic_bounds_split_m.py InPlaceClosure) does its arithmetic in two exact
float64 products per batch:

    _reduce:  P - P[:, piv] @ B          (rows x rank) @ (rank x n)
    _absorb:  B - B[:, newpiv] @ Enew    (rank x s)    @ (s x n)

A dense product (r x s) @ (s x n) executes r*s*n fused multiply-adds, whatever
the sparsity of its operands.  The wrapper replaces the two methods with
versions that add r*s*n from the shapes before calling the original, and time
the call.  Not counted: np.mod, the int64 echelon step (csd/_elim.c, which
has its own row-operation counter), the RREF among new rows, product-row
assembly, and everything outside the engine.  The counted number is therefore
the BLAS work only, and `kernel_seconds` (time inside the two methods)
contains uncounted work: the rate is a lower bound on the products' own rate.

With OPTLEVEL_REPLAY=N (N >= 1 passes) the wrapper does not report the engine.  After the run
it replays the recorded product shapes as dense float64 products of random
residues in [0, p) and reports those: the same operation count, with
`kernel_seconds` = time inside the products alone.  That is the BLAS library's
rate on these shapes, without the engine around it.

Outputs go to OUT_DIR (the script's OUT is redirected there); the compiled
echelon library goes to OUT_DIR/csd-cache.  The last stdout line is an
optlevel.ops/v1 record.
"""
import importlib.util
import json
import os
import pathlib
import sys
import time

root, out = pathlib.Path(sys.argv[1]).resolve(), pathlib.Path(sys.argv[2]).resolve()
argv = sys.argv[sys.argv.index("--") + 1:]
out.mkdir(parents=True, exist_ok=True)
os.environ.setdefault("CSD_CACHE", str(out / "csd-cache"))
sys.dont_write_bytecode = True
sys.path.insert(0, str(root))
spec = importlib.util.spec_from_file_location("run_ic_split_m", root / "scripts" / "run_ic_split_m.py")
R = importlib.util.module_from_spec(spec)
spec.loader.exec_module(R)
R.OUT = out / "records" / str(os.getpid())                      # never write into the sibling repository

from csd import ic_bounds_m3_batched as BM   # noqa: E402
from csd import ic_bounds_split_m as SM      # noqa: E402

SHAPES = []
C = {"fma": 0, "seconds": 0.0, "products": 0, "max_basis_bytes": 0, "traffic_min_bytes": 0, "max_product": [0, 0, 0]}


def note(r, s, n):
    if r and s and n:
        C["fma"] += r * s * n
        C["products"] += 1
        SHAPES.append((r, s, n))
        C["traffic_min_bytes"] += 8 * (r * s + s * n + 2 * r * n)   # each operand read once, the result read and written
        if r * s * n > C["max_product"][0] * C["max_product"][1] * C["max_product"][2]:
            C["max_product"] = [r, s, n]


def wrap_reduce(orig):
    def f(self, P):
        t = time.perf_counter()
        note(len(P), len(self.piv), self.n)
        try:
            return orig(self, P)
        finally:
            C["seconds"] += time.perf_counter() - t
    return f


def wrap_absorb(orig):
    def f(self, R_):
        t = time.perf_counter()
        rows, rank = self.B.shape[0], len(self.piv)
        try:
            return orig(self, R_)
        finally:
            note(rows, len(self.piv) - rank, self.n)
            C["max_basis_bytes"] = max(C["max_basis_bytes"], self.B.nbytes)
            C["seconds"] += time.perf_counter() - t
    return f


BM.BatchedClosure._reduce = wrap_reduce(BM.BatchedClosure._reduce)
BM.BatchedClosure._absorb = wrap_absorb(BM.BatchedClosure._absorb)
SM.InPlaceClosure._absorb = wrap_absorb(SM.InPlaceClosure.__dict__["_absorb"])

sys.argv = ["run_ic_split_m.py", *argv]
t0 = time.perf_counter()
try:
    R.main()
except SystemExit as e:
    if e.code not in (0, None):
        raise
wall = time.perf_counter() - t0
mode = "engine"
passes = int(os.environ.get("OPTLEVEL_REPLAY") or 0)
if passes:
    import numpy as np
    mode = "blas-replay"
    rng = np.random.default_rng(1)
    made, secs = {}, 0.0
    for shp in SHAPES * passes:
        if shp not in made:
            r, s_, n = shp
            made = {shp: (rng.integers(0, 7681, (r, s_)).astype(np.float64), rng.integers(0, 7681, (s_, n)).astype(np.float64))}
            made[shp][0] @ made[shp][1]                  # first touch of the result pages, untimed
        t = time.perf_counter()
        made[shp][0] @ made[shp][1]
        secs += time.perf_counter() - t
    C["seconds"] = secs
    C["fma"] *= passes
    C["traffic_min_bytes"] *= passes
print(json.dumps({"schema": "optlevel.ops/v1", "ops": C["fma"], "op_class": "fma_f64", "kernel_seconds": C["seconds"],
                  "bytes_declared": C["traffic_min_bytes"], "working_set_bytes": 2 * C["max_basis_bytes"],
                  "mode": mode, "products": C["products"], "largest_product_r_s_n": C["max_product"], "python_wall_seconds": wall,
                  "blas_threads_env": os.environ.get("OPENBLAS_NUM_THREADS")}))
