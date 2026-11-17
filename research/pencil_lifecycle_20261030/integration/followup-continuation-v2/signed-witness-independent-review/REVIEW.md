# Independent signed witness source review

Reviewed commit: `9c6d9c61c52bdbfb42b79148aef694d4bebdafdc`. Reviewer did not author the reviewed packet. Exact hashes and lengths are in source-hashes.json; machine-readable findings are in review.json. This review predates the reviewer-authored checker-v2 wrapper and is not independent review of that wrapper.

The determined certificate combines an isolating row combination with a particular solution, proving both existence and uniqueness of the target coordinate. The undetermined certificate gives a solution at target zero and a homogeneous direction with target one, hence solutions for every target field element. The inconsistent certificate combines rows into 1=0. Static inspection found the elimination and direct-sum checker faithful to these implications.

Independent read-only reconstruction checked all 64 unique source streams and 6,900 edge rows. Coefficients are accumulated for repeated nodes, node zero is omitted, and final columns are -b and -a. This agrees with signed-replay/src/lib.rs and the pinned stream header convention. Identity rows have the correct static formula but none occur in this archive. Actual classifications and summary joins agree: 39 determined certificates record scalar 2 and 25 undetermined certificates match capped terminals. Hashes and sizes of members, whole-archive artifact bindings, publication-manifest entries, and summary source hashes matched. Principal sources were compared byte-for-byte with the reviewed commit.

Two checker gaps were found. check_only.py:25-32 counts entries without enforcing distinct names or complete source-set equality. check_only.py:22-23 binds retained metadata without checking certificate scalar/classification against it or joining the summary. These are enforcement gaps; independent inspection confirmed actual coverage and result joins. The new sibling wrapper implements these checks without modifying the original packet.

The original review did not rerun certificate equalities, elimination, collectors, or scientific experiments. Separate algorithms alone do not establish reviewer independence. This separate non-author review was not blinded and does not newly validate the geometric source-group chain. The later wrapper is authored by this reviewer and requires its own independent review if that gate is needed.

One reviewer-script failure is retained here: the first inventory check accidentally compared 64 JSONL streams with all 192 archive entries, including header/result sidecars. Restricting the expected inventory to the declared JSONL scope passed on unchanged inputs.

Sources consulted: the original packet PLAN.md, PROOF.md, REPORT.md, README.md, certificate sources, native-stream report and signed-replay implementation. This is a local source-fidelity assessment, with no literature novelty claim or new scientific decision.
