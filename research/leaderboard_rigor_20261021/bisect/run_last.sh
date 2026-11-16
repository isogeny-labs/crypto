#!/bin/bash
cd /home/user/crypto
research/leaderboard_rigor_20261021/bisect/replay_at_commit.sh 60afe841939e4de8b93f78d93cc04d73083c581f /home/user/crypto/research/f6_ic_ecbench_ladder_20261005/sessions/n13 research/leaderboard_rigor_20261021/bisect d63 | tail -1 >> research/leaderboard_rigor_20261021/bisect/bisect.log
research/leaderboard_rigor_20261021/bisect/replay_at_commit.sh e49afb98f0dacea2469fc6a4e746c6e13f60c334 /home/user/crypto/research/f6_ic_ecbench_ladder_20261005/sessions/n13 research/leaderboard_rigor_20261021/bisect d71 | tail -1 >> research/leaderboard_rigor_20261021/bisect/bisect.log
