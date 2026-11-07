import json, os, re, sys, glob, filecmp
ORIG = "research/twist_ecdlp_followup_20261107/L2-binary-xonly-twist-filter"
REP = ORIG + "/replicate"
TIMING = re.compile(r"wall|time|elapsed|sec|_ms\b|_ns\b|lap", re.I)

def tsv_rows(path):
    with open(path) as f:
        lines = [l.rstrip("\n") for l in f]
    hdr = lines[0].split("\t")
    keep = [i for i, h in enumerate(hdr) if not TIMING.search(h)]
    dropped = [hdr[i] for i in range(len(hdr)) if i not in keep]
    rows = [[r.split("\t")[i] if i < len(r.split("\t")) else "" for i in keep] for r in lines]
    return rows, dropped, len(lines) - 1

def cmp_tsv(a, b):
    ra, da, na = tsv_rows(a)
    rb, db, nb = tsv_rows(b)
    cells = 0; bad = []
    if len(ra) != len(rb): bad.append(f"rowcount {len(ra)} vs {len(rb)}")
    for i, (x, y) in enumerate(zip(ra, rb)):
        for j, (p, q) in enumerate(zip(x, y)):
            cells += 1
            if p != q:
                bad.append(f"row{i} col{j} {p} vs {q}")
    return cells, bad, sorted(set(da)), len(ra) - 1

def cmp_json(a, b, path="", out=None):
    if out is None: out = {"leaves": 0, "bad": [], "skipped_keys": set()}
    if isinstance(a, dict) and isinstance(b, dict):
        for k in a:
            if TIMING.search(k):
                out["skipped_keys"].add(k); continue
            if k not in b: out["bad"].append(f"{path}/{k} missing in replicate"); continue
            cmp_json(a[k], b[k], f"{path}/{k}", out)
        for k in b:
            if k not in a and not TIMING.search(k): out["bad"].append(f"{path}/{k} extra in replicate")
    elif isinstance(a, list) and isinstance(b, list):
        if len(a) != len(b): out["bad"].append(f"{path} len {len(a)} vs {len(b)}")
        for i, (x, y) in enumerate(zip(a, b)): cmp_json(x, y, f"{path}[{i}]", out)
    else:
        out["leaves"] += 1
        if a != b: out["bad"].append(f"{path}: {a!r} vs {b!r}")
    return out

def compare_run(rep_dir, label, only_present=False):
    print(f"=== {label}: {rep_dir}")
    tot_cells = 0; tot_bad = []; files = 0
    o_tsv = sorted(glob.glob(ORIG + "/results/*.tsv"))
    dropped_names = set()
    for o in o_tsv:
        name = os.path.basename(o)
        r = os.path.join(rep_dir, "results", name)
        if not os.path.exists(r):
            if only_present: continue
            tot_bad.append(f"MISSING {name}"); continue
        cells, bad, dropped, nrows = cmp_tsv(o, r)
        dropped_names.update(dropped); files += 1; tot_cells += cells
        if bad: tot_bad += [f"{name}: {b}" for b in bad[:5]] + ([f"{name}: +{len(bad)-5} more"] if len(bad) > 5 else [])
    print(f"TSV files compared: {files}; non-timing cells compared: {tot_cells}; dropped timing columns: {sorted(dropped_names)}; mismatches: {len(tot_bad)}")
    for b in tot_bad[:20]: print("  ", b)
    sb = 0; sbad = []
    for o in sorted(glob.glob(ORIG + "/results/samples/*.txt")):
        r = os.path.join(rep_dir, "results/samples", os.path.basename(o))
        if not os.path.exists(r):
            if only_present: continue
            sbad.append("MISSING " + os.path.basename(o)); continue
        sb += 1
        if not filecmp.cmp(o, r, shallow=False): sbad.append(os.path.basename(o))
    print(f"sample files compared: {sb}; byte-identical mismatches: {sbad}")
    pb = 0; pbad = []
    for o in sorted(glob.glob(ORIG + "/results/pca/*")):
        r = os.path.join(rep_dir, "results/pca", os.path.basename(o))
        if not os.path.exists(r):
            if only_present: continue
            pbad.append("MISSING " + os.path.basename(o)); continue
        pb += 1
        if not filecmp.cmp(o, r, shallow=False): pbad.append(os.path.basename(o))
    print(f"pca files compared: {pb}; mismatches: {pbad}")
    gp_o = open(ORIG + "/results/gp_verify.txt").read().splitlines()
    gp_r_path = os.path.join(rep_dir, "results/gp_verify.txt")
    if os.path.exists(gp_r_path):
        gp_r = open(gp_r_path).read().splitlines()
        print(f"gp_verify.txt lines orig {len(gp_o)} rep {len(gp_r)} identical: {gp_o == gp_r}")
        if gp_o != gp_r:
            for i,(x,y) in enumerate(zip(gp_o,gp_r)):
                if x!=y: print("   line",i,repr(x)[:120],"|",repr(y)[:120]); break
    else:
        print("gp_verify.txt missing in replicate (expected only for partial runs)" if only_present else "gp_verify.txt MISSING")
    jr = os.path.join(rep_dir, "results.json")
    if os.path.exists(jr) and not only_present:
        out = cmp_json(json.load(open(ORIG + "/results.json")), json.load(open(jr)))
        print(f"results.json non-timing leaves compared: {out['leaves']}; mismatches: {len(out['bad'])}; skipped timing keys: {sorted(out['skipped_keys'])}")
        for b in out["bad"][:10]: print("  ", b)
    print()

if __name__ == "__main__":
    compare_run(REP + "/rerun_full", "FRESH FULL re-run")
    compare_run(REP + "/rerun_quick", "FRESH QUICK re-run (subset, compared only where present)", only_present=True)
    compare_run(REP + "/full", "EARLIER-ATTEMPT full output (secondary)")
