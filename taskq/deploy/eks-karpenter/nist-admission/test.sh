#!/usr/bin/env bash
# Synthetic administrative controls: no real AWS requests or scientific inputs.
set -euo pipefail
nist_here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
nist_tmp=$(mktemp -d)
trap 'rm -rf -- "$nist_tmp"' EXIT
cat > "$nist_tmp/aws" <<'MOCK'
#!/bin/bash
printf '%s\n' "$1 $2" >> "$NIST_TEST_CALLS"
case "$1 $2" in
 'sts get-caller-identity')
   [[ $NIST_TEST_MODE != denied ]] || exit 1
   if [[ $NIST_TEST_MODE == wrong-account ]]; then echo 000000000000; else echo 104186811093; fi;;
 'budgets describe-budget')
   [[ $NIST_TEST_MODE != budget-denied ]] || exit 1
   if [[ $NIST_TEST_MODE == missing-spend ]]; then echo '{"Budget":{}}'; else
     echo '{"Budget":{"BudgetLimit":{"Amount":"200","Unit":"USD"},"CalculatedSpend":{"ActualSpend":{"Amount":"10","Unit":"USD"}}}}'
   fi;;
 *) echo 'Unexpected mutating or unrelated AWS operation' >&2; exit 99;;
esac
MOCK
chmod 700 "$nist_tmp/aws"
for nist_mode in observed denied wrong-account budget-denied missing-spend; do
    export NIST_TEST_MODE=$nist_mode NIST_TEST_CALLS="$nist_tmp/$nist_mode.calls" NIST_ADMISSION_AWS_BIN="$nist_tmp/aws"
    set +e
    bash "$nist_here/check.sh" "$nist_tmp/$nist_mode/report.json" > "$nist_tmp/$nist_mode.stdout" 2> "$nist_tmp/$nist_mode.stderr"
    nist_status=$?
    set -e
    if [[ $nist_mode == observed ]]; then
        [[ $nist_status == 0 ]]
        jq -e '.account_matches and (.budgets|length)==3 and all(.budgets[]; .readable and .charged_remainder_positive)' "$nist_tmp/$nist_mode/report.json" >/dev/null
    else
        [[ $nist_status == 1 ]]
    fi
    jq -e '.launch_admitted==false and .scientific_job_started==false and .reservations=="unobserved"' "$nist_tmp/$nist_mode/report.json" >/dev/null
    if [[ $nist_mode == denied || $nist_mode == wrong-account ]]; then [[ $(wc -l < "$NIST_TEST_CALLS") == 1 ]]; fi
    # Public report contains neither private financial amounts nor credential fields.
    ! rg -q 'ActualSpend|BudgetLimit|AccessKey|Secret|Token|"Amount"' "$nist_tmp/$nist_mode/report.json"
    printf 'ok %s\n' "$nist_mode"
done
