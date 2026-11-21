import hashlib


def canon_key(p, anchors, residual, ip, im, even):
    pts = list(anchors) + list(residual)
    K = (tuple(sorted(pts)), ip, im) if even else tuple(sorted(pts))
    Ki_pts = tuple(sorted((x, (p - y) % p) for x, y in pts))
    Ki = (Ki_pts, im, ip) if even else Ki_pts
    return min(K, Ki)


def digest(keys):
    return hashlib.sha256("\n".join(repr(k) for k in sorted(keys)).encode()).hexdigest()


class RowStats:
    def __init__(self, n):
        self.colw = [0] * n
        self.hist = {}
        self.sumw = 0
        self.cnt = 0
        self.ndelta = 0

    def add(self, pts, col, has_delta):
        xs = [x for x, _ in pts]
        assert len(set(xs)) == len(xs)
        w = len(pts)
        self.hist[str(w)] = self.hist.get(str(w), 0) + 1
        self.sumw += w
        self.cnt += 1
        if has_delta:
            self.ndelta += 1
        for x in xs:
            self.colw[col[x]] += 1

    def report(self, even_has_delta_field):
        n = len(self.colw)
        return {
            "row_weight_hist": dict(sorted(self.hist.items(), key=lambda kv: int(kv[0]))),
            "mean_affine_weight": (self.sumw / self.cnt) if self.cnt else None,
            "delta_fraction": ((self.ndelta / self.cnt) if self.cnt else None) if even_has_delta_field else None,
            "max_col_weight": max(self.colw) if self.colw else 0,
            "cols_never_hit": n - sum(1 for c in self.colw if c > 0),
        }
