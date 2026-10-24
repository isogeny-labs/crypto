import sys
sys.path.insert(0, str(__import__("pathlib").Path(__file__).resolve().parent)); sys.path.insert(0, str(__import__("pathlib").Path(__file__).resolve().parent.parent / "lib"))
import importlib.util
spec = importlib.util.spec_from_file_location("L", "libs_bp.py")
src = open("libs_bp.py").read().split("for name in (\"P-224\"")[0]
L = {"__file__": "libs_bp.py"}; exec(src, L)
hits = 0; n = 0
for fn in ("bin_targets.txt", "k_targets.txt"):
    for ln in open(fn):
        name, m, kind, v = ln.split(); V = int(v); m = int(m); n += 1
        for pat in L["PATTERNS"]:
            need = 3 if pat[4] >= 31 else 4
            h = L["scan_pattern"](V, m, pat, need)
            if h: print("HIT", name, kind, pat[0], h[:2]); hits += 1
        h = L["scan_maple"](V)
        if h: print("HIT", name, kind, "Maple", h[:2]); hits += 1
print(n, "binary/Koblitz targets x", len(L["PATTERNS"]) + 1, "generator patterns; hits:", hits)
