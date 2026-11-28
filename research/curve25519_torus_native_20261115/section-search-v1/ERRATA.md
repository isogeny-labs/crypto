# Pre-call qualification amendments

The original prospective protocol/SOURCES bytes stay pinned by c9c89b.
Before the final controls call, the new issuer's hash adapter was switched
from importing src/hash/sha256.rs to the pinned sha2 crate with a thin
interface adapter. Strict Clippy on the imported legacy
SHA source reported five chunks_exact_to_as_chunks advisories. The shared
source was not edited, and no lint was suppressed. The initial committed
source, intermediate source archive and failed logs are retained. The final
native source qualification pins this adapter and Cargo.lock. Mathematical
statements, fixed streams, refusal controls and maintained identity module
remain unchanged. [pcc:unverified]

The first standalone Lean gate found lake absent from PATH; the already
installed pinned binary was located without downloading. The next attempt
used an import that supplies CommMonoidWithZero but not CommRing, so its
stronger unneeded ring binder did not elaborate. The endpoint now uses the
existing multiplicative structure it actually needs. No theorem about the
geometric quotient, count or solver is inferred. The failed source/logs are
retained; helper/trust/configuration bytes are unchanged. [pcc:unverified]
