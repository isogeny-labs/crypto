//! Presigned S3 GET/PUT URLs for the fixed AWS panel, using SigV4 query auth.
use super::Result;
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

type HmacSha256 = Hmac<Sha256>;

pub struct Credentials {
    pub access_key: String,
    pub secret_key: String,
    pub session_token: Option<String>,
}

pub struct Request<'a> {
    pub method: &'a str,
    pub bucket: &'a str,
    pub key: &'a str,
    pub region: &'a str,
    pub timestamp: &'a str,
    pub expires: u32,
    pub version_id: Option<&'a str>,
}

fn hmac(key: &[u8], message: &str) -> Result<Vec<u8>> {
    let mut state = HmacSha256::new_from_slice(key)?;
    state.update(message.as_bytes());
    Ok(state.finalize().into_bytes().to_vec())
}

fn encode(value: &str, keep_slash: bool) -> String {
    let mut result = String::new();
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric()
            || matches!(byte, b'-' | b'_' | b'.' | b'~')
            || (keep_slash && byte == b'/')
        {
            result.push(char::from(byte));
        } else {
            result.push_str(&format!("%{byte:02X}"));
        }
    }
    result
}

fn sign(request: Request<'_>, host: &str, credentials: &Credentials) -> Result<String> {
    let Request {
        method,
        key,
        region,
        timestamp,
        expires,
        version_id,
        ..
    } = request;
    if !matches!(method, "GET" | "PUT") || timestamp.len() != 16 || expires == 0 || expires > 604800
    {
        return Err("invalid S3 presign input".into());
    }
    let day = &timestamp[..8];
    let scope = format!("{day}/{region}/s3/aws4_request");
    let mut query = BTreeMap::from([
        ("X-Amz-Algorithm", "AWS4-HMAC-SHA256".to_owned()),
        (
            "X-Amz-Credential",
            format!("{}/{}", credentials.access_key, scope),
        ),
        ("X-Amz-Date", timestamp.to_owned()),
        ("X-Amz-Expires", expires.to_string()),
        ("X-Amz-SignedHeaders", "host".to_owned()),
    ]);
    if let Some(token) = &credentials.session_token {
        query.insert("X-Amz-Security-Token", token.clone());
    }
    if let Some(version) = version_id {
        query.insert("versionId", version.to_owned());
    }
    let encoded_query = query
        .iter()
        .map(|(name, value)| format!("{}={}", encode(name, false), encode(value, false)))
        .collect::<Vec<_>>()
        .join("&");
    let path = format!("/{}", encode(key, true));
    let canonical =
        format!("{method}\n{path}\n{encoded_query}\nhost:{host}\n\nhost\nUNSIGNED-PAYLOAD");
    let to_sign = format!(
        "AWS4-HMAC-SHA256\n{timestamp}\n{scope}\n{}",
        hex::encode(Sha256::digest(canonical.as_bytes()))
    );
    let day_key = hmac(format!("AWS4{}", credentials.secret_key).as_bytes(), day)?;
    let region_key = hmac(&day_key, region)?;
    let service_key = hmac(&region_key, "s3")?;
    let signing_key = hmac(&service_key, "aws4_request")?;
    let signature = hex::encode(hmac(&signing_key, &to_sign)?);
    Ok(format!(
        "https://{host}{path}?{encoded_query}&X-Amz-Signature={signature}"
    ))
}

pub fn presign(request: Request<'_>, credentials: &Credentials) -> Result<String> {
    let Request { bucket, region, .. } = request;
    let host = if region == "us-east-1" {
        format!("{bucket}.s3.amazonaws.com")
    } else {
        format!("{bucket}.s3.{region}.amazonaws.com")
    };
    sign(request, &host, credentials)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aws_documented_get_signature() -> Result<()> {
        let credentials = Credentials {
            access_key: "AKIAIOSFODNN7EXAMPLE".into(),
            secret_key: "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY".into(),
            session_token: None,
        };
        let url = sign(
            Request {
                method: "GET",
                bucket: "examplebucket",
                key: "test.txt",
                region: "us-east-1",
                timestamp: "20130524T000000Z",
                expires: 86400,
                version_id: None,
            },
            "examplebucket.s3.amazonaws.com",
            &credentials,
        )?;
        assert!(url.ends_with(
            "X-Amz-Signature=aeeed9bbccd4d02ee5c0109b86d86835f995330da4c265957d157751f604d404"
        ));
        Ok(())
    }

    #[test]
    fn query_values_and_object_path_are_encoded() -> Result<()> {
        let credentials = Credentials {
            access_key: "key".into(),
            secret_key: "secret".into(),
            session_token: Some("a+b/c=".into()),
        };
        let url = presign(
            Request {
                method: "PUT",
                bucket: "bucket",
                key: "x y/z+",
                region: "us-east-1",
                timestamp: "20261028T000000Z",
                expires: 60,
                version_id: None,
            },
            &credentials,
        )?;
        assert!(url.contains("/x%20y/z%2B?"));
        assert!(url.contains("X-Amz-Security-Token=a%2Bb%2Fc%3D"));
        Ok(())
    }
}
