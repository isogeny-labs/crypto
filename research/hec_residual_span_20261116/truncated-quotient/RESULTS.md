# Truncated residual quotients across genera 2,3,4

[pcc:unverified] Freeze3fe6d38b1b9 pushed before one invocation; input hashes pass; exit0. All1680 sampled residual quotients match two exact polynomial divisions and the full principal polynomial identity. Oracle results match brute rational root division with multiplicities. All split relations pass native original-Jacobian group checks. Both measured repetitions have identical16vectors/REDC accounting. This is a component/algebra gate, not full solvers or an end-to-end speed claim.

| genus | field | degree-g anchors | cases | split | nonsplit | repeated residual root occurrences | residual overlap with known roots | excluded u-zero places |
|---|---|---|---|---|---|---|---|---|
|2|101|31|233|233|0|0|17|15|
|3|113|29|775|75|700|2|4|30|
|4|109|28|672|28|644|3|6|29|

Repeated residual roots count occurrences in completely split quotients; overlap counts roots equal to either distinct known support root. A linear residual cannot have internal multiplicity>1; genus2 overlap covers repeated roots of the full h. Samples are deterministic first32 multiples/first pair per bucket, not exhaustive anchors or independent random samples. Degree-mismatched anchors were skipped (1,3,4 respectively). Genus4 is explicitly an algebra fixture with no qualified original target/order assumption.

Extraction plus identical split oracle base-field REDCs:

|genus|old|recurrence|old/new|percent fewer|
|---|---|---|---|---|
|2|13472|1864|7.2275|86.16|
|3|91592|40808|2.2445|55.45|
|4|246564|190880|1.2917|22.58|

SUMMARY.tsv supplies exact derived ratios. Both arms include their declared extraction and the same oracle; common anchor/bucket setup and diagnostic brute/group checks are excluded equally. The generic g2 recurrence differs from the previous hand-specialized full producer formula; this table cannot replace that complete-cost result. No mixed-modulus weighting, wall-time, external-fastest or full10x claim. Cubic residual splitting remains charged in genus4 and limits the component reduction. High nonsplit rates are retained observations, not discarded failures or claims of a new large-prime algorithm.

Decision: truncated coefficient extraction is executable-qualified on these three fixtures including multiplicities/nonsplits. A future collector can reuse it with the stated distinct-known-root/nonzero-denominator conditions and qualified model assumptions. Next must target collection/setup/certificate construction and, for g4, remaining root testing; reprice a complete original-target pipeline before claiming acceleration. Existing residual/large-prime experiments must be screened before proposing a nonsplit reuse mechanism. Original genus2+10x objective remains unresolved. Shared-native checks and algebra derivation need independent review. Git custody, no retry/S3/cloud.
