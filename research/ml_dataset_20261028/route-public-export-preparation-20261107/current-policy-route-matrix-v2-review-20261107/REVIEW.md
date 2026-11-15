# Independent V2 routing design review

No blocking finding for this hash-pinned design inventory. This supplements the immutable V1 review; it does not authorize or validate native execution.

Read the V2 checker and design and verified every manifest entry against its bytes and SHA-256. Independently checked the 103 groups, 58 derived blocks, 649,592,832 original nominal cells, exact preservation of pending rows, and null protocol hashes / zero authorized cells for all derived blocks. The parent separately replayed the static checker; this review did not run native code.

The ordinary wide high-arity F4 requirement correctly remains pending capability: the separate dynamic method is not a configuration fix for ordinary F4. The resulting totals are 316,342,272 conditional-route, 3,145,728 encoding-only, 317,915,136 pending-capability and 12,189,696 pending-configuration cells. Original templates remain preserved.

V2 adds explicit positive-work, paired-twist, binary m3 degree/size and owned-profile guards. The exact source admission corrections remain dynamic degree 12 and binary/Koblitz legacy F4/F5 m3 configured base size 4. Actual realized signed-point/domain conditions remain pending. Predicates are handwritten necessary-axis checks over the hash-pinned templates, not a complete Rust admission implementation or general capability API.

Default replay accepts --repo and recomputes/compares frozen outputs without writes. Optional output goes to a separately supplied directory; direct resolution to the frozen packet is refused. This is a controlled local replay interface, not an adversarial filesystem containment guarantee. Source drift fails the hash guard. The earlier DESIGN paragraph describing output writing is clarified by the explicit V2 replay section.

No new build, cloud submission, budget, native validation, solver success, scientific trace completeness or experimental authorization follows from this review. Native schema/admission, realized inputs, finite runtime resources and registered controls remain the concrete next gates.
