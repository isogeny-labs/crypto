"""Encode P1 + P2 = R on the ECC2K-130 curve y^2+xy=x^3+1 over GF(2^131)
(f = x^131+x^13+x^2+x+1) and check that the witness computed with reference
arithmetic satisfies every local predicate.  Nothing is contracted: the
contraction needs the unreviewed OpenAI algorithm and is infeasible at this
size by brute force.  Reports network size per the BTN conversion."""
import random, json, sys, time
from btn import Field, Curve, Net
from relation import add_gadget, on_curve

n = 131
F = Field(n, (1 << 131) | (1 << 13) | (1 << 2) | (1 << 1) | 1)
C = Curve(F)
rng = random.Random(int(sys.argv[1]) if len(sys.argv) > 1 else 7)

def trace(a):
    t = 0
    for _ in range(n): t ^= a; a = F.sq(a)
    return t  # 0 or 1
def halftrace(a):  # n odd: solves z^2+z=a when Tr(a)=0
    h = a
    for _ in range((n - 1) // 2): h = F.sq(F.sq(h)) ^ a
    return h
def randpoint():
    while True:
        x = rng.getrandbits(n)
        if x == 0: continue
        c = F.mul(F.sq(x ^ 0) , 1) if False else None
        # y = x*z: x^2 z^2 + x^2 z = x^3 + 1  ->  z^2 + z = x + 1/x^2
        a = x ^ F.inv(F.sq(x))
        if trace(a): continue
        z = halftrace(a); assert F.sq(z) ^ z == a
        P = (x, F.mul(x, z))
        assert C.on(P)
        return P

t0 = time.time()
P1 = randpoint()
while True:
    P2 = randpoint()
    if P2[0] != P1[0]: break
R = C.add(P1, P2)
net = Net()
W1 = (net.word(n), net.word(n)); W2 = (net.word(n), net.word(n)); WR = (net.word(n), net.word(n))
on_curve(net, F, W1); on_curve(net, F, W2)
lam = add_gadget(net, F, W1, W2, WR)
st = net.stats()
print("built in %.0fs" % (time.time() - t0), st)

val = {}
def put(ws, v):
    for i, w in enumerate(ws): val[w] = v >> i & 1
for ws, v in [(W1[0], P1[0]), (W1[1], P1[1]), (W2[0], P2[0]), (W2[1], P2[1]), (WR[0], R[0]), (WR[1], R[1])]:
    put(ws, v)
put(lam, F.mul(P1[1] ^ P2[1], F.inv(P1[0] ^ P2[0])))
# propagate: a constraint with one unknown variable and a unique satisfying value
cons = net.cons
users = {}
for k, (sc, t) in enumerate(cons):
    for v in sc: users.setdefault(v, []).append(k)
queue = list(range(len(cons)))
while queue:
    k = queue.pop()
    sc, t = cons[k]
    unk = [v for v in sc if v not in val]
    if len(unk) != 1: continue
    idx = [None if v == unk[0] else val[v] for v in sc]
    pos = sc.index(unk[0]); ok = []
    for b in (0, 1):
        idx[pos] = b
        if t[tuple(idx)]: ok.append(b)
    if len(ok) != 1: continue
    val[unk[0]] = ok[0]
    queue.extend(users[unk[0]])
unassigned = net.nv - len(val)
bad = sum(1 for sc, t in cons if not t[tuple(val.get(v, 0) for v in sc)])
print(f"variables assigned {len(val)}/{net.nv}, unassigned {unassigned}, violated predicates {bad}/{len(cons)}")
# negative control: flip one bit of the output y3
val[WR[1][5]] ^= 1
bad2 = sum(1 for sc, t in cons if not t[tuple(val.get(v, 0) for v in sc)])
print("negative control (R.y bit flipped): violated predicates", bad2)
out = dict(curve="y^2+xy=x^3+1", field="GF(2^131), x^131+x^13+x^2+x+1", **st, assigned=len(val), unassigned=unassigned,
           violated=bad, violated_negative_control=bad2, points=dict(P1=[hex(P1[0]), hex(P1[1])], P2=[hex(P2[0]), hex(P2[1])], R=[hex(R[0]), hex(R[1])]))
json.dump(out, open("ecc2k130_check.json", "w"), indent=1)
assert unassigned == 0 and bad == 0 and bad2 > 0
