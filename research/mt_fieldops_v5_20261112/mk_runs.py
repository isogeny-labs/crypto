import json, sys, hashlib
src, out = sys.argv[1], sys.argv[2]
F = ["arm","pb","T","idx","seed","param","k","doff","g","n","p","targets","ok","I","M","S","H","L","N"]
rows = [json.loads(l) for l in open(src)]
r0 = rows[-1]
doc = {"schema": "mt_fieldops.runs/v1", "source": sys.argv[3], "source_sha256": hashlib.sha256(open(src,"rb").read()).hexdigest(),
       "commit": sorted({r.get("commit") for r in rows}), "bin_sha256": sorted({r.get("bin_sha256") for r in rows}),
       "fields": F, "rows": [[r.get(f) for f in F] for r in rows]}
json.dump(doc, open(out, "w"), separators=(",", ":"))
print(len(rows), "rows ->", out)
