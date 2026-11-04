import os, re, sys, json
root = sys.argv[1]
src = os.path.join(root, "src")
extra = sys.argv[2:]  # extra "top" or "top::sub" seeds
topMods = []
for line in open(os.path.join(src, "lib.rs")):
    m = re.match(r"\s*pub mod (\w+);", line)
    if m:
        topMods.append(m.group(1))
caMods = []
for line in open(os.path.join(src, "cryptanalysis", "mod.rs")):
    m = re.match(r"\s*(?:pub(?:\(crate\))? )?mod (\w+);", line)
    if m:
        caMods.append(m.group(1))

def filesOf(node):
    # node: ("top", name) or ("ca", name)
    kind, name = node
    base = os.path.join(src, name) if kind == "top" else os.path.join(src, "cryptanalysis", name)
    out = []
    if os.path.isfile(base + ".rs"):
        out.append(base + ".rs")
    if os.path.isdir(base):
        for d, _, fs in os.walk(base):
            for f in fs:
                if f.endswith(".rs"):
                    out.append(os.path.join(d, f))
    return out

refRe = re.compile(r"crate::(\w+)(?:::(\w+))?")
superRe = re.compile(r"super::(\w+)")
pathRe = re.compile(r'#\[path\s*=\s*"([^"]+)"\]')
def refsOf(node):
    kind, name = node
    res = set()
    for f in filesOf(node):
        txt = open(f, errors="replace").read()
        for a, b in refRe.findall(txt):
            if a == "cryptanalysis" and b in caMods:
                res.add(("ca", b))
            elif a in topMods and a != "cryptanalysis":
                res.add(("top", a))
        # super:: from a direct child file of cryptanalysis refers to siblings
        if kind == "ca":
            depth = os.path.relpath(f, os.path.join(src, "cryptanalysis")).count(os.sep)
            if depth == 0:
                for s in superRe.findall(txt):
                    if s in caMods:
                        res.add(("ca", s))
            else:
                # nested: super::super::x or super::x within own dir; be generous
                for s in superRe.findall(txt):
                    if s in caMods:
                        res.add(("ca", s))
        # bare `use cryptanalysis::` inside top modules
        for s in re.findall(r"cryptanalysis::(\w+)", txt):
            if s in caMods:
                res.add(("ca", s))
    return res

seeds = {("top", "cryptanalysis")}
for s in extra:
    if "::" in s:
        a, b = s.split("::", 1)
        seeds.add(("ca", b) if a == "cryptanalysis" else ("top", a))
    else:
        seeds.add(("top", s))
seen = set()
todo = list(seeds)
while todo:
    n = todo.pop()
    if n in seen:
        continue
    seen.add(n)
    if n == ("top", "cryptanalysis"):
        # mod.rs itself: only its non-mod lines matter; handled by seeds
        continue
    for r in refsOf(n):
        if r not in seen:
            todo.append(r)
print(json.dumps({"top": sorted(x[1] for x in seen if x[0] == "top"),
                  "ca": sorted(x[1] for x in seen if x[0] == "ca")}))
