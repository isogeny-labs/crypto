# Public-coordinate lift search exhausts height 4,096 on the certified curve

For the frozen public target `T=(10,1)` on the reduction of
`E/ℚ: y²=x³−47x²+751x−3997`, the public-coordinate-only search exhausted
`H=max(|a|,b)≤4,096` under `x=a/b²`, `y=c/b³`. It tested **424,915 coprime
candidate polynomials**, including 639 nonnegative integer square tests;
none was a square. The capped arm took **1.917 seconds** on the ordinary
local host. The positive control `Q̄=(17,10)` found its exact rational
point `Q=(17,10)` at height 17 after eight polynomial tests. A separate
denominator-first audit reproduces every final count and both statuses.

| Public input | Height status | Modular-x candidates | Coprime polynomial tests | Nonnegative square tests | Wall interval |
| --- | --- | ---: | ---: | ---: | ---: |
| `(17,10)` control | found at 17 | 13 | 8 | 1 | 0.000207 s |
| `(10,1)` target | height cap 4,096 exhausted | 698,786 | 424,915 | 639 | 1.917191 s |

The [protocol](protocol.json) froze the curve, public points, exact
`(H,b,a)` search order, coprimality/reduction filters, height and wall caps,
checkpoints, and source receipts before execution. For each candidate,
the [runner](run.py) computes

`c² = a³ − 47a²b² + 751ab⁴ − 3997b⁶`

and checks both signs of `c` against the public reduced y-coordinate when
the integer is a square. No target scalar enters this search. The
[`result.json`](result.json) preserves each arm's exact status, counts,
start/stop interval and found point where applicable; the height
[`checkpoints`](target-checkpoints.jsonl) retain progress through 4,096.
The independent [`audit.py`](audit.py) enumerates the target candidates in
denominator-first order and recomputes the square tests and count totals;
[`audit-result.json`](audit-result.json) is `PASS`.

The [certified-generator packet](../xedni_rank1_generator_certificate_20261121/README.md)
establishes that every rational point on this curve is `nQ` and that a lift
of the target requires `n≡22 mod 43`; its smallest-magnitude multiplier
is `−21`, whose point has 455-bit maximum affine coordinates. That
certificate does not replace the finite height search: the result here is
the exhaustive status under the specified `a/b²` law and `H≤4,096` cap.
Formalizing the full finite candidate census in Lean would require a
checked enumeration certificate for 698,786 modular-x candidates; the
curve-specific no-hit statement remains computationally audited rather
than a Lean theorem.

The same public target was solved and scalar-replayed by three process-fresh
rho arms in the [matched panel](../xedni_process_fresh_target_20261121/README.md),
with online intervals 0.190–0.308 ms on this host. Those arms return a
scalar, while this capped arm returns a height-search status. The target
search cannot be converted into a completed scalar-recovery time. A next
step is a predeclared height-cap ladder or a direct method for finding the
required rational multiple from public coordinates, with every attempted
height and expression cost charged.

Replay from the repository root:

```sh
gtimeout 120 python3 research/exotic_ecdlp_20261021/xedni_rank1_public_lift_cap_20261121/run.py
python3 research/exotic_ecdlp_20261021/xedni_rank1_public_lift_cap_20261121/audit.py
```
