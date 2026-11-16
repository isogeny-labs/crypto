# Independent v6 preparation binding closure

Read-only delta review,2026-10-30. Reviewed runner-clock-v6-increment.patch SHAc425071b13b9340c08d7d66092538ae3da1704a45f4c6e79fafebda7dee1812c, specifically runner-v5.rs to runner-v6.rs SHA c89da50bc72e613f20bf44801f374c4339272c8c26039ff387fdadf3fa9dcfe2. No application, edits, build, tests or scientific execution.

The v5 exact-preparation binding gap is closed at producer source level. Publication returns SHA256 over the actual preparation bytes; clock_id is derived from runner_id and that digest. The preparation file omits clock_id to avoid circular hashing and records the derivation rule. All journal/tail/prefix clock identities use the derived value. Altered origin, whitespace or runner identity changes the binding. The actual resulting longer clock_id participates in the existing serialized capacity admission, so it is not omitted from the8MiB bound.

No new concrete defect identified in the delta. The pure mutation fixture is appropriate source coverage but remains unexecuted in this review. Consumers/external observers must recompute and compare this derivation against retained exact preparation bytes, rather than treating an arbitrary clock_id as proof. This integrity relationship does not authenticate the observer or establish restore/clock qualification.

The earlier v5 review remains preserved. Its remaining clock scope, unsupported restore, active-time-null, no external exit and software-verification limitations are unchanged. This closure note establishes neither an admitted experiment nor complete runtime/cost accounting.
