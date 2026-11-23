# Stream unique witness replay within the unchanged memory bound

The full E23 v2 panel completed with 360 profile-ledger entries under its
original cap. `assessment-20261023-v2` then exhausted its 4 GiB address-space
bound: the checker retained every parsed witness/matrix in a JSON cache.
Its allocation failure is preserved as an incomplete assessment. No resource
limit is increased and no mathematical check is removed.

The amended checker streams each unique witness's raw and echelon equations,
verifies its source scalar and records only its target and optional scalar
in the cache. The actual witness content is already SHA-256 inventoried and
its raw-row checksum is checked before group replay. Repeated references
reuse only the successful replay result. Rebuild and commit this source,
then run `assessment-20261023-v3` under the unchanged protocol and limits.

Historical v1 checksum failure, v2 memory failure and their partial outputs
remain separate. E23 v1 still has the cross-arity evidence-retention bug;
the completed E23 v2 raw panel is the input to this assessment.
