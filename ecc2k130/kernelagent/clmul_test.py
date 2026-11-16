"""Carry-less multiply from Triton: does it work, and what does it cost.

The reference product is a cyclic convolution mod 2 over Z/263 — a polynomial
multiply mod (x^263 - 1) over GF(2), which is what the production client does
with CLMAD. Triton has no primitive for it; PTX does, and
tl.inline_asm_elementwise can emit it.

This checks the two pieces that decide whether the fast product is reachable:
  1. 64x64 carry-less multiply (pclmulqdq) vs a Python reference, and its rate.
  2. a 263-bit cyclic polynomial product (5 limbs x 5 limbs, 25 clmuls, fold
     mod x^263-1) vs a Python reference, and its rate per element.
The normal-basis lift (bit reversal) is a separate step and is not exercised
here; if (1) and (2) pass, the lift is ordinary masking work.
"""
import json
import os
import pathlib
import statistics
import sys

import torch
import triton
import triton.language as tl

HERE = pathlib.Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))

ASM = tl.constexpr("clmad.lo.u64 $0, $2, $3, 0;\n\tclmad.hi.u64 $1, $2, $3, 0;")


@triton.jit
def _clmul_kernel(a_ptr, b_ptr, lo_ptr, hi_ptr, BLOCK: tl.constexpr):
    i = tl.arange(0, BLOCK)
    a = tl.load(a_ptr + i)
    b = tl.load(b_ptr + i)
    lo, hi = tl.inline_asm_elementwise(ASM, "=l,=l,l,l", [a, b],
                                       dtype=(tl.int64, tl.int64), is_pure=True, pack=1)
    tl.store(lo_ptr + i, lo)
    tl.store(hi_ptr + i, hi)


@triton.jit
def _poly263_kernel(a_ptr, b_ptr, o_ptr, BLOCK: tl.constexpr):
    """c = a*b mod (x^263 - 1) over GF(2); a, b, c are 5 int64 limbs (bits >=263 zero)."""
    r = tl.program_id(0) * BLOCK + tl.arange(0, BLOCK)
    av = [tl.load(a_ptr + r * 5 + 0), tl.load(a_ptr + r * 5 + 1), tl.load(a_ptr + r * 5 + 2),
          tl.load(a_ptr + r * 5 + 3), tl.load(a_ptr + r * 5 + 4)]
    bv = [tl.load(b_ptr + r * 5 + 0), tl.load(b_ptr + r * 5 + 1), tl.load(b_ptr + r * 5 + 2),
          tl.load(b_ptr + r * 5 + 3), tl.load(b_ptr + r * 5 + 4)]
    zero = tl.zeros((BLOCK,), tl.int64)
    c = [zero] * 9
    for i in range(5):
        for j in range(5):
            lo, hi = tl.inline_asm_elementwise(ASM, "=l,=l,l,l", [av[i], bv[j]],
                                               dtype=(tl.int64, tl.int64), is_pure=True, pack=1)
            c[i + j] = c[i + j] ^ lo
            c[i + j + 1] = c[i + j + 1] ^ hi
    # fold: F = P ^ (P >> 263)
    s0 = (c[4] >> 7) | (c[5] << 57)
    s1 = (c[5] >> 7) | (c[6] << 57)
    s2 = (c[6] >> 7) | (c[7] << 57)
    s3 = (c[7] >> 7) | (c[8] << 57)
    tl.store(o_ptr + r * 5 + 0, c[0] ^ s0)
    tl.store(o_ptr + r * 5 + 1, c[1] ^ s1)
    tl.store(o_ptr + r * 5 + 2, c[2] ^ s2)
    tl.store(o_ptr + r * 5 + 3, c[3] ^ s3)
    tl.store(o_ptr + r * 5 + 4, c[4] ^ (c[8] >> 7))


def py_clmul(a, b):
    r = 0
    while b:
        low = b & -b
        r ^= a << (low.bit_length() - 1)
        b ^= low
    return r


def py_poly263(a_limbs, b_limbs):
    a = 0
    for k in range(4, -1, -1):
        a = (a << 64) | (a_limbs[k] & ((1 << 64) - 1))
    b = 0
    for k in range(4, -1, -1):
        b = (b << 64) | (b_limbs[k] & ((1 << 64) - 1))
    p = py_clmul(a, b)
    low = p & ((1 << 263) - 1)
    high = p >> 263
    f = low ^ high                      # mod x^263 - 1
    return [(f >> (64 * k)) & ((1 << 64) - 1) for k in range(5)]


def median_ms(fn, reps=20):
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
    torch.manual_seed(7)
    dev = "cuda"
    rep = {"gpu": torch.cuda.get_device_name()}

    n = 1 << 20
    a = torch.randint(-(1 << 63), (1 << 63) - 1, (n,), dtype=torch.int64, device=dev)
    b = torch.randint(-(1 << 63), (1 << 63) - 1, (n,), dtype=torch.int64, device=dev)
    lo, hi = torch.empty_like(a), torch.empty_like(a)
    _clmul_kernel[(n // 1024,)](a, b, lo, hi, BLOCK=1024)
    ok = True
    for k in range(16):
        av = int(a[k].item()) & ((1 << 64) - 1)
        bv = int(b[k].item()) & ((1 << 64) - 1)
        got = ((int(hi[k].item()) & ((1 << 64) - 1)) << 64) | (int(lo[k].item()) & ((1 << 64) - 1))
        if got != py_clmul(av, bv):
            ok = False
            break
    rep["clmul64_ok"] = ok
    ms = median_ms(lambda: _clmul_kernel[(n // 1024,)](a, b, lo, hi, BLOCK=1024))
    rep["clmul64_per_s"] = n / ms * 1e3
    rep["clmul64_lanes_per_ns"] = n / (ms * 1e6)

    if os.environ.get('CLMUL_ONLY'):
        print(json.dumps(rep)); return 0 if (rep['clmul64_ok']) else 1

    m = 1 << 14
    A = torch.zeros((m, 5), dtype=torch.int64, device=dev)
    B = torch.zeros((m, 5), dtype=torch.int64, device=dev)
    A[:, :4] = torch.randint(-(1 << 63), (1 << 63) - 1, (m, 4), dtype=torch.int64, device=dev)
    B[:, :4] = torch.randint(-(1 << 63), (1 << 63) - 1, (m, 4), dtype=torch.int64, device=dev)
    A[:, 4] = torch.randint(0, 1 << 7, (m,), dtype=torch.int64, device=dev)
    B[:, 4] = torch.randint(0, 1 << 7, (m,), dtype=torch.int64, device=dev)
    C = torch.zeros((m, 5), dtype=torch.int64, device=dev)
    _poly263_kernel[(m // 64,)](A, B, C, BLOCK=64)
    ok_p = True
    for k in range(8):
        want = py_poly263([int(v) for v in A[k].tolist()], [int(v) for v in B[k].tolist()])
        got = [int(v) & ((1 << 64) - 1) for v in C[k].tolist()]
        if got != want:
            ok_p = False
            rep["bad_poly_row"] = k
            break
    rep["poly263_ok"] = ok_p
    ms = median_ms(lambda: _poly263_kernel[(m // 64,)](A, B, C, BLOCK=64))
    rep["poly263_ms"] = ms
    rep["poly263_per_s"] = m / ms * 1e3

    print(json.dumps(rep))
    out = pathlib.Path("/results") if pathlib.Path("/results").is_dir() else HERE
    (out / "clmul.json").write_text(json.dumps(rep, indent=2))
    return 0 if (rep["clmul64_ok"] and rep["poly263_ok"]) else 1


if __name__ == "__main__":
    raise SystemExit(main())
