from l08lib import *
import random
# functional check of parametrised gadget on random inputs
from tensor_encoding import WordCircuit
for (d,J,k) in [(19,9,9),(19,9,5),(19,27,19),(131,27,27),(131,27,20),(131,9,9),(131,27,6)]:
    f = BinaryField(d, {19:(1<<19)|(1<<5)|(1<<2)|1, 131:(1<<131)|(1<<13)|(1<<2)|(1<<1)|1}[d])
    c = FreeCircuit(); c.J_FIXED=J; c.K_FIXED=k
    a, b = c.word(d), c.word(d)
    try:
        out = c.gf_multiply(a, b, f)
    except AssertionError as e:
        print(d,J,k,"assert",e); continue
    ok=True
    for _ in range(3):
        x, y = random.getrandbits(d), random.getrandbits(d)
        v = c.assignment({a:x, b:y})
        ok &= bits_value(tuple(v[t] for t in out)) == f.mul(x, y) and not c.check(v)
    st=c.statistics()
    print(d,J,k,"ok" if ok else "FAIL", st["boolean_network_edges"], st["maximum_degree"])
