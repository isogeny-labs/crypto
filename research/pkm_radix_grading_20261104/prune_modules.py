import json, re, sys
closure = json.load(open(sys.argv[1]))
extraKeep = set(sys.argv[4:])
def pruneFile(path, keep, modPrefix):
    lines = open(path).read().split("\n")
    out, buf = [], []
    i = 0
    while i < len(lines):
        ln = lines[i]
        s = ln.strip()
        if s.startswith("#[") or s.startswith("///") or s.startswith("#!") and False:
            buf.append(ln); i += 1; continue
        m = re.match(r"\s*(?:pub(?:\([^)]*\))? )?mod (\w+);", ln)
        if m:
            if m.group(1) in keep or m.group(1) in extraKeep:
                out.extend(buf); out.append(ln)
            buf = []
            i += 1; continue
        m = re.match(r"\s*(?:pub(?:\([^)]*\))? )?use (?:self::|crate::%s)?(\w+)::" % modPrefix, ln)
        if m and m.group(1) not in keep and m.group(1) not in extraKeep and not m.group(1) in ("std", "core", "alloc", "super", "crate"):
            # drop the whole use statement (may span lines)
            j = i
            while not lines[j].rstrip().endswith(";"):
                j += 1
            buf = []
            i = j + 1; continue
        out.extend(buf); buf = []
        out.append(ln); i += 1
    out.extend(buf)
    open(path, "w").write("\n".join(out))
pruneFile(sys.argv[2], set(closure["top"]), "")
pruneFile(sys.argv[3], set(closure["ca"]), "cryptanalysis::")
