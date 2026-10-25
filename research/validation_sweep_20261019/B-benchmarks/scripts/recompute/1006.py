import glob, os
# HEADLINE: "Seven frozen perf-history snapshots ..." -> count committed history snapshot records
H = "research/f4_f5_history_20261012/records/history/"
files = sorted(glob.glob(H + "*.json"))
print("history snapshot files:", len(files))
for f in files:
    print(os.path.basename(f))
