#set page(paper: "a4", margin: 18mm)
#set text(size: 11pt)
= Independent materializer source review
Published producer: 357f73fe0eb945c9406fab542def431de51ee64c.
Preread plan written before implementation inspection. [pcc:unverified]

#image("review.svg", width: 100%)

M1 — Line114 checks a candidate hash, then separately reads/copies the file. The final manifest accepts the newly copied digest. Changed bytes between those reads can pass the final comparison. Bind the bytes actually consumed and require the destination to match the approved digest; recheck bound inputs. [pcc:unverified]

M2 — Lines49 and171 reap the owned child without a timeout. Process-group cleanup is skipped when the leader has exited. Bound cleanup and retain terminal/descendant status; require outer cgroup confirmation. [pcc:unverified]

Pinned Git source selection, archive checksum matching, safe extraction checks, fresh manifests and declared resource guards are present. The concrete admission and bounded outer supervisor remain to be published and reviewed. Initial compiled-cache hashes record current bytes; they do not establish independent build provenance. [pcc:unverified]

The owner's dollar waiver leaves the46,800-worker-second ceiling and November6 04:00 UTC deadline unchanged. V1 remains immutable; corrections need a versioned delta review. No materializer, compiler or experiment was executed. [pcc:unverified]

Reference: Cargo Book, Source Replacement. Full source traces, line references and bindings: REVIEW.md, result.json and findings.csv. [pcc:unverified]
