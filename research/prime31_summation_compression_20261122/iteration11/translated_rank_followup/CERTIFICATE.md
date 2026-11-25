# Deterministic integer certificate (not a cryptographic experiment)

The inherited subgroup parameter is r=2147446831, distinct from coordinate p=2147483647. The factorization of r−1 is 2·3·5·71·233·4327; trial division through integer square roots verifies all six factors prime. Lucas witness 12 has 12^(r−1) mod r = 1 and gcd(12^((r−1)/q)−1,r)=1 for every prime divisor q. Residues:

| q | 12^((r−1)/q) mod r |
|---|---:|
|2|2147446830|
|3|1643917325|
|5|674384102|
|71|1460651230|
|233|635987700|
|4327|479857007|

Set e=(r−1)/5=429489366. p^e mod r=1. For prime divisors of e:

|q|p^(e/q) mod r|
|---|---:|
|2|2147446830|
|3|1643917325|
|71|1611049387|
|233|876401498|
|4327|638445363|

None is one. Thus e is exact. This is not a requalification of curve order. The existing curve parameters are used from the pinned research record.

Replay with standard Python 3 integer arithmetic (no random source; no curve arithmetic, solver or benchmark):

```python
from math import gcd, prod, isqrt
p, r = 2147483647, 2147446831
qs = [2, 3, 5, 71, 233, 4327]
assert prod(qs) == r-1
assert all(all(q % i for i in range(2,isqrt(q)+1)) for q in qs)
assert pow(12,r-1,r)==1
assert all(gcd(pow(12,(r-1)//q,r)-1,r)==1 for q in qs)
e=(r-1)//5
assert pow(p,e,r)==1
assert all(pow(p,e//q,r)!=1 for q in qs if e%q==0)
print(e, [1+k*e for k in range(1,6)])
```

Observed coordinator integer arithmetic output: 429489366 and [429489367,858978733,1288468099,1717957465,2147446831]. No independent proof or original-Rust validation claimed; no seed manifest applicable. [pcc:unverified]
