# Stale N83 PR history repair — 2026-10-27

PRs #2124 and #2138 are titled as retained N83 factor-base studies. Their only remaining merge delta against current main is the same unrelated deletion of the existing 20261016-isogeny-round5 benchmark row. No N83 source or data change remains in either merge result. Preserving that unrelated row integrates both author histories without changing main's content.

Each merge was performed normally, with only benchmarks/README.md restored from main. The complete Git tree after each merge equals main 62c166aaa9e96e077c638b4828ef4c2ebac80e6b, before this audit note and the session-log entry are added. The recorded tree proofs below can be checked with git rev-parse. No scientific protocol, source, result, counter, row or evidence file changes.

[
  {
    "pr": 2124,
    "head": "c502f34973924022985f5876786423cfc98b09ce",
    "base": "62c166aaa9e96e077c638b4828ef4c2ebac80e6b",
    "tree": "3dd26d063f7e7121aafafaa90afddda4ac1937c7",
    "same_tree_after_preserving_unrelated_row": true
  },
  {
    "pr": 2138,
    "head": "09270bccae13cab796c9bdba2783b4934bb48e98",
    "base": "62c166aaa9e96e077c638b4828ef4c2ebac80e6b",
    "tree": "3dd26d063f7e7121aafafaa90afddda4ac1937c7",
    "same_tree_after_preserving_unrelated_row": true
  }
]
