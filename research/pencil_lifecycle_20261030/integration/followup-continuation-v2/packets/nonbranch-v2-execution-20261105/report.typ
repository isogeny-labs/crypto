#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Nonbranch replay: complete pre-custody, supervisor stop
2026-11-05 · attempt 2 · evidence status: [pcc:unverified]

The native pre-custody phase checked all 16,736 files and 3,111,892,318 bytes. It returned passed after 310.744 seconds. The outer supervisor ended after 311.113 seconds and recorded failure because systemd cleared the invocation ID after successful termination. No Lean compiler call or post phase started.

#image("phases.svg", width: 100%)

== Terminal evidence
The live unit initially had invocation 5821bdab5e2d48a0921c3f155b21cabf and PID 882274. The systemd manager journal retains matching start and successful deactivation events, with the same invocation, unit and boot. Later readback found the unit collected and the original PID and cgroup absent. These observations support termination reconciliation; they do not change the preserved failed supervisor receipt.

== Accounting and continuation
Charge this attempt conservatively as 900 worker-seconds. With the preceding 300-second charge, campaign charges total 1,200 seconds, leaving 45,600 of 46,800 seconds before later attempts. The absolute deadline remains November 6, 2026 at 04:00 UTC. A proposed compiler/post continuation did not launch because publication and prospective review did not finish before the original clock could reserve both phases. The old clock was not reset.

== Reproduction and limitations
result.json and phases.csv expose measured values separately from conservative charges. native-and-outer.tar.gz contains all native and supervisor outputs from this attempt; archive-inventory.json pins every member. The source, freeze, publication, live-limit and journal receipts are retained alongside this report. Pre-hash success establishes this recorded custody pass. Named theorem checks, compiler controls and full post comparison are absent from this attempt. A separate full fresh attempt is proposed; no result from it is represented here.
