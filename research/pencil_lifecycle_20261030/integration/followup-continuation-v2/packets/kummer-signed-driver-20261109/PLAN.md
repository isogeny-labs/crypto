# Opt-in literal Kummer driver callbacks: source plan

Preserve the exact existing run loop and advance/lift functions. Wrap run through a fallible internal function; disabled observation performs no JSON construction and keeps all existing arithmetic, RNG, counters, branch ordering and output rows. A new run_observed entry point supplies callbacks. Each target emits its own header/terminal; callers must select a fresh NativeEvents/Trace per target.

Record begin/result pairs around the two actual advance calls and the actual signed lift call. Record all gate/restart/duplicate/graph outcomes with original signed divisor and original labels. Preserve native fallback validation and final scalar checks. Do not insert Labels::ingest into the existing collector: that would change its algorithm. Extend the existing NativeEvents protocol rather than replace Trace limits, first-failure poisoning or output ordering. Sink failure propagates immediately and leaves an incomplete prefix; no synthetic successful terminal.

Source-only scope. Runtime qualification must compare disabled and observed actual producer output/counters on frozen fixtures, use actual exceptional branch fixtures, and inject callback errors at call boundaries. Claims concern diagnostic observation only; serialization cost is not field arithmetic and observed timing cannot be presented as baseline timing. Full1260target scope remains a later separately released experiment.

Prior art: exact signed-event-adapter and signed-producer-qualification packets plus pruned.rs native advance/lift/control sources. KB tools unavailable and scripts/research-kb absent; exact source fallback only, no novelty claim.
