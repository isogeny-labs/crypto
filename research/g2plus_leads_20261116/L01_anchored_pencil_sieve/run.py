import argparse
import json
import math
import os
import sys
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from l01.instance import Tables, random_f, curve_seeds
from l01.curve import Curve
from l01 import pencil, baseline, rows as rowsmod, linalg
from l01.pencil import GateFailure, Budget
from l01.gate import run_gate


def dump(fh, obj):
    fh.write(json.dumps(obj, sort_keys=True) + "\n")
    fh.flush()


def make_budget(budget):
    t0 = time.process_time()

    def check():
        if time.process_time() - t0 > budget:
            raise Budget()
    return check


def instance(a):
    cs = curve_seeds(a.p, a.g, a.curve_index)[-1]
    f = random_f(a.p, a.g, cs)
    T = Tables(a.p, a.g, f)
    return cs, f, T


def base_rec(schema, arm, a, cs, f, T):
    return {"schema": schema, "arm": arm, "g": a.g, "p": a.p, "curve_index": a.curve_index,
            "curve_seed": cs, "solver_seed": a.seed, "f": f}


def finish_rows(rec, acc, T, N_target, rows_dir, arm, a, cs):
    rec.update(acc.stats())
    dg, lines = acc.digest(N_target)
    rec["rows_digest"] = dg
    if rows_dir and a.p <= 4099 and arm in ("P_full", "B_lab"):
        os.makedirs(rows_dir, exist_ok=True)
        hdr = "# L01.rows/v1 arm=%s g=%d p=%d curve_seed=%d solver_seed=%d n_cols=%d n_rows=%d" % (
            arm, a.g, a.p, cs, a.seed, T.n, len(lines))
        rowsmod.write_rows(os.path.join(rows_dir, "rows_%s_g%d_p%d_c%d_s%d.txt" % (
            arm, a.g, a.p, a.curve_index, a.seed)), hdr, lines)


def cmd_relphase(a):
    cs, f, T = instance(a)
    vc = Curve(a.p, a.g, f, T)
    N_target = T.N_target
    arms = a.arms.split(",")
    budget = make_budget(a.cpu_budget)
    exhausted = False
    ctx = {}
    with open(a.out, "w") as fh:
        for arm in arms:
            rec = base_rec("L01.relphase/v1", arm, a, cs, f, T)
            rec.update({"n_cols": T.n, "n_target": N_target, "setup_mul": T.ctr.mul,
                        "setup_inv": T.ctr.inv, "table_setup_mul": T.ctr.mul})
            if exhausted:
                rec["status"] = "not_run"
                dump(fh, rec)
                continue
            t0 = time.process_time()
            try:
                if arm == "P_full":
                    r = pencil.p_full(T, a.g, cs, a.seed, N_target, vc, budget_check=budget)
                    ctr, acc = r["ctr"], r["acc"]
                    ctx["first_cands"] = r["first_cands"]
                    rec.update({"status": r["status"], "relations": acc.count,
                                "duplicates": acc.duplicates, "mul": ctr.mul, "inv": ctr.inv,
                                "add": ctr.add, "fd_add": ctr.fd_add,
                                "bucket_ops": ctr.bucket_ops, "bucket_scan": ctr.bucket_scan,
                                "fpm": ctr.fpm(), "tuples": r["tuples"],
                                "per_tuple": r["per_tuple"], "split_model_ratio": r["split_model_ratio"]})
                    rec["fpm_per_rel"] = ctr.fpm() / acc.count if acc.count else None
                    rec.update(r["stat"])
                    finish_rows(rec, acc, T, N_target, a.rows_dir, arm, a, cs)
                elif arm == "P_naive1":
                    anchors = pencil.first_anchors(T, a.g, cs, a.seed)
                    r = pencil.p_naive1(T, a.g, cs, a.seed, anchors, budget_check=budget)
                    ctr = r["ctr"]
                    rs = r["relset"]
                    fc = ctx.get("first_cands")
                    rec.update({"status": "ok", "relations": len(rs), "duplicates": 0,
                                "mul": ctr.mul, "inv": ctr.inv, "add": ctr.add,
                                "fd_add": 0, "bucket_ops": 0, "bucket_scan": 0,
                                "fpm": ctr.fpm(), "relset_equal": (fc is not None and rs == fc),
                                "root_failures": r["root_failures"], "lambdas": a.p,
                                "max_bucket": 0, "v1_checked": 0, "v1_failed": 0,
                                "v2_checked": 0, "v2_failed": 0})
                    rec["fpm_per_rel"] = ctr.fpm() / len(rs) if rs else None
                    acc = rowsmod.Acc(T)
                    for lam, pts in sorted(rs):
                        acc.add(list(pts))
                    finish_rows(rec, acc, T, N_target, None, arm, a, cs)
                elif arm == "B_lab":
                    r = baseline.b_run(T, a.g, cs, a.seed, vc, N_target=N_target, budget_check=budget)
                    acc = r.pop("acc")
                    st = r.pop("status")
                    rec.update(r)
                    rec.update({"status": st, "relations": acc.count, "duplicates": acc.duplicates,
                                "mul": r["mul"], "inv": r["inv"], "add": r["rel_add"], "fd_add": 0,
                                "bucket_ops": 0, "bucket_scan": 0,
                                "fpm": r["mul"] + 10 * r["inv"], "max_bucket": 0})
                    rec["fpm_per_rel"] = rec["fpm"] / acc.count if acc.count else None
                    rec["setup_mul"], rec["setup_inv"] = r["setup_mul"], r["setup_inv"]
                    nm = math.comb(T.n, a.g) * 2 ** a.g / a.p ** a.g
                    rec["split_rate_model_ratio"] = ((r["splits"] / r["candidates"]) / nm
                                                     if r["candidates"] else None)
                    finish_rows(rec, acc, T, N_target, a.rows_dir, arm, a, cs)
                    if st == "budget_exhausted":
                        exhausted = True
                else:
                    raise SystemExit("unknown arm " + arm)
            except Budget:
                rec["status"] = "budget_exhausted"
                exhausted = True
            except GateFailure as e:
                rec["status"] = "gate_failure"
                rec["message"] = str(e)
                rec["cpu_seconds_diag"] = time.process_time() - t0
                dump(fh, rec)
                sys.exit(2)
            rec["cpu_seconds_diag"] = time.process_time() - t0
            dump(fh, rec)


def cmd_lp2(a):
    cs, f, T = instance(a)
    vc = Curve(a.p, a.g, f, T)
    budget = make_budget(a.cpu_budget)
    with open(a.out, "w") as fh:
        for arm in ("P_2lp", "B_2lp"):
            rec = base_rec("L01.lp2/v1", arm, a, cs, f, T)
            t0 = time.process_time()
            try:
                if arm == "P_2lp":
                    r = pencil.p_2lp(T, a.g, cs, a.seed, vc, budget_check=budget)
                    ctr = r["ctr"]
                    r.pop("ctr"); r.pop("kept")
                    r["mul"], r["inv"], r["add"] = ctr.mul, ctr.inv, ctr.add
                    r["fd_add"], r["bucket_ops"] = ctr.fd_add, ctr.bucket_ops
                    r["strong_fpm"] = None
                else:
                    r = baseline.b_run(T, a.g, cs, a.seed, vc, lp_mode=True, budget_check=budget)
                    r["add"] = r["rel_add"]
                rec.update(r)
                rec["fpm"] = rec["mul"] + 10 * rec["inv"]
                rec["fpm_per_rel"] = rec["fpm"] / rec["relations"] if rec["relations"] else None
                rec["fpm_per_cycle"] = rec["fpm"] / rec["cycles"] if rec["cycles"] else None
            except Budget:
                rec["status"] = "budget_exhausted"
            except GateFailure as e:
                rec["status"] = "gate_failure"
                rec["message"] = str(e)
                dump(fh, rec)
                sys.exit(2)
            rec["cpu_seconds_diag"] = time.process_time() - t0
            dump(fh, rec)


def cmd_la(a):
    with open(a.out, "w") as fh:
        for arm in ("P_full", "B_lab"):
            path = os.path.join(a.rows_dir, "rows_%s_g%d_p%d_c%d_s%d.txt" % (arm, a.g, a.p, a.curve_index, a.seed))
            hdr, rws, lines = rowsmod.read_rows(path)
            import hashlib
            dg = hashlib.sha256("\n".join(lines).encode()).hexdigest()
            cs, f, T = instance(a)
            r = linalg.sparse_eliminate(rws, T.n)
            rec = base_rec("L01.la/v1", arm, a, cs, f, T)
            rec.update({"rows": len(rws), "cols": T.n, "q": linalg.Q61, "rows_digest": dg})
            rec.update(r)
            dump(fh, rec)


def cmd_gate(a):
    t0 = time.process_time()
    budget = make_budget(a.cpu_budget)
    try:
        rec = run_gate(a.g, a.p, budget)
    except Budget:
        rec = {"schema": "L01.gate/v1", "g": a.g, "p": a.p, "status": "budget_exhausted", "pass": False}
    except GateFailure as e:
        rec = {"schema": "L01.gate/v1", "g": a.g, "p": a.p, "status": "gate_failure",
               "message": str(e), "pass": False}
    rec["cpu_seconds_diag"] = time.process_time() - t0
    with open(a.out, "w") as fh:
        dump(fh, rec)
    if rec["status"] == "gate_failure":
        sys.exit(2)


def main():
    ap = argparse.ArgumentParser()
    sub = ap.add_subparsers(dest="cmd", required=True)
    for name in ("gate", "relphase", "lp2", "la"):
        s = sub.add_parser(name)
        s.add_argument("--g", type=int, required=True)
        s.add_argument("--p", type=int, required=True)
        s.add_argument("--out", required=True)
        s.add_argument("--cpu-budget", type=float, default=540.0)
        if name != "gate":
            s.add_argument("--curve-index", type=int, required=True)
            s.add_argument("--seed", type=int, required=True)
        if name in ("relphase", "la"):
            s.add_argument("--rows-dir", default=None)
        if name == "relphase":
            s.add_argument("--arms", required=True)
    a = ap.parse_args()
    {"gate": cmd_gate, "relphase": cmd_relphase, "lp2": cmd_lp2, "la": cmd_la}[a.cmd](a)


if __name__ == "__main__":
    main()
