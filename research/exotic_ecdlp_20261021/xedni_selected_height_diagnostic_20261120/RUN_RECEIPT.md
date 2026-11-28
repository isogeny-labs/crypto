# Reproduction receipt

All commands below ran from `/Volumes/SSD990-2/crypto-worktrees/exotic-ecdlp-recovery-20261119` and exited zero. The local Sage jobs used `/Volumes/SSD990/cryptanalysis/sage`, whose saved `--runtime-info` status is `verified` (Sage `10.10.rc0`, SHA-256 `073ca37250b100a7f961de3475da8a4d489f3f2140c15f4c72457816893f3f01`). Rank subprocesses were started by the Python drivers with the same absolute launcher. Individual rank arms retain their own completed, error, or timeout status.

| Output | Command | SHA-256 |
| --- | --- | --- |
| `fixtures.json` | `python3 research/exotic_ecdlp_20261021/xedni_selected_height_diagnostic_20261120/make_fixtures.py` | `7df5e375792c2c9cd0368ffd878e47ef268078bc14b42b6d1384fddf9cf3f0b7` |
| `height-result.json` | `/Volumes/SSD990/cryptanalysis/sage -python /Volumes/SSD990-2/crypto-worktrees/exotic-ecdlp-recovery-20261119/research/exotic_ecdlp_20261021/xedni_selected_height_diagnostic_20261120/height_sage.py` | `7705af7aaaadb0f33f04f654e438703dc5920b34e314f9adfc3f819416ce20de` |
| `rank-result.json` | `python3 research/exotic_ecdlp_20261021/xedni_selected_height_diagnostic_20261120/run_rank.py` | `c9433e28e78bcb72ec7479b18e4ae37db3433c4b2f50ec2d0989bc6a0e34a3b8` |
| `rank-proof-result.json` | `python3 research/exotic_ecdlp_20261021/xedni_selected_height_diagnostic_20261120/run_rank_proof.py` | `9fa02ac56dc9a246ea4ce8e7558bbabc3c48ebfa94663814758eb4e90ffa4391` |
| `c2-effort-result.json` | `python3 research/exotic_ecdlp_20261021/xedni_selected_height_diagnostic_20261120/run_c2_pari_effort.py` | `30d95f545a4fc89aaa27b0c08e62f63aa43e208239cad19df3776857e5470f1a` |
| `simon-c2-result.json` | `/Volumes/SSD990/cryptanalysis/sage -python /Volumes/SSD990-2/crypto-worktrees/exotic-ecdlp-recovery-20261119/research/exotic_ecdlp_20261021/xedni_selected_height_diagnostic_20261120/simon_c2_witness_sage.py` | `543c34a2a5d87d3c3fa95844294870a79b6167062e22e178d66468528b78763f` |
| `perturb-result.json` | `/Volumes/SSD990/cryptanalysis/sage -python /Volumes/SSD990-2/crypto-worktrees/exotic-ecdlp-recovery-20261119/research/exotic_ecdlp_20261021/xedni_selected_height_diagnostic_20261120/simon_perturb_sage.py` | `9f616341e0a38344420e8470e7cf7ecd8e410eedcf22b1d0cd18302231da4390` |
| `torsion-result.json` | `python3 research/exotic_ecdlp_20261021/xedni_selected_height_diagnostic_20261120/torsion_count.py` | `31f2761fbfca3ff2aafba6e6e3ed29db6a03bbcbd81597d014403ce82b37fd22` |
| `torsion-certificate.json` | `python3 research/exotic_ecdlp_20261021/xedni_selected_height_diagnostic_20261120/torsion_certificate.py` | `2e4e092a1cc2cac35208ae9a2ae4b183672fc86441b615590a0aba683bd484d9` |
| `audit-result.json` | `python3 research/exotic_ecdlp_20261021/xedni_selected_height_diagnostic_20261120/audit.py` | `ee6259750e852a028045571212d8405ce5803fb00110cf6d25082b49cea8cc30` |

`audit.py` and `audit-result.json` have SHA-256 values `ffc23562e6c31413080c226cdbd8dc9fe347f6fa82562df97cbb8ce37a76617d` and `ee6259750e852a028045571212d8405ce5803fb00110cf6d25082b49cea8cc30`, respectively. The first audit invocation detected a Python tuple/list representation mismatch in the verifier's own comparison; the corrected verifier passed. That failure preceded the published audit output and did not change the saved experiment rows.
