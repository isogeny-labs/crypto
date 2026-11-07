# Actual source activation audit

Read-only comparison against Git HEAD `11ad50cd86e60055d481af88869c407d6ef814d1` in `/home/ec2-user/crypto-ml-ordinary-ownership-20261101`.

All 1458 hash-bound assembled inputs were compared to authoritative Git object IDs, including non-src compile inputs. 1416 are byte-identical to HEAD; precisely 42 differ (17 modified, 25 added). No extra non-src/include drift was found. Every HEAD src/Cargo path is represented in the assembled manifest. Source file absence due to sparse checkout was not treated as a deletion or addition: comparison used Git objects.

The 42-file set is 35 ml_dataset paths, 6 native cryptanalysis paths, and Cargo.toml. Cargo.toml only adds `cfg(sat_qualification_child)` to the check-cfg lint allowlist; dependency and lock contents are unchanged. Native SAT changes include the inherited admitted-native accounting/instrumentation and tests (+793/-82), so runtime scope extends to that SAT implementation rather than only new Boolean report files. This is an intended frozen SAT donor, not unrelated HEAD drift. The remaining changes connect original ownership, lift verification, retained Boolean search, capture and report transport. ALL_INPUT_DIFFERENCES.json records each path's last content-changing composition layer; inherited SAT updates must be activated too to reproduce the checked assembly.

`git diff --name-only HEAD -- src Cargo.toml Cargo.lock` returned empty. Each target working path was separately inspected: all 17 modified files are present, none is a symlink; all 25 new paths are absent, so there is no untracked collision. No source mutation, checkout, index edit, fetch, build, cloud job or Git publication occurred in this audit.

Recommendation: the exact 42-file set can be activated from the bound assembly without overwriting detected unrelated changes, provided HEAD and clean-target preconditions still hold at mutation time. This audit does not itself apply files or independently reproduce the parent-reported compiler result. After activation, verify all 1458 inputs against ASSEMBLED_HASHES.json to bind the real source tree to the compiled assembly. Typechecking is not runtime validation, particularly for inherited SAT native changes.

The same 42 diffs are recorded in DIFFERENCES.json and ALL_INPUT_DIFFERENCES.json. MODIFIED.diff includes exact textual diffs for the 17 modified files; added source hashes are in the manifest. WORKING_PATHS.json captures collision checks.
