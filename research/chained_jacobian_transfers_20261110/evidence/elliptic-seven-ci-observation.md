# PR1483 CI observation during workspace interruption

Operational snapshot read through the GitHub connector after local execution
became unavailable. This is an extracted observation, not a full raw log,
new frozen PCC record, signed receipt or scientific standing upgrade.
Existing consumer mathematical claims remain unsigned. [pcc:unverified]

- Repository:isogeny-labs/cryptanalysis,head ef9a542598c1b0ebfde03cf27f77929b928391cd.
- Run43038507853,workflow Chained transfer differential reference,manual event.
- Completed success14:33:50Z,reference job130583694992,all named steps success.
- Full log81,588 characters;artifact upload succeeded. Full native log/
  artifact still needs S3 archival when admission/credentials return.
- https://github.com/isogeny-labs/cryptanalysis/actions/runs/43038507853
- https://github.com/isogeny-labs/cryptanalysis/actions/runs/43038507853/job/130583694992
- Log endpoint:https://api.github.com/repos/isogeny-labs/cryptanalysis/actions/jobs/130583694992/logs
[pcc:unverified]

## Exact extracted test outcome lines

```text
2026-11-11T14:27:01.7008424Z test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.77s
2026-11-11T14:27:01.7287881Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
2026-11-11T14:33:45.4722860Z test result: ok. 51 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 255.09s
2026-11-11T14:33:45.4732209Z      Running tests/frobenius_independent.rs (cryptanalysis/experiments/chained-transfer-reference/target/release/deps/frobenius_independent-331236ddb264dfb6)
2026-11-11T14:33:45.4749155Z test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
2026-11-11T14:33:45.4751271Z      Running tests/seven_independent.rs (cryptanalysis/experiments/chained-transfer-reference/target/release/deps/seven_independent-155dc38f999b512a)
2026-11-11T14:33:45.4899577Z test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

The two zero-test lines,if present,are separate binary/doc entrypoints and
are not added to the25/51+two integration test counts. The ignored test is
the pre-existing benchmark. This is correctness CI,not a timing comparison.
[pcc:unverified]

## Required and broader checks

Active ruleset25796910 requires store,gate,records; all three were successful
on ef9. Merge method merge;required approving review count0;thread resolution
required;no bypass actors,current_user_can_bypass=never. No submitted review
or inline review comment existed at the snapshot. PR mergeable=true,state
unstable. Analyze(rust) was in progress;no attempt to merge while that
applicable check remained pending. Branch-protection endpoint403 is an
integration permission limit;ruleset read succeeded. [pcc:unverified]

Five failed general CI jobs were individually inspected. All stopped during
apt package installation with temporary DNS resolution failures and exit100,
before the named analysis/test could run. The PR has no C++ source or package
installation-step changes. These are source-independent infrastructure
failures,not passed checks; do not patch unrelated code or waive requirements.
The following excerpts preserve the last package error and exit line only.
Full public logs have finite retention; archive them when S3 is available.
[pcc:unverified]

### format_log — job130583690271

https://github.com/isogeny-labs/cryptanalysis/actions/runs/43038508909/job/130583690271

Original decoded log length:207244 characters. Extract:

```text
2026-11-11T14:31:43.8055762Z E: Failed to fetch https://security.ubuntu.com/ubuntu/pool/universe/l/llvm-defaults/clang-format_21.1.6-71_amd64v3.deb  Temporary failure resolving 'security.ubuntu.com'
2026-11-11T14:31:43.8090498Z ##[error]Process completed with exit code 100.
```

[pcc:unverified]

### coverage_log — job130583690367

https://github.com/isogeny-labs/cryptanalysis/actions/runs/43038508909/job/130583690367

Original decoded log length:31302 characters. Extract:

```text
2026-11-11T14:33:01.6055586Z E: Failed to fetch https://security.ubuntu.com/ubuntu/pool/universe/p/python-colorlog/python3-colorlog_6.10.1-1_all.deb  Temporary failure resolving 'security.ubuntu.com'
2026-11-11T14:33:01.6058100Z E: Failed to fetch mirror+file:/etc/apt/apt-mirrors.txt/pool/main/l/lxml/python3-lxml_6.0.2-1ubuntu0.1_amd64v3.deb  Temporary failure resolving 'security.ubuntu.com'
2026-11-11T14:33:01.6059900Z E: Failed to fetch https://security.ubuntu.com/ubuntu/pool/universe/g/gcovr/gcovr_7.2%2breally-2_all.deb  Temporary failure resolving 'security.ubuntu.com'
2026-11-11T14:33:01.6100219Z ##[error]Process completed with exit code 100.
```

[pcc:unverified]

### cppcheck_log — job130583690324

https://github.com/isogeny-labs/cryptanalysis/actions/runs/43038508909/job/130583690324

Original decoded log length:28655 characters. Extract:

```text
2026-11-11T14:32:20.0966814Z E: Failed to fetch https://security.ubuntu.com/ubuntu/pool/universe/t/tinyxml2/libtinyxml2-11_11.0.0%2bdfsg-1build1_amd64v3.deb  Temporary failure resolving 'security.ubuntu.com'
2026-11-11T14:32:20.0969248Z E: Failed to fetch https://security.ubuntu.com/ubuntu/pool/universe/c/cppcheck/cppcheck_2.19.0-3_amd64v3.deb  Temporary failure resolving 'security.ubuntu.com'
2026-11-11T14:32:20.1009523Z ##[error]Process completed with exit code 100.
```

[pcc:unverified]

### clangtidy_log — job130583690215

https://github.com/isogeny-labs/cryptanalysis/actions/runs/43038508909/job/130583690215

Original decoded log length:27041 characters. Extract:

```text
2026-11-11T14:31:39.4392539Z E: Failed to fetch https://security.ubuntu.com/ubuntu/pool/universe/l/llvm-defaults/clang-tidy_21.1.6-71_amd64v3.deb  Temporary failure resolving 'security.ubuntu.com'
2026-11-11T14:31:39.4427566Z ##[error]Process completed with exit code 100.
```

[pcc:unverified]

### valgrind_log — job130583688587

https://github.com/isogeny-labs/cryptanalysis/actions/runs/43038508774/job/130583688587

Original decoded log length:27216 characters. Extract:

```text
2026-11-11T14:32:16.6341609Z E: Failed to fetch mirror+file:/etc/apt/apt-mirrors.txt/pool/main/g/glibc/libc6-dbg_2.43-2ubuntu2.4_amd64v3.deb  Temporary failure resolving 'security.ubuntu.com'
2026-11-11T14:32:16.6343071Z E: Failed to fetch https://security.ubuntu.com/ubuntu/pool/main/v/valgrind/valgrind_3.26.0-0ubuntu1_amd64v3.deb  Temporary failure resolving 'security.ubuntu.com'
2026-11-11T14:32:16.6383893Z ##[error]Process completed with exit code 100.
```

[pcc:unverified]
