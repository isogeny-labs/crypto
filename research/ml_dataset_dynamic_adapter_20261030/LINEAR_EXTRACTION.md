# Linear extraction revision 3

Complete replacement patch against base342eef26d2f1c5af3a8f20033102a20bf063d379, not incremental over revisions1/2.
Path: `/home/ec2-user/ml-dataset-resume-20261029/dynamic-linear-extraction-v3.patch`
SHA256: `22b511981de3a06bbaa253ee0292ed15c0f0c05ac50ff083db3586480f891c3c`.

Retains revision2's external NonlinearOrUnderdetermined mapping and length preflight, but corrects reviewer-noted polling/performed accounting: one nonrefundable parent reservation32+16*basis_count supplies a local ticket; spend32 initially then spend16 per visited entry, checking the same phase deadline before each length read. An early oversized polynomial preserves the full consumed reservation but records only visited performed work. No duplicate parent debit or fresh allowance. Exact/one-less preflight tests remain authored.

Original and revision2 artifacts preserved. rustfmt parsing and git apply --check passed. Worktree remains untouched, no Cargo/test/panel/cloud execution. All scope and claim limits in original/v2 handoffs remain; eight authored test functions are unexecuted and real-native-positive extraction remains a separate gate.
