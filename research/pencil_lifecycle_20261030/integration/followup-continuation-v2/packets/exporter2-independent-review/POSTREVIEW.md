# Exporter2 failed compile postreview

The one released attempt failed compilation with E0624: Spec::identity_view is private. Elapsed52.914083899seconds; no tests executed; compiler exit1, reaped, process group empty; native pre/post inputs unchanged; no generated output. Exact receipt/log hashes are in failed-attempt-postreview.json.

The reviewer missed this third private fixture API after identifying two others. That was an incomplete visibility audit. The prospective acceptance remains preserved as a historical review record and is not build-success evidence. No retry is authorized by this addendum. The remaining imported calls have now been checked for public visibility against the exact library source.

A minimal prospective fixture correction constructs the spec identity JSON from public schema, workloads, arms and measurement fields, then uses the existing public canonical short_id. This avoids invoking spec::plan and preserves real validation and all test assertions. A separately frozen successor and root release remain required.
