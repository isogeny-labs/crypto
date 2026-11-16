import sys, numpy as np
from btn import Net, contract
from prime_field import Fp, mul_mod, _lt_const, _fix_const

p = int(sys.argv[1]) if len(sys.argv) > 1 else 11
F = Fp(p); d = F.d
# Test 1: fix A,B constants, check unique Z equals A*B mod p
import random
rng = random.Random(0)
okc = 0
for _ in range(6):
    a = rng.randrange(p); b = rng.randrange(p)
    net = Net()
    A = net.word(d); B = net.word(d)
    _fix_const(net, A, a); _fix_const(net, B, b)
    Z = mul_mod(net, F, A, B)
    # count should be exactly 1 (parsimonious, all determined)
    cnt = contract(net.nv, net.cons)
    # recover Z by fixing-and-counting each bit
    val = {}
    cons = list(net.cons)
    for w in range(d):
        c0 = cons + [((Z[w],), np.array([1, 0]))]
        val[w] = 0 if contract(net.nv, c0) == cnt else 1
    z = sum(val[w] << w for w in range(d))
    good = (z == a * b % p) and cnt == 1
    okc += good
    print(f"{a}*{b} mod {p} = {z} expect {a*b%p} count={cnt}", "OK" if good else "BAD")
# Test 2: free A,B both <p -> language size p^2
net = Net(); A = net.word(d); B = net.word(d)
_lt_const(net, A, p); _lt_const(net, B, p)
Z = mul_mod(net, F, A, B)
tot = contract(net.nv, net.cons)
print(f"free A,B<{p}: count={tot} expect {p*p}", "OK" if tot == p * p else "BAD")
sys.exit(0 if okc == 6 and tot == p * p else 1)
