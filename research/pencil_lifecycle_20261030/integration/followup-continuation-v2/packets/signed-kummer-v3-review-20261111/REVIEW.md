# Independent v3 trace-envelope source review

The v3 source addresses the v2 blocking envelope mismatch. Rust serializes `Record { sequence, event }`, with `Event` internally using `kind` and `data`; the parser now follows that shape and rejects a nonzero or noninteger first sequence. Both changed-identity controls use the real envelope. [pcc:unverified]

All 51 manifest members matched their recorded hashes and sizes before publication packaging. Native worker and staged input contracts are unchanged; empty header-fault behavior, panels and resource limits remain preserved. This is a source review, with no resolver, compiler, test or scientific execution. Actual locks, published final bindings and runtime qualification remain pending. [pcc:unverified]

Sources checked: sealed v2 independent review, v3 successor.patch, panel_acceptance.py, acceptance_negative_controls.py, lock generate_lock.py and kummer-signed-driver-20261109/relation_stream.rs.candidate lines123-140 and242-248. KB tools are unavailable and local wrapper absent; exact source files supply the serialization contract.
