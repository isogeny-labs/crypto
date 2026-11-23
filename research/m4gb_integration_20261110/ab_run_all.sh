#!/usr/bin/env bash
AB=$(dirname "$(readlink -f "$0")")
{
  printf 'input\tconfig\tarm\trep\twall_s\tmaxdeg\tpeak_mib\toutput_hash\n'
  $AB/run.sh n20_gf31 g31_n12 3 600; $AB/run.sh n20_gf31 g31_n14 3 600; $AB/run.sh n20_gf31 g31_n15 3 600; $AB/run.sh n20_gf31 g31_n16 3 900
  $AB/run.sh n12_gf65521 overdet_n12_m24_p65521 3 600; $AB/run.sh n12_gf65521 overdet_n10_m15_p65521 3 600; $AB/run.sh n12_gf65521 quad_n9_p65521 3 600
  for r in 1 2 3 4 5 6 7 8; do $AB/run.sh n35_gf2 root$r 3 600; done
  $AB/run.sh n128_gf2 root1 3 900; $AB/run.sh n128_gf2 root3 3 900; $AB/run.sh n64_gf2 wd_n16 3 600
  $AB/run.sh n30_gf2 g2_n30 2 900; $AB/run.sh n64_gf2 g2_n30 1 1800
} > $AB/results.tsv 2>$AB/run.err
cat $AB/results.tsv
