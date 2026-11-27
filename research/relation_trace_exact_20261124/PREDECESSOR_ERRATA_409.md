# Correction to the proposal review

The existing certified-relations crate already contains trace(), hilbert() and analyze(), including certified closure, single-addition, leave-one-out and greedy ablations. They predate PR5102. The proposal review missed these implementation details. The earlier statement that no tracer was implemented was scoped to that review, but the recommendation to build a tracer overstated what was missing locally. [pcc:unverified]

This follow-up reuses the existing polynomial/provenance arithmetic and extends the public diagnostics with raw-versus-closed fixed-degree ranks, bounded-derivation replay without a reference-basis completion oracle, and a P/Q augmented-row information audit. The original trace/analyze algorithms and assertions remain. Neither this extension nor the old tracer is an F4/F5 runtime measurement. Earlier report405 and its claim payloads stay frozen; this correction is additive.
