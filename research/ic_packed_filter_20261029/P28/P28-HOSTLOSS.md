# P28 counted session: host loss record (RULING-P28.md (b) B4)

Study XP-20261029-packed-filter. Session `ic-dgbf-rho-p28-20261029` (counted). The ruling is `research/ic_packed_filter_20261029/cards/RULING-P28.md`, sha256 `bff4e46df7b522f64c86a76631ae3088af7aa6f41a65e80cd1f9fadae8954275`, section (b). Written by the main session before step 8 of RELAUNCH b=28 UNIT rho-r2.

## Launcher builds (from `PFD/receipts-P28*/build.json`)

| tag | build id | buildStatus | startTime | endTime | timeoutInMinutes |
|:--|:--|:--|:--|:--|--:|
| P28 (wall) | isolab-experiment-launcher:d297f5e5-c42b-47a7-a489-bbc6331f44c6 | FAILED (BUILD TIMED_OUT, 2,704 s) | 2026-10-29T19:13:03Z | 2026-10-29T19:58:26Z (1793303906.736) | 45 |
| P28-rho (counted) | isolab-experiment-launcher:bb16de3e-08bc-4fdd-add3-ca98430f2055 | FAILED (BUILD TIMED_OUT, 2,704 s) | 2026-10-29T19:13:02Z | 2026-10-29T19:58:27Z (1793303907.185) | 45 |

## B1: deadline arithmetic

- E = endTime of the original counted build = 1793303907.185 = 2026-10-29T19:58:27.185Z.
- The earliest dispatch is E + 3,600 s + 900 s = 1793308407.185, which is 2026-10-29T21:13:27.185Z.
- VERIFY (B3) was dispatched after 21:14:00Z, so B1 holds.
- Stated assumption, from the ruling: the Job's 3,600 s deadline counts from Job creation, including pending time. This is unverified for this deployment.

## B2: Kubernetes evidence

None is available. This container cannot reach the EKS API: the agent proxy reset the connection to the cluster endpoint, and kubectl is not installed. The owner has been asked for the Job and pod status, and to delete the original Job if it is still active or pending, but has not answered yet. With no status available, B1 and B3 are sufficient (ruling (b) B2).

## B3: VERIFY b=28 PAIR (card-launch-v2.md, sha256 8fa7610fa63ae4043a55c17459167ee874ebcfde7616e332c42ebe6a048cd526), output verbatim

```
8fa7610fa63ae4043a55c17459167ee874ebcfde7616e332c42ebe6a048cd526  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf/card-launch-v2.md
8fa7610fa63ae4043a55c17459167ee874ebcfde7616e332c42ebe6a048cd526  -
step0_exit=0
PRELUDE_OK b=28 unit=PAIR TAGS=P28 P28-rho PREDTAG=none NEED_SLOTS=2
BUILDS_LISTED 35
==== VERIFY_P28 ic-dgbf-p28-20261029 ====
LOCAL_EXISTS receipts-P28
LOCAL_EXISTS launch-P28.log
LOCAL_EXISTS pid-P28
PID_STATE pid-P28 exited
PAIR_DONE
S3_KEYCOUNT ic-dgbf-p28-20261029 1
REQUEST_DIGEST P28 e5ce01a724b92ec6079546c74c37694e8644b30d4976365261ee4876539feed3
REQUEST_UPLOADED_KEYCOUNT 1
BUILDS_WITH_REQUEST_KEY P28 [{"id":"d297f5e5-c42b-47a7-a489-bbc6331f44c6","buildStatus":"FAILED","startTime":1793301183.025}]
==== VERIFY_P28-rho ic-dgbf-rho-p28-20261029 ====
LOCAL_EXISTS receipts-P28-rho
LOCAL_EXISTS launch-P28-rho.log
LOCAL_EXISTS pid-P28-rho
PID_STATE pid-P28-rho exited
PAIR_DONE
S3_KEYCOUNT ic-dgbf-rho-p28-20261029 0
REQUEST_DIGEST P28-rho 09877924909b4c5e6eb74e6030234e83352e868eac1253ee5b6d43213482428f
REQUEST_UPLOADED_KEYCOUNT 1
BUILDS_WITH_REQUEST_KEY P28-rho [{"id":"bb16de3e-08bc-4fdd-add3-ca98430f2055","buildStatus":"FAILED","startTime":1793301182.935}]
VERIFY_OK
```

The counted prefix `runs/ic-dgbf-rho-p28-20261029/` holds 0 objects, the `P28-rho` launcher pid has exited, and no build with the `P28-rho` request key is `IN_PROGRESS`.

## B5: session of record (fixed before any counted data)

The counted session of record is decided at the moment the counted analysis is dispatched, from file existence alone:
- if `runs/ic-dgbf-rho-p28-20261029/` then holds a `job-exit.json`, the original is the session of record and `-r2` is descriptive only;
- otherwise `ic-dgbf-rho-p28-20261029-r2` is the session of record.

## B6

If `-r2` also loses its host, the case goes back to the Coordinator. The card has no rho-r3 unit.
