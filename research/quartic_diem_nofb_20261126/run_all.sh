#!/bin/sh
# Driver: PROTOCOL section 3 order (run_main.sh, then run_131101.sh), wall stamps. Not a frozen file.
set -e
cd /home/user/crypto/research/quartic_diem_nofb_20261126
sha256sum -c frozen.sha256 > raw/frozen_check.txt
date -u +%FT%TZ > raw/main_start.txt; ./run_main.sh > raw/main_stdout.txt 2>&1; date -u +%FT%TZ > raw/main_end.txt
date -u +%FT%TZ > raw/r131101_start.txt; ./run_131101.sh > raw/r131101_stdout.txt 2>&1; date -u +%FT%TZ > raw/r131101_end.txt
