"""v12: replay the step from a CUDA graph.

Each sigma for 65,536 rows moves only ~3 MB and does ~8.6M lane-ops, but
measures ~12 us, so the step is paying launch overhead across ~30 launches.
Fusing them into one kernel failed (v10: register spilling). A CUDA graph keeps
the launches and removes the per-launch submission cost: capture the sequence
once for a fixed shape, then replay it, copying the inputs into place first.

    python3 v12.py
"""
import json
import pathlib
import statistics
import sys

import torch

HERE = pathlib.Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import walk_step_problem as wsp  # noqa: E402
import kernel_v5 as K  # noqa: E402
import clmul_mul as CM  # noqa: E402
import v8  # noqa: E402
import v11  # noqa: E402  (sigma/frob kernels)

NSLOT = 14
SIGMA_R, SIGMA_W = 16, 1
FROB_R, FROB_W = 8, 1
MUL_B, MUL_W = 128, 4

_GRAPHS = {}


class Step:
    """One step for a fixed row count, captured as a CUDA graph."""

    def __init__(self, n, device, shapes=None):
        self.n = n
        self.sr, self.sw, self.fr, self.fw = shapes or (SIGMA_R, SIGMA_W, FROB_R, FROB_W)
        self.npad = ((n + MUL_B - 1) // MUL_B) * MUL_B
        self.L = self.npad * 3
        self.limbs = torch.empty((NSLOT * self.L,), dtype=torch.int64, device=device)
        self.x = torch.empty((n, 3), dtype=torch.int64, device=device)
        self.y = torch.empty((n, 3), dtype=torch.int64, device=device)
        self.out = torch.empty((self.npad, 2, 3), dtype=torch.int64, device=device)
        # warm up outside the capture (the table build and first kernel JIT
        # must not happen inside it)
        for _ in range(2):
            self._launch(device)
        torch.cuda.synchronize()
        self.graph = torch.cuda.CUDAGraph()
        with torch.cuda.graph(self.graph):
            self._launch(device)

    def _launch(self, device):
        n, npad, L = self.n, self.npad, self.L
        limbs, x, y, out = self.limbs, self.x, self.y, self.out
        table = K.get_table(device)
        limbs[0:L].view(npad, 3)[:n].copy_(x)
        limbs[L:2 * L].view(npad, 3)[:n].copy_(y)
        gs = (npad // self.sr + 1,)
        gf = (npad // self.fr + 1,)
        gm = (npad // MUL_B,)

        def mul(sa, sb, dst):
            CM._mul_clmad_kernel[gm](limbs[sa * L:], limbs[sb * L:], limbs[dst * L:],
                                     BLOCK=MUL_B, num_warps=MUL_W)

        def sig(src, dst, k):
            v11._sigma_slot[gs](limbs, table, src, dst, k, n, L, R=self.sr, num_warps=self.sw)

        v11._frob_slot[gf](limbs, table, n, L, R=self.fr, num_warps=self.fw)
        v8._xor_limbs[gs](limbs, 0, 2, 4, n, L, R=self.sr, num_warps=self.sw)
        v8._copy_limbs[gs](limbs, 4, 5, n, L, R=self.sr, num_warps=self.sw)
        acc, tmp = 5, 7
        for k in (1, 2, 4, 8, 16, 32):
            sig(acc, 6, k)
            mul(6, acc, tmp)
            acc, tmp = tmp, acc
        sig(acc, 6, 1)
        mul(6, 4, tmp)
        acc, tmp = tmp, acc
        sig(acc, 6, 65)
        mul(6, acc, tmp)
        acc, tmp = tmp, acc
        sig(acc, 6, 1)
        v8._xor_limbs[gs](limbs, 1, 3, 12, n, L, R=self.sr, num_warps=self.sw)
        mul(12, 6, 8)
        sig(8, 9, 1)
        v8._xor3_limbs[gs](limbs, 9, 8, 4, 9, n, L, R=self.sr, num_warps=self.sw)
        v8._xor_limbs[gs](limbs, 0, 9, 12, n, L, R=self.sr, num_warps=self.sw)
        mul(8, 12, 13)
        v8._xor3_limbs[gs](limbs, 13, 9, 1, 13, n, L, R=self.sr, num_warps=self.sw)
        out[:, 0] = limbs[9 * L:10 * L].view(npad, 3)
        out[:, 1] = limbs[13 * L:14 * L].view(npad, 3)

    def __call__(self, x, y):
        self.x.copy_(x)
        self.y.copy_(y)
        self.graph.replay()
        return self.out[:self.n]


def kernel_function(x, y):
    key = (x.shape[0], str(x.device))
    st = _GRAPHS.get(key)
    if st is None:
        st = Step(x.shape[0], x.device)
        _GRAPHS[key] = st
    return st(x, y)


def median_ms(fn, reps=30):
    fn()
    torch.cuda.synchronize()
    ts = []
    for _ in range(reps):
        e0, e1 = torch.cuda.Event(True), torch.cuda.Event(True)
        e0.record()
        fn()
        e1.record()
        torch.cuda.synchronize()
        ts.append(e0.elapsed_time(e1))
    return statistics.median(ts)


def main():
    torch.manual_seed(127)
    n = 1 << 16
    x = wsp._random_elements(n).cuda()
    y = wsp._random_elements(n).cuda()
    rep = {"gpu": torch.cuda.get_device_name(), "rows": n}
    want = K.kernel_function(x, y)
    got = kernel_function(x, y)
    rep["exact"] = bool(torch.equal(got, want))

    best = None
    for shapes in ((16, 1, 8, 1), (8, 1, 8, 1), (32, 1, 8, 1), (16, 1, 16, 1),
                   (32, 1, 16, 1), (8, 1, 4, 1), (64, 1, 8, 1), (16, 2, 8, 1)):
        st = Step(n, x.device, shapes=shapes)
        ok = bool(torch.equal(st(x, y), want))
        ms = median_ms(lambda _s=st: _s(x, y))
        rec = {"shapes": list(shapes), "exact": ok, "ms": ms, "updates_per_s": n / ms * 1e3}
        print(json.dumps(rec), flush=True)
        rep.setdefault("variants", []).append(rec)
        if ok and (best is None or rec["ms"] < best["ms"]):
            best = rec
    rep["best"] = best
    _GRAPHS.clear()
    ms = median_ms(lambda: kernel_function(x, y))
    rep["graph_ms"] = ms
    rep["graph_updates_per_s"] = n / ms * 1e3

    # a second, different input must also be right (the graph replays, the
    # copies change)
    x2 = wsp._random_elements(n).cuda()
    y2 = wsp._random_elements(n).cuda()
    rep["exact_second_input"] = bool(torch.equal(kernel_function(x2, y2), K.kernel_function(x2, y2)))

    m = 40_001
    rep["exact_odd"] = bool(torch.equal(kernel_function(x[:m].contiguous(), y[:m].contiguous()),
                                        K.kernel_function(x[:m].contiguous(), y[:m].contiguous())))
    print(json.dumps(rep))
    (pathlib.Path("/results") if pathlib.Path("/results").is_dir() else HERE).joinpath("v12.json").write_text(
        json.dumps(rep, indent=2))
    return 0 if all(rep[k] for k in ("exact", "exact_second_input", "exact_odd")) else 1


if __name__ == "__main__":
    raise SystemExit(main())
