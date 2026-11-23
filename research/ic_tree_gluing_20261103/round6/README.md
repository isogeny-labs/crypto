# Round 6: local long run of the deciding cell m5 t4 (memory-bound, undecided)

2026-11-04 · pre-registered in `.resume/RESUME.md` (commit `795055f322`)
before the run · input `../round3/` system `m5_t4_k0_T.ms`, sha256
`0ae81d1c…3368488`, matching `../round3/SYSTEMS.sha256` · log [`logs/`](logs/).

**Class: stage diagnostic.** Every result below is an unverified assertion
[pcc:unverified].

| run | cap | outcome |
|---|---|---|
| msolve 0.6.5 `-g 2 -t 1 -v 2`, alone on the host | 18,000 s, 14.5 GiB | exit 139, memory exhausted, at 4,046 s, inside the degree-9 round on a 572,048 × 999,374 matrix (density 0.18 %) [pcc:unverified] |

Reading against the registered outcomes:

- The run neither completed at D = 9 nor selected degree 10, so it is cut off
  again. **P2 at m = 5 stays undecided** [pcc:unverified].
- Round 3's 13.5 GiB run timed out at 2,400 s in the same round. This run got
  1,646 s further and then ran out of memory, so on this 15 GB host the cell
  is memory-bound, not time-bound [pcc:unverified].
- Deciding it needs either a host with at least 64 GiB (`../round4/BIGHOST.md`)
  or a memory-lean F4 (`../round7_memory/`, in progress).
