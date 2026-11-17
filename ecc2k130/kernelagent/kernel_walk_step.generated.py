"""
ECC2K-130 Pollard-rho walk step, fully in Triton kernels.

GF(2^131), permuted type-II optimal normal basis, NRING=263.
Elements: 131 bits packed into 3 int64 limbs (bit i -> limb i//64, bit i%64).

Fusion audit: sigma (bit permutation) and the convolution gathers are
cross-lane data movements. Fusing stages into a single kernel would require
cross-thread visibility of global scratch writes inside one CTA program,
which is racy without device-wide sync; therefore stages run as separate
batch-vectorized Triton launches (sync at kernel boundaries). All arithmetic
lives in Triton kernels; the wrapper only validates, allocates and launches.
"""
import torch
import triton
import triton.language as tl

M = 131
NSLOT = 14
SLOT = 264            # bytes per slot (263 needed for convolution index space)
STRIDE = NSLOT * SLOT

# slot layout: 0:xb 1:yb 2:x2 3:y2 4:d 5/7:inv acc ping-pong 6:sigma tmp
#              8:lam 9:x3 11:(unused B) 12:tmp 13:y3


@triton.jit
def _unpack_kernel(x_ptr, y_ptr, scratch, STRIDE: tl.constexpr):
    pid = tl.program_id(0)
    base = scratch + pid * STRIDE
    offs = tl.arange(0, 256)
    m = offs < 131
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
def _frob_kernel(scratch, STRIDE: tl.constexpr):
    # per-element j = 3 + ((HW(x) >> 1) & 7); x2 = sigma^j(x), y2 = sigma^j(y)
    pid = tl.program_id(0)
    base = scratch + pid * STRIDE
    offs = tl.arange(0, 256)
    m = offs < 131
    xb = tl.load(base + 0 * 264 + offs, mask=m, other=0)
    hw = tl.sum(xb.to(tl.int32), axis=0)
    j = 3 + ((hw >> 1) & 7)
    q = j * 0 + 1                      # q = 132^j mod 263 = 2^{-j}
    for _ in range(0, j):
        q = (q * 132) % 263
    t1 = offs + 1
    i = (t1 * q) % 263
    i = tl.where(i > 131, 263 - i, i)  # fold
    vx = tl.load(base + 0 * 264 + i - 1, mask=m, other=0)
    tl.store(base + 2 * 264 + offs, vx, mask=m)
    vy = tl.load(base + 1 * 264 + i - 1, mask=m, other=0)
    tl.store(base + 3 * 264 + offs, vy, mask=m)


@triton.jit
def _sigma_kernel(scratch, src, dst, k, STRIDE: tl.constexpr):
    # dst[t] = src[i-1], i = (t+1) * 2^{-k} mod 263, folded to 1..131
    pid = tl.program_id(0)
    base = scratch + pid * STRIDE
    q = k * 0 + 1                      # q = 132^k mod 263 = 2^{-k}
    for _ in range(0, k):
        q = (q * 132) % 263
    offs = tl.arange(0, 256)
    m = offs < 131
    t1 = offs + 1
    i = (t1 * q) % 263
    i = tl.where(i > 131, 263 - i, i)
    v = tl.load(base + src * 264 + i - 1, mask=m, other=0)
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
def _mul_kernel(scratch, sa, sb, dst, STRIDE: tl.constexpr):
    # GF(2) cyclic conv over Z/263 with symmetric lift:
    # out[t-1] = XOR_{i=1..131} a[i-1] & (B[(t-i)%263] ^ B[(t+i)%263])
    # where B[idx] = b[min(idx,263-idx)-1] for idx != 0, B[0] = 0.
    pid = tl.program_id(0)
    base = scratch + pid * STRIDE
    t_offs = tl.arange(0, 256)
    tm = t_offs < 131
    t1 = t_offs + 1
    acc = tl.zeros((256,), tl.int32)
    for it in tl.static_range(0, 5):
        i_val = it * 32 + tl.arange(0, 32) + 1
        im = i_val <= 131
        av = tl.load(base + sa * 264 + i_val - 1, mask=im, other=0).to(tl.int32)
        idxm = (t1[None, :] - i_val[:, None] + 263) % 263
        idxp = (t1[None, :] + i_val[:, None]) % 263
        fm = tl.minimum(idxm, 263 - idxm)
        fp = tl.minimum(idxp, 263 - idxp)
        bm = tl.load(base + sb * 264 + fm - 1, mask=(idxm != 0), other=0)
        bp = tl.load(base + sb * 264 + fp - 1, mask=(idxp != 0), other=0)
        term = av[:, None] & (bm ^ bp).to(tl.int32)
        acc = acc ^ tl.sum(term, axis=0)   # parity accumulation
    tl.store(base + dst * 264 + t_offs, (acc & 1).to(tl.int8), mask=tm)


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


def kernel_function(x, y):
    """Wrapper: validation, allocation and Triton kernel launches only."""
    assert x.is_cuda and y.is_cuda, "inputs must be CUDA tensors"
    assert x.dtype == torch.int64 and y.dtype == torch.int64
    assert x.dim() == 2 and y.dim() == 2 and x.shape[1] == 3 and y.shape[1] == 3
    assert x.shape == y.shape
    assert x.is_contiguous() and y.is_contiguous()
    N = x.shape[0]
    out = torch.empty((N, 2, 3), dtype=torch.int64, device=x.device)
    if N == 0:
        return out
    scratch = torch.empty((N, STRIDE), dtype=torch.int8, device=x.device)
    grid = (N,)

    _unpack_kernel[grid](x, y, scratch, STRIDE=STRIDE)
    _frob_kernel[grid](scratch, STRIDE=STRIDE)               # x2, y2
    _xor_kernel[grid](scratch, 0, 2, 4, STRIDE=STRIDE)       # d = xb ^ x2

    # Itoh-Tsujii inversion of d (e = 130 = 0b10000010), acc starts as d
    _copy_kernel[grid](scratch, 4, 5, STRIDE=STRIDE)
    acc, tmp = 5, 7
    for k in (1, 2, 4, 8, 16, 32):                            # doubling steps
        _sigma_kernel[grid](scratch, acc, 6, k, STRIDE=STRIDE)
        _mul_kernel[grid](scratch, 6, acc, tmp, STRIDE=STRIDE)
        acc, tmp = tmp, acc
    # bit 1 of e is set: acc = mul(sigma(acc,1), d)
    _sigma_kernel[grid](scratch, acc, 6, 1, STRIDE=STRIDE)
    _mul_kernel[grid](scratch, 6, 4, tmp, STRIDE=STRIDE)
    acc, tmp = tmp, acc
    _sigma_kernel[grid](scratch, acc, 6, 65, STRIDE=STRIDE)   # b = 0 step (k=65)
    _mul_kernel[grid](scratch, 6, acc, tmp, STRIDE=STRIDE)
    acc, tmp = tmp, acc
    _sigma_kernel[grid](scratch, acc, 6, 1, STRIDE=STRIDE)    # slot6 = d^{-1}

    # lam = (yb ^ y2) * d^{-1}
    _xor_kernel[grid](scratch, 1, 3, 12, STRIDE=STRIDE)
    _mul_kernel[grid](scratch, 12, 6, 8, STRIDE=STRIDE)
    # x3 = sigma(lam,1) ^ lam ^ d
    _sigma_kernel[grid](scratch, 8, 9, 1, STRIDE=STRIDE)
    _xor3_kernel[grid](scratch, 9, 8, 4, 9, STRIDE=STRIDE)
    # y3 = lam*(xb ^ x3) ^ x3 ^ yb
    _xor_kernel[grid](scratch, 0, 9, 12, STRIDE=STRIDE)
    _mul_kernel[grid](scratch, 8, 12, 13, STRIDE=STRIDE)
    _xor3_kernel[grid](scratch, 13, 9, 1, 13, STRIDE=STRIDE)

    _pack_kernel[grid](scratch, out, STRIDE=STRIDE)
    return out