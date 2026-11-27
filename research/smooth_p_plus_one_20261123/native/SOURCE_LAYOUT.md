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

The semantic-acceptance integration also compiles `src/full_cutoff_current.rs`
through an explicit path in `src/main.rs`. `src/full_cutoff.rs` remains the
immutable liftable-fiber source pinned by PCC-20261125-8eaff2, at SHA-256
`aebb71359d931b12ed4cf84643350c83c09a06ce4a4b6868a5cba07f202e1938`.
The current module adds the shared canonical form serializer and semantic
residue guard from `src/semantic.rs`; its failure records retain raw RURs.
Historical manifests describe their source commits and are not rewritten
to describe this corrected policy. The pinned system/liftable modules and
the original production.rs remain unchanged.
