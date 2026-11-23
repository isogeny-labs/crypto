## 2026-11-07 — Combined monomial transport (T-1735/T-1738)

### What was done (commands, files, features)

Multiplied the fixed-index transport laws into one homogeneous pair for the character monomial. Implemented the combined coordinate-difference factor, parity-aware -P anchor, direct initializations and unchanged residual consumption. Three bounded native tests pass, checking499 complete-subgroup and176 shifted-window ratios/signs against the frozen source;60 imported tests filtered. Seventeen dependency hashes and binary digest retained. [pcc:unverified]

### Decisions made (and why)

Retain all geometry lanes and the existing conditional seed-law domain. Do not infer a cost improvement from reducing pair storage: coordinate-difference powers now occur during transport. Claim PCC-20261107-0c701d remains unsigned. Sources checked (KB): MCP and wrapper unavailable; read predecessor derivations/source and DLMF23.10.3. No timing/cloud panel; original4563-case cohort unchanged.

### Open threads (unfinished, pick up next time)

Integrate an adaptive cursor retaining the monomial pair and geometry tail, including skipped residuals. Then instrument against the frozen four-column path with setup, residuals and allocations included. PR3597 is open; extend that PR. The shared daily journal is reserved by T-1736 in another checkout; preserve this component journal and append to the daily journal when its reservation becomes free.

### Errors (what went wrong, how it was handled)

No native build or test failure. Compiler warnings retained in build.log. An initial lookup assumed a source.rs filename; the component uses lib.rs. The daily-journal reservation conflicts, so no edits were made there. KB wrapper unavailable.
