# Pending bounded native executable link

Prepared, not launched. The preceding approved local typecheck attempt was consumed successfully. This plan requires one new explicit authorization at the final published source revision.

One local invocation, two Cargo build jobs,1200seconds total for build plus CLI help, zero retries, shared heavy-build lock, offline locked dependencies, no cloud jobs and no solver or scientific execution. Command: cargo +1.98 build --release --bin crypto --locked --offline, followed only on success by crypto dataset --help. Original source hashes and Git revision must match before launch and source hashes after return. An existing started.json prevents implicit retry. Timeout terminates the process group and retains logs. Lock contention consumes no attempt.

Source manifest binds the actual proposed source files independently of documentation changes. Authorization must be recorded with the exact published GitHub revision; the checked-in default is false. This plan does not authorize native dataset acceptance or renew a scientific deadline. The existing local target cache is reused; no other active build or generator is restarted.
