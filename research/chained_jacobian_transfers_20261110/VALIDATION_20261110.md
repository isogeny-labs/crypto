# Reproduction receipt — 2026-11-10 (local reference Python)

The committed `cantor.py` coefficient construction is **correct**: with `x=root+1/t`, `t^6(x^6+1)=sum(binomial(6,i)*root^i*t^i)+t^6`. For `root=10` in F_101, `root^6=-1`, so the degree-six coefficient vanishes. An initial suspicion that the coefficients were reversed was incorrect and was withdrawn after executing the original construction.

## Commands and actual outputs
Executed with CPython in a local container, no paid cloud instance:

```
python cantor.py
PASS 1500 random triples
python test_maps.py
PASS independently represented genus-2 divisor pullback/pushforward for 20 rational split cases
python verify_nonisomorphic.py
PASS: p=101, #E=102, r=17, P= (4, 41) image= ...
```

Additional full isogeny homomorphism checks (all ordered pairs of 101 affine points) passed in the locally reconstructed equivalent script. Approximate single-run wall times: Cantor 3.34 s; split-fiber map test 1.16 s; isogeny all-pairs 1.07 s. These are **reference smoke-test runtimes**, not controlled benchmarks or solver speedups.

## Exact limits
The 20 map tests use rational split fibers only; the push routine returns None for non-split support and certain exceptional points. The polynomial arithmetic and Cantor group law are checked by seeded property tests, not a second independent genus-2 implementation. The map identity is proved generally but only this subset has been numerically exercised. There is no EKS run, no end-to-end DLP speed measurement, and no cross-characteristic computational map.

## Next gated tasks
1. Add extension-field and nonsplit support plus exceptional/ramified divisors, with an independent implementation or computer algebra oracle.
2. Enumerate explicit quotient pairs and compute the induced homomorphism, including zero and nonzero controls.
3. Compare full transport-plus-DLP costs against direct solving on identical public toy targets.
4. Check primary literature and research-KB records, then run CI and merge only after review.
