# Attempt-bound typed event bridge

The approved Stage3/resource contract requires the new profile to use typed
capture with no legacy downgrade. This additive solvers.rs bridge forwards the
closed capture kind, borrowed Serialize payload and caller-owned reusable
buffer directly to CaptureEmitter::event_typed. Missing and Legacy sinks return
an error before serialization or persistence. Existing live_event behavior is
unchanged. No runtime dispatcher or capability is activated.

The caller must debit TypedEventBuffer::requested_bytes once from its real
ledger before allocation, reuse that buffer, and account both encoding passes.
The bridge cannot authenticate a claimed admission or infer that ledger debit.
Capture's sticky refusal and committed-prefix rules remain authoritative.

One authored integration control exercises absent/Legacy rejection, successful
Bound persistence, oversized serialization and the subsequent sticky refusal,
unchanged committed event bytes, and thread-local cleanup. This is not a
runtime test result; root-owned compilation/focused/full gates remain pending.

Source references: approved contracts/RESOURCE_BOUNDS.md and
contracts/STAGE3_ADAPTER_DESIGN.md; typed capture design and source1eb1fef3624f.
This follows the existing envelope/hash mechanism, introducing no new
mathematical algorithm or cryptanalytic claim.

The same bridge control also captures the committed prefix into an immutable
snapshot after the sticky refusal and runs verify_captured_delta on it. This
checks compatibility with the existing snapshot consumer, not only raw JSON
parsing. It remains an authored, unexecuted control until the runtime gate.
