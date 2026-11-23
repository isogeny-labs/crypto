"""ECC2K-130 walk step, v5: row-packed multiply.

v3 spends 49% of every multiply's lanes on padding (131 outputs in a 256-lane
tile) and recomputes the index tensors per row. v5 packs R rows per program
with a 128-lane output chunk (t = 0..127) plus a 4-lane tail (t = 128..130),
so the main tile is ~97% useful, and makes the operands 3D — (ITILE, R, T) —
so a[i] is loaded once per row instead of once per (row, t) lane.

Scratch and the output are padded up to a multiple of R so no kernel needs a
row mask; the returned tensor is the leading N rows of the padded output.
"""
import torch
import triton
import triton.language as tl

M = 131
NSLOT = 14
SLOT = 264
STRIDE = NSLOT * SLOT


@triton.jit
def _unpack_kernel(x_ptr, y_ptr, scratch, n_rows, STRIDE: tl.constexpr):
    pid = tl.program_id(0)
    base = scratch + pid * STRIDE
    offs = tl.arange(0, 256)
    m = (offs < 131) & (pid < n_rows)      # padded rows are left untouched
    li = offs // 64
    sh = (offs % 64).to(tl.int64)
    x0 = tl.load(x_ptr + pid * 3 + 0)
    x1 = tl.load(x_ptr + pid * 3 + 1)
    x2 = tl.load(x_ptr + pid * 3 + 2)
    lx = tl.where(li == 0, x0, tl.where(li == 1, x1, x2))
    tl.store(base + 0 * 264 + offs, ((lx >> sh) & 1).to(tl.int8), mask=m)
    y0 = tl.load(y_ptr + pid * 3 + 0)
    y1 = tl.load(y_ptr + pid * 3 + 1)
    y2 = tl.load(y_ptr + pid * 3 + 2)
    ly = tl.where(li == 0, y0, tl.where(li == 1, y1, y2))
    tl.store(base + 1 * 264 + offs, ((ly >> sh) & 1).to(tl.int8), mask=m)


@triton.jit
def _frob_kernel(scratch, table_ptr, STRIDE: tl.constexpr):
    pid = tl.program_id(0)
    base = scratch + pid * STRIDE
    offs = tl.arange(0, 256)
    m = offs < 131
    xb = tl.load(base + 0 * 264 + offs, mask=m, other=0)
    hw = tl.sum(xb.to(tl.int32), axis=0)
    j = 3 + ((hw >> 1) & 7)
    idx = tl.load(table_ptr + j * 131 + offs, mask=m, other=0)
    vx = tl.load(base + 0 * 264 + idx, mask=m, other=0)
    tl.store(base + 2 * 264 + offs, vx, mask=m)
    vy = tl.load(base + 1 * 264 + idx, mask=m, other=0)
    tl.store(base + 3 * 264 + offs, vy, mask=m)


@triton.jit
def _sigma_kernel(scratch, table_ptr, src, dst, k, STRIDE: tl.constexpr):
    pid = tl.program_id(0)
    base = scratch + pid * STRIDE
    offs = tl.arange(0, 256)
    m = offs < 131
    idx = tl.load(table_ptr + k * 131 + offs, mask=m, other=0)
    v = tl.load(base + src * 264 + idx, mask=m, other=0)
    tl.store(base + dst * 264 + offs, v, mask=m)


@triton.jit
def _xor_kernel(scratch, s1, s2, dst, STRIDE: tl.constexpr):
    pid = tl.program_id(0)
    base = scratch + pid * STRIDE
    offs = tl.arange(0, 256)
    m = offs < 131
    a = tl.load(base + s1 * 264 + offs, mask=m, other=0)
    b = tl.load(base + s2 * 264 + offs, mask=m, other=0)
    tl.store(base + dst * 264 + offs, a ^ b, mask=m)


@triton.jit
def _xor3_kernel(scratch, s1, s2, s3, dst, STRIDE: tl.constexpr):
    pid = tl.program_id(0)
    base = scratch + pid * STRIDE
    offs = tl.arange(0, 256)
    m = offs < 131
    a = tl.load(base + s1 * 264 + offs, mask=m, other=0)
    b = tl.load(base + s2 * 264 + offs, mask=m, other=0)
    c = tl.load(base + s3 * 264 + offs, mask=m, other=0)
    tl.store(base + dst * 264 + offs, a ^ b ^ c, mask=m)


@triton.jit
def _copy_kernel(scratch, src, dst, STRIDE: tl.constexpr):
    pid = tl.program_id(0)
    base = scratch + pid * STRIDE
    offs = tl.arange(0, 256)
    m = offs < 131
    v = tl.load(base + src * 264 + offs, mask=m, other=0)
    tl.store(base + dst * 264 + offs, v, mask=m)


@triton.jit
def _mul_kernel(scratch, sa, sb, dst, R: tl.constexpr, ITILE: tl.constexpr, STRIDE: tl.constexpr):
    pid = tl.program_id(0)
    r = tl.arange(0, R)
    rowbase = scratch + (pid * R + r) * STRIDE
    abase = rowbase + sa * 264
    bbase = rowbase + sb * 264

    t = tl.arange(0, 128)                      # main chunk: t = 0..127
    t1 = t + 1
    acc = tl.zeros((ITILE, R, 128), tl.int32)

    tt = 128 + tl.arange(0, 4)                 # tail: t = 128..130 (131 masked)
    tt1 = tt + 1
    tacc = tl.zeros((ITILE, R, 4), tl.int32)

    for it in tl.static_range(0, (131 + ITILE - 1) // ITILE):
        i_val = it * ITILE + tl.arange(0, ITILE) + 1
        im = i_val <= 131
        av = tl.load(abase[None, :, None] + i_val[:, None, None] - 1,
                     mask=im[:, None, None], other=0).to(tl.int32)

        idxm = t1[None, :] - i_val[:, None]
        idxm = tl.where(idxm < 0, idxm + 263, idxm)
        idxp = t1[None, :] + i_val[:, None]
        fm = tl.minimum(idxm, 263 - idxm)
        fp = tl.minimum(idxp, 263 - idxp)
        bm = tl.load(bbase[None, :, None] + fm[:, None, :] - 1,
                     mask=(idxm != 0)[:, None, :], other=0)
        bp = tl.load(bbase[None, :, None] + fp[:, None, :] - 1)
        acc = acc ^ (av & (bm ^ bp).to(tl.int32))

        tim = tt1[None, :] - i_val[:, None]
        tim = tl.where(tim < 0, tim + 263, tim)
        tip = tt1[None, :] + i_val[:, None]
        tfm = tl.minimum(tim, 263 - tim)
        tfp = tl.minimum(tip, 263 - tip)
        tbm = tl.load(bbase[None, :, None] + tfm[:, None, :] - 1,
                      mask=(tim != 0)[:, None, :], other=0)
        tbp = tl.load(bbase[None, :, None] + tfp[:, None, :] - 1)
        tacc = tacc ^ (av & (tbm ^ tbp).to(tl.int32))

    tl.store(rowbase[:, None] + dst * 264 + t[None, :], (tl.sum(acc, axis=0) & 1).to(tl.int8))
    tl.store(rowbase[:, None] + dst * 264 + tt[None, :],
             (tl.sum(tacc, axis=0) & 1).to(tl.int8), mask=(tt < 131)[None, :])


@triton.jit
def _pack_kernel(scratch, out_ptr, STRIDE: tl.constexpr):
    pid = tl.program_id(0)
    base = scratch + pid * STRIDE
    offs = tl.arange(0, 256)
    m = offs < 131
    sh = (offs % 64).to(tl.int64)
    bx = tl.load(base + 9 * 264 + offs, mask=m, other=0).to(tl.int64)
    cx = tl.where(m, bx << sh, 0)
    tl.store(out_ptr + pid * 6 + 0, tl.sum(tl.where(offs < 64, cx, 0), axis=0))
    tl.store(out_ptr + pid * 6 + 1,
             tl.sum(tl.where((offs >= 64) & (offs < 128), cx, 0), axis=0))
    tl.store(out_ptr + pid * 6 + 2, tl.sum(tl.where(offs >= 128, cx, 0), axis=0))
    by = tl.load(base + 13 * 264 + offs, mask=m, other=0).to(tl.int64)
    cy = tl.where(m, by << sh, 0)
    tl.store(out_ptr + pid * 6 + 3, tl.sum(tl.where(offs < 64, cy, 0), axis=0))
    tl.store(out_ptr + pid * 6 + 4,
             tl.sum(tl.where((offs >= 64) & (offs < 128), cy, 0), axis=0))
    tl.store(out_ptr + pid * 6 + 5, tl.sum(tl.where(offs >= 128, cy, 0), axis=0))


_TABLE = {}


def get_table(device):
    key = str(device)
    tbl = _TABLE.get(key)
    if tbl is None:
        rows = [[0] * 131 for _ in range(131)]
        e = 1
        for k in range(131):
            for i in range(1, 132):
                t = i * e % 263
                if t > 131:
                    t = 263 - t
                rows[k][t - 1] = i - 1
            e = 2 * e % 263
        tbl = torch.tensor(rows, dtype=torch.int32, device=device).reshape(-1)
        _TABLE[key] = tbl
    return tbl


def kernel_function(x, y, r=4, itile=8, warps=4):
    assert x.is_cuda and y.is_cuda
    assert x.dtype == torch.int64 and y.dtype == torch.int64
    assert x.shape == y.shape and x.dim() == 2 and x.shape[1] == 3
    N = x.shape[0]
    npad = ((N + r - 1) // r) * r
    out = torch.empty((npad, 2, 3), dtype=torch.int64, device=x.device)
    if N == 0:
        return out[:0]
    table = get_table(x.device)
    scratch = torch.empty((npad, STRIDE), dtype=torch.int8, device=x.device)
    grid = (npad,)
    kw = dict(num_warps=warps)
    _unpack_kernel[grid](x, y, scratch, N, STRIDE=STRIDE, **kw)
    _frob_kernel[grid](scratch, table, STRIDE=STRIDE, **kw)
    _xor_kernel[grid](scratch, 0, 2, 4, STRIDE=STRIDE, **kw)
    _copy_kernel[grid](scratch, 4, 5, STRIDE=STRIDE, **kw)
    acc, tmp = 5, 7
    for k in (1, 2, 4, 8, 16, 32):
        _sigma_kernel[grid](scratch, table, acc, 6, k, STRIDE=STRIDE, **kw)
        _mul_kernel[(npad // r,)](scratch, 6, acc, tmp, R=r, ITILE=itile, STRIDE=STRIDE, **kw)
        acc, tmp = tmp, acc
    _sigma_kernel[grid](scratch, table, acc, 6, 1, STRIDE=STRIDE, **kw)
    _mul_kernel[(npad // r,)](scratch, 6, 4, tmp, R=r, ITILE=itile, STRIDE=STRIDE, **kw)
    acc, tmp = tmp, acc
    _sigma_kernel[grid](scratch, table, acc, 6, 65, STRIDE=STRIDE, **kw)
    _mul_kernel[(npad // r,)](scratch, 6, acc, tmp, R=r, ITILE=itile, STRIDE=STRIDE, **kw)
    acc, tmp = tmp, acc
    _sigma_kernel[grid](scratch, table, acc, 6, 1, STRIDE=STRIDE, **kw)
    _xor_kernel[grid](scratch, 1, 3, 12, STRIDE=STRIDE, **kw)
    _mul_kernel[(npad // r,)](scratch, 12, 6, 8, R=r, ITILE=itile, STRIDE=STRIDE, **kw)
    _sigma_kernel[grid](scratch, table, 8, 9, 1, STRIDE=STRIDE, **kw)
    _xor3_kernel[grid](scratch, 9, 8, 4, 9, STRIDE=STRIDE, **kw)
    _xor_kernel[grid](scratch, 0, 9, 12, STRIDE=STRIDE, **kw)
    _mul_kernel[(npad // r,)](scratch, 8, 12, 13, R=r, ITILE=itile, STRIDE=STRIDE, **kw)
    _xor3_kernel[grid](scratch, 13, 9, 1, 13, STRIDE=STRIDE, **kw)
    _pack_kernel[grid](scratch, out, STRIDE=STRIDE, **kw)
    return out[:N]
