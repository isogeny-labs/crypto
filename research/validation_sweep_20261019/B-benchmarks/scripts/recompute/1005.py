# SWB-RECOMPUTE-20 row 1005: the card gives an empty HEADLINE and the directory has no README.md.
# Computes only the directory listing and the count of raw-record-type files.
import os, json
D = "/home/user/crypto/benchmarks/20261012-f4-fp-vs-msolve"
files = sorted(os.listdir(D))
raw = [f for f in files if f.lower().endswith((".csv", ".jsonl", ".json", ".log", ".txt", ".tsv"))]
print(json.dumps({"dir": D, "files": files, "readme_present": "README.md" in files, "raw_type_files": raw}, indent=1))
