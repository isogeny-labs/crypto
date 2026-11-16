# R109: actual N51 valuation at the four x=1 branches

All 105 rational N51 terms from the pinned R51 donor were substituted into the two R108 order-128 base series. After translating lambda = alpha + u over F4, the combined u-coefficient series were evaluated with an explicit precision bound. The independent Hasse/schoolbook evaluator reproduced every coefficient order, support and bound.

For both y=0 and y=1, the degree-1 branch has a unique weight-zero term at u^0 with F4 scalar 1. No unknown coefficient can compete, so v_P(n)=0 on these two branches and their Frobenius conjugates.

For both degree-7 branches, u^0 and s^-23 u^7 tie at weighted order zero under (v(s),v(u))=(7,23). Their leading F4 scalars are 1 and 1, hence cancel. There is no precision competitor at weight zero; the cancellation is exact. The valuation above zero remains unresolved and needs a lift of u beyond its Newton leading term. It would be incorrect to enter zero as the degree-7 valuation or to assert the complete divisor of n.

The largest denominator s-orders are 27 (y=0) and 36 (y=1). A fresh process reproduced the result JSON byte-for-byte, and a modified donor failed the hash gate. The independent verifier initially indexed an exponent beyond its 128-bit truncation; after restricting Hasse summation to retained degrees, it passed. No producer/replay failure occurred. Lean 4.34.1 proves the finite weighted-minimum index sets and F2 leading cancellation from the independently checked order lists. The Lean theorem does not formalize the rational-series substitution or precision bounds.

Run: `python3 rounds/109/verify109.py` from the repository root path to the script. Producer and replay commands and logs are in the raw archive; `norm109.py` and `protocol.json` were committed before data. Raw archive SHA-256: `263825efee003c4075a89f892765620c485cb7d6b10e675c6da0e2f9d20c4987` (24,344 bytes). Result SHA-256: `92d6e8ae94bc3a2d7457a11c07508b95e0f384a0ce290a7a225ce4f985eee938`.

Next use a degree-7 local uniformizer, for example w=u^4/s^13 with valuation 1, and lift G and n to determine the first noncancelling coefficient. Then calculate the remaining finite supports and infinity before claiming a complete divisor.

Sources: R51 donor, R102 coefficient audit, R107 branch normalization, R108 Hensel series, [Stacks normal curve DVRs](https://stacks.math.columbia.edu/tag/0B8Y), [Newton polygon theorem](https://www.math.columbia.edu/~gyujinoh/ANT_revised.pdf).
