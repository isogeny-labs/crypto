# Post-run editorial annotation to the frozen protocol

After the source-pinned run, `pcc lint-prose` identified two paragraphs
in `PROTOCOL.md` that said "verified" without an evidence tag. The
original frozen text remains at commit `15a7782ef3`, SHA-256
`ac04d731f2f5af3beb93fbf7afa38659bdb3cd027da212e9bd6cc4a6a01bc18c`.
Two `[pcc:unverified]` tags were added to the current protocol for prose
lint. They change no input, arm, metric, sample count, stopping rule,
control, or decision. This annotation occurred after data and is not a
pre-data amendment. The result record binds both the original frozen
protocol and the annotated copy. [pcc:unverified]
