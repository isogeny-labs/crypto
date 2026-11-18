# Integration-review correction

Round C's measured `q=223` path did not encounter an identity walker, so its
recorded operations were the operations that path executed. Integration
review nevertheless rejected the source because its rare identity fallback
restored counters after tentative work, making the implementation capable of
under-accounting another path.

Round C1 fails closed by deleting batched initial-walker construction and all
counter/RNG rollback. The 32 jump points remain batched. Initial walkers are
created only by `new_walker`, which charges each attempted scalar construction
and owns all `Walker` field initialization. The old evidence remains retained
for audit but is superseded for integration and performance accounting.

No checkpointed performance experiment was run before or after this fix.
