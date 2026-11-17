# Native qualification diff adapter proposal

The proposed Rust library implements the exact-three-addition predicate and the identity, terminal-state, mount, resource and security prerequisites that must precede it. Fifty-eight fixed native controls pass, including valid diff bytes paired with invalid prerequisites. The original Python v5 supervisor and failed receipt remain unchanged. This is a native adapter component; a complete native supervisor and its callsite are not implemented or admitted here.

## Native interface and checked conditions

src/lib.rs exposes exact_diff(&[u8]) and qualify(case, transport_exit, bytes, inspected_container, expected_identity). qualify accepts only Q1–Q3, full container/image IDs, the nonce-bound name/label, expected user/entrypoint and case-specific stopped exit state. It checks read-only rootfs, no network, the existing PID/memory/CPU bounds, dropped capabilities/security options, namespace/device restrictions, exactly the fixture and gate read-only binds, unique mount destinations and approved tmpfs options. It then requires transport exit zero and the strict bytes predicate. Result fields preserve null writer attribution and false no-writes claim.

The expected source paths are supplied by a future trusted, manifest-bound supervisor; the adapter does not itself inspect host files or verify Docker output provenance. It checks structured inspection data, not the daemon. Missing/wrong prerequisites return Err; the fixed-control executable returns nonzero if a control fails. It cannot launch Docker, a subprocess, a container or a payload.

## Controls and source preservation

Nine positive controls comprise Q1/Q2/Q3 and all six byte orderings. Forty-nine negative controls cover identity, image, nonce, user, entrypoint, terminal state, network/rootfs, strict numeric limits, privilege/namespace/device restrictions, missing/wrong/writable/duplicate mounts, tmpfs, transport, inactive cases and malformed diff bytes. Every accepted output retains the explicit claim boundaries. Results are deterministic and match a second replay; source inputs are hash-checked unchanged.

The published legacy parser and v5 supervisor are reference bytes only. Initial source-first work drafted a pure Python adapter, but the coordinator identified AGENTS585–611 before any Python test or supervisor change. That draft is retained unexecuted and rejected in unexecuted-python-draft/. NATIVE_AMENDMENT.md records the correction. The Rust deliverable neither invokes nor extends Python execution. A first single-file Rust layout passed controls and was preserved before extracting the public library and formatting the final source.

## Eight amendment requirements: implementation versus remaining work

1. Exact revised three-addition contract is implemented in Rust. Historical v5 remains failed; no historical outcome changes.
2. binding-template.json binds native adapter/control artifacts but deliberately leaves independent review, native supervisor, runtime image/binary/fixture/gate and admission fields unfilled. No executable admission results from it.
3. The pure adapter implements identity/read-only mount and resource/security predicates. Actual trusted collection of that inspection and host manifest validation remain supervisor responsibilities.
4. The pure adapter rejects nonzero transport and malformed/incomplete bytes. Collecting capped bytes from the exact owned container, preserving command receipts and cleanup remain unported runtime work.
5. Native function-level controls test valid diff with invalid prerequisites. No actual supervisor callsite is claimed; that composition needs a native port and independent tests/review.
6. Q0 static inspection, Q1 diagnostic semantics, Q2/Q3 negative outcomes, Q4 fixture immutability and exact-ID cleanup remain preserved in frozen v5 evidence. They have not been reimplemented or rerun here.
7. Proposed limits remain 300 total seconds,30 cleanup reserve,20 per active case. A full native supervisor must enforce them; this pure library uses no clock and grants no execution authorization.
8. Neither parser nor adapter attributes the writer or claims absence of all writes. Pre-exec PID evidence retains its historical temporal limit.

The reviewable proposed-native-additions.patch adds the complete native library and fixed-control binary source. It does not patch the frozen Python supervisor. No Docker/runtime invocation, scientific execution, deployment, production edit or repository edit occurred. Independent review and full native integration remain separate next steps.
