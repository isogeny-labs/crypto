import lzma, os, re, glob
BASE = "/home/user/crypto/research/eigen_seta_rust_20261011/reports/run-20261011T1955Z-verify/records"
def raw_total(path):
    with lzma.open(path, "rb") as f:
        for line in f:
            if line.startswith(b"totals:"):
                return int(line.split()[1])
    return None
def summary(path):
    rows = {}
    for line in open(path):
        p = line.split()
        if len(p) == 4 and p[0].startswith("cg."):
            rows[p[0]] = tuple(int(x) for x in p[1:])
    return rows
for label, d in [("Toy16 (toyprobe)", "toyprobe"), ("L1 (l1probe)", "l1probe")]:
    s = summary(os.path.join(BASE, d, "ctprobe_summary.txt"))
    ok = 0
    for name, (tot, alloc, rest) in sorted(s.items()):
        if name == "run": continue
        rt = raw_total(os.path.join(BASE, d, "callgrind", name + ".xz"))
        ok += (rt == tot)
        if rt != tot:
            print("  total mismatch", d, name, rt, tot)
    rests = sorted({v[2] for k, v in s.items() if k != "run"})
    print(f"{label}: runs={len(s)-0} raw_total_matches_summary={ok} distinct_rest_values={rests}")
    print(f"  runs listed: {sorted(k for k in s)}")
# round-trip counts from RESULT lines
tot_ok = tot_n = 0
for f in ["failrate/l1.txt", "failrate/toy16.txt", "failrate/toy32.txt"]:
    txt = open(os.path.join(BASE, f)).read()
    m = re.search(r"RESULT (\d+)/(\d+) decapsulations correct", txt)
    print(f, m.group(0)); tot_ok += int(m.group(1)); tot_n += int(m.group(2))
print("round trips:", tot_ok, "/", tot_n)
