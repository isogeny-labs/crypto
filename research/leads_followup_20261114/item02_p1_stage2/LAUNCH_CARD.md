# Launch card: P1 stage 2 (k = 15, D = 6) [pcc:unverified, not executed]

Run from /home/user/ml-cryptanalysis (e8aecdd). Engine sha256 prefixes: ic_bounds_big.py e4f15a08..., ic_bounds_big_run.py 6c8bb10e...,
ic_bounds_big_pod.py c272d242..., nopool_run.py 9a71a45b....

## Memory plan (python3 scripts/ic_bounds_big_run.py plan --k 15 --D 6 --device gpu --dev-gb G --work-gb 20 --tile-mb 1024)
columns 768,212 (parts 384,394 + 383,818). Worst matrices 290.6 GB, target part worst 145.5 GB, predicted peak 313.1 GB.
G = 105 or 135: spill.feasible false (H200 insufficient). G = 170: feasible, tile budget 147.5 GB, host spill worst 143.1 GB.
=> need device >= ~175 GB (B200 180 GB) and host RAM >= 300 GB. Time extrapolated about 10 h on H200-class (k=14 took 3.24 h);
a B200 may be faster, unmeasured. Cost: H200 was $5.29/h; B200 price must be read from the console/`prices` before creation;
at the 12 h cap worst case = 12 x rate.

## Launch
    export POD_STATE_DIR=/home/user/.pod-state-item02   # RUNPOD_API_KEY must be in the environment
    cd /home/user/ml-cryptanalysis; T="python3 scripts/ic_bounds_big_pod.py"
    $T create --name icb-k15 --gpu "NVIDIA B200" --min-ram-per-gpu 300 --min-vcpu-per-gpu 16 --max-seconds 43200 \
        --pip "numpy threadpoolctl cupy-cuda12x[ctk]" --volume-gb 600 --disk-gb 40 --dry-run   # inspect, then --yes-create
        # if the volume is refused: --volume-gb 0 --disk-gb 600 (then a stop at the cap loses ckpt)
    $T ping icb-k15; $T sys icb-k15
    for f in csd/ic_bounds_big.py scripts/ic_bounds_big_run.py scripts/ic_bounds_cert_check.py \
             experiments/ic-bounds-k14/relaunch/nopool_run.py experiments/ic-bounds-k14/instances/unit-p65537-a1-s10-k15.json; do $T push icb-k15 $f; done
    # self-test first (about 7 min on H200); abort and terminate if it fails on this SKU:
    $T run icb-k15 st -- python3 nopool_run.py ic_bounds_big_run.py selftest --device gpu --out /workspace/out/selftest-gpu.json
    # closure (spot-safe: checkpoint every 30 min, --resume restarts from it; exit 75 on SIGTERM):
    $T run icb-k15 k15 -- python3 nopool_run.py ic_bounds_big_run.py closure --instance /workspace/in/unit-p65537-a1-s10-k15.json \
        --D 6 --device gpu --dev-gb 170 --work-gb 20 --tile-mb 1024 --batch 1024 --threads 8 \
        --ckpt /workspace/ckpt/unit-p65537-k15-D6 --ckpt-minutes 30 --resume \
        --status /workspace/out/unit-p65537-k15-D6.status.json --out /workspace/out/unit-p65537-k15-D6.json
    # copy loop (outside the session, terminates the pod when finished; mirror.sh needs POD_STATE_DIR):
    nohup experiments/ic-bounds-k14/relaunch/mirror.sh icb-k15 k15 results status:unit-p65537-k15-D6.status.json 300 \
        out/unit-p65537-k15-D6.json out/unit-p65537-k15-D6.status.json out/selftest-gpu.json logs/k15.log > /dev/null 2>&1 &
    # (mirror.sh DEST arg 'results' = a local dir, e.g. research/leads_followup_20261114/item02_p1_stage2/fetched)

## Fetch results later
    $T get icb-k15 out/unit-p65537-k15-D6.json fetched/unit-p65537-k15-D6.json; $T jobs icb-k15; $T terminate <id> --yes; $T ls

## Pass/fail rule (against the prediction)
Pass: status done, closed true, clean sweep, contains_one false, rank 600,547 (q = 167,665) and checker agrees.
Report the distance otherwise (rank - 600,547), F1/F2/F3 of PROTOCOL.md. Record the pod id, billed cost and checkpoints.
