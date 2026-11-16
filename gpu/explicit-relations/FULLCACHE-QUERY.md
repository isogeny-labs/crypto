# Uninstrumented full-cache query specialization

Build `make ARCH=sm_89 build/explicit-relations-fullcache`. This opt-in
`ER_QUERY_FULL_CACHE` kernel requires persistent queries, complete right-cache
support and affine subtraction. The backend selects it only when admission
actually retained the full right cache; partial-cache and uncached queries keep
the existing persistent kernel.

The specialized kernel loads the cached right point directly, negates and
converts it, reuses the converted target, then applies the existing affine
subtraction and batch normalization. The grid traversal, minimum right ID,
lookup, coverage counter and winner resolution are preserved. Infinity,
doubling and opposite-point cases use the shared checked arithmetic helpers.
There are no clocks, compiler fences, additional barriers or profiling buffers.
`ER_QUERY_PROFILE` and `ER_QUERY_FULL_CACHE` cannot be combined.

`query-fullcache-check.cu` checks the original and specialized kernels against
host witnesses, including exhausted misses, infinities, duplicate keys, minimum
IDs and tails. `query-check.cu` built with the new flag exercises public-backend
selection and forced full-cache refusal/partial-cache fallback.

The matched complete-recovery study is
`research/explicit_relations_fullcache_20261101`. No default is changed before
its results. The baseline generic kernel remains unchanged for comparison.
