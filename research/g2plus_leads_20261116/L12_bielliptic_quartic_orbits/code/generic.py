"""Generic plane quartic control (SPEC.md section 9.3): instance acceptance."""
import random


def draw_coeffs(cs, p):
    rng = random.Random(cs)
    g = [[0] * 5 for _ in range(5)]  # g[i][jj]
    g[0][4] = 1
    for jj in range(5):
        for i in range(5 - jj):
            if i == 0 and jj == 4:
                continue
            g[i][jj] = rng.randrange(p)
    return g


def affine_points(g, p):
    """Return (sorted affine points, singular flag) by brute force."""
    pts = []
    singular = False
    for x in range(p):
        xp = [pow(x, i, p) for i in range(5)]
        cj = [sum(g[i][jj] * xp[i] for i in range(5 - jj)) % p for jj in range(5)]
        # d/dx coefficient polynomial in y
        dx = [sum(i * g[i][jj] * (xp[i - 1] if i > 0 else 0) for i in range(1, 5 - jj)) % p
              for jj in range(5)]
        for y in range(p):
            v = (((cj[4] * y + cj[3]) * y + cj[2]) * y + cj[1]) * y + cj[0]
            if v % p:
                continue
            pts.append((x, y))
            fx = (((dx[4] * y + dx[3]) * y + dx[2]) * y + dx[1]) * y + dx[0]
            fy = (4 * cj[4] * y * y * y + 3 * cj[3] * y * y + 2 * cj[2] * y + cj[1])
            if fx % p == 0 and fy % p == 0:
                singular = True
    pts.sort()
    return pts, singular


def infinity_singular(g, p):
    for t in range(p):
        tp = [pow(t, jj, p) for jj in range(5)]
        F4 = sum(g[i][jj] * tp[jj] for i in range(5) for jj in range(5) if i + jj == 4) % p
        if F4:
            continue
        FX = sum(i * g[i][jj] * tp[jj] for i in range(5) for jj in range(5) if i + jj == 4) % p
        FY = sum(jj * g[i][jj] * (tp[jj - 1] if jj > 0 else 0)
                 for i in range(5) for jj in range(5) if i + jj == 4) % p
        FZ = sum(g[i][jj] * tp[jj] for i in range(5) for jj in range(5) if i + jj == 3) % p
        if FX == 0 and FY == 0 and FZ == 0:
            return True
    return False


def accept(cs, p):
    g = draw_coeffs(cs, p)
    if g[4][0] % p == 0:
        return None, "g40_zero"
    pts, sing = affine_points(g, p)
    if sing or infinity_singular(g, p):
        return None, "singular"
    if len(pts) < 8:
        return None, "few_points"
    return (g, pts), None
