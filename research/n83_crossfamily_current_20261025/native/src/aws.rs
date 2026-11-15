//! Read-only provider gate and bounded AWS Spot allocation for the N83 panel.
use super::{arms, bundle, fixture, sigv4, source, Result};
use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub const ACCOUNT: &str = "590183823895";
pub const REGION: &str = "us-east-1";
pub const ZONE: &str = "us-east-1d";
pub const VPC: &str = "vpc-0da45a1bf46c94fa9";
pub const SUBNET: &str = "subnet-0561480728d2b1ffd";
pub const INSTANCE: &str = "g5g.4xlarge";
pub const SECURITY_GROUP: &str = "sg-031c140ef9d078b3d";
pub const PROFILE: &str = "n83-online-screen-ssm-20261019";
pub const BUCKET: &str = "aburan28-n83-tier1-20261016-590183823895";
pub const PROJECT_TAG: &str = "N83HighKCurrent";
pub const CAP_SECONDS: u64 = 21600;
pub const BID_USD_HOURLY: &str = "0.27";
const AMI_PARAMETER: &str =
    "/aws/service/canonical/ubuntu/server/24.04/stable/current/arm64/hvm/ebs-gp3/ami-id";

pub fn cli(args: &[&str], region: &str) -> Result<Value> {
    let output = Command::new("aws")
        .args(args)
        .args(["--region", region, "--output", "json", "--no-cli-pager"])
        .output()?;
    if !output.status.success() {
        return Err(format!(
            "AWS CLI operation failed: {} {}",
            args.first().unwrap_or(&""),
            args.get(1).unwrap_or(&"")
        )
        .into());
    }
    if output.stdout.is_empty() {
        return Ok(Value::Null);
    }
    Ok(serde_json::from_slice(&output.stdout)?)
}

fn checked_source(root: &Path, directory: &Path) -> Result<Value> {
    source(root)?;
    fixture(root)?;
    arms(root)?;
    let bundle = bundle::verify(directory, None)?;
    let head = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(root)
        .output()?;
    let status = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(root)
        .output()?;
    if !head.status.success()
        || !status.status.success()
        || !status.stdout.is_empty()
        || bundle["source_head_sha"] != String::from_utf8(head.stdout)?.trim()
    {
        return Err("source bundle differs from clean exact checkout".into());
    }
    Ok(bundle)
}

fn field<'a>(value: &'a Value, key: &str) -> Result<&'a str> {
    value[key]
        .as_str()
        .ok_or_else(|| format!("AWS response missing {key}").into())
}

fn latest_quote(rows: &Value, now: DateTime<Utc>) -> Result<(String, String)> {
    let candidates = rows["SpotPriceHistory"]
        .as_array()
        .ok_or("Spot price history absent")?;
    let mut latest: Option<(&Value, DateTime<Utc>)> = None;
    for row in candidates {
        if row["InstanceType"] != INSTANCE || row["AvailabilityZone"] != ZONE {
            continue;
        }
        let timestamp = DateTime::parse_from_rfc3339(field(row, "Timestamp")?)?.to_utc();
        if latest.as_ref().is_none_or(|(_, prior)| timestamp > *prior) {
            latest = Some((row, timestamp));
        }
    }
    let (row, timestamp) = latest.ok_or("no matching Spot quote")?;
    if timestamp > now || now.signed_duration_since(timestamp).num_seconds() > 12 * 3600 {
        return Err("Spot quote is stale".into());
    }
    let price = field(row, "SpotPrice")?;
    let rate: f64 = price.parse()?;
    if !(0.0..=0.27).contains(&rate) || rate == 0.0 {
        return Err("Spot quote exceeds approved bid".into());
    }
    Ok((price.to_owned(), field(row, "Timestamp")?.to_owned()))
}

pub fn plan(root: &Path, source_bundle: &Path, output: &Path) -> Result<Value> {
    if output.exists() {
        return Err("AWS plan output already exists".into());
    }
    let bundle = checked_source(root, source_bundle)?;
    let now = DateTime::<Utc>::from(SystemTime::now());
    let identity = cli(&["sts", "get-caller-identity"], REGION)?;
    if identity["Account"] != ACCOUNT {
        return Err("AWS account differs".into());
    }
    let quota = cli(
        &[
            "service-quotas",
            "get-service-quota",
            "--service-code",
            "ec2",
            "--quota-code",
            "L-3819A6DF",
        ],
        REGION,
    )?;
    let quota_value = quota["Quota"]["Value"]
        .as_f64()
        .ok_or("Spot vCPU quota absent")?;
    if quota_value < 32.0 {
        return Err("G and VT Spot vCPU quota below 32".into());
    }
    let kinds = cli(
        &[
            "ec2",
            "describe-instance-types",
            "--instance-types",
            INSTANCE,
        ],
        REGION,
    )?;
    let spec = &kinds["InstanceTypes"][0];
    if spec["InstanceType"] != INSTANCE
        || spec["VCpuInfo"]["DefaultVCpus"].as_u64().unwrap_or(0) < 16
        || spec["MemoryInfo"]["SizeInMiB"].as_u64().unwrap_or(0) < 32768
        || !spec["ProcessorInfo"]["SupportedArchitectures"]
            .as_array()
            .is_some_and(|items| items.iter().any(|item| item == "arm64"))
    {
        return Err("AWS instance resources or architecture differ".into());
    }
    let subnets = cli(&["ec2", "describe-subnets", "--subnet-ids", SUBNET], REGION)?;
    let subnet = &subnets["Subnets"][0];
    if subnet["VpcId"] != VPC
        || subnet["AvailabilityZone"] != ZONE
        || subnet["State"] != "available"
        || subnet["MapPublicIpOnLaunch"] != true
        || subnet["AvailableIpAddressCount"].as_u64().unwrap_or(0) < 2
    {
        return Err("approved east-region subnet differs".into());
    }
    let zones = cli(
        &["ec2", "describe-availability-zones", "--zone-names", ZONE],
        REGION,
    )?;
    let zone_id = field(&zones["AvailabilityZones"][0], "ZoneId")?;
    let scores = cli(
        &[
            "ec2",
            "get-spot-placement-scores",
            "--instance-types",
            INSTANCE,
            "--target-capacity",
            "2",
            "--target-capacity-unit-type",
            "units",
            "--single-availability-zone",
            "--region-names",
            REGION,
        ],
        REGION,
    )?;
    let score = scores["SpotPlacementScores"]
        .as_array()
        .ok_or("Spot placement scores absent")?
        .iter()
        .find(|item| item["AvailabilityZoneId"] == zone_id)
        .and_then(|item| item["Score"].as_u64())
        .ok_or("east-zone placement score absent")?;
    if score < 3 {
        return Err("east-zone placement score below approved gate".into());
    }
    let parameter = cli(&["ssm", "get-parameter", "--name", AMI_PARAMETER], REGION)?;
    let image_id = field(&parameter["Parameter"], "Value")?;
    let images = cli(&["ec2", "describe-images", "--image-ids", image_id], REGION)?;
    let image = &images["Images"][0];
    if image["ImageId"] != image_id
        || image["Architecture"] != "arm64"
        || image["State"] != "available"
    {
        return Err("Ubuntu ARM64 image differs".into());
    }
    let history = cli(
        &[
            "ec2",
            "describe-spot-price-history",
            "--instance-types",
            INSTANCE,
            "--product-descriptions",
            "Linux/UNIX",
            "--availability-zone",
            ZONE,
            "--max-results",
            "100",
        ],
        REGION,
    )?;
    let (quote, quote_time) = latest_quote(&history, now)?;
    let zone = |stratum| {
        json!({"stratum":stratum,"zone":ZONE,"subnet_id":SUBNET,
        "zone_id":zone_id,"spot_placement_score":score,"quote_usd_per_hour":quote,
        "quote_time":quote_time,"max_hourly_usd":BID_USD_HOURLY})
    };
    let record = json!({"schema":"n83.crossfamily-aws-spot-plan/v1",
        "status":"READY_FOR_APPROVED_ALLOCATION","tier":"cloud_instance_tier1",
        "placement_model":"two_distinct_instances_same_availability_zone",
        "recorded_utc":now.to_rfc3339(),"account_id":ACCOUNT,"region":REGION,
        "instance_type":INSTANCE,"architecture":"arm64","image_id":image_id,
        "source_head_sha":bundle["source_head_sha"],
        "source_archive_sha256":bundle["source_archive_sha256"],
        "source_manifest_sha256":bundle["source_manifest_sha256"],
        "spot_quota_vcpus":quota_value,"cap_seconds":CAP_SECONDS,
        "root_disk_gb_per_host":50,"gp3_usd_per_gb_month":"0.08",
        "s3_allowance_usd":"0.05","projected_at_bids_usd":"3.37",
        "total_cap_usd":"4.20","zones":[zone(0),zone(1)]});
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output)?;
    serde_json::to_writer_pretty(&mut file, &record)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    Ok(record)
}

pub fn verify_plan(root: &Path, source_bundle: &Path, plan_path: &Path) -> Result<Value> {
    let plan: Value = serde_json::from_slice(&fs::read(plan_path)?)?;
    let bundle = checked_source(root, source_bundle)?;
    if plan["schema"] != "n83.crossfamily-aws-spot-plan/v1"
        || plan["status"] != "READY_FOR_APPROVED_ALLOCATION"
        || plan["account_id"] != ACCOUNT
        || plan["region"] != REGION
        || plan["instance_type"] != INSTANCE
        || plan["architecture"] != "arm64"
        || plan["source_head_sha"] != bundle["source_head_sha"]
        || plan["source_archive_sha256"] != bundle["source_archive_sha256"]
        || plan["source_manifest_sha256"] != bundle["source_manifest_sha256"]
        || plan["cap_seconds"] != CAP_SECONDS
        || plan["root_disk_gb_per_host"] != 50
        || plan["projected_at_bids_usd"] != "3.37"
        || plan["total_cap_usd"] != "4.20"
        || plan["zones"].as_array().is_none_or(|zones| {
            zones.len() != 2
                || zones.iter().enumerate().any(|(index, zone)| {
                    zone["stratum"] != index
                        || zone["zone"] != ZONE
                        || zone["subnet_id"] != SUBNET
                        || zone["max_hourly_usd"] != BID_USD_HOURLY
                })
        })
    {
        return Err("approved AWS plan differs from sealed inputs".into());
    }
    Ok(plan)
}

fn atomic_json(path: &Path, record: &Value) -> Result<()> {
    let temporary = path.with_extension("json.tmp");
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&temporary)?;
    serde_json::to_writer_pretty(&mut file, record)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    fs::rename(temporary, path)?;
    Ok(())
}

fn now_epoch() -> Result<u64> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())
}

fn credentials() -> Result<sigv4::Credentials> {
    let output = Command::new("aws")
        .args(["configure", "export-credentials", "--format", "process"])
        .output()?;
    if !output.status.success() {
        return Err("AWS credentials are unavailable".into());
    }
    let record: Value = serde_json::from_slice(&output.stdout)?;
    Ok(sigv4::Credentials {
        access_key: field(&record, "AccessKeyId")?.to_owned(),
        secret_key: field(&record, "SecretAccessKey")?.to_owned(),
        session_token: record["SessionToken"].as_str().map(str::to_owned),
    })
}

fn timestamp() -> Result<String> {
    let output = Command::new("date")
        .args(["-u", "+%Y%m%dT%H%M%SZ"])
        .output()?;
    if !output.status.success() {
        return Err("UTC clock unavailable".into());
    }
    Ok(String::from_utf8(output.stdout)?.trim().to_owned())
}

fn upload(path: &Path, key: &str, digest: &str, head: &str) -> Result<Value> {
    let body = path.to_str().ok_or("non-UTF8 source bundle path")?;
    let metadata = format!("sha256={digest},source-head={head}");
    let response = cli(
        &[
            "s3api",
            "put-object",
            "--bucket",
            BUCKET,
            "--key",
            key,
            "--body",
            body,
            "--metadata",
            &metadata,
        ],
        REGION,
    )?;
    let version = field(&response, "VersionId")?;
    Ok(json!({"key":key,"version_id":version,"sha256":digest}))
}

fn check_presigned_transfer(
    manifest_url: &str,
    manifest_digest: &str,
    put_url: &str,
    canary_key: &str,
    output: &Path,
) -> Result<Value> {
    let downloaded = output.join("presign-manifest-check.json");
    let got = Command::new("curl")
        .args(["--fail", "--silent", "--show-error", "--output"])
        .arg(&downloaded)
        .arg(manifest_url)
        .output()?;
    if !got.status.success() || super::hash(&fs::read(&downloaded)?) != manifest_digest {
        return Err("presigned versioned S3 GET failed its content check".into());
    }
    fs::remove_file(downloaded)?;
    let canary = output.join("presign-put-canary.txt");
    fs::write(&canary, b"n83-presign-put-canary\n")?;
    let sent = Command::new("curl")
        .args(["--fail", "--silent", "--show-error", "--upload-file"])
        .arg(&canary)
        .arg(put_url)
        .output()?;
    if !sent.status.success() {
        return Err("presigned S3 PUT canary failed".into());
    }
    let head = cli(
        &[
            "s3api",
            "head-object",
            "--bucket",
            BUCKET,
            "--key",
            canary_key,
        ],
        REGION,
    )?;
    if head["ContentLength"] != fs::metadata(&canary)?.len()
        || head["VersionId"].as_str().is_none_or(str::is_empty)
    {
        return Err("presigned S3 PUT canary readback differs".into());
    }
    fs::remove_file(canary)?;
    Ok(json!({"key":canary_key,"version_id":head["VersionId"],
        "status":"PASS_PRESIGNED_GET_AND_PUT"}))
}

fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\"'\"'"))
}

struct Links<'a> {
    source_url: &'a str,
    manifest_url: &'a str,
    result_url: &'a str,
    status_url: &'a str,
}
fn user_data(
    plan: &Value,
    stratum: usize,
    links: Links<'_>,
    run_id: &str,
    root: &Path,
) -> Result<String> {
    let template = json!({"schema":"n83.crossfamily-current-aws-allocation/v1",
        "stratum":stratum,"account_id":ACCOUNT,"region":REGION,"instance_type":INSTANCE,
        "availability_zone":ZONE,"image_id":plan["image_id"],"market":"spot",
        "source_head_sha":plan["source_head_sha"],
        "source_archive_sha256":plan["source_archive_sha256"],
        "source_manifest_sha256":plan["source_manifest_sha256"],"run_id":run_id});
    let values = [
        ("N83_STRATUM", stratum.to_string()),
        ("N83_CAP_SECONDS", CAP_SECONDS.to_string()),
        ("N83_SOURCE_GET_URL", links.source_url.to_owned()),
        ("N83_MANIFEST_GET_URL", links.manifest_url.to_owned()),
        ("N83_RESULTS_PUT_URL", links.result_url.to_owned()),
        ("N83_STATUS_PUT_URL", links.status_url.to_owned()),
        (
            "N83_SOURCE_SHA256",
            field(plan, "source_archive_sha256")?.to_owned(),
        ),
        (
            "N83_MANIFEST_SHA256",
            field(plan, "source_manifest_sha256")?.to_owned(),
        ),
        ("N83_ALLOCATION_JSON", serde_json::to_string(&template)?),
    ];
    let bootstrap = fs::read_to_string(
        root.join("research/n83_crossfamily_current_20261025/native/aws_bootstrap.sh"),
    )?;
    if !bootstrap.starts_with("#!/") {
        return Err("native AWS bootstrap shebang absent".into());
    }
    let mut script = String::from("#!/bin/bash\n");
    for (name, value) in values {
        script.push_str(&format!("export {name}={}\n", shell_quote(&value)));
    }
    script.push_str(
        bootstrap
            .split_once('\n')
            .ok_or("native bootstrap empty")?
            .1,
    );
    if script.len() > 16_384 {
        return Err("AWS user data exceeds 16 KiB".into());
    }
    Ok(script)
}

fn launch(
    plan: &Value,
    stratum: usize,
    script: &str,
    run_id: &str,
    output: &Path,
) -> Result<String> {
    let user_data_path = output.join(format!("userdata-s{stratum}.sh"));
    let mut script_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(&user_data_path)?;
    script_file.write_all(script.as_bytes())?;
    script_file.sync_all()?;
    let market = json!({"MarketType":"spot","SpotOptions":{"MaxPrice":BID_USD_HOURLY,
        "SpotInstanceType":"one-time","InstanceInterruptionBehavior":"terminate"}})
    .to_string();
    let metadata = json!({"HttpTokens":"required","HttpEndpoint":"enabled"}).to_string();
    let disk = json!([{"DeviceName":"/dev/sda1","Ebs":{"VolumeSize":50,
        "VolumeType":"gp3","Encrypted":true,"DeleteOnTermination":true}}])
    .to_string();
    let tags = ["instance", "volume"]
        .iter()
        .map(|kind| {
            json!({"ResourceType":kind,
        "Tags":[{"Key":"Project","Value":PROJECT_TAG},{"Key":"RunId","Value":run_id},
            {"Key":"SourceHead","Value":plan["source_head_sha"]},
            {"Key":"Stratum","Value":stratum.to_string()}]})
        })
        .collect::<Vec<_>>();
    let tags = serde_json::to_string(&tags)?;
    let token = format!("n83-{run_id}-{stratum}");
    let profile = format!("Name={PROFILE}");
    let script_arg = format!("file://{}", user_data_path.display());
    let result = cli(
        &[
            "ec2",
            "run-instances",
            "--image-id",
            field(plan, "image_id")?,
            "--instance-type",
            INSTANCE,
            "--min-count",
            "1",
            "--max-count",
            "1",
            "--client-token",
            &token,
            "--subnet-id",
            SUBNET,
            "--security-group-ids",
            SECURITY_GROUP,
            "--iam-instance-profile",
            &profile,
            "--instance-market-options",
            &market,
            "--metadata-options",
            &metadata,
            "--instance-initiated-shutdown-behavior",
            "terminate",
            "--block-device-mappings",
            &disk,
            "--tag-specifications",
            &tags,
            "--user-data",
            &script_arg,
        ],
        REGION,
    );
    fs::remove_file(user_data_path)?;
    let result = result?;
    let instances = result["Instances"]
        .as_array()
        .ok_or("EC2 launch instances absent")?;
    if instances.len() != 1 {
        return Err("EC2 launch did not return one instance".into());
    }
    Ok(field(&instances[0], "InstanceId")?.to_owned())
}

fn discover(run_id: &str) -> Result<Vec<Value>> {
    let project_filter = format!("Name=tag:Project,Values={PROJECT_TAG}");
    let run_filter = format!("Name=tag:RunId,Values={run_id}");
    let response = cli(
        &[
            "ec2",
            "describe-instances",
            "--filters",
            &project_filter,
            &run_filter,
        ],
        REGION,
    )?;
    Ok(response["Reservations"]
        .as_array()
        .ok_or("EC2 reservations absent")?
        .iter()
        .flat_map(|reservation| {
            reservation["Instances"]
                .as_array()
                .into_iter()
                .flatten()
                .cloned()
        })
        .collect())
}

fn terminate(ids: &[String]) -> Result<()> {
    if ids.is_empty() {
        return Ok(());
    }
    let mut args = vec!["ec2", "terminate-instances", "--instance-ids"];
    args.extend(ids.iter().map(String::as_str));
    cli(&args, REGION)?;
    Ok(())
}

fn active_ids(instances: &[Value]) -> Vec<String> {
    instances
        .iter()
        .filter(|item| {
            matches!(
                item["State"]["Name"].as_str(),
                Some("pending" | "running" | "stopping" | "stopped")
            )
        })
        .filter_map(|item| item["InstanceId"].as_str().map(str::to_owned))
        .collect()
}

fn guard_path(output: &Path) -> &Path {
    output
}

pub fn guard(receipt_path: &Path, output: &Path) -> Result<Value> {
    let initial: Value = serde_json::from_slice(&fs::read(receipt_path)?)?;
    if initial["schema"] != "n83.crossfamily-aws-controller/v1"
        || initial["account_id"] != ACCOUNT
        || initial["region"] != REGION
    {
        return Err("guard receipt identity differs".into());
    }
    let run_id = field(&initial, "run_id")?.to_owned();
    let deadline = initial["deadline_epoch"]
        .as_u64()
        .ok_or("guard deadline absent")?;
    while now_epoch()? < deadline {
        std::thread::sleep(Duration::from_secs(
            deadline.saturating_sub(now_epoch()?).min(30),
        ));
    }
    let mut attempts = Vec::new();
    for _ in 0..12 {
        let instances = match discover(&run_id) {
            Ok(instances) => instances,
            Err(error) => {
                attempts.push(json!({"at_epoch":now_epoch()?,"provider_error":error.to_string()}));
                std::thread::sleep(Duration::from_secs(15));
                continue;
            }
        };
        let ids = active_ids(&instances);
        if ids.is_empty() {
            let result = json!({"status":"ABSENT_AFTER_DEADLINE","run_id":run_id,
                "deadline_epoch":deadline,"attempts":attempts});
            atomic_json(guard_path(output), &result)?;
            return Ok(result);
        }
        let termination = terminate(&ids);
        attempts.push(json!({"at_epoch":now_epoch()?,"instance_ids":ids,
            "termination_status":if termination.is_ok() {"REQUESTED"} else {"PROVIDER_ERROR"}}));
        std::thread::sleep(Duration::from_secs(15));
    }
    let result = json!({"status":"TERMINATION_UNCONFIRMED","run_id":run_id,
        "deadline_epoch":deadline,"attempts":attempts});
    atomic_json(guard_path(output), &result)?;
    Err("local AWS deletion guard could not confirm absence".into())
}

fn start_guard(receipt: &Path, output: &Path) -> Result<u32> {
    let log = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(output.join("local_guard.log"))?;
    let error_log = log.try_clone()?;
    let mut call = Command::new(std::env::current_exe()?);
    call.arg("aws-guard")
        .arg(receipt)
        .arg(output.join("local_guard.json"))
        .stdin(Stdio::null())
        .stdout(Stdio::from(log))
        .stderr(Stdio::from(error_log));
    unsafe {
        call.pre_exec(|| {
            if libc::setsid() < 0 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
    Ok(call.spawn()?.id())
}

fn preflight_provider() -> Result<()> {
    let group = cli(
        &[
            "ec2",
            "describe-security-groups",
            "--group-ids",
            SECURITY_GROUP,
        ],
        REGION,
    )?;
    let group = &group["SecurityGroups"][0];
    if group["VpcId"] != VPC
        || group["IpPermissions"]
            .as_array()
            .is_none_or(|rules| !rules.is_empty())
    {
        return Err("N83 security group differs or permits inbound traffic".into());
    }
    let profile = cli(
        &[
            "iam",
            "get-instance-profile",
            "--instance-profile-name",
            PROFILE,
        ],
        REGION,
    )?;
    if !field(&profile["InstanceProfile"], "Arn")?.contains(&format!(":{ACCOUNT}:")) {
        return Err("EC2 instance profile belongs to another account".into());
    }
    cli(&["s3api", "head-bucket", "--bucket", BUCKET], REGION)?;
    let filter = format!("Name=tag:Project,Values={PROJECT_TAG}");
    let existing = cli(&["ec2", "describe-instances", "--filters", &filter], REGION)?;
    let instances = existing["Reservations"]
        .as_array()
        .ok_or("EC2 reservations absent")?
        .iter()
        .flat_map(|r| r["Instances"].as_array().into_iter().flatten().cloned())
        .collect::<Vec<_>>();
    if !active_ids(&instances).is_empty() {
        return Err("another N83 allocation is active".into());
    }
    Ok(())
}

fn run_id() -> Result<String> {
    let stamp = timestamp()?;
    let mut bytes = [0u8; 4];
    std::io::Read::read_exact(&mut File::open("/dev/urandom")?, &mut bytes)?;
    Ok(format!("{stamp}{}", hex::encode(bytes)))
}

pub fn allocate(
    root: &Path,
    source_bundle: &Path,
    plan_path: &Path,
    output: &Path,
) -> Result<Value> {
    let approved_plan = verify_plan(root, source_bundle, plan_path)?;
    preflight_provider()?;
    if output.exists() {
        return Err("AWS allocation output already exists".into());
    }
    fs::create_dir(output)?;
    // Placement and price are mutable. Recheck both at the allocation boundary.
    let plan = plan(root, source_bundle, &output.join("fresh-plan.json"))?;
    if plan["image_id"] != approved_plan["image_id"]
        || plan["zones"][0]["zone_id"] != approved_plan["zones"][0]["zone_id"]
        || plan["zones"][1]["zone_id"] != approved_plan["zones"][1]["zone_id"]
    {
        return Err("approved image or zone changed before allocation".into());
    }
    let run_id = run_id()?;
    let deadline = now_epoch()? + CAP_SECONDS;
    let receipt_path = output.join("allocation.json");
    let mut receipt = json!({"schema":"n83.crossfamily-aws-controller/v1",
        "account_id":ACCOUNT,"region":REGION,"source_head_sha":plan["source_head_sha"],
        "run_id":run_id,"deadline_epoch":deadline,"instance_ids":[],"strata":[],
        "source_objects":{},"result_objects":{},"status_objects":{},
        "allocation_cap_usd":"4.20","bid_projection_usd":"3.37"});
    atomic_json(&receipt_path, &receipt)?;
    let prefix = format!(
        "n83-highk-current/{}/{run_id}",
        field(&plan, "source_head_sha")?
    );
    for (name, sha_key) in [
        ("source.tar.gz", "source_archive_sha256"),
        ("manifest.json", "source_manifest_sha256"),
    ] {
        let key = format!("{prefix}/{name}");
        receipt["source_objects"][name] = upload(
            &source_bundle.join(name),
            &key,
            field(&plan, sha_key)?,
            field(&plan, "source_head_sha")?,
        )?;
        atomic_json(&receipt_path, &receipt)?;
    }
    let credentials = credentials()?;
    let date = timestamp()?;
    let source_url = sigv4::presign(
        sigv4::Request {
            method: "GET",
            bucket: BUCKET,
            key: field(&receipt["source_objects"]["source.tar.gz"], "key")?,
            region: REGION,
            timestamp: &date,
            expires: CAP_SECONDS as u32,
            version_id: Some(field(
                &receipt["source_objects"]["source.tar.gz"],
                "version_id",
            )?),
        },
        &credentials,
    )?;
    let manifest_url = sigv4::presign(
        sigv4::Request {
            method: "GET",
            bucket: BUCKET,
            key: field(&receipt["source_objects"]["manifest.json"], "key")?,
            region: REGION,
            timestamp: &date,
            expires: CAP_SECONDS as u32,
            version_id: Some(field(
                &receipt["source_objects"]["manifest.json"],
                "version_id",
            )?),
        },
        &credentials,
    )?;
    let canary_key = format!("{prefix}/control/presign-canary.txt");
    let canary_url = sigv4::presign(
        sigv4::Request {
            method: "PUT",
            bucket: BUCKET,
            key: &canary_key,
            region: REGION,
            timestamp: &date,
            expires: 900,
            version_id: None,
        },
        &credentials,
    )?;
    receipt["presigned_transfer_canary"] = check_presigned_transfer(
        &manifest_url,
        field(&plan, "source_manifest_sha256")?,
        &canary_url,
        &canary_key,
        output,
    )?;
    atomic_json(&receipt_path, &receipt)?;
    receipt["local_guard_pid"] = json!(start_guard(&receipt_path, output)?);
    atomic_json(&receipt_path, &receipt)?;
    let result = (|| -> Result<()> {
        for stratum in 0..2 {
            let result_key = format!("{prefix}/stratum{stratum}/raw_results.tar.gz");
            let status_key = format!("{prefix}/stratum{stratum}/status.tar.gz");
            let result_url = sigv4::presign(
                sigv4::Request {
                    method: "PUT",
                    bucket: BUCKET,
                    key: &result_key,
                    region: REGION,
                    timestamp: &date,
                    expires: (CAP_SECONDS + 900) as u32,
                    version_id: None,
                },
                &credentials,
            )?;
            let status_url = sigv4::presign(
                sigv4::Request {
                    method: "PUT",
                    bucket: BUCKET,
                    key: &status_key,
                    region: REGION,
                    timestamp: &date,
                    expires: (CAP_SECONDS + 900) as u32,
                    version_id: None,
                },
                &credentials,
            )?;
            let script = user_data(
                &plan,
                stratum,
                Links {
                    source_url: &source_url,
                    manifest_url: &manifest_url,
                    result_url: &result_url,
                    status_url: &status_url,
                },
                &run_id,
                root,
            )?;
            let id = launch(&plan, stratum, &script, &run_id, output)?;
            receipt["instance_ids"]
                .as_array_mut()
                .ok_or("instance IDs absent")?
                .push(json!(id));
            receipt["strata"]
                .as_array_mut()
                .ok_or("strata absent")?
                .push(json!({
                "stratum":stratum,"instance_id":id,"zone":ZONE,"subnet_id":SUBNET,
                "max_hourly_usd":BID_USD_HOURLY}));
            receipt["result_objects"][stratum.to_string()] = json!(result_key);
            receipt["status_objects"][stratum.to_string()] = json!(status_key);
            atomic_json(&receipt_path, &receipt)?;
        }
        let instances = discover(&run_id)?;
        let ids = active_ids(&instances);
        if ids.len() != 2 || ids[0] == ids[1] {
            return Err("two distinct AWS instances absent".into());
        }
        let frozen = instances
            .iter()
            .map(|item| {
                let tags = item["Tags"]
                    .as_array()
                    .map(|rows| {
                        rows.iter()
                            .filter_map(|row| {
                                Some((row["Key"].as_str()?.to_owned(), row["Value"].clone()))
                            })
                            .collect::<serde_json::Map<String, Value>>()
                    })
                    .unwrap_or_default();
                json!({"account_id":ACCOUNT,"instance_id":item["InstanceId"],
                "instance_type":item["InstanceType"],
                "instance_lifecycle":item["InstanceLifecycle"],
                "image_id":item["ImageId"],
                "availability_zone":item["Placement"]["AvailabilityZone"],
                "subnet_id":item["SubnetId"],"state":item["State"]["Name"],
                "launch_time":item["LaunchTime"],"tags":tags})
            })
            .collect::<Vec<_>>();
        if frozen.iter().any(|item| {
            item["instance_type"] != INSTANCE
                || item["instance_lifecycle"] != "spot"
                || item["image_id"] != plan["image_id"]
                || item["availability_zone"] != ZONE
                || item["subnet_id"] != SUBNET
                || item["tags"]["RunId"] != run_id
        }) {
            return Err("EC2 provider snapshot differs from launched panel".into());
        }
        atomic_json(
            &output.join("provider_snapshot_at_launch.json"),
            &json!({
            "schema":"n83.crossfamily-aws-provider-snapshot/v1",
            "captured_utc":DateTime::<Utc>::from(SystemTime::now()).to_rfc3339(),
            "snapshot_source":"ec2_describe_instances_at_launch","controller":receipt,
            "instances":frozen}),
        )?;
        Ok(())
    })();
    if let Err(error) = result {
        let ids = discover(&run_id)
            .map(|items| active_ids(&items))
            .unwrap_or_default();
        let _ = terminate(&ids);
        return Err(error);
    }
    Ok(json!({"status":"ALLOCATED_GUARDED","run_id":run_id,
        "instance_ids":receipt["instance_ids"],"deadline_epoch":deadline,
        "receipt":receipt_path}))
}
