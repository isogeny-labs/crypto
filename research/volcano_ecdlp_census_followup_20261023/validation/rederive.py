#!/usr/bin/env python3
"""Validator-written raw-record re-derivation (follow-up study).

Independent of the producer's pipeline (does not import or read analysis/*.py).
For every session with a `done` line in runs/progress.jsonl:

* checks the committed archive's SHA-256 against the progress line;
* streams <top>/records.jsonl out of the archive (no disk), parses every
  record and writes one compact row per record to OUTDIR/<session>.csv.gz;
* the A2.1 uncharged count is re-derived with this file's own reading of the
  rule: for each counter named in cost.unpriced, the record's top-level
  `counters` value if present, else the sum over phases of native[name without
  the `_uncharged` suffix];
* every distinct (curve, generator, target, recovered) is checked with this
  file's own affine group law: [recovered]G == Q, Q on the curve and [r]G == O.

Usage: python3 -I rederive.py STUDY_DIR OUTDIR [SESSION ...]
Writes OUTDIR/sessions.jsonl (one summary per session).
"""
import csv
import gzip
import hashlib
import io
import json
import math
import os
import subprocess
import sys

STUDY, OUT = sys.argv[1], sys.argv[2]
ONLY = set(sys.argv[3:])
os.makedirs(OUT, exist_ok=True)

COLS = ["session", "spec", "V", "fam", "lvl", "cid", "seq", "round", "warmup", "arm",
        "wid", "slug", "p", "a", "b", "gx", "gy", "r", "tseed", "tidx", "planted", "recovered",
        "status", "matches_planted", "matches_target", "total_gae", "s", "s_recomputed_ok",
        "unc", "unc_detail", "unc_source", "solve_wall_ns", "iso", "binary_ok", "scalar_ok"]

BIN = "ac4bc5b26f858a4d8a374208297157840af25fb1dc25ea1c19dd41360735fb35"


def inv(x, p):
    return pow(x, p - 2, p)


def ec_add(P, Q, a, p):
    if P is None:
        return Q
    if Q is None:
        return P
    x1, y1 = P
    x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % p == 0:
            return None
        lam = (3 * x1 * x1 + a) * inv(2 * y1, p) % p
    else:
        lam = (y2 - y1) * inv(x2 - x1, p) % p
    x3 = (lam * lam - x1 - x2) % p
    return (x3, (lam * (x1 - x3) - y1) % p)


def ec_mul(k, P, a, p):
    R = None
    while k:
        if k & 1:
            R = ec_add(R, P, a, p)
        P = ec_add(P, P, a, p)
        k >>= 1
    return R


_scal = {}


def scalar_ok(p, a, b, g, q, k, r):
    key = (p, a, b, g, q, k, r)
    if key in _scal:
        return _scal[key]
    on_curve = (q[1] * q[1] - (q[0] ** 3 + a * q[0] + b)) % p == 0
    gkey = (p, a, b, g, r)
    if gkey not in _scal:
        gon = (g[1] * g[1] - (g[0] ** 3 + a * g[0] + b)) % p == 0
        _scal[gkey] = gon and ec_mul(r, g, a, p) is None
    ok = on_curve and _scal[gkey] and ec_mul(k % r, g, a, p) == q
    _scal[key] = ok
    return ok


def uncharged(rec):
    tot = 0
    det = []
    src = set()
    counters = rec.get("counters") or {}
    for name in rec["cost"].get("unpriced") or []:
        if name in counters:
            v = counters[name]
            src.add("counters")
        else:
            base = name[:-len("_uncharged")] if name.endswith("_uncharged") else name
            v = sum((ph.get("native") or {}).get(base, 0) for ph in rec.get("phases") or [])
            src.add("phases")
        tot += int(v)
        det.append(f"{name}={int(v)}")
    return tot, ";".join(det), "+".join(sorted(src)) or "none"


def sha256_file(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def main():
    prog = [json.loads(l) for l in open(os.path.join(STUDY, "runs/progress.jsonl"))]
    done = [d for d in prog if d.get("status") == "done"]
    summ = open(os.path.join(OUT, "sessions.jsonl"), "a")
    for d in done:
        sess = os.path.basename(d["out"])
        if ONLY and sess not in ONLY:
            continue
        spec = d["spec"]
        parts = spec.split("-")
        V, fam, lvl, cid = parts[0], parts[1], int(parts[2][1:]), int(parts[3][1:])
        arc = os.path.join(STUDY, "sessions", sess + ".tar.xz")
        arc_sha = sha256_file(arc)
        proc = subprocess.Popen(["tar", "-xJOf", arc, "--wildcards", "*/records.jsonl"],
                                stdout=subprocess.PIPE)
        n = nm = nw = unver = scal_bad = s_bad = bin_bad = 0
        sids = set()
        isos = {}
        with gzip.open(os.path.join(OUT, sess + ".csv.gz"), "wt", newline="") as gz:
            w = csv.writer(gz)
            w.writerow(COLS)
            for line in io.TextIOWrapper(proc.stdout, encoding="utf-8"):
                rec = json.loads(line)
                n += 1
                sids.add(rec["session_id"])
                wl = rec["workload"]
                cs = wl["curve_spec"]
                p, a, b = int(cs["p"]), int(cs["a"]), int(cs["b"])
                g = (int(cs["gx"]), int(cs["gy"]))
                r = int(cs["r"])
                q = (int(wl["target"][0], 16), int(wl["target"][1], 16))
                planted = int(wl["planted"])
                out = rec["outcome"]
                rec_k = out.get("recovered")
                ok = rec_k is not None and scalar_ok(p, a, b, g, q, int(rec_k), r)
                tg = float(rec["cost"]["total_gae"])
                s = float(rec["cost"]["s"])
                s_ok = abs(s - tg / math.sqrt(r)) <= 1e-12 * max(1.0, s)
                unc, det, src = uncharged(rec)
                iso = (rec.get("isolation") or {}).get("level")
                warm = bool(rec["warmup"])
                bok = rec.get("binary_sha256") == BIN
                if warm:
                    nw += 1
                else:
                    nm += 1
                    isos[iso] = isos.get(iso, 0) + 1
                    if out.get("status") != "verified":
                        unver += 1
                if not ok:
                    scal_bad += 1
                if not s_ok:
                    s_bad += 1
                if not bok:
                    bin_bad += 1
                w.writerow([sess, spec, V, fam, lvl, cid, rec["seq"], rec["round"], int(warm), rec["arm"],
                            wl["workload_id"], wl["curve"]["slug"], p, a, b, g[0], g[1], r,
                            wl["target_seed"], wl["target_index"], planted, rec_k,
                            out.get("status"), int(bool(out.get("matches_planted"))),
                            int(bool(out.get("matches_target"))), repr(tg), repr(s), int(s_ok),
                            unc, det, src, (rec.get("time") or {}).get("solve_wall_ns"), iso,
                            int(bok), int(ok)])
        rc = proc.wait()
        summ.write(json.dumps({
            "session": sess, "spec": spec, "tar_rc": rc,
            "archive_sha256": arc_sha, "progress_sha256": d["sha256_tar_xz"],
            "archive_sha256_match": arc_sha == d["sha256_tar_xz"],
            "records": n, "progress_runs": d["runs"], "records_match_progress_runs": n == d["runs"],
            "measured": nm, "warmup": nw, "unverified_measured": unver,
            "scalar_check_failures": scal_bad, "s_recompute_failures": s_bad,
            "binary_mismatch_records": bin_bad, "session_ids": sorted(sids),
            "isolation_measured": isos}) + "\n")
        summ.flush()
        print(sess, n, nm, nw, unver, scal_bad, s_bad, bin_bad, arc_sha == d["sha256_tar_xz"], flush=True)


if __name__ == "__main__":
    main()
