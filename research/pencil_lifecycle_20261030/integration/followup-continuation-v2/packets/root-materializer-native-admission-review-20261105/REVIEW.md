# Concrete materializer admission review

Recommendation: ready for coordinator release after final live and publication receipts. No source/config blocking finding.

**Identity.** All 15 packet files matched published8febc4ff; all10 input hashes match. Native guard4e01f95c and helper2d506 are unchanged from source review7362ee80.

**Command.** Exact argv uses python3 -B with no optimization, 2 CPU/4 GiB/zero swap/512tasks. Native880, unit885+5stop, timeout895+5kill and total900 retained. RemainAfterExit requires owned stop and empty cgroup.

**Owner.** T1716 claimed at epoch1, current attempt ID matches configured fence. New unit not-found/inactive/MainPID0; workspace/control/output absent. Known prior materializer/norm units failed with MainPID0 and empty ControlGroup. Repeat live checks at release.

**Accounting.** 46800 total minus4500 charged leaves42300 before new900 allocation;41400 afterward. Separate1800 validation leaves39600. Prior1/1 remains consumed; newversion1attempt, no automatic retry. Nov6 04UTC unchanged.

**Authorization.** Existing user authorization permits prospective version amendment. No new user permission needed. Coordinator serial release remains required and must record new900 charge before launch.

**Publication.** Frozen publication.json records source/review readbacks but leaves self-admission and this-review/root-release null. Supply exact admission/current-review GitHub readbacks and explicit root release as external durable receipts; do not silently edit frozen files.

**Custody.** Native limit gate precedes children. Actual materialized-inputs manifest remains future output. Require native passed/noerrors, digest match, captured invocation terminal evidence and empty cgroup. No compiler stage released.

This is a prospective recommendation, not a launch or native result. External durable receipt completion avoids circular self-publication and preserves frozen configuration.
