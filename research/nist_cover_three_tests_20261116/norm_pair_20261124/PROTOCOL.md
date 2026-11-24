# Frozen tangent trace signed-pair audit

H-G:at least one residual trace of a recorded tangent has a decomposition
into two signed old-base symbols that yields a new old-column relation;
its old-point divisor lift may or may not be principal. Distinguish norm
relations,principal target classes and additional coefficient rank. A pair
trace equality does not identify the actual residual's individual images.

This is static/development analysis of the immutable226 curve points,their
recorded images,225 old canonical base points,and226 already-recorded tangent
traces. No new curve points,root extraction,point-pool expansion,parameter
scan,scalar recovery or paid campaign. Old files stay byte-equal.

Use alphabet O,+B0,-B0,...,+B224,-B224 and every unordered pair including
diagonals. Compute full ordered pair fingerprints with OpenSSL named P-192
and direct GMP arithmetic. Bind p,a,b,n/cofactor and all old projection signs.
Compute kappa from the first recorded reference section by both backends.
For every recorded point P compute T=kappa-2f(P) independently and compare it
to the frozen tangent record. Retain all226 queries and split/non-split labels.

First join:grouped pair-key buckets queried by T. Second join:independently
sorted direct-GMP pair keys with range lookup. Compare every query's complete
unordered-pair witness set,including formal duplicate/zero rows. Every match
gives the coefficient row 2*projection(P)+symbol(A)+symbol(B)-Krow.
Check elliptic sums by both backends. Retain all witnesses and deduplicate
exact signed coefficient rows only for target/rank work. Do not count zero
rows as useful. Report old-plus-all-norm rank modulo n and split-subset counts.

Known-answer query controls on the same fixed alphabet require retrieval of
+B0+B1,2B0,O+B0,and O+O;include identities and repeated support. Verify a
changed query rejects its unchanged pair witness,remove one fingerprint item
and require rejection,and change a row coefficient and require nonzero sum.
No outcome-based fixture replacement or cap tuning follows a failure.

For each distinct candidate row use the existing signed old-point lift:
replace a canonical elliptic symbol by its recorded curve lift/sign,balance
degree at the recorded identity-image point,split positive/negative support,
and pad both to degree12 at the same old anchor. Use the unchanged complete
Newton-jet target encoder. Classify each actual target difference as zero or
nonzero;never relabel norm-only as principal. Run six original principal
controls and reject a coefficient-mutated baseline. Report old-plus-principal
and old-plus-all-norm ranks separately after checking all classification
bindings. A nonprincipal norm row may be an elliptic identity but does not
satisfy the principal-cover-relation obligation.

Caps:4096 total pair witnesses,256 distinct target rows,120seconds/1GiB audit,
600seconds/1GiB target,30seconds/1GiB rank classification,300seconds/1GiB per
strict native build. Cap exhaustion is a retained incomplete outcome. Fresh
write-once source/input/binary snapshots and manifests precede every worker.
No performance/global-absence/novelty claim. [pcc:unverified]

H-G fails as a one-row old-base route if there is no norm witness,if no
candidate principal class is new,or if principal rank stays6. None of these
outcomes settles closure among several actual new-point rows. The proposed
102-section collection remains a separate unexecuted campaign.

Sources:recorded_pool_20261123 original,signed and tangent protocols/reports;
actual signed target code and rank implementation;Diem/Magma reviews already
retained in the study. These are ordinary pair-sum joins and divisor norms.
KB unavailable as recorded in the journal;no novelty conclusion from that gap.
