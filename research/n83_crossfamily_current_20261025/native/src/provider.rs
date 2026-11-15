//! Bind one native stratum to the sealed source bundle and EC2 IMDSv2 identity.
use super::Result;
use flate2::read::GzDecoder;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::path::Path;
use std::time::Duration;

pub struct Bound {
    pub allocation: Value,
    pub allocation_bytes: Vec<u8>,
    pub identity: Value,
    pub allocation_sha256: String,
    pub archive_sha256: String,
    pub manifest_sha256: String,
}
fn sha_file(path: &Path) -> Result<String> {
    let mut source = File::open(path)?;
    let mut state = Sha256::new();
    let mut block = [0u8; 1024 * 1024];
    loop {
        let n = source.read(&mut block)?;
        if n == 0 {
            break;
        }
        state.update(&block[..n]);
    }
    Ok(hex::encode(state.finalize()))
}
fn metadata_request(method: &str, path: &str, token: Option<&str>) -> Result<String> {
    let endpoint = SocketAddr::from(([169, 254, 169, 254], 80));
    let mut stream = TcpStream::connect_timeout(&endpoint, Duration::from_secs(3))?;
    stream.set_read_timeout(Some(Duration::from_secs(3)))?;
    stream.set_write_timeout(Some(Duration::from_secs(3)))?;
    let auth = token
        .map(|value| format!("X-aws-ec2-metadata-token: {value}\r\n"))
        .unwrap_or_else(|| "X-aws-ec2-metadata-token-ttl-seconds: 21600\r\n".into());
    let request = format!("{method} /latest/{path} HTTP/1.1\r\nHost: 169.254.169.254\r\n{auth}Content-Length: 0\r\nConnection: close\r\n\r\n");
    stream.write_all(request.as_bytes())?;
    let mut response = Vec::new();
    stream.read_to_end(&mut response)?;
    let split = response
        .windows(4)
        .position(|bytes| bytes == b"\r\n\r\n")
        .ok_or("IMDS response header absent")?;
    let header = std::str::from_utf8(&response[..split])?;
    if !header.starts_with("HTTP/1.1 200 ") && !header.starts_with("HTTP/1.0 200 ") {
        return Err("IMDS request did not return HTTP 200".into());
    }
    Ok(String::from_utf8(response[split + 4..].to_vec())?)
}
fn instance_identity() -> Result<Value> {
    let token = metadata_request("PUT", "api/token", None)?;
    if token.trim().is_empty() {
        return Err("IMDSv2 token absent".into());
    }
    let document = metadata_request("GET", "dynamic/instance-identity/document", Some(&token))?;
    let pkcs7 = metadata_request("GET", "dynamic/instance-identity/pkcs7", Some(&token))?;
    let lifecycle = metadata_request("GET", "meta-data/instance-life-cycle", Some(&token))?;
    Ok(
        json!({"document":serde_json::from_str::<Value>(&document)?,"pkcs7":pkcs7,"instance_life_cycle":lifecycle.trim()}),
    )
}
fn safe_path(name: &Path) -> bool {
    use std::path::Component;
    !name.as_os_str().is_empty()
        && name
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
}
fn source_bundle(root: &Path, directory: &Path, allocation: &Value) -> Result<(String, String)> {
    let archive = directory.join("source.tar.gz");
    let manifest_path = directory.join("manifest.json");
    let archive_sha = sha_file(&archive)?;
    let manifest_sha = sha_file(&manifest_path)?;
    let manifest: Value = serde_json::from_slice(&fs::read(&manifest_path)?)?;
    if manifest["schema"] != "n83.crossfamily-source-bundle/v1"
        || manifest["source_head_sha"] != allocation["source_head_sha"]
        || manifest["source_archive_sha256"] != archive_sha
        || manifest["paths"] != json!(crate::bundle::PATHS)
        || allocation["source_archive_sha256"] != archive_sha
        || allocation["source_manifest_sha256"] != manifest_sha
        || manifest["members_sha256"].as_object().is_none()
    {
        return Err("allocation/source bundle identity differs".into());
    }
    let expected = manifest["members_sha256"]
        .as_object()
        .ok_or("bundle member map absent")?;
    let mut seen = BTreeMap::new();
    let mut package = tar::Archive::new(GzDecoder::new(File::open(&archive)?));
    for member in package.entries()? {
        let mut member = member?;
        let path = member.path()?.into_owned();
        if !safe_path(&path) {
            return Err("unsafe source archive member".into());
        }
        if member.header().entry_type().is_dir() {
            continue;
        }
        if matches!(member.header().entry_type().as_byte(), b'g' | b'x') {
            continue;
        }
        if !member.header().entry_type().is_file() {
            return Err("unsafe source archive member".into());
        }
        let name = path.to_str().ok_or("non-UTF8 source member")?.to_owned();
        let mut state = Sha256::new();
        let mut block = [0u8; 1024 * 1024];
        loop {
            let n = member.read(&mut block)?;
            if n == 0 {
                break;
            }
            state.update(&block[..n]);
        }
        let digest = hex::encode(state.finalize());
        if seen.insert(name.clone(), digest.clone()).is_some()
            || expected.get(&name).and_then(Value::as_str) != Some(digest.as_str())
            || !root.join(&path).is_file()
            || root.join(&path).is_symlink()
            || sha_file(&root.join(&path))? != digest
        {
            return Err(format!("source member differs: {name}").into());
        }
    }
    if seen.len() != expected.len() {
        return Err("source bundle member set differs".into());
    }
    Ok((archive_sha, manifest_sha))
}
pub fn verify(root: &Path, stratum: usize, allocation_path: &Path, bundle: &Path) -> Result<Bound> {
    let allocation_bytes = fs::read(allocation_path)?;
    let allocation: Value = serde_json::from_slice(&allocation_bytes)?;
    if allocation["schema"] != "n83.crossfamily-current-aws-allocation/v1"
        || allocation["stratum"] != stratum
        || allocation["account_id"] != "590183823895"
        || allocation["region"] != "us-east-1"
        || allocation["availability_zone"] != "us-east-1d"
        || allocation["instance_type"] != "g5g.4xlarge"
        || allocation["market"] != "spot"
        || allocation["state"] != "running"
        || allocation["instance_id"]
            .as_str()
            .is_none_or(|id| !id.starts_with("i-"))
        || allocation["run_id"].as_str().is_none_or(str::is_empty)
        || allocation["image_id"].as_str().is_none_or(str::is_empty)
        || allocation["source_head_sha"]
            .as_str()
            .is_none_or(|id| id.len() != 40)
        || allocation["source_archive_sha256"]
            .as_str()
            .is_none_or(|id| id.len() != 64)
        || allocation["source_manifest_sha256"]
            .as_str()
            .is_none_or(|id| id.len() != 64)
    {
        return Err("incomplete east-region AWS allocation".into());
    }
    let (archive_sha256, manifest_sha256) = source_bundle(root, bundle, &allocation)?;
    let identity = instance_identity()?;
    let document = &identity["document"];
    for (document_key, allocation_key) in [
        ("accountId", "account_id"),
        ("instanceId", "instance_id"),
        ("instanceType", "instance_type"),
        ("availabilityZone", "availability_zone"),
        ("region", "region"),
        ("imageId", "image_id"),
    ] {
        if document[document_key] != allocation[allocation_key] {
            return Err(format!("EC2 instance identity differs: {document_key}").into());
        }
    }
    if identity["instance_life_cycle"] != "spot"
        || identity["pkcs7"].as_str().is_none_or(str::is_empty)
    {
        return Err("EC2 Spot identity is incomplete".into());
    }
    Ok(Bound {
        allocation,
        allocation_bytes,
        identity,
        allocation_sha256: sha_file(allocation_path)?,
        archive_sha256,
        manifest_sha256,
    })
}

pub fn bind_allocation(template: &Path, output: &Path) -> Result<Value> {
    let mut allocation: Value = serde_json::from_slice(&fs::read(template)?)?;
    if allocation["schema"] != "n83.crossfamily-current-aws-allocation/v1"
        || allocation["region"] != "us-east-1"
        || allocation["availability_zone"] != "us-east-1d"
        || allocation["instance_type"] != "g5g.4xlarge"
        || allocation["market"] != "spot"
        || allocation["instance_id"].as_str().is_some()
    {
        return Err("AWS allocation template differs".into());
    }
    let identity = instance_identity()?;
    let document = &identity["document"];
    for (document_key, allocation_key) in [
        ("accountId", "account_id"),
        ("instanceType", "instance_type"),
        ("availabilityZone", "availability_zone"),
        ("region", "region"),
        ("imageId", "image_id"),
    ] {
        if document[document_key] != allocation[allocation_key] {
            return Err(format!("EC2 template identity differs: {document_key}").into());
        }
    }
    if identity["instance_life_cycle"] != "spot"
        || identity["pkcs7"].as_str().is_none_or(str::is_empty)
    {
        return Err("EC2 Spot identity is incomplete".into());
    }
    allocation["instance_id"] = document["instanceId"].clone();
    allocation["state"] = Value::String("running".to_owned());
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output)?;
    serde_json::to_writer_pretty(&mut file, &allocation)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    Ok(json!({"allocation_sha256":sha_file(output)?,"instance_id":allocation["instance_id"]}))
}
