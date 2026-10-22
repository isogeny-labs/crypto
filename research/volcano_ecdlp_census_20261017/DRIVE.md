# Drive storage

Folder: [`volcano-ecdlp-census-20261017`](https://drive.google.com/drive/folders/11MPowjLRt25fg1RypJnbDDvWrNc_7PZ3) (id `11MPowjLRt25fg1RypJnbDDvWrNc_7PZ3`).

| phase | file | Drive id | bytes | SHA-256 | read back |
|:--|:--|:--|--:|:--|:--|
| A | FROZEN.json | 1_Y8auw7w4SsxeZ_Hpen7mcpyPw-RohDw | 3095 | 345028509a93be46f3b0867d33af90350a9dde001d90bbc51dd1771a5a98ed73 | size read back from the upload result = 3095 (matches); content not re-downloaded |
| A | phaseA-calibration-summary.md (= calibration/summary.md) | 1flai8c2P756HeyJFo33-dKShIHHlAfV2 | 2854 | 52ec924c415d454662a3713a4737ffffbc944a8616c8a402d89c05b3d913a6f3 | size matches |
| B | progress-20261017T072330Z.jsonl | 1dPDJt6aDfGMbWbfqNv9o8LoVXhCFc50k | 1961 | 4c44d01631ad83cbdd17f9fc04de02bbef653bd4d0eaf65f7624fdb8b5aa7f35 | upload result fileSize = 1961 (equals local bytes); content not re-downloaded |
| B | progress-20261017T074020Z.jsonl | 13Gb2mxn5-0b8Dc3MaSgm545ye5cj9yxk | 3592 | 7b02693a47fa04d83e526ec6db2623876b99754239a659f9421c61d8a749de1a | upload result fileSize = 3592 (equals local bytes); content not re-downloaded |
| B | progress-summary-20261022T055755Z.tsv | 15kEx19i0jNcF9BXX_SdFMHwRbV-Dt3jC | 4208 | f528821db140645954d199511a1bc62ae43e9f44aa25ddbb8337c0fbd971647e | upload result fileSize = 4208 (equals local bytes); content not re-downloaded |

Staged but NOT uploaded (the only upload path in the executor is a base64 argument typed through the model context: about 120k tokens for the 94 KB specs archive and about 600k tokens for the 0.97 MB census archive, with no way to hash-check the copy beforehand). Their contents are committed in git on this branch. SHA-256 of the staged archives (built with `tar | xz -9e`, PNGs excluded from the census one; kept in the executor scratchpad only):

| archive | bytes | SHA-256 |
|:--|--:|:--|
| census-phaseA.tar.xz (census/, checks/, FROZEN.json; no PNG) | 971516 | 6d9be453cad9c63fc3cabee6e28257367486d7cbd569c26296b16001d98a9708 |
| specs-phaseA.tar.xz (specs/, runs/) | 93708 | 23ab2d128912d092063477f4a8398a11b7f7032f96e107b5199281f247c04a0d |

Phase B final note: the full `runs/progress.jsonl` is 41 KB (sha256 623e920f5f163a5c442555e5db8008f494aab390a7a39fe906e8ce492dc03407 at upload time) and lives in git; Drive holds the intermediate progress snapshots above and the per-session summary `progress-summary-20261022T055755Z.tsv` (derived with jq from the `status=done` lines, `runs/progress_summary.tsv` in git), not the full jsonl, to avoid retyping 41 KB through the model context. Archives stay in git only.
