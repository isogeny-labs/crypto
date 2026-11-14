# Desk check (stdlib, deterministic): best IC exponent (in log p) reachable by the
# IDEA-20261109-2debfa route (enumerate m-2 base variables, joint k=2 solve on mu_B^2 at B^e2)
# under the CONTINUATION_PROTOCOL all-output optimistic model W = max(1,p/B^(m-1))*c(B) + B^2,
# c(B)=B^(m-2+e2); and the one-output model calls=max(B,p/B^(m-1)).
# Also: the cost-floor model c>=B^4 at m=8 (research/prime_norm_rank_20261112/cost-floor-report).
def best(e2, m, one_output=False, steps=20000):
    g = m - 2 + e2
    bestE = 9; bestb = None
    for i in range(1, steps):
        b = i / steps  # B = p^b
        if one_output:
            calls = max(b, 1 - (m - 1) * b)
        else:
            calls = max(0.0, 1 - (m - 1) * b)
        E = max(calls + g * b, 2 * b)
        if E < bestE: bestE, bestb = E, b
    return bestE, bestb
out = {}
for e2 in (0.0, 0.25, 0.5, 0.8, 1.0):
    rows = []
    for m in range(2, 17):
        Ea, ba = best(e2, m); Eo, bo = best(e2, m, True)
        rows.append((m, round(Ea, 4), round(Eo, 4)))
    mn = min(rows, key=lambda r: r[1])
    out[e2] = mn
    print("e2=%.2f best all-output exponent %.4f at m=%d (one-output %.4f)" % (e2, mn[1], mn[0], mn[2]))
# cost floor: t >= max(B^4, p/B^3)
E = min(max(4 * i / 10000, 1 - 3 * i / 10000) for i in range(1, 10000))
print("cost-floor m=8 dense charge c>=B^4: min_b max(4b,1-3b) = %.4f (4/7=%.4f)" % (E, 4 / 7))
# gamma gate at m=8: route gives gamma = 6+e2 >= 6 vs gate 3.5
print("m=8 route gamma = 6+e2 >= 6 > 3.5 gate (CONTINUATION_PROTOCOL A1)")
