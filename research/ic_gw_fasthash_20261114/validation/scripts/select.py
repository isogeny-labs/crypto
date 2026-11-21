# Validator sample selection (rule fixed before applying it).
# Universe: q in 5 sizes x seed in 1..9 (45 (q, seed) units).
# Rank by sha256("<nonce>:<q>:<seed>") ascending. Take the first 6.
# Then append, in rank order, units needed until: >= 2 at q >= 2097091; >= 2 with seed in 4..9;
# >= 2 with seed in 1..3 (so committed producer records can be compared directly).
import hashlib, json, sys
nonce = sys.argv[1]
Q = [262111, 524287, 1048507, 2097091, 4194271]
U = [(q, s) for q in Q for s in range(1, 10)]
rank = sorted(U, key=lambda u: hashlib.sha256(f"{nonce}:{u[0]}:{u[1]}".encode()).hexdigest())
sel = rank[:6]
def need(pred, n):
    for u in rank:
        if sum(1 for v in sel if pred(v)) >= n: break
        if pred(u) and u not in sel: sel.append(u)
need(lambda u: u[0] >= 2097091, 2)
need(lambda u: 4 <= u[1] <= 9, 2)
need(lambda u: 1 <= u[1] <= 3, 2)
out = [{"q": q, "seed": s, "rank": rank.index((q, s)), "key": hashlib.sha256(f"{nonce}:{q}:{s}".encode()).hexdigest()} for q, s in sel]
print(json.dumps({"nonce": nonce, "rank_first10": [list(u) for u in rank[:10]], "sample": out}, indent=1))
