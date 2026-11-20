# Shared-store upload staging for the companion report

The opaque `report.pdf.bin` is byte-identical to the inspected companion
cryptanalysis PDF identified by ORIGIN.json. This repository has the maintained
trusted artifact-autofix uploader; cryptanalysis has a store-policy check but
no upload workflow. Both configure s3://setec-astronomy-artifacts/store.

The payload intentionally uses the data policy so artifact-autofix uploads
and downloads the bytes for verification, then commits its manifest pointer.
After that succeeds, the companion report directory can reference the same
hash/size under its original report.pdf name. No policy exemption, credential
export, experiment or object deletion is needed. Keep this staging manifest
as the upload provenance. ORIGIN.json records pre-upload state; the trusted
follow-up commit establishes the upload/readback, not that Boolean.
