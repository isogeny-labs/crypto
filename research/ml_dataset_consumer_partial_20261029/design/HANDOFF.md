# Partial-consumer design handoff

Design completed for parent review on 2026-10-29. Implementation remains unauthorized pending the parent's freeze and fresh scope assignment. Parent reviewed the substantive design during drafting; the final immutable packet adds exact hash-domain vectors, the retained mock-phase fixture pointer and the clarified anchor-pin ordering test. No repository, owner, cloud resource, job or dataset was changed.

Primary documents:

- `DESIGN.md` — SHA256 `bd18ddaea375cef06f34f302c3f93a2c22657aed91a6a02658485e3ac8b31172`.
- `WIRE_AND_MODULES.md` — SHA256 `2d0e5bea91ffebd112e39bee111fd018a192cb0850cd79698f5c6f33e884cba5`.
- `ACCEPTANCE.md` — SHA256 `7bd4d45054845571774dbe992148fa9be6647f525fd6a9bc8106c34bf46af71f`.
- `vectors/README.md` — SHA256 `9b2fffb5faa52d4f3b2a8b48230ba2f9d15fc572b8201940c7607ed94b469856`.

The proposed increment verifies complete v1/v2 predecessor chains, exact metadata/object memberships, actual restored authoritative context, committed event bytes and same-delta payload references. A consumer-owned typed context/phase core is reused without fabricating a sealed cell. Separate local restore, fresh read-only audit and read-only remote-fetch adapters share this verifier. No capture-containing lineage can pass by omitting event replay.

The acceptance matrix contains 16 positive cases, 30 semantic/transport negative classes with full outer rehashing where applicable, 14 admission/TOCTOU cases and explicit legacy/leakage/CLI gates. Literal binding/genesis/two-event/empty-payload vectors pin five exact byte strings and expected cursor values. They are design vectors, not executed consumer tests or complete dataset fixtures.

Current retained producer CLI evidence is one initial policy-only v2 snapshot. The later test-retention packet supplies four-phase and registryless start-only bytes with mocked counter acquisition, not partial lineages. Its older `{event,sequence}` live stream is not the committed capture-envelope format and is not silently relabeled. Full partial fixtures must still be assembled independently and tested after implementation authorization.

The 256 MiB parent allocation model, carried 32 MiB phase model, finite metadata/object/chain caps, exact-size I/O, strict namespace and final fresh audit are admission requirements. They are not RSS guarantees or measured cost. Reports keep hardware/platform/science/cell-completion/solver-resume/training admissions false, whole-worker counts null and public sealed exports unchanged. Local restore, controller S3 archival and worker S3 publication remain separate evidence boundaries.

Source pins are producer27679 and consumer0ceec70; `SOURCE_PINS.json` maps the copied source files and retained fixture manifest. `SOURCES.md` preserves the parent's live-read primary AWS/Linux references and the POSIX403 limitation. `SOURCE_SHA256SUMS` binds source snapshots and `SHA256SUMS` binds this complete final packet. T766/T780 snapshots are read-only ownership evidence; no scope was claimed. The original USD200 total ceiling, deadline and worker-role restrictions remain unchanged. This task ran no build, consumer test, producer/generator, cloud command or PR operation.
