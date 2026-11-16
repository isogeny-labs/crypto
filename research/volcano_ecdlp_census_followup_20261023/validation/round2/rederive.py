#!/usr/bin/env python3
"""Round-2 validator: re-derive every measured row from the committed raw archives.

Validator-written; shares no code with the producer (analysis/*.py) or with the
round-1 validator (validation/*.py). Each committed session archive is streamed
(tarfile 'r|xz', nothing written to disk) and its records.jsonl is parsed:

- the archive's SHA-256 must equal its progress line (status "done");
- the archive's spec.json must equal the committed specs/<spec>.json;
- every record: binary sha256 = frozen binary; outcome verified and
  matches_planted; [recovered]G == Q recomputed with independent affine
  arithmetic, Q on the curve; S == total_gae / sqrt(r);
- [r]G == O once per (curve, generator);
- the census vertex is recovered from j(a, b) (Tcr rows: the source vertex of the
  spec, with the crater model and generator from census/V5/tcr/<id>.json; M rows:
  the model index from census/<V>/M.json);
- the A2.1 uncharged count: for each counter named in cost.unpriced, the record's
  top-level counters value if present, else the sum over phases of native[name
  without "_uncharged"].

Writes one gzip CSV of all rows (warm-up included) and a JSON summary; then
compares with runs/extract and runs/uncharged row by row.

Usage: rederive.py STUDY_DIR OUT_DIR [workers]
"""
import csv, gzip, hashlib, io, json, lzma, math, os, sys, tarfile
from multiprocessing import Pool

FROZEN = "ac4bc5b26f858a4d8a374208297157840af25fb1dc25ea1c19dd41360735fb35"
STUDY = None


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


def on_curve(P, a, b, p):
    x, y = P
    return (y * y - (x * x * x + a * x + b)) % p == 0


def jinv(a, b, p):
    d = (4 * a ** 3 + 27 * b * b) % p
    return 1728 * 4 * a ** 3 * inv(d, p) % p


def load_census(study):
    out = {}
    for V in ("V5", "V6"):
        verts = [json.loads(l) for l in open(os.path.join(study, "census", V, "vertices.jsonl"))]
        jmap = {}
        for v in verts:
            assert jinv(v["a"], v["b"], v["p"]) == v["j"] % v["p"], (V, v["id"])
            jmap.setdefault(v["j"], []).append(v["id"])
        M = json.load(open(os.path.join(study, "census", V, "M.json")))
        models = {}
        for c in M["curves"]:
            for m in c["models"]:
                models[(c["id"], m["a"], m["b"])] = m["m"]
        out[V] = {"verts": {v["id"]: v for v in verts}, "jmap": jmap, "models": models}
    tcr = {}
    d = os.path.join(study, "census", "V5", "tcr")
    for f in os.listdir(d):
        t = json.load(open(os.path.join(d, f)))
        tcr[t["id"]] = t
    out["tcr"] = tcr
    return out


CEN = None


def init(study):
    global STUDY, CEN
    STUDY = study
    CEN = load_census(study)


COLS = ["session", "spec", "V", "fam", "level", "chunk", "vertex", "model", "slug", "workload_id",
        "target_index", "arm", "round", "warmup", "verified", "total_gae", "S", "unc", "unc_detail",
        "solve_wall_ns", "iso", "seq"]


def process(item):
    sess, spec, want_sha = item
    arc = os.path.join(STUDY, "sessions", sess + ".tar.xz")
    h = hashlib.sha256()
    with open(arc, "rb") as f:
        for blk in iter(lambda: f.read(1 << 20), b""):
            h.update(blk)
    sha_ok = h.hexdigest() == want_sha
    V, fam, lv, ch = spec.split("-")
    level = int(lv[1:])
    chunk = int(ch[1:])
    cen = CEN[V]
    committed_spec = json.load(open(os.path.join(STUDY, "specs", spec + ".json")))
    spec_ok = None
    rows = []
    fails = {"point": 0, "oncurve": 0, "S": 0, "outcome": 0, "binary": 0, "vertex": 0, "level": 0,
             "tcr_gen": 0, "model": 0, "session_id": 0}
    gen_checked = {}
    sids = set()
    n = 0
    with tarfile.open(arc, "r|xz") as tf:
        for m in tf:
            if not m.isfile():
                continue
            name = m.name.split("/", 1)[1] if "/" in m.name else m.name
            if name == "spec.json":
                spec_ok = json.load(tf.extractfile(m)) == committed_spec
            elif name == "records.jsonl":
                fo = tf.extractfile(m)
                buf = b""
                lines = []
                def _lines(fo=fo):
                    rest = b""
                    while True:
                        blk = fo.read(1 << 20)
                        if not blk:
                            break
                        rest += blk
                        parts = rest.split(b"\n")
                        rest = parts.pop()
                        for q in parts:
                            if q.strip():
                                yield q
                    if rest.strip():
                        yield rest
                for line in _lines():
                    r = json.loads(line)
                    n += 1
                    sids.add(r["session_id"])
                    if r.get("binary_sha256") != FROZEN:
                        fails["binary"] += 1
                    wl = r["workload"]
                    cs = wl["curve_spec"]
                    p, a, b, rr = cs["p"], cs["a"], cs["b"], cs["r"]
                    G = (cs["gx"], cs["gy"])
                    key = (p, a, b, G)
                    if key not in gen_checked:
                        gen_checked[key] = on_curve(G, a, b, p) and ec_mul(rr, G, a, p) is None
                    Q = (int(wl["target"][0], 16), int(wl["target"][1], 16))
                    oc = r["outcome"]
                    ok_out = oc["status"] == "verified" and oc["matches_planted"] is True and \
                        oc["matches_target"] is True and int(oc["recovered"]) == int(wl["planted"])
                    if not ok_out:
                        fails["outcome"] += 1
                    if not on_curve(Q, a, b, p):
                        fails["oncurve"] += 1
                    pt_ok = ec_mul(int(oc["recovered"]) % rr, G, a, p) == Q if oc["recovered"] is not None else False
                    if not pt_ok:
                        fails["point"] += 1
                    c = r["cost"]
                    tg = float(c["total_gae"])
                    if abs(tg / math.sqrt(rr) - float(c["s"])) > 1e-12 * max(1.0, float(c["s"])):
                        fails["S"] += 1
                    unc = 0
                    det = []
                    top = r.get("counters") or {}
                    for nm in c.get("unpriced") or []:
                        if nm in top:
                            v = int(top[nm])
                        else:
                            base = nm[:-len("_uncharged")] if nm.endswith("_uncharged") else nm
                            v = sum(int((ph.get("native") or {}).get(base, 0)) for ph in r["phases"])
                        unc += v
                        det.append(f"{nm}={v}")
                    # census vertex
                    if fam == "Tcr":
                        vtx = chunk
                        t = CEN["tcr"][chunk]
                        g = tuple(int(x) for x in t["g"].split(","))
                        if not (jinv(a, b, p) == 0 and G == g and cen["verts"][0]["a"] == a and cen["verts"][0]["b"] == b):
                            fails["tcr_gen"] += 1
                        model = -1
                    else:
                        ids = cen["jmap"].get(jinv(a, b, p), [])
                        vtx = ids[0] if len(ids) == 1 else -1
                        if vtx < 0:
                            fails["vertex"] += 1
                        model = -1
                        if fam == "M":
                            model = cen["models"].get((chunk, a, b), -1)
                            if model < 0 or vtx != chunk:
                                fails["model"] += 1
                    if vtx >= 0 and cen["verts"][vtx]["level"] != level:
                        fails["level"] += 1
                    rows.append([sess, spec, V, fam, level, chunk, vtx, model, wl["curve"]["slug"],
                                 wl["workload_id"], wl["target_index"], r["arm"], r["round"],
                                 int(bool(r["warmup"])), int(ok_out and pt_ok), repr(tg), repr(float(c["s"])),
                                 unc, ";".join(det), r["time"]["solve_wall_ns"], r["isolation"]["level"], r["seq"]])
    if len(sids) != 1:
        fails["session_id"] += 1
    return {"session": sess, "spec": spec, "archive_sha_ok": sha_ok, "spec_json_equal": spec_ok,
            "records": n, "generators": len(gen_checked), "generators_ok": all(gen_checked.values()),
            "fails": fails, "rows": rows}


def main():
    study, out = sys.argv[1], sys.argv[2]
    workers = int(sys.argv[3]) if len(sys.argv) > 3 else 2
    os.makedirs(out, exist_ok=True)
    items = []
    for l in open(os.path.join(study, "runs", "progress.jsonl")):
        d = json.loads(l)
        if d.get("status") == "done":
            items.append((os.path.basename(d["out"]), d["spec"], d["sha256_tar_xz"]))
    summ = []
    with gzip.open(os.path.join(out, "rows.csv.gz"), "wt", newline="") as gz, \
            Pool(workers, initializer=init, initargs=(study,)) as pool:
        w = csv.writer(gz)
        w.writerow(COLS)
        for res in pool.imap(process, items):
            w.writerows(res.pop("rows"))
            summ.append(res)
            print(res["session"], res["records"], res["archive_sha_ok"], res["spec_json_equal"],
                  {k: v for k, v in res["fails"].items() if v}, flush=True)
    json.dump(summ, open(os.path.join(out, "rederive_summary.json"), "w"), indent=1)


if __name__ == "__main__":
    main()
