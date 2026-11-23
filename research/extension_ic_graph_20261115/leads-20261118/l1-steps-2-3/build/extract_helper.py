"""extract(): byte-copy named top-level items (copied unchanged from ../../l1-pattern-tally/build/assemble.py; tooling only)."""
import re

pat = re.compile(r'^(pub(\([a-z]+\))? )?(fn|struct|enum|type|const|static|impl(<[^>]*>)?|trait) +([A-Za-z0-9_]+)')


def extract(path, names):
    src = open(path).read().split("\n")
    n = len(src)

    def end(i):
        depth, started = 0, False
        for j in range(i, n):
            s = re.sub(r'//.*', '', src[j])
            s = re.sub(r'"(\\.|[^"\\])*"', '""', s)
            s = re.sub(r"'(\\.|[^'\\])'", "''", s)
            depth += s.count('{') - s.count('}')
            if '{' in s:
                started = True
            if started and depth <= 0:
                return j
            if not started and s.rstrip().endswith(';'):
                return j
        return n - 1
    out, i = [], 0
    while i < n:
        m = pat.match(src[i])
        if m:
            nm = m.group(5)
            if m.group(3).startswith('impl'):
                mm = re.match(r'^impl(<[^>]*>)? +(?:[A-Za-z0-9_:<>, ]+ for )?([A-Za-z0-9_]+)', src[i])
                nm = mm.group(2) if mm else nm
            e = end(i)
            if nm in names:
                k = i
                while k > 0 and (src[k - 1].startswith('///') or src[k - 1].startswith('#[')):
                    k -= 1
                out.append("\n".join(src[k:e + 1]))
            i = e + 1
        else:
            i += 1
    return "\n\n".join(out)


