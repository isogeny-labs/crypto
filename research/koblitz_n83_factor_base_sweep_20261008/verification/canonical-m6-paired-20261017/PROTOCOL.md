# Frozen N83 equal-domain canonical 4+2 pair

Use the exact public target, first-trial coefficients, sequential K600 base,
Gray-prefix K600 pool and hash-derived K600 pool in the prior
[4+2 plan](../hosted-m6-paired-protocol-20261017/plan.json), SHA-256
`7788514127328a78cf2b0f634003ddae3ff18a9ff0c0c8cd75b50fcc006152cd`.
Regenerate the three archived factor bases from the committed 54-object panel,
then verify their compressed hashes against the plan and the S3 custody
receipts before search. The pools leave 97 and 600 residual signed-Frobenius
orbits, respectively. Both arms receive the **same** target and coefficients;
the only factor-base input difference is the residual pool.

The worker's new `run-cross-pilot-canonical` mode orders the four small-base
summand x coordinates and the two residual x coordinates independently. It
does not order across the disjoint domains. Each run uses the same complete
full-point set within a four- or two-position block, so sorting a valid point
tuple preserves the allowed group sum. Sorting may change which partial sum
is the identity; the worker therefore retains all 16 intermediate-identity
patterns. A 20,000-conflict or wall cap is censored, never a completeness
claim. The canonical constraints add four 82-variable lex comparators,
versus one comparator in the earlier first-pair arm. Predicted peak variable
counts are 118,930 for Gray and 120,942 for hash, below the 150,000 cap;
the exact worker receipts control.

Run Gray then hash on the same Linux ARM host, digest-pinned Rust image and
one-CPU, 4-GiB, zero-swap, networkless read-only containers. Each gets one
public query, at most 20,000 conflicts and 64 models, a 2,300-second worker
cap and a 2,400-second external cap. The entire hosted job has a 7,500-second
cap, charged conservatively in full, including regeneration, build, imports,
both searches and evidence upload. Preserve source/workflow/image hashes,
archive bytes, container inspection and cleanup, raw search logs and all
stage receipts. An independent audit must check every group candidate over
the original curve before promoting a partial. A missing second arm, cap,
timeout, invalid model or infrastructure error is separately classified.

This is a matched *one-query solver-access screen*. Even a validated partial
would require disjoint repeated queries, double-large-prime graph closure,
useful rank, sparse linear algebra, one individual logarithm, verification,
matched rho and full setup cost before choosing a minimum-total-runtime base.
Keep `selected_best_total_runtime` null until those gates pass.
