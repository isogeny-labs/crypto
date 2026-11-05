# SAT search core: compiled, strict-decoding failure retained

Release library compilation passed on the isolated 1440-input snapshot; source hashes remained unchanged. The SAT-filtered native run selected322tests:317passed,1failed,4ignored. Twenty-two of23required controls passed. The failing strict_reason_and_claim_progress_reject_hidden_fields control demonstrated acceptance of an extra field on the internally tagged Decision unit variant despite deny_unknown_fields.

The reviewed correction changes the private decoder's Decision and XorMatrixReset variants to empty struct variants, preserving their kind-only wire shapes and native conversions. The original failing wire test remains; added controls cover valid input and unknown-field rejection for both variants. This is a parser correction, not a relaxed assertion.

The failed binary224556fb6c2159fac5f862362a773bcb5b64b467d273d94cb0348cc78596c41c, all1440tested inputs, logs and receipts are preserved in the S3 archive. Fresh download/byte comparison and all contained hashes passed. Download the URI in s3.receipt.json, verify bytes and SHA-256, extract into a fresh directory and run sha256sum -c SHA256SUMS.

A subsequent isolated1442-input build combines the correction with the independently reviewed CLI route and V11native boundary changes. It was running at publication. Native activation remains disabled; no dedicated qualification or original-owned positive search has run. The earlier original-owner full-library gate also remains running on its separate untouched checkout. No source push is justified by this failed component run.
