#!/usr/bin/env python3
"""L4 analysis: evaluate the numbered predictions of PROTOCOL.md from the run JSONs.

  python3 analyze_l4.py --dir DIR --tag TAG [--k16 exhaustive_k16_std_TAG.json] ...

Reads, from DIR: exhaustive_k<k>_std_<tag>.json, sample_k<k>_std_<tag>.json (and the 'given'
variants if present), controls_k<k>_<tag>.json.  Writes DIR/results_<tag>.json and
DIR/results_<tag>.md.  Verdicts are PASS / FAIL against the thresholds written in PROTOCOL.md;
a FAIL stays a FAIL.  Every sentence of the .md carries [pcc:unverified].
"""
import argparse, json, math, os, sys

U = "[pcc:unverified]"


def load(path):
    if not os.path.exists(path):
        return None
    with open(path) as f:
        return json.load(f)


def rows_of(d, cls, degen=None):
    return [r for r in d["hist_rows"] if r["pair_class"] == cls and (degen is None or r["degenerate_a0"] == degen)]


def tot(rows):
    return sum(r["count"] for r in rows)


def rule3_upper(n):
    return 3.0 / n if n else float("nan")


def eval_run(d, ctl, label, exhaustive, predicted):
    """Evaluate P1-P6 on one run JSON.  Returns (list of dict verdicts, summary table rows)."""
    v = []
    facts = d["curve_facts"]
    # ---- table: per pair class, number of distinct F_p roots, roots by curve
    table = []
    for cls in ("EE", "ET", "TT", "ZE", "ZT", "ZZ"):
        for degen in (False, True):
            rs = rows_of(d, cls, degen)
            if not rs:
                continue
            n = tot(rs)
            by_n = {k: sum(r["count"] for r in rs if r["roots"] == k) for k in (0, 1, 2)}
            e = sum(r["count"] * r["roots_on_E"] for r in rs)
            t = sum(r["count"] * r["roots_on_twist"] for r in rs)
            z = sum(r["count"] * r["roots_on_Z"] for r in rs)
            table.append(dict(pair_class=cls, x1_eq_x2=degen, pairs=n, roots0=by_n[0], roots1=by_n[1],
                              roots2=by_n[2], root_on_E=e, root_on_twist=t, root_on_Z=z))
    # P1: ET pairs with an F_p root
    et = rows_of(d, "ET")
    n_et = tot(et)
    n_et_root = sum(r["count"] for r in et if r["roots"] > 0)
    v.append(dict(id="P1", text="(E,twist) pairs with any F_p root = 0", n=n_et, observed=n_et_root,
                  upper95_rate=rule3_upper(n_et) if n_et_root == 0 else None,
                  verdict="PASS" if n_et_root == 0 else "FAIL",
                  exceptions=d["et_with_root"][:50]))
    # P2: Z pairs
    ok2 = True
    det = []
    for cls, want_e, want_t in (("ZE", 1, 0), ("ZT", 0, 1)):
        rs = [r for r in rows_of(d, cls) if not r["degenerate_a0"]]
        n = tot(rs)
        good = sum(r["count"] for r in rs if r["roots"] == 1 and r["roots_on_E"] == want_e and r["roots_on_twist"] == want_t)
        det.append(dict(pair_class=cls, n=n, good=good))
        ok2 &= (good == n)
    v.append(dict(id="P2", text="(Z,E) pairs: exactly 1 root, on E; (Z,T) pairs: exactly 1 root, on twist; x1 != x2",
                  detail=det, verdict="PASS" if ok2 else "FAIL"))
    # P3: same-class non-Z nondegenerate pairs have exactly 2 distinct roots
    det, ok3 = [], True
    for cls in ("EE", "TT"):
        rs = rows_of(d, cls, False)
        n = tot(rs)
        two = sum(r["count"] for r in rs if r["roots"] == 2)
        det.append(dict(pair_class=cls, n=n, with_two_roots=two, fraction=(two / n if n else None)))
        ok3 &= (two == n)
    v.append(dict(id="P3", text="(E,E) and (T,T) pairs with x1 != x2: 100% have exactly 2 distinct F_p roots",
                  detail=det, verdict="PASS" if ok3 else "FAIL", exceptions=d["same_class_bad"][:50]))
    # P4: root curve
    bad_e = sum(r["count"] * r["roots_on_twist"] for r in d["hist_rows"] if r["pair_class"] == "EE")
    bad_t = sum(r["count"] * r["roots_on_E"] for r in d["hist_rows"] if r["pair_class"] == "TT")
    v.append(dict(id="P4", text="roots of (E,E) pairs on twist = 0; roots of (T,T) pairs on E = 0",
                  observed=dict(EE_roots_on_twist=bad_e, TT_roots_on_E=bad_t),
                  verdict="PASS" if bad_e == 0 and bad_t == 0 else "FAIL",
                  exceptions=d["same_class_wrong_curve"][:50]))
    # P5: Z-rooted pairs equal the phi_t prediction (exhaustive: exact; sample: rate within 4 sigma)
    if predicted:
        ee_z = sum(r["count"] for r in rows_of(d, "EE", False) if r["roots_on_Z"] > 0)
        tt_z = sum(r["count"] for r in rows_of(d, "TT", False) if r["roots_on_Z"] > 0)
        et_z = sum(r["count"] for r in rows_of(d, "ET") if r["roots_on_Z"] > 0)
        if exhaustive:
            ok5 = (ee_z == predicted["EE"] and tt_z == predicted["TT"] and et_z == predicted["ET"])
            v.append(dict(id="P5", text="pairs with a root on Z: counts equal the x(P+T) = phi_t(x) prediction exactly",
                          observed=dict(EE=ee_z, TT=tt_z, ET=et_z), predicted=predicted,
                          verdict="PASS" if ok5 else "FAIL"))
        else:
            ok5, det = True, []
            nE, nT = facts["n_x_E"], facts["n_x_T"]
            nsamp = {c: tot([r for r in rows_of(d, c, False)]) for c in ("EE", "TT")}
            # sampled strata are ordered pairs drawn with replacement; expected rate of Z-root
            for cls, obs, npred, nx in (("EE", ee_z, predicted["EE"], nE), ("TT", tt_z, predicted["TT"], nT)):
                rate = 2.0 * npred / (nx * nx)
                n = nsamp[cls]
                exp = rate * n
                sd = math.sqrt(max(exp, 1e-9))
                det.append(dict(pair_class=cls, n=n, observed=obs, expected=exp, sigma=sd, z=(obs - exp) / sd))
                ok5 &= abs(obs - exp) <= 4 * sd
            ok5 &= (et_z == 0)
            v.append(dict(id="P5", text="sampled pairs with a root on Z: rate equals phi_t prediction within 4 sigma; ET: 0",
                          detail=det, predicted=predicted, verdict="PASS" if ok5 else "FAIL"))
    # P6: x1 == x2: one finite root x(2P) on own class or Z
    det, ok6 = [], True
    for cls, bad in (("EE", "root_on_twist"), ("TT", "root_on_E")):
        rs = rows_of(d, cls, True)
        n = tot(rs)
        t = sum(r["count"] * r["roots_on_twist"] for r in rs)
        e = sum(r["count"] * r["roots_on_E"] for r in rs)
        one = sum(r["count"] for r in rs if r["roots"] == 1)
        det.append(dict(pair_class=cls, n=n, with_one_finite_root=one, roots_on_E=e, roots_on_twist=t))
        ok6 &= (one == n) and ((cls == "EE" and t == 0) or (cls == "TT" and e == 0))
    v.append(dict(id="P6", text="x1 == x2 pairs: exactly one finite root x(2P) on the point's own curve (or Z); the other root is infinity",
                  detail=det, verdict="PASS" if ok6 else "FAIL"))
    return v, table


def eval_controls(c):
    v = []
    c1, c2, c3, c4 = (c["C1_explicit_addition_EE_TT"], c["C2_mixed_in_Fp2"], c["C3_wrongA_sensitivity"],
                      c["C4_identities"])
    ok = all(c1[k]["match"] == c1[k]["tested"] and c1[k]["tested"] > 0 for k in ("EE", "TT"))
    v.append(dict(id="P7a", text="explicit point arithmetic: roots == {x(P1+P2), x(P1-P2)} for 100% of (E,E) and (T,T) sample pairs",
                  observed=c1, verdict="PASS" if ok else "FAIL"))
    ok = (c2["tested"] > 0 and c2["both_roots_verified"] == c2["tested"] and c2["conjugate"] == c2["tested"]
          and c2["not_in_Fp"] == c2["tested"] and c2["Fp_roots_found_by_counter"] == 0)
    v.append(dict(id="P7b", text="mixed pairs in F_p^2: both x(P1+-P2) are S3 roots, conjugate, not in F_p, for 100% of sampled pairs",
                  observed=c2, verdict="PASS" if ok else "FAIL"))
    ok = (c3["ET"]["true_A"]["frac_with_Fp_root"] == 0.0 and c3["EE"]["true_A"]["frac_with_Fp_root"] == 1.0
          and c3["TT"]["true_A"]["frac_with_Fp_root"] == 1.0
          and all(0.47 <= c3[k]["wrong_A_plus1"]["frac_with_Fp_root"] <= 0.53 for k in ("ET", "EE", "TT")))
    v.append(dict(id="P8", text="sensitivity: with the true A the F_p-root fractions are 0 (ET), 1 (EE, TT); with A+1 all three are in [0.47, 0.53]",
                  observed=c3, verdict="PASS" if ok else "FAIL"))
    g = c4["given_roots_are_inverses"]
    ok = c4["disc_equals_4_f1_f2"] and c4["same_root_count_std_vs_given"] and g["match"] == g["tested"] and g["tested"] > 0
    v.append(dict(id="P9", text="identities: disc/4 = 4 f(x1) f(x2); task-text S3 roots are the inverses of the standard S3 roots",
                  observed=c4, verdict="PASS" if ok else "FAIL"))
    return v


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--dir", required=True)
    ap.add_argument("--tag", required=True)
    ap.add_argument("--label", default="")
    ap.add_argument("--ks", default="16,20")
    a = ap.parse_args()
    out = dict(tag=a.tag, label=a.label, runs={}, controls={})
    md = [f"# L4 results ({a.tag}) {a.label}", "", f"Generated by analyze_l4.py. {U}", ""]
    for k in [int(x) for x in a.ks.split(",")]:
        c = load(os.path.join(a.dir, f"controls_k{k}_{a.tag}.json"))
        predicted = c.get("C5_predicted_Z_pairs") if c else None
        if c:
            cv = eval_controls(c)
            out["controls"][k] = cv
            md += [f"## k = {k} controls", ""] + [f"- {x['id']} {x['verdict']}: {x['text']} {U}" for x in cv] + [""]
            f = c["curve_facts"]
            md += [f"Curve facts k={k}: p={f['p']}, A={f['A']}, #E={f['#E']} = {f['#E_factored']}, twist order {f['#twist']} = "
                   f"{f['#twist_factored']}, x-classes Z/E/T = {f['n_x_Z']}/{f['n_x_E']}/{f['n_x_T']}, Z x = {f['Z_x_values']} {U}", ""]
        for mode in ("exhaustive", "sample"):
            for variant in ("std", "given"):
                d = load(os.path.join(a.dir, f"{mode}_k{k}_{variant}_{a.tag}.json"))
                if not d:
                    continue
                key = f"{mode}_k{k}_{variant}"
                complete = d.get("complete", True)
                v, table = eval_run(d, c, key, mode == "exhaustive" and complete, predicted if (variant == "std" and (mode == "sample" or complete)) else None)
                out["runs"][key] = dict(verdicts=v, table=table, npairs=d["npairs"], cpu_seconds=d["cpu_seconds"],
                                        complete=complete)
                md += [f"## {key}  (pairs {d['npairs']}, complete={complete}, {d['cpu_seconds']:.0f}s CPU)", "",
                       "| pair class | x1=x2 | pairs | 0 roots | 1 root | 2 roots | roots on E | roots on twist | roots on Z |",
                       "|:--|:--|--:|--:|--:|--:|--:|--:|--:|"]
                for t in table:
                    md.append(f"| {t['pair_class']} | {t['x1_eq_x2']} | {t['pairs']} | {t['roots0']} | {t['roots1']} | {t['roots2']} | "
                              f"{t['root_on_E']} | {t['root_on_twist']} | {t['root_on_Z']} |")
                md += [""] + [f"- {x['id']} {x['verdict']}: {x['text']} {U}" for x in v] + [""]
    with open(os.path.join(a.dir, f"results_{a.tag}.json"), "w") as f:
        json.dump(out, f, indent=1)
    with open(os.path.join(a.dir, f"results_{a.tag}.md"), "w") as f:
        f.write("\n".join(md) + "\n")
    print("\n".join(md))


if __name__ == "__main__":
    main()
