# Direct uncached signs as a stronger reference

Draft next experiment, **not executed**. The signed-cache round preserves all
ordered lifts and reduces point-add requests against the frozen binary sign
loop. That loop includes an identity addition and unused final doubling for
plus/minus1. The request reduction must be compared with direct sign handling
before attributing it to useful cache reuse or interpreting cost.

Preregister three arms: frozen binary signs, direct uncached signs (P or -P),
and the setup-validated signed cache. Keep40 setups/400 separate source targets
per arm, all ten policies and four roles:120 setups/1200 paired scalar checks.
Name construction, sign/point validation, reads and target-dependent requests;
include every setup/cold cost. Preserve every ordered lift, root, certificate,
degree, row/rank, failed shift, base log and main-source scalar. Recovery receives
Q only. Use the existing general group/list proof faithfully; independently
replay actual points and counters. Keep false-sign/point, stale-source, missing
failed-query and false-scalar controls. Retain ties/losses; a changed component
profile is not an overall ranking or speedup. Nativecap600s/iteration3600s,
existing local resources only, no paid compute/upload/credential reuse.

The future run-direct-sign.sh does not yet exist. Draft status means implement
and preregister, not stop the investigation. Main-ancestry/Cairn gates remain;
no task lease, funding or publication is claimed. Broader encoding, useful-yield,
geometry and fully calibrated reference obligations remain open.
