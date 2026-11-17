# Independent exact mount-diff parser review

The exact published parser passed 43 independent fixed software cases: six orderings of the proposed three additions plus the retained v5 bytes accepted; 36 malformed/type/operation cases rejected. All accepted results preserve writer_attribution=null and no_writes_claim=false. The parser faithfully implements its proposed exact-byte language.

## Source and retained failure

Reviewed publication: PR3056 commit05fc018e64e948fbe91d5463036e55d4c97d13f1, mount-diff-proposal/mount_diff.py, SHA256 f38a3070281df684381fe3fd2d33f9ef30cce92f445f869356d6c1413dc3d518. Exact source and associated inputs were copied privately and hash-checked unchanged. The parser requires exact bytes type, ASCII, at most1024 bytes, exactly one terminal newline, three unique lines and set equality with the three additions. It performs no normalization or prefix acceptance.

The retained v5 receipt is still failed: Q0 passed, Q1 stopped at unexpected writable-layer change, cleanup_complete=true, qualification_complete=false. The native diff contains exactly A /fixtures, A /qualification and A /qualification/exec-gate. Applying the proposed parser to those saved bytes tests the revised rule only; it does not make v5 pass. Original receipt/diff/supervisor hashes match the proposal bindings.

## Identity and mount prerequisites

Source supervise.py creates cases with exact owner nonce/name/image and full container ID checks, read-only rootfs, restricted user/entrypoint, no network, narrow resource/security/namespace conditions, exact fixture and gate bind sources/destinations with RW=false, and only the approved /tmp and /work tmpfs settings. The selected retained Q1 create and terminal metadata agree byte-for-byte: same full ID/image/name, two expected read-only binds, read-only rootfs and expected user. This is retained metadata consistency, not a fresh daemon observation or writer attribution.

The existing run_case still checks rc==0 and empty diff. It does not import or call mount_diff. The parser accepts bytes in isolation and cannot know whether identity/mount predicates were established. A versioned adapter and its composed prerequisite checks therefore require review before any qualification rerun. AMENDMENT_REQUIREMENTS.md gives the exact integration and binding obligations while preserving the original limits and failed evidence.

## Independent controls

Cases cover all orderings; missing/duplicate/extra lines; C/D for every path; empty input; invalid types including bytes subclass; NUL and non-ASCII; CRLF/CR; whitespace/framing differences; dot/parent/double-slash/trailing-slash/prefix paths; and oversized input. All expected outcomes matched without importing the supervisor. The producer's historical control report was inspected but not treated as independent evidence. No Docker/client/container, solver, timing or scientific workload was executed.

## Reference and claim boundary

The official Docker CLI reference (https://docs.docker.com/reference/cli/docker/container/diff/, inspected2026-11-01) defines A/C/D and changes since creation. It provides no causality or writer identity. Accepting these entries cannot support a daemon-attribution claim or absence of all writes. Read-only bind/root metadata, tmpfs activity and counterfactual execution are separate questions. Likewise the retained pre-exec pids observation does not prove pre-start or continuous enforcement.

The reviewer did not author this parser and knows the historical context; this was source-first, not blind. Machine status: fixed software controls passed. Fidelity: exact parser contract holds. Integration/qualification/admission status: not established by this review. No deployment or execution admission was performed.
