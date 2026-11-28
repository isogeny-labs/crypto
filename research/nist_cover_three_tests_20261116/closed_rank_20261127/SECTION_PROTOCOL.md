# Complete line sections on the lifted pool

Preserve the original 225 elliptic columns, six baseline rows, first split
reference fibre and 108-row tangent matrix. The native collector loads either
the original curve pool (qualification) or all 15 fixed fibre batches. In the
latter mode it requires coverage of old indices 0..224 and O exactly once,
rechecks every supplied point/image, and recovers the original 226 points
with their original ordering and image bindings. Extra points must map to
O or one of the original signed elliptic points; no new column is admitted.

Enumerate every pair of distinct lifted points and every tangent. For a
secant through P,Q, the line restriction has known roots 0,1; divide by
t(t-1) and solve its residual quadratic, retaining repeated roots. For a
tangent at P, divide by t^2. Reconstruct the complete quartic factor product.
Accept only sections whose entire support belongs to the fixed supplied pool;
deduplicate the full four-index multiset. The native no-rational-infinity
check ensures that these rational lines lose no support at infinity.

Freeze limits: at most 3000 lifted points, 4096 accepted sections, and 64
anchor indices per scan worker. Secants are assigned uniquely to their lesser
anchor index; tangents to their own anchor. The complete run partitions all
indices into consecutive blocks of 64. No repeated-pair or changed-reference
rows count as rank gain. A collector timeout is retained, not converted into
an absence result. The old-pool qualification must recover exactly seven
sections and old rank six.

Each accepted section gives the actual signed elliptic row H_line-H_ref.
Check its elliptic sum and retain its complete restriction, line coefficients,
origin, support multiplicities and old-column row. Rank uses the fixed prime
subgroup coefficient field, not the coordinate field. Recheck that each of
the original 204 auxiliary columns is private to an original added tangent
row, all 102 added rows have private witnesses and the six baseline rows
agree. Since the lifted-pool sections use no auxiliary column, the combined
closed rank is the rank of baseline plus these rows, with unchanged auxiliary
rank 102. A positive **candidate** gain still requires actual KM principality
and separately checked elliptic/rank witnesses before being established.
[pcc:unverified]

Qualification controls: a changed row coefficient must fail the elliptic
check; a changed baseline row or private-column incidence must be rejected;
an omitted scan block must reject complete accounting. Synthetic detector
controls remain separate from actual data. All compilers/workers retain the
one-CPU/1GiB and 300s-build/600s-worker caps, exact exits and all stage costs.
No scalar recovery, new PCC claim or speedup statement. [pcc:unverified]
