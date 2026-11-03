# Independent review: fresh monotonic bound and API defaults

Read-only source review. No builds, root edits, watchdog execution or cloud actions. Rehashed exact native patch793c9c0ee7ef804293ba1f30a5a2c37d3b460702bf37950ffad38d19a1ff2efb and proposed main325b85a78a71f50c810521e338e3b571240ecc8a3aed137d2db53443a17ca312. Rehashed API patch6aa1f9a14c7c04e699dad9ef7930dbb8262556d2e9a7b6ca86a00194a8a3dce3 and cloud33748bb7cc58a4c3ce7258127f785fdd5855d3a53df9439d09eaa7d1497c3113 on base34cdc2bc.

## Native monotonic source

One StopBound is captured before validation and reused before spawn and in the observation loop. Instant is sampled before SystemTime; remaining duration preserves subsecond precision, cannot be enlarged by a later backward UTC step, and UTC forward expiry still stops. Invalid clock observations fail closed. Config parsing, request identities, receipt schema and existing child-failure/signal preservation are unchanged. The native owner remains opt-in through existing wrapper wiring. No defect found in this scoped delta.

Read final native test-v3 and clippy-v2 logs and verified their manifest digests:6unit+2integration tests pass, all-target Clippy completes. Did not rerun them. These qualify source software behavior, not asynchronous cutoff during every filesystem operation, actual Pod teardown or allocation release. Existing static66be334c… binary was built from old main e7fc190e… and does NOT qualify proposed main325b85a7; a new source-bound static build is required before packaging the correction.

## API-default source finding

The helper is correctly gated by fresh_owner.is_some(), preserving absent-option manifest behavior. It pins the documented ObjectFieldSelector.apiVersion v1 default while retaining exact env/volume equality, fields, paths and explicit versions. The official source is https://kubernetes.io/docs/reference/kubernetes-api/core/pod-v1/#ObjectFieldSelector . Existing API fixture tests default roundtrip and identity mutation rejection; the final rebased full corpus is not established by its earlier pre-rebase logs.

One narrow defect remains in the reviewed exact API patch: item["fieldRef"].as_object_mut() invokes mutable Value indexing, which inserts fieldRef:null when a downwardAPI item instead contains resourceFieldRef. That is an unintended mutation beyond setting the known default. Current diagnostic volume has fieldRef and therefore does not trigger this defect, but the helper's default-only contract is violated. Recommended exact fix: item.get_mut("fieldRef").and_then(Value::as_object_mut), with a resourceFieldRef-only item unchanged fixture. Finding sent to author and root. This review does not qualify a later patch automatically.

No research admission, scientific result, budget reset, publisher authorization or measured-worker claim follows. Original25200seconds and2026-10-30T22:08:40Z are preserved.
