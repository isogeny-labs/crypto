import sys, time
sys.path.insert(0, str(__import__("pathlib").Path(__file__).resolve().parent.parent / "lib"))
from scan_unexplained import freelip_scan, lcg_scan
from curves import CURVES
from others import BCURVES
targets = [("CONTROL P-256 seed (drand48 bytes)", int(CURVES["P-256"][1], 16), 160),
           ("CONTROL B-233 seed (lrand48 words)", int(BCURVES["B-233"][1], 16), 160)]
Q = 2**107 - 1; AL = pow(3, 121, Q)
def fl_limbs(seed, n):
    st = seed; out = []
    for _ in range(n):
        st = st * AL % Q; out.append(st & (2**26 - 1))
    return out
x = 0
for l in fl_limbs(424242, 7 + 10)[7:]: x = x << 26 | l
targets.append(("CONTROL synthetic FreeLIP x, m=233 (no reduction)", x & ((1 << 233) - 1), 233))
st = 0x330E | (892826658 << 16); w = 0
for _ in range(40): st = (0x5DEECE66D * st + 0xB) % 2**48
for _ in range(8): st = (0x5DEECE66D * st + 0xB) % 2**48; w = w << 32 | (st >> 17)
targets.append(("CONTROL synthetic lrand48 x, m=233", w & ((1 << 233) - 1), 233))
for ln in open("bin_targets.txt"):
    n, m, kind, v = ln.split()
    targets.append((f"{n} {kind}", int(v), int(m)))
sel = [int(a) for a in sys.argv[1:]] if len(sys.argv) > 1 else range(len(targets))
for ti in sel:
    label, V, L = targets[ti]
    t0 = time.time()
    f = freelip_scan(label, V, L); g = lcg_scan(label, V, L)
    print(f"{label:34s} FreeLIP: {'HIT ' + str(f[:1]) if f else 'none'}  LCG48: {'HIT ' + str(g[:1]) if g else 'none'}  ({time.time()-t0:.0f}s)", flush=True)
