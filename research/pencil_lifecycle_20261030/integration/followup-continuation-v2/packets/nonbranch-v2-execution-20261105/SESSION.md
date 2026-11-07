# Attempt 2 archive

## What was done
Native full pre-custody passed; the supervisor stopped on systemd garbage collection. Preserved full native/outer archive, exact journal identity, live limits, publication and source bindings, PDF, SVG, CSV and JSON. [pcc:unverified]

## Decisions made
Keep original supervisor failure and old-clock cutoff. Charge900seconds in addition to prior300. Record no compiler/post invocation and no launched same-clock recovery. [pcc:unverified]

## Open threads
A versioned full fresh attempt is under source review, limited to2400seconds and attempt3of3 within the existing campaign. No result from that attempt is represented here. [pcc:unverified]

## Errors
Short coordination heartbeat expired while the original pre kept running; reclaimed the same task with unchanged unit invocation/PID, no payload restart, then used bounded30-second heartbeat supervision. Original pre success and outer failure remain separate. [pcc:unverified]

Sources checked (KB): new main instructions inspected. KB MCP tools are not loaded and the wrapper is absent in this sparse checkout. Used the exact published norm replay, independent source-review packets and native systemd evidence; no new mathematical mechanism or novelty claim is made.
