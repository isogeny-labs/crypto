#!/usr/bin/env python3
import sys, math, itertools
sys.path.insert(0, "/tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/eval2/g1")
from model import *
def T0(lgc, u, kap):
    """closed form optimum of R=120 c_w L^5/B^4 + kap B^2  (valid while B<=L): R=LA/2 at optimum"""
    cw = lgc + lg(u)
    lgB = (lg(240) + cw + 5*LGL - lg(kap)) / 6
    if lgB > LGL:
        lgB = LGL
    R = lg(120) + cw + 5*LGL - 4*lgB
    LA = lg(kap) + 2*lgB
    return lg_add(R, LA), lgB
print("closed form vs numeric at c=2^40:", T0(40, 4, kappa(5)), subset_nolp(40)[:2])
print("kappa lo/mid/hi =", kappa(5,8,100), kappa(5), kappa(5,20,400))
print("\nlog2c  | T lo-conv | T mid | T hi-conv | rho lo | rho mid | rho hi")
for lgc in (32, 40, 44, 50, 54, 62.3):
    lo = T0(lgc, 3, kappa(5,8,100))[0]
    mid = T0(lgc, 4, kappa(5))[0]
    hi = T0(lgc, 12, kappa(5,20,400))[0]
    print("%5.1f | %6.1f | %6.1f | %6.1f | %6.1f | %6.1f | %6.1f" % (lgc, lo, mid, hi, rho_words(500), rho_words(800), rho_words(1830)))
# crossing of 2^128 under conversions
def cross(u, kap, target=128.0):
    lo, hi = 10, 140
    for _ in range(100):
        mid = (lo+hi)/2
        if T0(mid, u, kap)[0] > target: hi = mid
        else: lo = mid
    return lo
print("\n2^128 crossing log2c: lo-conv %.1f  mid %.1f  hi-conv %.1f" %
      (cross(3, kappa(5,8,100)), cross(4, kappa(5)), cross(12, kappa(5,20,400))))
# if the designer's 128 is read as group operations (1 group op = 800 words) the target in words is
print("2^128 group ops in words (800/op) = 2^%.1f ; crossing log2c = %.1f" % (128+lg(800), cross(4, kappa(5), 128+lg(800))))
# Block-Lanczos/no-normal-equation 2x saving
print("kappa halved: crossing %.1f" % cross(4, kappa(5)/2))
# JV (n-1): c~ break-even vs rho for the three rho prices
for gw in (500, 800, 1830):
    print("JV(n-1) full base break-even c~ (F_p ops, u=4): 2^%.1f  [rho %d words/op]" % (rho_words(gw) - (lg(12)+2*LGP+lg(4)), gw))
