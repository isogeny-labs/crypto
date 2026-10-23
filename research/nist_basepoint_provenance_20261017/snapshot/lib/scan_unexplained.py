"""Seed-size-agnostic generator scan over values with no known origin.
 (1) FreeLIP zrandom: limbs = state mod 2^26, state <- 3^121 * state mod (2^107-1).
 (2) drand48-family LCG mod 2^48: word = (X >> s) mod 2^w  ==  top w bits of the LCG mod 2^(s+w).
A hit counts only if the recovered state explains >= (state bits + 64) observed bits."""
from fpylll import IntegerMatrix, LLL, CVP, GSO
from freelip import Q, AL
from drand48 import A as A48, C as C48
import itertools, sys

def cvp_bounded(Ai, Di, Mod, bound):
    """find z0 in [0,bound) with (Ai[i]*z0 + Di[i]) mod Mod in [0,bound) for all i; Ai[0]=1, Di[0]=0."""
    n = len(Ai)
    B = IntegerMatrix(n, n)
    B[0, 0] = 1
    for i in range(1, n):
        B[0, i] = Ai[i] % Mod; B[i, i] = Mod
    LLL.reduction(B)
    half = bound // 2
    target = [half] + [(half - Di[i]) % Mod for i in range(1, n)]
    G = GSO.Mat(B); G.update_gso()
    coeffs = G.babai(target)                     # Babai nearest plane: polynomial time
    v0 = sum(int(c) * B[j, 0] for j, c in enumerate(coeffs))
    return v0 % Mod

# ---------- FreeLIP ----------
INV26 = pow(2**26, -1, Q)
def freelip_recover(limbs):
    """limbs[i] = low 26 bits of consecutive states s_1..s_n. Return s_1 or None."""
    n = len(limbs)
    Ai = [pow(AL, i, Q) for i in range(n)]
    # s_i = 2^26 h_i + l_i ;  h_i = Ai h_1 + (Ai l_1 - l_i) / 2^26  (mod Q)
    Di = [((Ai[i] * limbs[0] - limbs[i]) * INV26) % Q for i in range(n)]
    h1 = cvp_bounded(Ai, Di, Q, 2**81)
    for dh in range(-3, 4):
        s1 = ((h1 + dh) << 26 | limbs[0]) % Q
        st, ok = s1, True
        for l in limbs[1:]:
            st = st * AL % Q
            if st & (2**26 - 1) != l: ok = False; break
        if ok: return s1
    return None

def freelip_scan(label, V, L, win=7):
    out = []
    for order in ("top-first", "bottom-first"):
        for off in range(26):
            nl = (L - off) // 26
            if nl < win: continue
            limbs = [(V >> (off + 26 * j)) & (2**26 - 1) for j in range(nl)]   # bottom-up
            seq = limbs[::-1] if order == "top-first" else limbs
            for st in range(0, nl - win + 1):
                s1 = freelip_recover(seq[st:st + win])
                if s1:
                    # extend
                    k, x = win, s1
                    for _ in range(win - 1): x = x * AL % Q
                    while st + k < nl:
                        x = x * AL % Q
                        if x & (2**26 - 1) != seq[st + k]: break
                        k += 1
                    out.append((order, off, st, k, s1))
    return out

# ---------- LCG mod 2^K, top-w-bit outputs ----------
def lcg_recover(ys, w, K):
    M = 1 << K; a = A48 % M; c = C48 % M; hid = K - w
    n = len(ys)
    if hid <= 10:   # brute force
        for z in range(1 << hid):
            x = (ys[0] << hid) | z; ok = True
            for y in ys[1:]:
                x = (a * x + c) % M
                if x >> hid != y: ok = False; break
            if ok: return (ys[0] << hid) | z
        return None
    Ai, Ci = [1], [0]
    for _ in range(n - 1):
        Ai.append(Ai[-1] * a % M); Ci.append((Ci[-1] * a + c) % M)
    Di = [(Ai[i] * (ys[0] << hid) + Ci[i] - (ys[i] << hid)) % M for i in range(n)]
    z = cvp_bounded(Ai, Di, M, 1 << hid)
    for dz in range(-3, 4):
        x0 = (ys[0] << hid) | ((z + dz) % (1 << hid)); x = x0; ok = True
        for y in ys[1:]:
            x = (a * x + c) % M
            if x >> hid != y: ok = False; break
        if ok: return x0
    return None

def bswap32(v):
    return int.from_bytes(v.to_bytes(4, "big")[::-1], "big")

def lcg_scan(label, V, L):
    out = []
    for off in range(32):
        nw = (L - off) // 32
        if nw < 4: continue
        words = [(V >> (off + 32 * j)) & 0xFFFFFFFF for j in range(nw)]
        for vname, seq in (("lrand48-slot top-first", words[::-1]), ("lrand48-slot bottom-first", words),
                           ("lrand48-slot top-first,bswap", [bswap32(x) for x in words[::-1]])):
            for st in range(0, nw - 4 + 1):
                ws = seq[st:st + 4]
                if any(x >> 31 for x in ws): continue
                x0 = lcg_recover(ws, 31, 48)
                if x0 is not None: out.append((31, 17, off, vname, st, 4, x0))
    for w in (8, 15, 16, 24, 31, 32):
        for s in range(0, 48 - w + 1):
            K = s + w
            win = max(4, -(-(K + 64) // w))           # enough words to beat chance by 2^64
            for off in range(w):
                nw = (L - off) // w
                if nw < win: continue
                words = [(V >> (off + w * j)) & ((1 << w) - 1) for j in range(nw)]
                variants = [("top-first", words[::-1]), ("bottom-first", words)]
                if w == 32:
                    variants += [("top-first,bswap", [bswap32(x) for x in words[::-1]])]
                for vname, seq in variants:
                    for st in (0, nw - win):          # windows at both ends
                        x0 = lcg_recover(seq[st:st + win], w, K)
                        if x0 is not None:
                            out.append((w, s, off, vname, st, win, x0))
    return out

if __name__ == "__main__":
    from curves import CURVES
    from others import BCURVES
    targets = [("B-163 seed", int(BCURVES["B-163"][1], 16), 160)]
    for n in ("P-224", "P-256", "P-384", "P-521"):
        p = CURVES[n][0]
        targets += [(n + " Gx", CURVES[n][4], p.bit_length()), (n + " Gy", CURVES[n][5], p.bit_length())]
    for n, (m, seed, b, gx, gy) in BCURVES.items():
        targets += [(n + " Gx", gx, m), (n + " Gy", gy, m)]
    # positive controls: things known to be generator output
    from drand48 import Drand48
    targets.insert(0, ("CONTROL P-256 seed (drand48 bytes)", int(CURVES["P-256"][1], 16), 160))
    targets.insert(1, ("CONTROL B-233 seed (lrand48 words)", int(BCURVES["B-233"][1], 16), 160))
    from freelip import base_point_x
    for cn, sd in (("P-384", 987654321), ("P-521", 31337)):
        p, b = CURVES[cn][0], CURVES[cn][2]
        targets.insert(2, (f"CONTROL synthetic {cn} x (FreeLIP seed {sd}, mod p)", base_point_x(sd, p, b)[0], p.bit_length()))
    sel = [int(a) for a in sys.argv[1:]] if len(sys.argv) > 1 else range(len(targets))
    if sys.argv[1:] == ["count"]:
        print(len(targets)); sys.exit(0)
    for ti in sel:
        label, V, L = targets[ti]
        import time; t0 = time.time()
        f = freelip_scan(label, V, L)
        g = lcg_scan(label, V, L)
        fbest = max(f, key=lambda r: r[3]) if f else None
        print(f"{label:38s} FreeLIP: {('run of %d limbs (%s, off %d)' % (fbest[3], fbest[0], fbest[1])) if fbest else 'none':34s}  LCG48: " +
              ("; ".join(sorted({f'w={r[0]} s={r[1]} off={r[2]} {r[3]}' for r in g})[:3]) if g else "none") + f"   [{time.time()-t0:.0f}s]", flush=True)
