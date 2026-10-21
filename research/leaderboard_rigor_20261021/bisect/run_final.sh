#!/bin/bash
cd /home/user/crypto
research/leaderboard_rigor_20261021/bisect/replay_at_commit.sh 16e74770da6b6d911e8a67adaf1f3b6ff936f40b /home/user/crypto/research/f6_ic_ecbench_ladder_20261005/sessions/n13 research/leaderboard_rigor_20261021/bisect b74 | tail -1 >> research/leaderboard_rigor_20261021/bisect/bisect.log
research/leaderboard_rigor_20261021/bisect/replay_at_commit.sh 8a41eca793207682b53ca35f04fe8ac99ee230f7 /home/user/crypto/research/f6_ic_ecbench_ladder_20261005/sessions/n13 research/leaderboard_rigor_20261021/bisect b75 | tail -1 >> research/leaderboard_rigor_20261021/bisect/bisect.log
