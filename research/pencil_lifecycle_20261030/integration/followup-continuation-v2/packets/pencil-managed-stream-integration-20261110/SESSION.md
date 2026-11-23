# Managed pencil stream integration session

- Date: 2026-11-10/11 UTC
- Repository HEAD inspected: `f3c1b8cc7663ca7d15374ae5e9a16c86839c9d18`
- Scope: source-only I1/I2 implementation packet.
- Conductor: exact packet path checked before creation; result was clear.
- Execution: no compiler, test, payload, target, materializer, or cloud job was run.
- Frozen evidence: unchanged.
- Root source: unchanged; exact proposed edits exist only in `overlay/` and
  `integration.patch`; the initial semantic sketch is preserved separately.
- Shared session log: unchanged because it is under separate conflict resolution.

The exact managed and donor solver bodies were read before choosing the design. The
managed solver remains the mathematical implementation. The patch adds the donor's
ordered trace observations to that body and does not dispatch to the donor solver.
Progress publication remains diagnostic and nonfatal. Relation-stream failure is a
distinct attempt error, retains a durable prefix, and does not fall back to an
untraced solve.

The retained exporter is composed from the already source-built and independently
reviewed `retained_export.rs.txt` component. This packet adds the missing attempt and
cost join contract; it does not reinterpret caller-supplied custody as authentication.

`git apply -p1 --check` passed against an exact private copy of `original/`.
Independent source review is required before applying the patch or admitting a
build. Compilation, runtime behavior, and arithmetic parity have not been checked.
