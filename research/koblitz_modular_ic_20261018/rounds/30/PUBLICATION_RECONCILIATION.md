# Scratch-log durability correction

The checkpoint helper saves RESUME.md, Git changes/commits and eligible
untracked files, but explicitly excludes the rest of ignored .resume/.
Earlier R29 recorder notes overstated that its .resume software-validation
logs were included in the checkpoint. Their local files and published SHA256
values survived, but those checkpoints alone did not preserve them.

R30's raw archive now additionally retains all four exact R29 recorder logs
and both isolated test source crates under publication-attempts/r29-recorder.
Their bytes are unchanged; the earlier RECORDER_VALIDATION.md hashes match.
The R30 failed initial renderer source/log is separately retained under
publication-attempts/r30-render-initial. files.json pins every copied file.

This supersedes the earlier checkpoint-path claim. It does not change any
old canonical archive, scientific input, theorem or recorded test result.
Fresh raw-bundle GetObject/member/repack verification covers these files too.
