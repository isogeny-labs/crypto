#!/usr/bin/env python3
"""Compare an independent replay of the k = 4 pipeline runs with the committed records.

Validator tooling (2026-10-21).  Stdlib only.  For each committed run file
`<committed_dir>/p<P>s<S>.json` the replay file of the same name under `<replay_dir>` is
compared leaf by leaf: every field the committed record carries must be present in the
replay and equal (integers and floats compared exactly), except the wall-clock fields
`wall_ms` and `solve_ms`, which are reported but not compared.  Fields the replay carries
and the committed record does not (code added after the recorded binary) are listed; for
E19/E24 the added `rho_negation_*` fields must equal the committed matched (negation)
walk means.  The instance file is the output of the validator's independent Python check
(`check_instances.py`): planted scalar d, [d]G = Q, [n]G = O, n prime, #E = h n.

    replay_compare.py --spec SPEC.json --out replay-certificate.json

SPEC.json (paths relative to the repository root, which defaults to three levels above this file):
           {"evidence_dir": "benchmarks/<dir>",
            "experiments": [{"id": "e24", "committed_dir": "...", "replay_dir": "...",
                             "runs": [[p, s], ...]}], "instances": "...json",
            "extra": {...merged into the certificate...}}
"""
import argparse, hashlib, json, os, sys

TIMING = {"wall_ms", "solve_ms"}


def sha(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest()


def leaves(a, b, path, out):
    """Compare committed `a` against replay `b`; append (path, a, b) for every leaf."""
    if isinstance(a, dict):
        if not isinstance(b, dict):
            out.append((path, a, b, False))
            return
        for k in a:
            p = f"{path}.{k}" if path else k
            if k in TIMING and not path:
                continue
            if k not in b:
                out.append((p, a[k], "<missing in replay>", False))
                continue
            leaves(a[k], b[k], p, out)
    elif isinstance(a, list):
        if not isinstance(b, list) or len(a) != len(b):
            out.append((path, f"list[{len(a)}]", f"list[{len(b) if isinstance(b, list) else '?'}]", False))
            return
        for i, (x, y) in enumerate(zip(a, b)):
            leaves(x, y, f"{path}[{i}]", out)
    else:
        same = (a == b) and (type(a) is type(b) or (isinstance(a, (int, float)) and isinstance(b, (int, float))
                                                  and not isinstance(a, bool) and not isinstance(b, bool)))
        out.append((path, a, b, same))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--spec", required=True)
    ap.add_argument("--repo", default=os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "..")))
    ap.add_argument("--out", required=True)
    a = ap.parse_args()
    spec = json.load(open(a.spec))
    repo = spec.get("repo") or a.repo
    J = lambda q: q if os.path.isabs(q) else os.path.join(repo, q)
    ev = J(spec["evidence_dir"])
    inst = {(r["experiment"], r["p"], r["seed"]): r for r in json.load(open(J(spec["instances"])))}
    runs, ok_all, scalars_all = [], True, True
    for ex in spec["experiments"]:
        if ex.get("whole_file"):
            # A gate run recorded as one file (E25 Stage A): every field of every row compared.
            cpath = os.path.join(repo, ex["committed"])
            rpath = J(ex["replay"])
            com, rep = json.load(open(cpath)), json.load(open(rpath))
            cmp_ = []
            leaves(com, rep, "", cmp_)
            bad = [(k, x, y) for k, x, y, same in cmp_ if not same]
            matched = not bad and len(com) == len(rep)
            ok_all &= matched
            runs.append({
                "experiment": ex["id"], "command": ex.get("command"),
                "committed": {"path": ex["committed"], "sha256": sha(cpath)},
                "replay_output": {"path": os.path.relpath(rpath, ev), "sha256": sha(rpath)},
                "replayed": True, "rows": len(rep),
                "fields_compared": len(cmp_), "fields_identical": len(cmp_) - len(bad),
                "mismatches": [{"field": k, "committed": x, "replay": y} for k, x, y in bad][:50],
                "matched": matched,
                "note": ex.get("note", ""),
            })
            continue
        for p, s in ex["runs"]:
            cpath = os.path.join(repo, ex["committed_dir"], f"p{p}s{s}.json")
            rpath = os.path.join(J(ex["replay_dir"]), f"p{p}s{s}.json")
            if not os.path.isfile(rpath):
                runs.append({"experiment": ex["id"], "p": p, "seed": s, "replayed": False,
                             "matched": False, "note": "no replay output"})
                ok_all = scalars_all = False
                continue
            com, rep = json.load(open(cpath)), json.load(open(rpath))
            if len(com) != len(rep):
                runs.append({"experiment": ex["id"], "p": p, "seed": s, "replayed": True, "matched": False,
                             "note": f"{len(com)} committed rows, {len(rep)} replay rows"})
                ok_all = scalars_all = False
                continue
            I = inst.get((ex["id"], p, s))
            for rc, rr in zip(com, rep):
                cmp_ = []
                leaves(rc, rr, "", cmp_)
                bad = [(k, x, y) for k, x, y, same in cmp_ if not same]
                added = sorted(set(rr) - set(rc))
                added_checks = {}
                if "rho_negation_muls" in added:
                    added_checks["rho_negation_muls == committed rho_matched_muls"] = rr["rho_negation_muls"] == rc["rho_matched_muls"]
                    added_checks["rho_negation_steps == committed rho_matched_steps"] = rr["rho_negation_steps"] == rc["rho_matched_steps"]
                    added_checks["matched_fold == Negation"] = rr.get("matched_fold") == "Negation"
                walks = rr["rho"]
                kinds = {}
                for w in walks:
                    kinds[w["fold"]] = kinds.get(w["fold"], 0) + 1
                log_ok = bool(rr["folded"]["solved"] and rr["folded"]["correct"] and rc["folded"]["correct"])
                inst_ok = bool(I and I["all_checks_pass"] and I["n"] == rr["n"]
                               and I["group_order"] == rr["group_order"] and I["cofactor"] == rr["cofactor"])
                walks_ok = all(w["correct"] for w in walks)
                matched = not bad and all(added_checks.values())
                verified = matched and log_ok and inst_ok and walks_ok
                ok_all &= matched
                scalars_all &= verified
                rec = {
                    "experiment": ex["id"], "p": p, "seed": s,
                    "arm": rc.get("arm"),
                    "committed": {"path": f"{ex['committed_dir']}/p{p}s{s}.json", "sha256": sha(cpath)},
                    "replay_output": {"path": os.path.relpath(rpath, ev), "sha256": sha(rpath)},
                    "replayed": True,
                    "fields_compared": len(cmp_),
                    "fields_identical": len(cmp_) - len(bad),
                    "mismatches": [{"field": k, "committed": x, "replay": y} for k, x, y in bad][:50],
                    "timing_not_compared": {k: {"committed": rc.get(k), "replay": rr.get(k)} for k in sorted(TIMING)},
                    "fields_added_in_replay_binary": added,
                    "added_field_cross_checks": added_checks,
                    "matched": matched,
                    "logarithm": {
                        "replay_solved": rr["folded"]["solved"], "replay_correct": rr["folded"]["correct"],
                        "committed_correct": rc["folded"]["correct"],
                        "planted_d": I["planted_d"] if I else None,
                        "independent_dG_eq_Q": I["checks"]["dG_eq_Q"] if I else None,
                        "independent_instance_checks_pass": I["all_checks_pass"] if I else None,
                    },
                    "walks": {"count": len(walks), "by_fold": kinds, "all_correct": walks_ok,
                              "identical_to_committed": rc["rho"] == rr["rho"]},
                    "counters": {k: rr[k] for k in ("residuals", "unsolved", "decompositions", "total_muls",
                                                     "solve_muls", "la_fp_muls", "macaulay_entries",
                                                     "rho_matched_muls", "rho_matched_steps", "ratio")},
                    "relations": rr["folded"]["relations"], "columns": rr["folded"]["columns"],
                    "la_ops": rr["folded"]["la_ops"],
                    "scalar_verified": verified,
                }
                runs.append(rec)
    cert = dict(spec.get("extra", {}))
    cert.update({
        "status": "verified" if ok_all and scalars_all else "failed",
        "independent": True,
        "scalars_verified": bool(scalars_all),
        "runs_total": len(runs),
        "runs_matched": sum(1 for r in runs if r.get("matched")),
        "runs": runs,
    })
    json.dump(cert, open(a.out, "w"), indent=1, ensure_ascii=False)
    open(a.out, "a").write("\n")
    print(f"{a.out}: status {cert['status']}, {cert['runs_matched']}/{len(runs)} matched, scalars_verified {cert['scalars_verified']}")
    for r in runs:
        if not r.get("matched"):
            print("MISMATCH", r["experiment"], r["p"], r["seed"], r.get("arm"), r.get("mismatches", r.get("note"))[:5])
    return 0 if cert["status"] == "verified" else 1


if __name__ == "__main__":
    sys.exit(main())
