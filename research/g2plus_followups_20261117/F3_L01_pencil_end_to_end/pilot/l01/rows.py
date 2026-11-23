import hashlib


def canon(points, p):
    K = tuple(sorted(points))
    Kn = tuple(sorted((x, (p - y) % p) for x, y in points))
    return min(K, Kn)


class Acc:
    """Dedup, row building and statistics over accepted relations (full base)."""

    def __init__(self, T, keep_rows=True):
        self.T = T
        self.p = T.p
        self.seen = set()
        self.duplicates = 0
        self.rows = []
        self.keep_rows = keep_rows
        self.colw = {}
        self.hist = {}
        self.wsum = 0
        self.count = 0

    def row_of(self, points):
        T = self.T
        half = (self.p - 1) // 2
        row = {}
        for x, y in points:
            if y == 0:
                continue
            c = T.col[x]
            row[c] = row.get(c, 0) + (1 if 1 <= y <= half else -1)
        return tuple(sorted((c, v) for c, v in row.items() if v != 0))

    def add(self, points):
        k = canon(points, self.p)
        if k in self.seen:
            self.duplicates += 1
            return False
        self.seen.add(k)
        row = self.row_of(points)
        self.count += 1
        w = len(row)
        self.hist[w] = self.hist.get(w, 0) + 1
        self.wsum += w
        for c, _ in row:
            self.colw[c] = self.colw.get(c, 0) + 1
        self.rows.append(row)
        return True

    def stats(self):
        n = self.T.n
        return {
            "row_weight_mean": (self.wsum / self.count) if self.count else 0.0,
            "row_weight_hist": {str(k): v for k, v in sorted(self.hist.items())},
            "col_weight_max": max(self.colw.values()) if self.colw else 0,
            "cols_unhit": n - len(self.colw),
        }

    @staticmethod
    def line(row):
        return " ".join("%d:%d" % cv for cv in row)

    def digest(self, n_target):
        lines = [self.line(r) for r in self.rows[:n_target]]
        return hashlib.sha256("\n".join(lines).encode()).hexdigest(), lines


def write_rows(path, header, lines):
    with open(path, "w") as fh:
        fh.write(header + "\n")
        for ln in lines:
            fh.write(ln + "\n")


def read_rows(path):
    rows = []
    with open(path) as fh:
        header = fh.readline().strip()
        lines = [ln.rstrip("\n") for ln in fh]
    for ln in lines:
        rows.append(tuple(tuple(int(t) for t in tok.split(":")) for tok in ln.split()) if ln else ())
    return header, rows, lines


class DSU:
    def __init__(self):
        self.par = {0: 0}
        self.size = {0: 1}
        self.ids = {}
        self.cycles = 0
        self.ops = 0

    def vid(self, x):
        v = self.ids.get(x)
        if v is None:
            v = len(self.ids) + 1
            self.ids[x] = v
            self.par[v] = v
            self.size[v] = 1
        return v

    def find(self, a):
        self.ops += 1
        par = self.par
        while par[a] != a:
            par[a] = par[par[a]]
            a = par[a]
        return a

    def edge(self, a, b):
        ra, rb = self.find(a), self.find(b)
        if ra == rb:
            self.cycles += 1
            return True
        if self.size[ra] < self.size[rb]:
            ra, rb = rb, ra
        self.par[rb] = ra
        self.size[ra] += self.size[rb]
        return False
