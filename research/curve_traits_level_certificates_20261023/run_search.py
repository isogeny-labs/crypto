"""Drive search.gp over the open depth-1 levels of GF(p^k) curves in
docs/curves/traits.json and append each certificate found to certs.jsonl
(resumable: levels already there are skipped).  Layout only; every
verdict is re-verified by curve_traits from the certificate file.

usage: run_search.py [--only SLUG_PREFIX] [--jobs N]"""
import json, os, subprocess, sys, concurrent.futures as cf

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", ".."))
OUT = os.path.join(HERE, "certs.jsonl")

def jobs():
    reg = {c["slug"]: c for c in json.load(open(os.path.join(ROOT, "docs/curves/registry.json")))["curves"]}
    tr = json.load(open(os.path.join(ROOT, "docs/curves/traits.json")))
    for r in tr["curves"]:
        if r["field"]["kind"] != "extension":
            continue
        for lv in r["endomorphism"].get("levels", []):
            if lv["level"] is None and lv["depth"] == 1:
                m = json.loads(reg[r["slug"]]["model_json"])
                yield dict(slug=r["slug"], ell=int(lv["ell"]), p=m["p"], k0=int(m["k"]),
                           modv=m["modulus"], av=m["a"], bv=m["b"], t=r["trace"])

def run(j, seed=1, tries=64):
    pre = "".join([
        f"p={j['p']}; k0={j['k0']}; t={j['t']}; ell={j['ell']}; seed={seed}; tries={tries};\n",
        f"modv=[{','.join(j['modv'])}]; av=[{','.join(j['av'])}]; bv=[{','.join(j['bv'])}];\n",
    ])
    src = pre + open(os.path.join(HERE, "search.gp")).read() + "\nquit;\n"
    out = subprocess.run(["gp", "-q", "-s", "4000000000"], input=src, capture_output=True, text=True)
    line = [l for l in out.stdout.splitlines() if l.startswith("CERT|")]
    if not line:
        return j, None, out.stdout[-400:] + out.stderr[-400:]
    _, kind, m, twist, v, g, d, pts = line[0].split("|")
    gp2j = lambda s: json.loads(s.replace(";", ","))
    cert = dict(slug=j["slug"], ell=j["ell"], kind=kind, degree=int(m), twist=twist == "1",
                v=int(v), modulus=gp2j(g), d=gp2j(d) if twist == "1" else None, points=gp2j(pts))
    return j, cert, None

if __name__ == "__main__":
    only = sys.argv[sys.argv.index("--only") + 1] if "--only" in sys.argv else ""
    n = int(sys.argv[sys.argv.index("--jobs") + 1]) if "--jobs" in sys.argv else 3
    done = set()
    if os.path.exists(OUT):
        for l in open(OUT):
            c = json.loads(l); done.add((c["slug"], c["ell"]))
    todo = [j for j in jobs() if j["slug"].startswith(only) and (j["slug"], j["ell"]) not in done]
    print(f"{len(todo)} levels to search", flush=True)
    with cf.ProcessPoolExecutor(n) as ex:
        for j, cert, err in ex.map(run, todo):
            if cert and cert["kind"] != "none":
                with open(OUT, "a") as f:
                    f.write(json.dumps(cert, separators=(",", ":")) + "\n")
                print("OK", j["slug"], j["ell"], cert["kind"], cert["degree"], flush=True)
            else:
                print("FAIL", j["slug"], j["ell"], err, flush=True)
