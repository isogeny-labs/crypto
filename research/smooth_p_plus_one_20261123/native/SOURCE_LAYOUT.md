# Production adapter source layout

`src/production_current.rs` is the compiled `production` module, selected by the
explicit path attribute in `src/main.rs`. At the layout repair it is byte-for-byte
the current source from cold-control revision a4cd5ee0b3; no algorithm changed.

`src/production.rs` is a frozen historical source artifact, restored byte-for-byte
from ed5bdacdcb. Claim PCC-20261124-f6947c pins that exact path and SHA-256
`e34dcf1292350c400296b93335c87b7a02f5c3d81ee9bd8c0c6aa3b08feb48bb`.
It records the earlier ten-test adapter state and is not the compiled module in
the current tree. Later additions at that mutable path had broken the historical
binding. The repair preserves the original claim and evidence while retaining
the later implementation under a distinct path. Do not modify this snapshot.

The frozen cold-control executable remains from a4cd5ee0b3, before this path-only
repair. Its source identity, observations and hash have not been rewritten.
Current claims should pin immutable evidence or explicit source snapshots, not
this moving implementation path.
