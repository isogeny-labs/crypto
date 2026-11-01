#!/usr/bin/env python3
import math, sys
sys.path.insert(0, "/tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/eval2/g1")
from model import *

print("== check of Pornin 2.2 count ==")
print("p^{2-2/5} = 2^%.2f ; x D^2=2^40 -> 2^%.2f ; with 5! = 2^%.2f -> 2^%.2f" %
      (1.6 * LGP, 1.6 * LGP + 40, lg(120), 1.6 * LGP + 40 + lg(120)))
print("with nominal 5 D^3 (D=2^20) = 2^%.2f: Pornin-style 2^%.2f ; with 5!: 2^%.2f" %
      (lg(5) + 60, 1.6 * LGP + lg(5) + 60, 1.6 * LGP + lg(5) + 60 + lg(120)))
print("JV(n-1) attempts: 4! p^2 = 2^%.2f ; /2 relations-needed = 2^%.2f" % (lg(24) + 2 * LGP, lg(12) + 2 * LGP))

print("\n== relation/LA balance for c in F_p ops; totals as log2 64-bit word ops ==")
hdr = "log2c | full  | nolp(B,tot) | lp1(B,tot) | lp2(B,tot,edges) | Pornin-style p^1.6*c*u | best | rho(800)"
print(hdr)
rows = []
for lgc in [30, 32, 36, 40, 44, 48, 50, 54, 60, 62.3, 70, 80, 82.6, 90]:
    f, fr, fl = full_base(lgc)
    n0, b0, r0, l0 = subset_nolp(lgc)
    o1 = lp1(lgc)
    o2 = (n0, 0, 0, 0, 0, 0)
    pornin = 1.6 * LGP + lgc + lg(U_FP)
    best = min(f, n0, o1[0], o2[0])
    rows.append((lgc, f, n0, b0, o1[0], o1[1], o2[0], o2[1], o2[4], pornin, best))
    print("%5.1f | %6.1f | %6.1f B=2^%.1f | %6.1f B=2^%.1f | %6.1f B=2^%.1f E=2^%.1f d=%.2f | %6.1f | %6.1f | %6.1f" %
          (lgc, f, n0, b0, o1[0], o1[1], o2[0], o2[1], o2[4], o2[5], pornin, best, rho_words()))

print("\n== crossovers (central conversions) ==")
def find(fun, target, lo=20, hi=140):
    for _ in range(100):
        mid = (lo + hi) / 2
        if fun(mid) > target:
            hi = mid
        else:
            lo = mid
    return lo
for name, fn in [("nolp", lambda c: subset_nolp(c)[0]), ("lp1", lambda c: lp1(c)[0])]:
    c128 = find(fn, 128.0)
    crho = find(fn, rho_words())
    print("%-5s T=2^128 at log2 c = %.1f ; T = rho_words(2^%.1f) at log2 c = %.1f" % (name, c128, rho_words(), crho))

print("\n== c-range per ideal degree D (F_p ops): D^2 .. 5 D^3 ==")
for nm, lgD in [("D=2^16 quotient", 16), ("D=2^20 S5", 20), ("D=2^26.9 raw", 26.9)]:
    lo, hi = 2 * lgD, 3 * lgD + lg(5)
    for tag, lgc in [("D^2", lo), ("5D^3", hi)]:
        n0 = subset_nolp(lgc)[0]; o2 = n0 + 0.0; o1 = lp1(lgc)[0]
        print("%-16s %-5s log2c=%5.1f -> nolp 2^%.1f  lp1 2^%.1f  lp2(n/a) 2^%.1f   (128: %s ; rho 2^%.1f: %s)" %
              (nm, tag, lgc, n0, o1, o2, "below" if min(n0, o1, o2) < 128 else "above", rho_words(),
               "below" if min(n0, o1, o2) < rho_words() else "above"))

print("\n== JV (n-1) and hybrid at full base (words) ==")
for lgct in [30, 33.5, 35, 36, 40]:
    t, R, LA = jv_nminus1(lgct)
    print("c~ = 2^%.1f F_p ops: JV(n-1) full base = 2^%.2f (R 2^%.2f, LA 2^%.2f)  vs rho_words 2^%.2f  -> gap %.2f bits" %
          (lgct, t, R, LA, rho_words(), t - rho_words()))
print("break-even c~ vs rho_words(800): log2 c~ = %.2f F_p ops" % (rho_words() - (lg(12) + 2 * LGP + lg(U_FP))))
print("break-even c~ vs rho_words(500..1830): %.2f .. %.2f" %
      (rho_words(500) - (lg(12) + 2 * LGP + lg(U_FP)), rho_words(1830) - (lg(12) + 2 * LGP + lg(U_FP))))
for lgch in [38, 40, 45]:
    print("hybrid c_h=2^%d: 2^%.1f words" % (lgch, hybrid(lgch)))
# JV measured: 8.5 s at 2.6 GHz Core 2 -> cycles
cyc = 8.496 * 2.6e9
print("JV measured F4' 32-bit: %.3g cycles = 2^%.2f ; as words (x1.5 IPC) 2^%.2f ; 64-bit p x2: 2^%.2f" %
      (cyc, lg(cyc), lg(cyc * 1.5), lg(cyc * 3)))
for lgw in [lg(cyc * 1.5), lg(cyc * 3)]:
    t = lg(12) + 2 * LGP + lgw
    print("JV(n-1) full base with c~_words = 2^%.2f : 2^%.2f  (gap to rho_words %.2f)" % (lgw, t, t - rho_words()))
# JV(n-1) with B < L
print("JV(n-1) B-sweep, c~_words=2^36 (c~=2^34 F_p ops)")
for lgB in [63, 62.5, 62, 61, 60, 58, 55]:
    t, R, LA = jv_nminus1(34.0, B_lg=lgB)
    print("  B=2^%.1f  total 2^%.1f  (rho 2^%.1f)  LA mem %.3g B" % (lgB, t, rho_words(), la_mem_bytes(2.0 ** lgB, 5)))
