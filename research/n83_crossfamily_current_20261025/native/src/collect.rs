//! Version-pinned S3 custody and early EC2 termination after both host uploads.
use super::{aws, Result};
use flate2::read::GzDecoder;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Component, Path};

fn version(key: &str) -> Result<Value> {
    let rows = aws::cli(
        &[
            "s3api",
            "list-object-versions",
            "--bucket",
            aws::BUCKET,
            "--prefix",
            key,
        ],
        aws::REGION,
    )?;
    rows["Versions"]
        .as_array()
        .ok_or("S3 object version absent")?
        .iter()
        .filter(|row| row["Key"] == key)
        .max_by_key(|row| row["LastModified"].as_str().unwrap_or(""))
        .cloned()
        .ok_or_else(|| format!("S3 object absent: {key}").into())
}
fn download(key: &str, path: &Path) -> Result<Value> {
    let object = version(key)?;
    let id = object["VersionId"].as_str().ok_or("S3 version ID absent")?;
    let name = path.to_str().ok_or("non-UTF8 output path")?;
    let fetched = aws::cli(
        &[
            "s3api",
            "get-object",
            "--bucket",
            aws::BUCKET,
            "--key",
            key,
            "--version-id",
            id,
            name,
        ],
        aws::REGION,
    )?;
    let size = fs::metadata(path)?.len();
    if fetched["VersionId"] != id || fetched["ContentLength"] != size {
        return Err(format!("S3 version or length differs: {key}").into());
    }
    let mut file = File::open(path)?;
    let mut state = Sha256::new();
    let mut block = [0u8; 1024 * 1024];
    loop {
        let count = file.read(&mut block)?;
        if count == 0 {
            break;
        }
        state.update(&block[..count]);
    }
    Ok(
        json!({"key":key,"version_id":id,"bytes":size,"sha256":hex::encode(state.finalize()),
        "last_modified":object["LastModified"]}),
    )
}
fn extract(archive: &Path, output: &Path) -> Result<()> {
    fs::create_dir(output)?;
    let file = File::open(archive)?;
    let mut package = tar::Archive::new(GzDecoder::new(file));
    for member in package.entries()? {
        let mut member = member?;
        let name = member.path()?.into_owned();
        if name.as_os_str().is_empty()
            || !name
                .components()
                .all(|part| matches!(part, Component::Normal(_)))
            || !(member.header().entry_type().is_dir() || member.header().entry_type().is_file())
        {
            return Err("unsafe raw result archive member".into());
        }
        if !member.unpack_in(output)? {
            return Err("raw result archive escaped output directory".into());
        }
    }
    Ok(())
}
pub fn run(receipt_path: &Path, output: &Path) -> Result<Value> {
    let controller: Value = serde_json::from_slice(&fs::read(receipt_path)?)?;
    if controller["schema"] != "n83.crossfamily-aws-controller/v1"
        || controller["account_id"] != aws::ACCOUNT
        || controller["region"] != aws::REGION
        || controller["instance_ids"]
            .as_array()
            .is_none_or(|ids| ids.len() != 2 || ids[0] == ids[1])
        || controller["strata"]
            .as_array()
            .is_none_or(|rows| rows.len() != 2)
    {
        return Err("AWS controller receipt differs".into());
    }
    if output.exists() {
        return Err("collection output already exists".into());
    }
    fs::create_dir(output)?;
    fs::copy(receipt_path, output.join("allocation.json"))?;
    let mut rows = Vec::new();
    for stratum in 0..2 {
        let folder = output.join(format!("stratum{stratum}"));
        fs::create_dir(&folder)?;
        let key = controller["result_objects"][stratum.to_string()]
            .as_str()
            .ok_or("result object key absent")?;
        let status_key = controller["status_objects"][stratum.to_string()]
            .as_str()
            .ok_or("status object key absent")?;
        let raw = folder.join("raw_results.tar.gz");
        let status = folder.join("status.tar.gz");
        let raw_record = download(key, &raw)?;
        let status_record = download(status_key, &status)?;
        extract(&raw, &folder)?;
        if !folder.join("results/host.json").is_file()
            || !folder.join("results/source.json").is_file()
        {
            return Err("raw result archive lacks host/source receipts".into());
        }
        rows.push(json!({"stratum":stratum,"raw":raw_record,"status":status_record}));
    }
    // With both immutable S3 versions in local custody, stop billing before
    // the six-hour deletion guards whenever the host work finished early.
    let ids = controller["instance_ids"]
        .as_array()
        .ok_or("instance IDs absent")?
        .iter()
        .map(|row| row.as_str().ok_or_else(|| "instance ID absent".into()))
        .collect::<Result<Vec<_>>>()?;
    let mut args = vec!["ec2", "terminate-instances", "--instance-ids"];
    args.extend(ids.iter().copied());
    let termination = aws::cli(&args, aws::REGION)?;
    let receipt = json!({"schema":"n83.crossfamily-aws-collection/v1",
        "status":"BOTH_RAW_ARCHIVES_COLLECTED_TERMINATION_REQUESTED",
        "source_head_sha":controller["source_head_sha"],
        "instance_ids":ids,"objects":rows,"termination":termination});
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output.join("collection.json"))?;
    serde_json::to_writer_pretty(&mut file, &receipt)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    Ok(receipt)
}
