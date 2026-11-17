#!/usr/bin/env python3
"""Seeded native replay of Table C runs (stdlib only). usage: replay_C.py REPO IC_BINARY OUTDIR SUMMARY.json
Resumable: a replay whose output file exists is not re-run. Compares counts, recovered, all_verified."""
import hashlib, json, os, re, subprocess, sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[3]

SEED = "ic_validation_gate_20261024"

def leaves(d, p=()):
    if isinstance(d, dict):
        for k, v in d.items(): yield from leaves(v, p + (k,))
    elif isinstance(d, list):
        for i, v in enumerate(d): yield from leaves(v, p + (i,))
    else:
        yield p, d

def get(d, p):
    for k in p:
        try: d = d[k]
        except (KeyError, IndexError, TypeError): return None
    return d

def slug_of(stem):
    """Output files are named by ICV1 slug (scripts/check_curve_names.py)."""
    sys.path.insert(0, str(REPO_ROOT / "scripts"))
    import check_curve_names as cn
    a, n = re.fullmatch(r"k([01])n(\d+)", stem).groups()
    res = cn.Resolver()
    return res.name(res.koblitz(int(a), int(n)))


def main(repo, ic, outdir, summary):
    repo, outdir = Path(repo), Path(outdir); outdir.mkdir(parents=True, exist_ok=True)
    main_dir = repo / "research/ic_descent_20260930/runs-isolated/main"
    res = []
    for d in sorted(main_dir.glob("k*n*")):
        cand = []
        for j in (1, 2, 3, 4):
            for i in range(1, 6):
                for arm in ("baseline", "candidate"):
                    f = d / f"M{j}" / f"r{i}-{arm}.price.json"
                    if f.is_file(): cand.append((f"M{j}/r{i}-{arm}", f))
        keyed = sorted(cand, key=lambda t: hashlib.sha256(f"{SEED}|{d.name}|{t[0]}".encode()).hexdigest())[:3]
        for rid, f in keyed:
            arch = json.load(open(f))
            out = outdir / f"{slug_of(d.name)}-{rid.replace('/', '_')}.json"
            if not out.is_file() or out.stat().st_size == 0:
                env = dict(os.environ, RAYON_NUM_THREADS="1")
                with open(outdir / (out.stem + ".err"), "w") as e:
                    subprocess.run([ic, "price", "--params", arch["params"], "--json", "--out", str(out)],
                                   stdout=subprocess.DEVNULL, stderr=e, env=env, check=False)
            new = json.load(open(out))
            counts_eq, extra = True, set()
            for p, v in leaves(arch["counts"]):
                if get(new["counts"], p) != v: counts_eq = False
            for p, v in leaves(new["counts"]):
                if get(arch["counts"], p) is None: extra.add(".".join(map(str, p)))
            rec = {"curve": arch["curve"], "run": f"{d.name}/{rid}", "archived_binary_blake3": arch["software"]["binary_blake3"],
                   "replay_binary_blake3": new["software"]["binary_blake3"],
                   "replay_all_verified": new["all_verified"], "archived_all_verified": arch["all_verified"],
                   "recovered_n": len(new["recovered"]), "recovered_equal": new["recovered"] == arch["recovered"],
                   "archived_counts_equal": counts_eq, "new_binary_extra_counters": sorted(extra)}
            rec["identical"] = counts_eq and rec["recovered_equal"] and rec["replay_all_verified"] is True
            res.append(rec)
            print(rec["run"], rec["identical"], flush=True)
    json.dump(res, open(summary, "w"), indent=1)

if __name__ == "__main__":
    main(*sys.argv[1:])
