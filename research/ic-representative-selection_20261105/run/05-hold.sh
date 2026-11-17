#!/usr/bin/env bash
# Stage 2: after every cal job of a class is collected, freeze its selection and write
# its hold and holdaa specs. selection.json is committed and pushed before any hold job.
#   run/05-hold.sh <tier> <class-id>     e.g. run/05-hold.sh 24 t24-s2403
source "$(dirname "$0")/common.sh"
tier=$1; class=$2
dir=$D/inputs/classes/t$tier/${class#t$tier-}
recs=$(ls $D/sessions/repsel-$class-cal-*/session/records.jsonl | paste -sd,)
labs=$(ls $D/inputs/specs/$class/repsel-$class-cal-*.labels.jsonl | paste -sd,)
rounds=$(jq -er ".tiers[\"$tier\"].rounds" $DESIGN); targets=$(jq -er ".tiers[\"$tier\"].targets.cal" $DESIGN)
$REPSEL select --records "$recs" --labels "$labs" --expected-runs $((rounds * targets)) \
  --out $D/inputs/specs/$class/selection.json
$REPSEL hold --class-dir "$dir" --design $DESIGN --tier $tier --class "$class" \
  --selection $D/inputs/specs/$class/selection.json --out $D/inputs/specs/$class | tee -a $D/inputs/jobs-hold.tsv
sha256sum $D/inputs/specs/$class/selection.json
