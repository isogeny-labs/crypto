# Amendment 5A: preserve the attempt-4 build cache

This amends only item 1 of [Amendment 5](AMENDMENT-5.md), before
attempt-5 execution. Automatic approval review rejected deletion of the
incomplete attempt-4 `target/` directory. Instead, move that exact
directory from SSD990-2 to
`/private/tmp/n83-v5-colima-output-20261116/attempt-4-target-censored`
on the boot volume, and record its original and destination sizes.
Preserve all attempt-4 raw receipts in the repository. Attempt 5 must
use its fresh, write-once `attempt-5/target` and must not read the moved
cache. All source, image, dependency, resource, and stop gates in
Amendment 5 remain in force.
