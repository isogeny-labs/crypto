# Independent source review — fixture-view renewal

The operational renewal preserves the reviewed fixture materializer and cached-suite implementation. Both launchers and both workers are byte-identical to the predecessor. The only executable source change is the paired `durable_io.py` admission check: the campaign ceiling becomes 129,600 seconds, the UTC deadline becomes `2026-11-11T12:00:00Z`, and charged plus pre-attempt remaining must equal the ceiling.

All 24 producer-manifest members match their recorded hashes and sizes. The preserved inputs contain 6,179 unique expected test names, 135 additive input rows totaling 41,117,354 bytes, and the original 18 failure-triage rows. The materialization and suite caps remain 120 and 1,800 seconds, respectively, with one attempt each and no automatic retry.

No source blocker was found for publication and final binding. Both draft admissions remain false, and their campaign counts, lease, serial predecessor, independent review, publication, and materialization receipts remain unresolved. A final coordinator binding must attach the exact live ledger; the native helper checks arithmetic but does not read the ledger itself. The suite may be admitted only after successful materialization receipts bind the exact view.

No materialization, compiler, test, service, or suite execution was performed during this review.
