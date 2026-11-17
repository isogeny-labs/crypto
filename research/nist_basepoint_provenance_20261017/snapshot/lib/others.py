"""Seeds and base points not covered by the blog post: NIST binary B-curves (FIPS 186-2), base points."""
from curves import CURVES, h
# name: (m, seed, b, Gx, Gy)  -- FIPS 186-2 Appendix 6, recalled; checked below by seed->b derivation and G on-curve
BCURVES = {
 "B-163": (163, "85e25bfe5c86226cdb12016f7553f9d0e693a268",
   h("2 0a601907 b8c953ca 1481eb10 512f7874 4a3205fd"),
   h("3 f0eba162 86a2d57e a0991168 d4994637 e8343e36"), h("0 d51fbc6c 71a0094f a2cdd545 b11c5c0c 797324f1")),
 "B-233": (233, "74d59ff07f6b413d0ea14b344b20a2db049b50c3",
   h("066 647ede6c 332c7f8c 0923bb58 213b333b 20e9ce42 81fe115f 7d8f90ad"),
   h("0fa c9dfcbac 8313bb21 39f1bb75 5fef65bc 391f8b36 f8f8eb73 71fd558b"), h("100 6a08a419 03350678 e58528be bf8a0bef f867a7ca 36716f7e 01f81052")),
 "B-283": (283, "77e2b07370eb0f832a6dd5b62dfc88cd06bb84be",
   h("27b680a c8b8596d a5a4af8a 19a0303f ca97fd76 45309fa2 a581485a f6263e31 3b79a2f5"),
   h("5f93925 8db7dd90 e1934f8c 70b0dfec 2eed25b8 557eac9c 80e2e198 f8cdbecd 86b12053"), h("3676854 fe24141c b98fe6d4 b20d02b4 516ff702 350eddb0 826779c8 13f0df45 be8112f4")),
 "B-409": (409, "4099b5a457f9d69f79213d094c4bcd4d4262210b",
   h("021a5c2 c8ee9feb 5c4b9a75 3b7b476b 7fd6422e f1f3dd67 4761fa99 d6ac27c8 a9a197b2 72822f6c d57a55aa 4f50ae31 7b13545f"),
   h("15d4860 d088ddb3 496b0c60 64756260 441cde4a f1771d4d b01ffe5b 34e59703 dc255a86 8a118051 5603aeab 60794e54 bb7996a7"), h("061b1cf ab6be5f3 2bbfa783 24ed106a 7636b9c5 a7bd198d 0158aa4f 5488d08f 38514f1f df4b4f40 d2181b36 81c364ba 0273c706")),
 "B-571": (571, "2aa058f73a0e33ab486b0f610410c53a7f132310",
   h("2f40e7e 2221f295 de297117 b7f3d62f 5c6a97ff cb8ceff1 cd6ba8ce 4a9a18ad 84ffabbd 8efa5933 2be7ad67 56a66e29 4afd185a 78ff12aa 520e4de7 39baca0c 7ffeff7f 2955727a"),
   h("303001d 34b85629 6c16c0d4 0d3cd775 0a93d1d2 955fa80a a5f40fc8 db7b2abd bde53950 f4c0d293 cdd711a3 5b67fb14 99ae6003 8614f139 4abfa3b4 c850d927 e1e7769c 8eec2d19"),
   h("37bf273 42da639b 6dccfffe b73d69d7 8c6c27a6 009cbbca 1980f853 3921e8a6 84423e43 bab08a57 6291af8f 461bb2a8 b3531d2f 0485c19b 16e2f151 6e23dd3c 1a4827af 1b8ac15b")),
}
# reduction polynomials (exponents) for B/K curves
POLY = {163: (163, 7, 6, 3, 0), 233: (233, 74, 0), 283: (283, 12, 7, 5, 0), 409: (409, 87, 0), 571: (571, 10, 5, 2, 0)}

import hashlib
def x962_binary_b(seed: bytes, m: int) -> int:
    s = (m - 1) // 160; hb = m - 160 * s
    H = int.from_bytes(hashlib.sha1(seed).digest(), "big")
    W = H & ((1 << hb) - 1)
    z = int.from_bytes(seed, "big")
    for i in range(1, s + 1):
        W = (W << 160) | int.from_bytes(hashlib.sha1(((z + i) % (1 << 160)).to_bytes(20, "big")).digest(), "big")
    return W

def gf2_mul(a, b, m):
    poly = sum(1 << e for e in POLY[m]); r = 0
    while b:
        if b & 1: r ^= a
        b >>= 1; a <<= 1
        if a >> m & 1: a ^= poly
    return r

def on_curve_binary(m, b, x, y):  # y^2 + xy = x^3 + x^2 + b  (a = 1)
    x2 = gf2_mul(x, x, m)
    return gf2_mul(y, y, m) ^ gf2_mul(x, y, m) == gf2_mul(x2, x, m) ^ x2 ^ b
