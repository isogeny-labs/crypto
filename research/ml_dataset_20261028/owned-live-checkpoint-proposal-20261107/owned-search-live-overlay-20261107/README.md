# Borrowed live native search encoder overlay — proposal

Adds a private checked overlay to the existing encoder. The borrowed live continuation/search/native references substitute only active Running slots whose owned children are absent. Native state must have a coherent committed last batch, remain incomplete and have no active task. Live parent charge scalars replace their not-yet-returned stored counters. Existing fields, ordering and wire schema are unchanged. The snapshot source remains marked Incomplete.

The new checkpoint_native method uses the existing checkpoint_parts publisher and original Budget, sink, limits and deadline. Its actual owner method has not been compiled in the focused harness and no production caller selects it yet. Cadence and finite snapshot allowance remain an explicit policy integration requirement; the current owning worker is not changed.

28focusedtests pass:4new overlay/integration controls,7existing encoder controls,10BQI donor and7sparse donor controls. The historical fixture-emission test is deliberately skipped. Actual callback kernels run a first node and pass their borrowed state into the actual modified encoder; an admission adapter reserves encoder work/bytes on the same fixture Owner before publication. The native decoder verifies the resulting bytes, including Running status, nonempty committed basis and complete=false. Callback refusal retains nested child attempts.

Other controls compare encoded overlay bytes exactly with equivalent materialized state, verify no input mutation, reject incorrect/nonrunning slots, and retain acknowledged prefix/pending bytes when the sink refuses. The fixture sink is in memory and surrounding original owner/source identity is structural. This is not durable CaptureEmitter or original-cell acceptance, state rehydration, a new compute grant, or causal operation certification.

Source extraction omits with_parts/with_continuation_parts and actual owner checkpoint methods; their proposed source parses but remains untypechecked. Existing source identities and kernel/capture contracts informed the design. Published src and the pending build candidate70e0b1d8f435 remain unchanged.

The private constructor checks state eligibility, not provenance authentication. Internal attempt structures can be fabricated; the trusted original-source constructor, production callback callsite and downstream mathematical replay remain required.
