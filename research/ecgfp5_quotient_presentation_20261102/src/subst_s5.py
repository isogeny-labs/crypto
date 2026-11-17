#!/usr/bin/env python3
"""Amendment A1, arm U2: eliminate s_m from the u-ring system by s_m := w^2 (s5 at m = 5).

    subst_s5.py <uring.ms> <uring.planted> <out.ms> <out.planted>

The u-ring system (qsolve generate --arms uring) is five descended equations in s1..s5, w plus
the quadric s5 - w^2 (last). Substituting s5 = w^2 into the five equations and dropping the
quadric gives an isomorphic zero-dimensional system in s1..s4, w (same points, same degree D).
Writes the system in msolve format (variables s1,s2,s3,s4,w; w last = eliminating variable)
and the planted point without its s5 coordinate. Exits 1 unless the planted point satisfies
every rewritten equation and its s5 equals w^2.
"""
import re
import sys

src, spl, out, opl = sys.argv[1:5]
lines = open(src).read().split("\n", 2)
vs = lines[0].strip().split(",")
p = int(lines[1].strip())
m = len(vs) - 1
assert vs == [f"s{i}" for i in range(1, m + 1)] + ["w"], vs
sm = f"s{m}"
polys = [q.strip() for q in lines[2].replace("\n", "").split(",") if q.strip()]
assert len(polys) == m + 1, len(polys)
quad = polys[-1].replace(" ", "")
assert quad in (f"{p - 1}*w^2+1*{sm}", f"1*{sm}+{p - 1}*w^2"), quad

NEW = vs[:m - 1] + ["w"]
TERM = re.compile(r"([+-]?)([^+-]+)")


def parse(poly):
    d = {}
    for sign, body in TERM.findall(poly):
        f = body.split("*")
        c = 1
        e = dict.fromkeys(vs, 0)
        for x in f:
            if re.fullmatch(r"\d+", x):
                c = c * int(x) % p
            else:
                v, _, k = x.partition("^")
                e[v] += int(k) if k else 1
        if sign == "-":
            c = -c % p
        ne = tuple(e[v] for v in vs[:m - 1]) + (e["w"] + 2 * e[sm],)
        d[ne] = (d.get(ne, 0) + c) % p
    return {k: c for k, c in d.items() if c}


def fmt(d):
    ts = []
    for e, c in sorted(d.items(), key=lambda t: (-sum(t[0]), [-x for x in t[0]])):
        mon = "*".join(v if k == 1 else f"{v}^{k}" for v, k in zip(NEW, e) if k)
        ts.append(f"{c}*{mon}" if mon else f"{c}")
    return "+".join(ts)


sys_new = [parse(q) for q in polys[:m]]
pl = [int(x) for x in open(spl).read().split("\n")[1].split(",")]
assert pow(pl[m], 2, p) == pl[m - 1] % p, "planted s_m != w^2"
pt = pl[:m - 1] + [pl[m]]


def ev(d):
    t = 0
    for e, c in d.items():
        for x, k in zip(pt, e):
            c = c * pow(x, k, p) % p
        t += c
    return t % p


ok = all(ev(d) == 0 for d in sys_new)
with open(out, "w") as f:
    f.write(",".join(NEW) + "\n" + str(p) + "\n" + ",\n".join(fmt(d) for d in sys_new) + "\n")
with open(opl, "w") as f:
    f.write(",".join(NEW) + "\n" + ",".join(map(str, pt)) + "\n")
print({"p": p, "terms": [len(d) for d in sys_new], "max_total_degree": [max(map(sum, d)) for d in sys_new],
       "max_w_degree": [max(e[-1] for e in d) for d in sys_new], "planted_satisfies": ok})
sys.exit(0 if ok else 1)
