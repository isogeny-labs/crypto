#!/usr/bin/env python3
"""Build curves and cell files for the coefficient/model Semaev study.

usage: setup.py <cell> <bits> <B> <T> <mode> <xcheck> <arms-spec> <outdir>
  arms-spec: comma list from E0,E0_AA,E0_ISO1,E0_ISO2,SMALL1..4,SMALL1_ISO,SMALL2_ISO,A3_1,A3_2,RANDk,PLANTED
  or 'side' for side study M.
All randomness derives from 20261109 and the cell name.
"""
import json, subprocess, sys, hashlib, os, random
import cypari2

pari = cypari2.Pari()
pari.allocatemem(2 * 10**9, silent=True) if hasattr(pari, "allocatemem") else None
HERE = os.path.dirname(os.path.abspath(__file__))
BIN = os.path.join(HERE, "target", "release", "cmsem")


def rng_for(tag):
    return random.Random(int(hashlib.sha256(f"20261109/{tag}".encode()).hexdigest(), 16))


def card(a, b, p):
    return int(pari.ellcard(pari.ellinit([a % p, b % p], p)))


def find(mode, p, n, k, arg):
    out = subprocess.run([BIN, "find", mode, str(p), "0", "0", str(n), str(k), str(arg)],
                         capture_output=True, text=True, check=True).stdout
    return json.loads(out)


def main():
    cell, bits, B, T, mode, xcheck, spec, outdir = sys.argv[1:9]
    bits, B, T, xcheck = int(bits), int(B), int(T), int(xcheck)
    os.makedirs(outdir, exist_ok=True)
    R = rng_for(cell)
    p = int(pari.nextprime(2**bits + R.randrange(2**(bits - 2))))
    while p % 4 != 3:
        p = int(pari.nextprime(p + 1))
    meta = {"cell": cell, "p": p, "p_bits": p.bit_length(), "B": B, "T": T, "mode": mode}
    arms = []  # (name, a, b, coord, alpha, s, lo, root, planted, seedtag)

    if spec == "side":
        # Montgomery y^2 = x^3 + A x^2 + x with #E = 4 n', n' prime
        tries = 0
        while True:
            tries += 1
            A = R.randrange(3, p - 3)
            if (A * A - 4) % p == 0:
                continue
            inv3 = pow(3, -1, p)
            a = (1 - A * A * inv3) % p
            b = ((2 * A**3 - 9 * A) * pow(27, -1, p)) % p
            N = card(a, b, p)
            if N % 4 == 0 and pari.isprime(N // 4):
                break
        alpha = A * inv3 % p
        meta.update({"A_mont": A, "a": a, "b": b, "order": N, "nprime": N // 4, "tries": tries})
        u = R.randrange(2, p - 1)
        aiso, biso = a * pow(u, 4, p) % p, b * pow(u, 6, p) % p
        assert card(aiso, biso, p) == N
        ngroup = N
        arms += [("W", a, b, "X", 0, 0, 0, "WS3", 0, "W"),
                 ("W_AA", a, b, "X", 0, 0, 0, "WS3", 0, "W_AA"),
                 ("W_ISO", aiso, biso, "X", 0, 0, 0, "WS3", 0, "W_ISO"),
                 ("MONT", a, b, "MONT", alpha, 1, 0, "MS3", 0, "MONT"),
                 ("MONT_NEG", a, b, "MONT", alpha, p - 1, 0, "MS3", 0, "MONT_NEG"),
                 ("ED_Y", a, b, "EDY", alpha, 1, 0, "NONE", 0, "ED_Y"),
                 ("ED_YSYM", a, b, "EDY", alpha, 1, p - B // 2, "NONE", 0, "ED_YSYM")]
    else:
        tries = 0
        while True:
            tries += 1
            a0, b0 = R.randrange(1, p), R.randrange(1, p)
            if (4 * a0**3 + 27 * b0**2) % p == 0:
                continue
            n = card(a0, b0, p)
            if not pari.isprime(n):
                continue
            t = p + 1 - n
            D = t * t - 4 * p
            d = int(pari.coredisc(D))
            f2 = D // d
            if f2 in (1, 4):
                break
        h = int(pari.qfbclassno(d))
        meta.update({"a0": a0, "b0": b0, "n": n, "trace": t, "D": D, "disc_fund": d,
                     "conductor": int(round(f2 ** 0.5)), "class_number": h, "e0_tries": tries,
                     "p_over_h": p / h})
        ngroup = n
        want = set(spec.split(","))
        members = {}
        nsmall = max([int(x[5:]) for x in want if x.startswith("SMALL") and "_" not in x] + [0])
        if nsmall:
            r = find("small", p, n, nsmall, 1 << 20)
            meta["search_small"] = {"tested": r["tested"], "secs": r["secs"], "found": r["found"]}
            ms = r["members"][:nsmall]
            for i, m in enumerate(ms, 1):
                assert card(m["a"], m["b"], p) == n, m
                members[f"SMALL{i}"] = (m["a"], m["b"], m["a_small"], m["b_small"])
        if any(x.startswith("A3_") for x in want):
            r = find("a3", p, n, 2, 1 << 30)
            meta["search_a3"] = {"tested": r["tested"], "secs": r["secs"], "found": r["found"]}
            for i, m in enumerate(r["members"][:2], 1):
                assert card(m["a"], m["b"], p) == n, m
                members[f"A3_{i}"] = (m["a"], m["b"], m["a_small"], m["b_small"])
        nrand = len([x for x in want if x.startswith("RAND")])
        if nrand:
            r = find("rand", p, n, nrand, int(hashlib.sha256(cell.encode()).hexdigest()[:8], 16))
            meta["search_rand"] = {"tested": r["tested"], "secs": r["secs"], "found": r["found"],
                                   "tested_per_member": r["tested"] / max(1, r["found"])}
            for i, m in enumerate(r["members"][:nrand], 1):
                assert card(m["a"], m["b"], p) == n, m
                members[f"RAND{i}"] = (m["a"], m["b"], None, None)
        meta["members"] = {k: {"a": v[0], "b": v[1], "a_small": v[2], "b_small": v[3]} for k, v in members.items()}

        def iso(a, b, tag):
            u = rng_for(cell + "/" + tag).randrange(2, p - 1)
            aa, bb = a * pow(u, 4, p) % p, b * pow(u, 6, p) % p
            assert card(aa, bb, p) == n
            return aa, bb

        order = ["E0", "E0_AA", "E0_ISO1", "E0_ISO2", "SMALL1", "SMALL2", "SMALL3", "SMALL4",
                 "SMALL1_ISO", "SMALL2_ISO", "A3_1", "A3_2"] + [f"RAND{i}" for i in range(1, 7)] + ["PLANTED"]
        for name in order:
            if name not in want:
                continue
            if name in ("E0", "E0_AA", "PLANTED"):
                a, b = a0, b0
            elif name.startswith("E0_ISO"):
                a, b = iso(a0, b0, name)
            elif name.endswith("_ISO"):
                base = members[name[:-4]]
                a, b = iso(base[0], base[1], name)
            else:
                a, b = members[name][0], members[name][1]
            meta.setdefault("arms", {})[name] = {"a": a, "b": b}
            arms.append((name, a, b, "X", 0, 0, 0, "WS3", 1 if name == "PLANTED" else 0, name))

    order_seed = int(hashlib.sha256(f"20261109/{cell}/order".encode()).hexdigest()[:15], 16)
    lines = [f"cell {cell} {p} {ngroup} {B} {T} {mode} 4 {order_seed} {xcheck}"]
    for (name, a, b, coord, alpha, s, lo, root, planted, tag) in arms:
        lines.append(f"arm {name} {a % p} {b % p} {coord} {alpha} {s} {lo} {root} {planted} 20261109/{cell}/{tag}")
    with open(os.path.join(outdir, f"{cell}.cell"), "w") as fh:
        fh.write("\n".join(lines) + "\n")
    with open(os.path.join(outdir, f"{cell}.meta.json"), "w") as fh:
        json.dump(meta, fh, indent=1, default=str)
    print(json.dumps({k: v for k, v in meta.items() if k not in ("members", "arms")}, default=str))


if __name__ == "__main__":
    main()
