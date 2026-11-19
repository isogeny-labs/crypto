"""Pilot: run f1.py gates and statistics on the p = 8219 dress rehearsal (non-decision prime)."""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import f1
d = f1.load_dir(sys.argv[1])
p = 8219
inst = [r for r in d[(p, 2.0, "main")] if r.get("kind") == "instance"]
ref = os.path.join(sys.argv[1], "ref_from_cc2_main.jsonl.txt")
with open(ref, "w") as fh:
    for r in inst:
        fh.write(json.dumps(r) + "\n")
curves = {p: tuple(sorted(r["curve_seed"] for r in inst))}
gt = f1.gates(d, expect_primes=(p,), ref_path=ref, curves=curves, check_repro=False)
print("gates:", json.dumps({k: v["pass"] for k, v in gt["gates"].items()}), "all_pass", gt["all_pass"])
for k, v in gt["gates"].items():
    if not v["pass"]:
        print(k, json.dumps(v["detail"])[:500])
a, b = f1.runs(d[(p, 1.0, "main")], f1.IC_ARM), f1.runs(d[(p, 2.0, "main")], f1.IC_ARM)
rho = f1.runs(d[(p, 2.0, "main")], f1.RHO_ARM)
ks = sorted(set(a) & set(b))
print("curves", curves[p], "pairs", len(ks), "attempts max", max(r["attempts"] for r in list(a.values()) + list(b.values())))
for band in f1.BANDS:
    print("G", band, ["%.4f" % x for x in f1.hboot(ks, f1.gain(a, b, band))])
odd = {k: v for k, v in b.items() if k[1] % 2 == 1}
even = {(k[0], k[1] - 1): v for k, v in b.items() if k[1] % 2 == 0}
print("G_AA mid", ["%.4f" % x for x in f1.hboot(sorted(set(odd) & set(even)), f1.gain(even, odd, "mid"))])
for name, x in (("cc1", a), ("cc2", b)):
    n = len(ks)
    m = sum(f1.fpm_ic(x[k], "mid") for k in ks) / n
    la = sum(f1.fpm_la(x[k], "mid") for k in ks) / n
    rr = sum(f1.fpm_ic(x[k], "mid") for k in ks) / sum(f1.fpm_rho(rho[k], "mid") for k in ks)
    print(name, "mean trials %.0f" % (sum(x[k]["trials"] for k in ks) / n), "LA share mid %.3f" % (la / m), "IC/rho mid %.3f" % rr)
