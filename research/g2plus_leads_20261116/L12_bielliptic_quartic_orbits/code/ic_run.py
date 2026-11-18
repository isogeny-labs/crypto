"""One IC run (SPEC.md section 8)."""
import sys, os, json, time, random, hashlib, argparse, math
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from fp import Ops, Fp
from curve import Curve, setup_points, emul, free_field
from arms import column_map
from section import section, verify_relation, make_row
import la


def stream_pairs(ss, N_aff, n):
    rng = random.Random(ss)
    out = []
    for t in range(n):
        i = t % N_aff
        j = rng.randrange(N_aff - 1)
        if j >= i:
            j += 1
        out.append((i, j))
    return out


def stream_digest(ss, N_aff):
    s = "".join("%d,%d;" % ij for ij in stream_pairs(ss, N_aff, 1000))
    return hashlib.sha256(s.encode()).hexdigest()


def run(inst, s, arm, tamper_check=True):
    t0 = time.time()
    p = inst["p"]
    cv = Curve(p, inst["a1"], inst["a3"], inst["a2"], inst["a4"], inst["a6"])
    r, c, cs = inst["r"], inst["c"], inst["cs"]
    ops_setup = Ops()
    F = Fp(p, ops_setup)
    S = setup_points(F, cv)
    points, index, N_aff = S.points, S.index, S.N_aff
    # anchor
    i0 = None
    for i in range(N_aff):
        Z = emul(F, cv, (points[i][0], points[i][2]), c)
        if Z is not None:
            i0 = i
            break
    s_eff = s + 100 if arm == "Bp" else s
    ss = 7919 * cs + s_eff
    col, sign, reps = column_map(arm, points, cv, ss)
    ncols = len(reps)
    j0 = col[i0]
    digest = stream_digest(ss, N_aff)

    ops_col = Ops()
    Fc = Fp(p, ops_col)
    Fc.SQRT = F.SQRT
    rng = random.Random(ss)
    state = {"t": 0}
    rows = []
    rowset = set()
    cover = [0] * ncols
    covered = [0]
    cnt = {"split": 0, "dup": 0, "zero": 0, "nonsplit": 0, "verified": 0,
           "g2_fail": 0, "g3_fail": 0}
    first_rel = [None]
    status = [None]
    cap = 40 * N_aff

    def collect(target_extra=None):
        """Collect until stop rule (target_extra None) or target_extra more rows kept."""
        kept = 0
        while True:
            if target_extra is None:
                if len(rows) >= ncols + 20 and covered[0] == ncols:
                    return True
            else:
                if kept >= target_extra:
                    return True
            t = state["t"]
            if t >= cap:
                status[0] = "trial_cap"
                return False
            state["t"] = t + 1
            i = t % N_aff
            j = rng.randrange(N_aff - 1)
            if j >= i:
                j += 1
            st, terms = section(Fc, cv, points, index, i, j)
            if st == "nonsplit":
                cnt["nonsplit"] += 1
                continue
            cnt["split"] += 1
            v = verify_relation(cv, points, i, j, terms)
            if not v["G2"]:
                cnt["g2_fail"] += 1
                raise RuntimeError("G2 failure at trial %d" % t)
            if not v["G3"]:
                cnt["g3_fail"] += 1
                status[0] = "G3_fail"
                return False
            cnt["verified"] += 1
            if first_rel[0] is None:
                first_rel[0] = (i, j, dict(terms))
            key = make_row(terms, col, sign)
            if not key:
                cnt["zero"] += 1
                continue
            if key in rowset:
                cnt["dup"] += 1
                continue
            rowset.add(key)
            rows.append(key)
            kept += 1
            for cj, _ in key:
                if cover[cj] == 0:
                    covered[0] += 1
                cover[cj] += 1

    ops_la = Ops()
    FFc = free_field(p)
    Zs = [emul(FFc, cv, reps[jj], c) for jj in range(ncols)]
    cert_rejected = [0]

    def certify(xv):
        Z0 = Zs[j0]
        return sum(1 for jj in range(ncols) if emul(FFc, cv, Z0, xv[jj]) != Zs[jj])
    xs = None
    attempts = 0
    ok = collect()
    if ok:
        rows_used = len(rows)
        while True:
            attempts += 1
            x = la.attempt(rows, ncols, j0, r, ss, attempts, ops_la)
            if x is not None:
                xs_try = [0] * ncols
                xs_try[j0] = 1
                for cc in range(ncols):
                    if cc != j0:
                        xs_try[cc] = x[cc if cc < j0 else cc - 1]
                # DEVIATION D1: uncounted certification inside the attempt-success test
                if certify(xs_try) == 0:
                    xs = xs_try
                    status[0] = "ok"
                    break
                cert_rejected[0] += 1
            if attempts >= 12:
                status[0] = "la_fail"
                break
            if attempts % 2 == 0:
                if not collect(math.ceil(0.05 * ncols) + 1):
                    break
    # mark attempts
    out = {
        "family": inst["family"], "p": p, "k": inst["k"], "cs": cs, "j": inst["j"], "s": s,
        "arm": arm, "ss": ss, "status": status[0], "N_aff": N_aff, "N_ram": S.N_ram,
        "ncols": ncols, "anchor_point": i0, "anchor_col": j0,
        "trials": state["t"], "split_trials": cnt["split"], "relations": len(rows),
        "duplicate_rows": cnt["dup"], "zero_rows": cnt["zero"], "nonsplit": cnt["nonsplit"],
    }
    # matrix dims at the final attempt
    A_W = 0
    for row in rows:
        A_W += sum(1 for cc, _ in row if cc != j0)
    out.update({"rows_used": len(rows), "R": len(rows), "N": ncols - 1, "W": A_W,
                "la_attempts": attempts})
    out["setup"] = ops_setup.as_dict()
    out["collect"] = ops_col.as_dict()
    out["la"] = ops_la.as_dict()
    out["SETUP_FPM"] = ops_setup.fpm()
    out["COLLECT_FPM"] = ops_col.fpm()
    out["LA_FPM"] = ops_la.fpm()
    out["TOTAL"] = out["SETUP_FPM"] + out["COLLECT_FPM"] + out["LA_FPM"]
    out["g2_fail_count"] = cnt["g2_fail"]
    out["g3_fail_count"] = cnt["g3_fail"]
    out["relations_verified"] = cnt["verified"]
    # certification
    certified = False
    cert_fail = None
    sigma_equal = None
    if xs is not None:
        cert_fail = certify(xs)
        certified = (cert_fail == 0)
        if arm == "A":
            byx = {}
            for i, q in enumerate(points):
                byx.setdefault(q[0], []).append(i)
            tot = eq = 0
            for lst in byx.values():
                for a_ in range(len(lst)):
                    for b_ in range(a_ + 1, len(lst)):
                        i, i2 = lst[a_], lst[b_]
                        if points[i][1] != 0 and points[i2][1] == p - points[i][1]:
                            tot += 1
                            if xs[i] == xs[i2]:
                                eq += 1
            sigma_equal = (eq / tot) if tot else None
    out["cert_rejected_attempts"] = cert_rejected[0]
    out["certified"] = certified
    out["cert_fail_cols"] = cert_fail
    out["sigma_equal"] = sigma_equal
    tamper = None
    if arm == "A" and p == 263 and first_rel[0] is not None:
        i1, i2, terms = first_rel[0]
        others = sorted(k for k in terms if k not in (i1, i2))
        k = others[0] if others else i2
        nk = (k + 1) % N_aff
        if nk in terms:
            nk = (k + 2) % N_aff
        nt = dict(terms)
        mult = nt.pop(k)
        nt[nk] = nt.get(nk, 0) + mult
        v = verify_relation(cv, points, i1, i2, nt)
        tamper = not (v["G2"] and v["G3"])
    out["tamper_rejected"] = tamper
    out["stream_digest"] = digest
    out["python_version"] = sys.version.split()[0]
    out["wall_s"] = round(time.time() - t0, 3)
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--instance-json", required=True)
    ap.add_argument("--s", type=int, required=True)
    ap.add_argument("--arm", required=True, choices=["A", "B", "C", "Bp", "N"])
    ap.add_argument("--out", required=True)
    a = ap.parse_args()
    inst = json.loads(a.instance_json)
    out = run(inst, a.s, a.arm)
    with open(a.out, "w") as f:
        json.dump(out, f, sort_keys=True)
        f.write("\n")


if __name__ == "__main__":
    main()
