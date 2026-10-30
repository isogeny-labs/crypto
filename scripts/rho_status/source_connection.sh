#!/usr/bin/env bash
# Discover the original campaign connection using its existing AWS identity.
# Secret values stay inside the AWS CLI. Connection metadata is encrypted for
# the operator because this repository's workflow logs can be public.
set -euo pipefail
: "${RHO_SOURCE_REPORT_CERT:?Set the PEM public recipient certificate for the connection report.}"

aws_options=(--region us-west-2 --no-cli-pager --cli-connect-timeout 10 --cli-read-timeout 20)
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
printf '%s\n' "$RHO_SOURCE_REPORT_CERT" > "$scratch/report-cert.pem"
openssl x509 -in "$scratch/report-cert.pem" -noout >/dev/null

aws "${aws_options[@]}" sts get-caller-identity --output json > "$scratch/identity.json"
if ! jq -e '.Account == "590183823895"' "$scratch/identity.json" >/dev/null; then
    echo 'Refusing source discovery outside AWS account 590183823895.' >&2
    exit 1
fi

aws "${aws_options[@]}" rds describe-db-instances --db-instance-identifier rho-dp \
    --query 'DBInstances[0].{identifier:DBInstanceIdentifier,engine:Engine,status:DBInstanceStatus,host:Endpoint.Address,port:Endpoint.Port,database:DBName,publicly_accessible:PubliclyAccessible,vpc:DBSubnetGroup.VpcId,security_groups:VpcSecurityGroups[].VpcSecurityGroupId}' \
    --output json > "$scratch/database.json"
jq -e '.identifier == "rho-dp" and .engine == "postgres" and (.host | type == "string")' \
    "$scratch/database.json" >/dev/null

# Verify the configured identity can retrieve the connection, without sending
# its value to a file, shell variable, command argument, or workflow log.
aws "${aws_options[@]}" secretsmanager get-secret-value --secret-id rho/dp-rds \
    --query '{name:Name,arn:ARN}' --output json > "$scratch/secret.json"
jq -e '.name == "rho/dp-rds" and (.arn | startswith("arn:aws:secretsmanager:us-west-2:590183823895:secret:rho/dp-rds-"))' \
    "$scratch/secret.json" >/dev/null

jq -n --slurpfile identity "$scratch/identity.json" \
    --slurpfile database "$scratch/database.json" --slurpfile secret "$scratch/secret.json" \
    '{account:$identity[0].Account,principal:$identity[0].Arn,database:$database[0],secret:$secret[0],secret_read_verified:true}' \
    > "$scratch/report.json"
openssl cms -encrypt -binary -aes-256-cbc -outform PEM \
    -in "$scratch/report.json" -out "$scratch/report.pem" "$scratch/report-cert.pem"
case "${RHO_SOURCE_REPORT_DEST:-}" in
    '') cat "$scratch/report.pem" ;;
    s3://ecc-challenges-data-104186811093/connection-checks/*)
        aws "${aws_options[@]}" s3 cp "$scratch/report.pem" "$RHO_SOURCE_REPORT_DEST" \
            --content-type application/pkcs7-mime --sse AES256 --only-show-errors
        echo 'Encrypted connection report delivered to the destination account.'
        ;;
    *) echo 'Refusing a connection report destination outside the import account.' >&2; exit 1 ;;
esac
