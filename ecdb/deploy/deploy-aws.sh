#!/usr/bin/env bash
# Deploy ecdb to AWS: private RDS PostgreSQL 16, two Lambda functions in the
# default VPC (ecdb-admin: migrate / roles / import; ecdb-api: the HTTP API
# behind a function URL with a bearer token). Idempotent: existing resources
# are reused by name. IAM needed: deploy/iam-policy.json for the caller and the
# ecdb-lambda-exec role (deploy/README.md).
#
#   AWS_REGION=us-west-2 ecdb/deploy/deploy-aws.sh path/to/ecdb-lambda.zip
set -euo pipefail
zip=${1:?usage: deploy-aws.sh ecdb-lambda.zip}
export AWS_REGION=${AWS_REGION:-us-west-2} AWS_DEFAULT_REGION=${AWS_REGION:-us-west-2}
aws=${AWS:-aws}
name=${ECDB_NAME:-ecdb}
instance_class=${ECDB_INSTANCE_CLASS:-db.t4g.medium}
account=$($aws sts get-caller-identity --query Account --output text)
role_arn="arn:aws:iam::$account:role/ecdb-lambda-exec"
log() { printf '%s %s\n' "$(date -u +%H:%M:%S)" "$*" >&2; }

# ---------------------------------------------------------------- network
vpc=$($aws ec2 describe-vpcs --filters Name=is-default,Values=true --query 'Vpcs[0].VpcId' --output text)
[ "$vpc" != None ] || { echo "no default VPC in $AWS_REGION" >&2; exit 1; }
subnets=$($aws ec2 describe-subnets --filters Name=vpc-id,Values="$vpc" Name=default-for-az,Values=true \
  --query 'Subnets[].SubnetId' --output text)
log "vpc $vpc subnets $subnets"

sg() { # sg <name> <description> -> id (created with project=ecdb tags when missing)
  local id
  id=$($aws ec2 describe-security-groups --filters Name=vpc-id,Values="$vpc" Name=group-name,Values="$1" \
    --query 'SecurityGroups[0].GroupId' --output text)
  if [ "$id" = None ]; then
    id=$($aws ec2 create-security-group --vpc-id "$vpc" --group-name "$1" --description "$2" \
      --tag-specifications "ResourceType=security-group,Tags=[{Key=project,Value=ecdb},{Key=Name,Value=$1}]" \
      --query GroupId --output text)
    log "created security group $1 $id"
  fi
  echo "$id"
}
sg_lambda=$(sg "$name-lambda" "ecdb Lambda functions (egress to ecdb RDS only)")
sg_rds=$(sg "$name-rds" "ecdb RDS: PostgreSQL from ecdb Lambda functions only")
$aws ec2 authorize-security-group-ingress --group-id "$sg_rds" \
  --ip-permissions "IpProtocol=tcp,FromPort=5432,ToPort=5432,UserIdGroupPairs=[{GroupId=$sg_lambda}]" >/dev/null 2>&1 || true
# Lambda functions need nothing but PostgreSQL: replace allow-all egress with 5432 to the RDS group.
$aws ec2 authorize-security-group-egress --group-id "$sg_lambda" \
  --ip-permissions "IpProtocol=tcp,FromPort=5432,ToPort=5432,UserIdGroupPairs=[{GroupId=$sg_rds}]" >/dev/null 2>&1 || true
$aws ec2 revoke-security-group-egress --group-id "$sg_lambda" \
  --ip-permissions 'IpProtocol=-1,IpRanges=[{CidrIp=0.0.0.0/0}]' >/dev/null 2>&1 || true

if ! $aws rds describe-db-subnet-groups --db-subnet-group-name "$name" >/dev/null 2>&1; then
  # shellcheck disable=SC2086
  $aws rds create-db-subnet-group --db-subnet-group-name "$name" --db-subnet-group-description "ecdb" \
    --subnet-ids $subnets --tags Key=project,Value=ecdb >/dev/null
  log "created DB subnet group $name"
fi

# ---------------------------------------------------------------- secrets
password() { $aws secretsmanager get-random-password --password-length 40 --exclude-punctuation --query RandomPassword --output text; }
secret() { # secret <name> <json-producing command> -> value json
  local v
  if v=$($aws secretsmanager get-secret-value --secret-id "$1" --query SecretString --output text 2>/dev/null); then
    echo "$v"; return
  fi
  v=$(eval "$2")
  $aws secretsmanager create-secret --name "$1" --secret-string "$v" --tags Key=project,Value=ecdb >/dev/null
  log "created secret $1"
  echo "$v"
}
field() { python3 -c 'import json,sys; print(json.loads(sys.argv[1])[sys.argv[2]])' "$1" "$2"; }
master=$(secret "$name/master" 'printf "{\"username\":\"ecdb_admin\",\"password\":\"%s\"}" "$(password)"')
logins=$(secret "$name/api" 'printf "{\"api_password\":\"%s\",\"writer_password\":\"%s\",\"read_token\":\"%s\",\"write_token\":\"%s\"}" "$(password)" "$(password)" "$(password)" "$(password)"')

# ---------------------------------------------------------------- RDS
if ! $aws rds describe-db-instances --db-instance-identifier "$name" >/dev/null 2>&1; then
  $aws rds create-db-instance --db-instance-identifier "$name" --engine postgres --engine-version 16 \
    --db-instance-class "$instance_class" --allocated-storage 20 --storage-type gp3 --storage-encrypted \
    --master-username "$(field "$master" username)" --master-user-password "$(field "$master" password)" \
    --db-name ecdb --db-subnet-group-name "$name" --vpc-security-group-ids "$sg_rds" --no-publicly-accessible \
    --backup-retention-period 7 --deletion-protection --copy-tags-to-snapshot --auto-minor-version-upgrade \
    --tags Key=project,Value=ecdb >/dev/null
  log "creating RDS instance $name ($instance_class); this takes several minutes"
fi
$aws rds wait db-instance-available --db-instance-identifier "$name"
endpoint=$($aws rds describe-db-instances --db-instance-identifier "$name" --query 'DBInstances[0].Endpoint.Address' --output text)
log "RDS $endpoint"

# ---------------------------------------------------------------- Lambda
url() { printf 'postgres://%s:%s@%s:5432/ecdb' "$1" "$2" "$endpoint"; }
env_json() { # env_json KEY=VALUE ... -> {"Variables": {...}} (URLs break the CLI shorthand syntax)
  python3 -c 'import json,sys; print(json.dumps({"Variables": dict(a.split("=", 1) for a in sys.argv[1:])}))' "$@"
}
common=(ECDB_PG_TLS=require ECDB_PG_CA_FILE=/var/task/rds-ca.pem ECDB_POOL_SIZE=2 RUST_LOG=info)
admin_env=$(env_json "${common[@]}" ECDB_LAMBDA_MODE=admin ECDB_REPO_ROOT=/var/task/data \
  "ECDB_SOURCE_COMMIT=$(unzip -p "$zip" SOURCE_COMMIT)" \
  "DATABASE_URL=$(url "$(field "$master" username)" "$(field "$master" password)")" \
  "ECDB_API_PASSWORD=$(field "$logins" api_password)" "ECDB_WRITER_PASSWORD=$(field "$logins" writer_password)")
api_env=$(env_json "${common[@]}" ECDB_LAMBDA_MODE=api ECDB_SQL_ROLE=ecdb_reader \
  "DATABASE_URL=$(url ecdb_api "$(field "$logins" api_password)")" \
  "ECDB_WRITE_DATABASE_URL=$(url ecdb_api_writer "$(field "$logins" writer_password)")" \
  "ECDB_READ_TOKEN=$(field "$logins" read_token)" "ECDB_WRITE_TOKEN=$(field "$logins" write_token)")
vpc_config="SubnetIds=$(echo "$subnets" | tr '\t ' ',,'),SecurityGroupIds=$sg_lambda"

function_up() { # function_up <name> <memory MB> <timeout s> <env>
  if $aws lambda get-function --function-name "$1" >/dev/null 2>&1; then
    $aws lambda update-function-code --function-name "$1" --zip-file "fileb://$zip" >/dev/null
    $aws lambda wait function-updated --function-name "$1"
    $aws lambda update-function-configuration --function-name "$1" --memory-size "$2" --timeout "$3" \
      --environment "$4" --vpc-config "$vpc_config" >/dev/null
  else
    $aws lambda create-function --function-name "$1" --runtime provided.al2023 --handler bootstrap \
      --architectures x86_64 --role "$role_arn" --zip-file "fileb://$zip" --memory-size "$2" --timeout "$3" \
      --environment "$4" --vpc-config "$vpc_config" --tags project=ecdb >/dev/null
    log "created function $1"
  fi
  $aws lambda wait function-updated --function-name "$1" 2>/dev/null || $aws lambda wait function-active --function-name "$1"
}
function_up "$name-admin" 3008 900 "$admin_env"
function_up "$name-api" 1024 30 "$api_env"
# At most 10 concurrent API instances x 2 connections x 2 pools stays far below max_connections.
# Accounts whose total Lambda concurrency is small cannot reserve any (AWS keeps 10 unreserved);
# their account limit already bounds the connections.
$aws lambda put-function-concurrency --function-name "$name-api" --reserved-concurrent-executions 10 >/dev/null 2>&1 \
  || log "reserved concurrency not set (account concurrency limit too small); the account limit bounds connections"

if ! $aws lambda get-function-url-config --function-name "$name-api" >/dev/null 2>&1; then
  $aws lambda create-function-url-config --function-name "$name-api" --auth-type NONE >/dev/null
  # The API checks its own bearer token; the URL itself is public.
  $aws lambda add-permission --function-name "$name-api" --statement-id public-url \
    --action lambda:InvokeFunctionUrl --principal '*' --function-url-auth-type NONE >/dev/null
  $aws lambda add-permission --function-name "$name-api" --statement-id public-url-invoke \
    --action lambda:InvokeFunction --principal '*' --invoked-via-function-url >/dev/null 2>&1 || true
fi
api_url=$($aws lambda get-function-url-config --function-name "$name-api" --query FunctionUrl --output text)

# ---------------------------------------------------------------- schema and data
admin() { # admin <event json>
  local out ev; out=$(mktemp); ev=$(mktemp)
  printf '%s' "$1" > "$ev"   # fileb:// sends the bytes as-is under AWS CLI v1 and v2
  $aws lambda invoke --function-name "$name-admin" --cli-read-timeout 900 \
    --payload "fileb://$ev" "$out" --query FunctionError --output text >&2
  cat "$out"; echo; rm -f "$out" "$ev"
}
log "migrate";         admin '{"action":"migrate"}'
log "login roles";     admin '{"action":"bootstrap_roles"}'
if [ "${ECDB_SKIP_IMPORT:-0}" != 1 ]; then
  for what in registry walks traits covers transfers certificates siblings derived enrich results; do
    # Keep the full report in a variable: a pipe into head would SIGPIPE the script under pipefail.
    log "import $what"; report=$(admin "{\"action\":\"import\",\"what\":\"$what\"}"); echo "${report:0:2000}"
  done
  # ecbench sessions: the parts build-lambda.sh left beside the zip, one invocation each.
  parts="${zip%.zip}.ecbench"
  if [ -d "$parts" ]; then
    n=0
    for part in "$parts"/ecbench-*.jsonl.gz; do
      ev=$(mktemp)
      printf '{"action":"import_pack","name":"ecbench","data":"%s"}' "$(base64 -w0 "$part")" > "$ev"
      report=$(admin "$(cat "$ev")"); rm -f "$ev"; n=$((n + 1))
      log "ecbench part $n: ${report:0:300}"
    done
  else
    log "no ecbench parts at $parts; skipped"
  fi
fi
log "status";          admin '{"action":"status"}'
echo "API: $api_url  (bearer token: secret $name/api, key read_token)"
