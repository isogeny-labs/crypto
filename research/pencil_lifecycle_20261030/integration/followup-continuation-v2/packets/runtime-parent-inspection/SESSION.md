## 2026-11-02 exact base inspection

### What was done
Read latest scoped session and previous qualification metadata. T-1382 owns this directory. Inspected exact base configuration and saved its layers once, without creating or starting a container. Verified config and 10 layer hashes; examined 19,600 headers.

### Decisions made
Preserve unknown save exit; recover bytes by exact digest verification rather than repeat the save. Retain root metadata as unknown. No assembly/qualification/scientific admission inferred.

### Open threads
Mounted-root metadata, bounded assembly and qualification admission, independent review and full cohort.

### Errors
Original wrapper used unavailable hashlib.file_digest, exited1 before recording save exit. Retained original source and failure in save-receipt.json. Corrected streaming hash and parsed the existing archive successfully.

### Publication checks
Rendered one-page report and corrected a long-hash line overflow; inspected final PDF. Both render logs preserved byte-for-byte in base64. Metadata artifact-policy check selected no uploads; raw image archive remains explicitly local.
