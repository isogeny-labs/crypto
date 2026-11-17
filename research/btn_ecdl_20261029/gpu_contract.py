"""GPU backend for the relation-network contraction (CuPy, NumPy fallback).

Same counting as btn.contract, restructured for a device:

* the elimination order and einsum strings are compiled once on the host
  (min-fill over the network after constants are sliced out);
* cutset variables become a batch axis "Z" instead of a Python loop, so the
  2^k conditioned contractions run as one batched einsum per step, chunked to
  fit a memory budget;
* tables are float64 on the device (cuBLAS DGEMM); counts are exact while
  every partial sum stays below 2^53, which run() checks.

Usage:
  python3 gpu_contract.py --selftest              # n=5 vs btn.contract
  python3 gpu_contract.py --n 7 --cut 7 --brute   # n=7 count vs brute force
  python3 gpu_contract.py --backend numpy ...     # force the CPU path
"""
import argparse
import random
import sys
import time

import numpy as np

from btn import simplify

LET = "abcdefghijklmnopqrstuvwxyABCDEFGHIJKLMNOPQRSTUVWXY"  # "z"/"Z" kept for the batch axis
BATCH = "Z"
EXACT = float(2 ** 53)


def backend(name="auto"):
    if name in ("auto", "cupy"):
        try:
            import cupy as cp
            cp.cuda.runtime.getDeviceCount()
            return cp, "cupy"
        except Exception:
            if name == "cupy":
                raise
    return np, "numpy"


class Plan:
    """Elimination schedule for one network structure and one cutset."""

    def __init__(self, nv, cons, cut):
        facs, pinned = simplify(cons)
        self.empty = facs is None
        if self.empty:
            return
        cut = [v for v in cut if v not in pinned]
        self.cut = cut
        cutset = set(cut)
        alive = set()
        for s, _ in facs:
            alive |= set(s)
        self.free_scalar = 2 ** (nv - len(alive) - len(pinned) - len(cutset - alive))
        # initial factors: scope without cut vars, plus which cut vars they carry
        self.init = []
        for s, t in facs:
            self.init.append((tuple(v for v in s if v not in cutset),
                              tuple(v for v in s if v in cutset), s, t))
        elim = alive - cutset
        adj = {v: set() for v in elim}
        for s, _, _, _ in self.init:
            for v in s:
                adj[v] |= set(s) - {v}
        scopes = [s for s, _, _, _ in self.init]
        live = list(range(len(scopes)))
        self.steps, self.width = [], 0
        while elim:
            def fill(u):
                nb = list(adj[u])
                return sum(1 for i in range(len(nb)) for j in range(i) if nb[j] not in adj[nb[i]])
            cand = sorted(elim, key=lambda u: len(adj[u]))[:40]
            v = min(cand, key=lambda u: (fill(u), len(adj[u])))
            elim.discard(v)
            ins = [k for k in live if v in scopes[k]]
            live = [k for k in live if k not in ins]
            vs = []
            for k in ins:
                for u in scopes[k]:
                    if u not in vs:
                        vs.append(u)
            out = tuple(u for u in vs if u != v)
            if len(vs) > len(LET):
                raise ValueError(f"bucket of {len(vs)} variables exceeds einsum letters")
            self.width = max(self.width, len(vs))
            m = {u: LET[i] for i, u in enumerate(vs)}
            self.steps.append((ins, "".join(m[u] for u in out), [("".join(m[u] for u in scopes[k])) for k in ins]))
            for u in out:
                adj[u] |= set(out) - {u}
                adj[u].discard(v)
            del adj[v]
            scopes.append(out)
            live.append(len(scopes) - 1)
        self.final = live  # factors left with empty scope

    def peak_bytes(self, batch):
        return 8 * batch * 2 ** self.width


def _tables(plan, xp, assignments):
    """Initial device tables for a chunk of cut assignments (B x k ints)."""
    B = len(assignments)
    col = {v: i for i, v in enumerate(plan.cut)}
    out = []
    for scope, cuts, full, t in plan.init:
        if not cuts:
            out.append((xp.asarray(t, dtype=xp.float64), False))
            continue
        axes = [full.index(v) for v in cuts]
        keep = [i for i in range(len(full)) if i not in axes]
        tt = np.transpose(t, axes + keep).reshape((2 ** len(cuts),) + (2,) * len(keep))
        idx = np.zeros(B, dtype=np.int64)
        for j, v in enumerate(cuts):  # flat index in C order of the cut axes
            idx = idx * 2 + assignments[:, col[v]]
        out.append((xp.asarray(tt[idx], dtype=xp.float64), True))
    return out


def run(plan, xp, assignments):
    """Sum of contractions over the given cut assignments (array B x k of 0/1)."""
    if plan.empty:
        return 0
    tabs = _tables(plan, xp, assignments)
    for ins, out, subs in plan.steps:
        ops, terms, batched = [], [], False
        for k, sub in zip(ins, subs):
            t, b = tabs[k]
            ops.append(t)
            terms.append((BATCH if b else "") + sub)
            batched |= b
        res = xp.einsum(",".join(terms) + "->" + (BATCH if batched else "") + out, *ops)
        tabs.append((res, batched))
    total = xp.ones(len(assignments), dtype=xp.float64)
    for k in plan.final:
        t, b = tabs[k]
        total = total * t
    if float(total.max()) >= EXACT:
        raise OverflowError("count reached 2^53; float64 no longer exact")
    s = float(total.sum())
    if s >= EXACT:
        raise OverflowError("count reached 2^53; float64 no longer exact")
    return int(round(s)) * plan.free_scalar


def contract_gpu(nv, cons, cut=(), xp=None, mem_gb=8.0):
    """Count satisfying assignments, batching all 2^|cut| cut assignments."""
    xp = xp if xp is not None else backend()[0]
    plan = Plan(nv, cons, list(cut))
    if plan.empty:
        return 0
    k = len(plan.cut)
    allas = ((np.arange(2 ** k)[:, None] >> np.arange(k)[None, :]) & 1).astype(np.int64)
    if k == 0:
        allas = np.zeros((1, 0), dtype=np.int64)
    per = max(1, int(mem_gb * 2 ** 30 // max(1, plan.peak_bytes(1))))
    total = 0
    for i in range(0, len(allas), per):
        total += run(plan, xp, allas[i:i + per])
    return total


def main():
    from btn import Field, Curve, contract
    from relation import build
    ap = argparse.ArgumentParser()
    ap.add_argument("--n", type=int, default=5)
    ap.add_argument("--cut", type=int, default=0, help="condition on the first CUT bits of x1")
    ap.add_argument("--R", type=int, nargs=2)
    ap.add_argument("--seed", type=int, default=3)
    ap.add_argument("--backend", default="auto", choices=["auto", "cupy", "numpy"])
    ap.add_argument("--mem-gb", type=float, default=8.0)
    ap.add_argument("--brute", action="store_true")
    ap.add_argument("--selftest", action="store_true")
    a = ap.parse_args()
    xp, name = backend(a.backend)
    polys = {5: 0b100101, 7: (1 << 7) | 3, 9: (1 << 9) | (1 << 4) | 1}
    if a.selftest:
        a.n = 5
    F = Field(a.n, polys[a.n]); C = Curve(F)
    pts = [(x, y) for x in range(1 << a.n) for y in range(1 << a.n) if C.on((x, y))]
    rng = random.Random(a.seed)
    full = [1] * (1 << a.n)
    Rs = [tuple(a.R)] if a.R else [rng.choice(pts) for _ in range(3 if a.selftest else 1)]
    ok = True
    for R in Rs:
        net, iface, _, _ = build(F, R, (full, full))
        plan = Plan(net.nv, net.cons, iface[:a.cut])
        t = time.time()
        got = contract_gpu(net.nv, net.cons, iface[:a.cut], xp, a.mem_gb)
        dt = time.time() - t
        line = f"backend={name} n={a.n} R={R} cut={a.cut} width={plan.width} count={got} {dt:.1f}s"
        if a.selftest:
            ref = contract(net.nv, net.cons)
            cut_ref = contract_gpu(net.nv, net.cons, iface[:3], xp, a.mem_gb)
            line += f" ref={ref} cut3={cut_ref}"
            ok &= got == ref == cut_ref
        if a.brute:
            brute = sum(1 for A in pts for B in pts if A[0] != B[0] and C.add(A, B) == R)
            line += f" brute={brute}"
            ok &= got == brute
        print(line, "OK" if ok else "MISMATCH", flush=True)
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
