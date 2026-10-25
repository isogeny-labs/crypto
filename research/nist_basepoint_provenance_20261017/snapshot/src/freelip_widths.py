import sys
sys.path.insert(0, str(__import__("pathlib").Path(__file__).resolve().parent.parent / "lib"))
from scan_unexplained import cvp_bounded
from curves import CURVES
Q = 2**107 - 1; AL = pow(3, 121, Q)
WIN = {26: 7, 30: 6, 62: 3}

def recover(limbs, w):
    n = len(limbs); M = 2**w; inv = pow(M, -1, Q)
    Ai = [pow(AL, i, Q) for i in range(n)]
    Di = [((Ai[i] * limbs[0] - limbs[i]) * inv) % Q for i in range(n)]
    h1 = cvp_bounded(Ai, Di, Q, 2**(107 - w))
    for dh in range(-3, 4):
        s1 = (((h1 + dh) << w) | limbs[0]) % Q
        st, ok = s1, True
        for l in limbs[1:]:
            st = st * AL % Q
            if st % M != l: ok = False; break
        if ok: return s1
    return None

def scan(V, L, w):
    win = WIN[w]; hits = []
    for order in ("top-first", "bottom-first"):
        for off in range(w):
            nl = (L - off) // w
            if nl < win: continue
            limbs = [(V >> (off + w * j)) % 2**w for j in range(nl)]
            seq = limbs[::-1] if order == "top-first" else limbs
            for st in range(nl - win + 1):
                s1 = recover(seq[st:st + win], w)
                if s1: hits.append((order, off, st))
    return hits

def zrandomb_x(seed, p, w, skip):
    st = seed; nl = -(-p.bit_length() // w) + 1; out = []
    for _ in range(skip + nl):
        st = st * AL % Q; out.append(st % 2**w)
    x = 0
    for l in out[skip:]: x = (x << w) | l
    return x % p

if __name__ == "__main__":
    for name in ("P-224", "P-256", "P-384", "P-521"):
        p, gx, gy = CURVES[name][0], CURVES[name][4], CURVES[name][5]
        L = p.bit_length()
        for w in (26, 30, 62):
            ctrl = [bool(scan(zrandomb_x(sd, p, w, 7), L, w)) for sd in (987654321, 31337, 2**90 + 12345)]
            gxh, gyh = scan(gx, L, w), scan(gy, L, w)
            print(f"{name} NBITS={w:2d}  controls detected {sum(ctrl)}/3   Gx: {'HIT ' + str(gxh[:2]) if gxh else 'none'}   Gy: {'HIT ' + str(gyh[:2]) if gyh else 'none'}", flush=True)
