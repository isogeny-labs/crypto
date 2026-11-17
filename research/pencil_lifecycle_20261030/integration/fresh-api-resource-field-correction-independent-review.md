# Additive review: preserve non-fieldRef downward API entries

Read-only review of patch e7e9c5ffa18665b23200e2042e9639c8ba5f77bcb189827c16b6922ebb82bb52, exact cloud base33748bb7cc58a4c3ce7258127f785fdd5855d3a53df9439d09eaa7d1497c3113 to proposed5b235809b1b1069f74e0a4a1c99da6a0e732ab23bd43191256fa12712c229a0e. Rehashed patch and proposed source.

The change replaces mutable indexing with get_mut before accessing fieldRef. It no longer inserts fieldRef:null for a resourceFieldRef-only downward API item. The added fixture calls the actual helper with both resource env and volume entries, checks exact whole-manifest equality and explicitly checks fieldRef remains absent. Existing fresh_owner opt-in gating and documented v1 default scope are unchanged. This closes the sole finding in fresh-monotonic-and-api-v1-independent-review.md for this additive source delta. No remaining concrete defect found in this reviewed correction. Original findings/patches remain preserved.

No builds, root edits, cloud calls or watchdog execution in this review. Author targeted test was still running when reviewed; this is source review rather than an independent test result or runtime admission. No scientific or allocated-worker-cost conclusion follows.
