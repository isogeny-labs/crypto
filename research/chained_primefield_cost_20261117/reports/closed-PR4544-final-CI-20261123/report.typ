#set page(paper: "a4", margin: 16mm)
#set text(font: "DejaVu Sans", size: 10pt)
#set par(leading: .6em)
= Closed PR4544: final CI custody
#text(size: 9pt)[23 November 2026 · immutable old heads only · producer assertions unsigned]
#text("All-event/all-page metadata for old PR4544 records 30 checks on each head. ProtocolC has 21 workflows, 13 successful checks, 7 failed and 10 skipped. Final has 19 workflows, 11 successful checks, 9 failed and 10 skipped. Each observed head is within the 30-check limit. These counts do not erase the separately retained historical 56-check excess on an earlier head. [pcc:unverified]")

#image("checks.svg", width: 100%)

#text("The final validation gate and artifact gate failed; size passed. The validation log records the committed-tree positive fixture rejection. The artifact job passes its changed-file data check, then reports 35 existing workflows with stale or missing pull steps. Other failed steps include shared-code lock, data hydration, S3 credential setup, format/admission, recompute and lessons evidence. The records make no new baseline-rerun or defect-attribution claim. [pcc:unverified]")

#text("Thirteen selected complete decoded job logs were retrieved through the existing connector: protocol gate/artifacts/size, every final failed job and final size. The invalid curve-covers workflow has no jobs. Full decoded-text hashes and complete provider URLs are recorded; the local cache is temporary and a durable local mirror is not asserted. Raw API byte hashes are separate from decoded-log UTF8 hashes. No raw log was truncated, split or renamed into Git. [pcc:unverified]")

#text("PR4544 merged at 2026-11-17T15:16:40Z as 173a86cb. The final jobs complete later; merge-time CI success is not established by this observation. Corrected episode E-8a1dcd84022b supersedes the initial guessed start with the actual PR-created 09:07:31Z. The old record remains. Revert check: 2026-12-01T15:16:40Z. Exact human-message event timestamps and durable raw-log mirroring remain open. [pcc:unverified]")

#text("This is closed-PR historical custody only. No old branch push, new workflow dispatch, scientific replay or currentPR4990 CI query occurred. Current repository gates still apply. Claim PCC-20261123-29c03d is unsigned. [pcc:unverified]")

#link("https://github.com/isogeny-labs/crypto/pull/4544")[Closed PR4544, exact final head and discussion]
