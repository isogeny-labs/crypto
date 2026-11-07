# Finite existing-host assembly allocation

## What was done
Read-only AWS observations bound existing Spot request price caps, gp3 volumes, snapshots, NAT and EKS footprint. Independently estimated a USD 1 attempt/storage allocation and USD 55 shared standing/delay reserve through 06:00 UTC. Recorded these reserves in Conductor T-1442 before admission.

## Decisions made
No new infrastructure or additional paid launch is included. Use conservative request price caps and full snapshot source-volume capacity; variable and omitted costs remain an explicit estimate margin. The private runtime remains one attempt, 600 seconds, 2 CPU, 2 GiB memory and 4 GiB disk. Exact inputs and launch command must be published before launch.

## Open threads
Bind final independent review and publish exact command/inputs, then execute only the admitted assembly. Qualification and matched scientific cohort require their separate bindings and gates.

## Errors
No runtime execution has occurred in this packet. Earlier failed checks remain in their immutable packets.

## Final admission binding

Independent reviewer approved one assembly and pure validation passed against unchanged canonical inputs. The first report-source generator failed at Python parsing because its outer string interpreted a TeX escape; no file or runtime was produced. Replaced the wrapper with a literal source heredoc and rendered successfully. The source-generation failure is separate from runtime execution.
