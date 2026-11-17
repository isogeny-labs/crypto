# Independent native input-custody review

Exact published input b4684a67ca626d338099525aefae817a3d276e78, native-branch-composition/input-custody, was retrieved with git show and compared byte-for-byte to the producer source. Offline compilation and all35 producer fixed controls passed in a fresh private directory. The independent native checker exercises37 additional fixed cases, including actual qualify_inputs composition; all expected outcomes pass. No daemon, Docker, gate binary, collector or scientific job was executed.

## Claim fidelity

strict_json recursively wraps map/sequence values, checks decoded String keys before insertion, and calls deserializer.end. Escaped spellings and surrogate-pair/raw-Unicode aliases collide. Distinct Unicode normalization forms remain distinct; this is not Unicode normalization. Serde numeric/depth behavior is retained. The parser rejects unpaired surrogates, comments, trailing tokens, malformed numbers and a byte cap overflow. Ordinary scalar/array JSON is allowed; the downstream adapter enforces expected object structure.

snapshot requires a nonempty exact flat fixture inventory and mode0555 directory. Each expected regular file has mode0444, one link, exact bytes/digest and caller file cap; the gate has mode0555 with the same regular-file checks. Nonfiles, hardlinks, symlinks and noncanonical path spellings are rejected. An empty file with an explicitly matching empty digest and zero cap is accepted; empty inventory is rejected. Directory enumeration equality rejects extra empty files. The API neither authenticates caller-selected pins nor requires nonempty file content.

qualify_inputs parses raw inspect and expected JSON under2MiB/16KiB limits, joins mount-source names to custody paths, snapshots before and after the imported adapter, and compares snapshots. Independent controls reached the composed successful path and separately rejected writable mounts, wrong owner nonce, failed transport, extra diff, duplicate escaped expected key and wrong gate mode. The accepted result still records runtime_qualification=false.

## Concrete remaining interfaces

File identity observations use device, inode, length, mode, links, modification and change timestamps, with checks around an opened descriptor and a final pathname observation. They do not supply atomic pathname-to-mount custody or prevent transient replacement. Files are checked regular before File::open, but a replacement between check and open can be a FIFO or device and block before the descriptor identity check. Canonicalize, read_dir, metadata and ordinary regular-file reads can also block on the filesystem. The byte cap limits accepted file size, not wall time; read_dir accumulates names without an independent count cap. No blocking/race experiment was run. Integration needs an external bounded supervisor and a clearly scoped trusted-filesystem/ownership model; a late hash comparison cannot implement a deadline.

The module imports the reviewed metadata adapter and does not execute its process lifecycle. Full native supervisor, external pin admission, prepared-manifest/runtime-archive joins, daemon mount identity, cleanup and campaign limits remain outside this result. A descriptor/path stability check is accurately reported as an observation, not race freedom. No change to scientific authorization is made.

## Independence and retained failures

Reviewer portfolio_audit authored the imported prior adapter, but did not author this new custody module. Review was source-first after publication, not blind. The reviewer authored a separate native controls crate using the exact published library. Its first build hit an ordinary Rust double mutable borrow in fixture setup; source and compiler diagnostic are retained in attempt1. Splitting that setup expression into two statements repaired only reviewer code. Subject bytes are unchanged. No subject failure was repaired or hidden.

## References

RFC8259 sections4/8.3/9 explain duplicate object-name interoperability, escaped string comparison and permitted parser resource limits: https://www.rfc-editor.org/rfc/rfc8259. Rust canonicalize documentation specifies absolute normalized paths with symlinks resolved: https://doc.rust-lang.org/std/fs/fn.canonicalize.html. Both primary references were inspected in this review. The additional symlink and byte-spelling checks are explicit source behavior rather than a guarantee supplied by canonicalize.

Commands and source hashes accompany the packet. These are software fixed controls, not measurements or campaign execution. Full target scope1260 is unchanged.

The initial report/acceptance predicate mistakenly expected38 reviewer rows; the actual fixed inventory contains37. That fail-closed predicate result is retained. The final exact37 predicate passes; no test was removed or repeated to change this count.
