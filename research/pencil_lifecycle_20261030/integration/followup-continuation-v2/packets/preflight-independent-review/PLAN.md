# Independent preflight source-fidelity plan

Frozen before source or correction inspection. Scope: integrated four diagnostic_* files, CLI ecbench.rs, and full-host-digest correction patch. Parent reports prior-author integration audit and live root release compile; this reviewer neither edits nor builds root.

Threats to check: joins that silently accept duplicate or absent target IDs; input digests that bind a summary but not consumed bytes; host identity comparison allowing prefix collisions or missing properties; run/pair binding confused across controls; path or canonicalization differences between hashing and parsing; optional fields bypassing custody checks; report success despite missing qualification or incomplete controls. Check CLI actually invokes reviewed validation and structured errors carry failed gate.

Method: inspect prospective parent plan then exact pinned current source, bind hashes, apply correction only to private copy if needed, compare each semantic change against validation contract. Trace a concrete valid input and strongest malformed boundary cases. Existing bounded standalone fixtures may be replayed privately if directly useful, no cohorts/cloud/root build. Produce source-bound findings with clear producer versus independent evidence and neutral PDF/SVG/JSON. No repair of producer source.
