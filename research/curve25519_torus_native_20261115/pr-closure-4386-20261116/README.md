# PR4386 closure: publication coverage only

PR4386 merged at2026-11-16T03:07:37Z into the feature parent,
codex/curve25519-four-point-20261115, not main. Merge21f5d99e6aecf4189dd693522b3568a8bb8d52b9
has parents4c59c865bcf8e3a49f9650e951c7549c33a656d4 and6c123eb24953a6791ec066dc84d23fb0c19be634.
Its entire tree is byte-identical to the submitted head, including all
published journal/ledger/claim histories. This continuation issued no PR merge.

The PCC gate began03:07:39Z and failed03:12:08Z. Its full581381-byte CLI log
is retained losslessly as32947-byte gzip with hashes. The only failing
claim is PCC-20261115-806be3: missing historical root-library-compatibility/
incomplete-run.log. Claim and artifact manifest match the parent Git blobs.
No inherited proof, policy, exemption or artifact receipt was changed.
No all-green-at-merge or authorization override is inferred. The sign check
has inconsistent start/completion timestamps; raw fields are retained and
no duration is calculated from them. [pcc:unverified]

Episode E-5049c29c96d5 and matching outcome O-8eed22a3601e cover only PR
creation02:51:40Z through merge03:07:37Z. Result is partial. Original
prompt/message timestamps, intervention counts, incident recovery, costs
and full-session duration are unavailable; empty incident metadata is not
an assertion of zero incidents or human messages. The episode's false
CI-green flag records that an all-green merge is not established, rather
than inferring the exact queued state of every check at merge. Revert review
is due after2026-11-30T03:07:37Z, not now. [pcc:unverified]

The canonical raw Git merge commit is merge-commit.txt. merge-source.json is
an explicitly labelled projection of GitHub's commit API, omitting its large
per-file patches. The full485809-byte HTTP response remains unchanged at
the local path recorded there, with its SHA-256 and immutable API URL.
All necessary source objects are retained by the pinned Git tree. Raw PR,
check and annotation JSON plus complete failed-job logs remain here.
The direct log endpoint was denied; the maintained gh run view path supplied
the complete log. No signed access URL is committed.

The maintained lane helper found the remote lane deleted after merge and
stopped without editing files on its attempted new-main merge. Confirmed
that the old PR was terminal and no open lane PR existed, then merged the
actual updated feature parent into the local next-component commit. That
merge changes ancestry only: the entire b81e81fdd54fb52a402b27c7f7ca9d7ad86cffcf
tree is retained. A new regular PR carries the next component on the same
lane. It still requires parent integration, fresh gates and review.
