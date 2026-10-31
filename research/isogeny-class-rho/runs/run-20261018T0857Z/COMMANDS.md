# Frozen command card — run-20261018T0857Z

Protocol: `research/isogeny-class-rho/PROTOCOL.md`, sha256
`81c3232bdab481244324101d10e2d50199da1990ceba10c8dc66cc206286409e`.
Binary: `target/release/isoclass-rho` built from this branch at the commit
named in the run README. Working directory: the repository root. Every
command is run exactly as written; stdout and stderr are kept beside its
output (`<class>/walk.log`, `<class>/<mode>.log`).

## Walks (executed by the Sonnet-tier agent, verbatim)

```sh
R=research/isogeny-class-rho/runs/run-20261018T0857Z
target/release/isoclass-rho walk --curve 235212858169,228505936335,82673460332 --ells 2,3,5,7,11,13,17,19 --hard-ells 23,29,31,37,41,43,47,53,59,61 --hard-from 3 --radius 3 --max-vertices 5000 --per-stratum 8 --hard 8 --sources 4 --seed 20261018 --out $R/P1
target/release/isoclass-rho walk --curve 217710133177,0,205801627284 --ells 2,3,5,7,11,13 --radius 4 --max-vertices 3000 --per-stratum 6 --hard 4 --sources 3 --seed 20261018 --out $R/P0
target/release/isoclass-rho walk --binary --curve 37,89938361934,61869205585 --ells 7,17 --hard-ells 23,31 --hard-from 3 --radius 3 --max-vertices 400 --per-stratum 8 --hard 6 --sources 4 --seed 20261018 --out $R/B1
target/release/isoclass-rho walk --binary --curve 41,0,1 --ells 409,1721 --radius 2 --max-vertices 30 --per-stratum 4 --hard 4 --sources 2 --sample-kernels 3 --seed 20261018 --out $R/K
```

## Measurements (main session, after each walk has written `walk.json`)

```sh
R=research/isogeny-class-rho/runs/run-20261018T0857Z
B=target/release/isoclass-rho
$B measure --walk $R/P1/walk.json --mode class      --trials 100 --controls 3 --plants --folds --seed 20261018 --out $R/P1/class
$B measure --walk $R/P1/walk.json --mode generators --generators 100 --max-vertices 6 --trials 25 --controls 0 --seed 20261018 --out $R/P1/generators
$B measure --walk $R/P1/walk.json --mode shift      --trials 25 --seed 20261018 --out $R/P1/shift
$B measure --walk $R/P0/walk.json --mode class      --trials 100 --controls 3 --plants --folds --seed 20261018 --out $R/P0/class
$B measure --walk $R/P0/walk.json --mode generators --generators 100 --max-vertices 4 --trials 25 --controls 0 --seed 20261018 --out $R/P0/generators
$B measure --walk $R/P0/walk.json --mode shift      --trials 25 --seed 20261018 --out $R/P0/shift
$B measure --walk $R/B1/walk.json --mode class      --trials 100 --controls 3 --plants --folds --seed 20261018 --out $R/B1/class
$B measure --walk $R/B1/walk.json --mode generators --generators 64 --max-vertices 6 --trials 25 --controls 0 --seed 20261018 --out $R/B1/generators
$B measure --walk $R/B1/walk.json --mode shift      --trials 25 --seed 20261018 --out $R/B1/shift
$B measure --walk $R/K/walk.json  --mode class      --trials 100 --controls 3 --plants --folds --seed 20261018 --out $R/K/class
$B measure --walk $R/K/walk.json  --mode generators --generators 64 --max-vertices 3 --trials 25 --controls 0 --seed 20261018 --out $R/K/generators
$B measure --walk $R/K/walk.json  --mode shift      --trials 25 --seed 20261018 --out $R/K/shift
```

## Replay check (after the measurements)

Each class mode is re-run with `--trials 5` into `<class>/replay/`; the
iteration counts of the first five seeds must equal those recorded in
`<class>/class/trials.csv`.
