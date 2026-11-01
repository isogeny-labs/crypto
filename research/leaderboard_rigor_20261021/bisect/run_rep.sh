#!/bin/bash
cd /home/user/crypto
export PATCH_FROM=fb926c020:src/cryptanalysis/koblitz_factor_base_search.rs
research/leaderboard_rigor_20261021/bisect/replay_at_commit.sh 4065eb4b7890e42ed64cde21f78a10a15c02aa90 /home/user/crypto/research/f6_ic_ecbench_ladder_20261005/sessions/n13 research/leaderboard_rigor_20261021/bisect r70 | tail -1 >> research/leaderboard_rigor_20261021/bisect/bisect.log
research/leaderboard_rigor_20261021/bisect/replay_at_commit.sh e49afb98f0dacea2469fc6a4e746c6e13f60c334 /home/user/crypto/research/f6_ic_ecbench_ladder_20261005/sessions/n13 research/leaderboard_rigor_20261021/bisect r71 | tail -1 >> research/leaderboard_rigor_20261021/bisect/bisect.log
