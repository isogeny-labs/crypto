//! Software-only S3 permission/absence regressions. No cloud or process-wide
//! environment changes: each backend receives its own fake AWS executable.
//!
//! Contract references:
//! https://docs.aws.amazon.com/AmazonS3/latest/API/API_HeadObject.html
//! https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListObjectsV2.html
//! HEAD 403 is ambiguous, including for checksum/KMS permissions. Only a
//! complete, empty listing scoped to the exact requested full key proves absence.

#![cfg(unix)]

use crypto_lib::store::backend::{Backend, S3Backend};
use serde_json::{json, Value};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

const BUCKET: &str = "setec-astronomy-artifacts";
const PREFIX: &str = "store/ml-dataset";
const KEY: &str = "sha256/ab/abcdef";
const FORBIDDEN: &str = "An error occurred (403) when calling the HeadObject operation: Forbidden";

fn quote(text: &str) -> String {
    format!("'{}'", text.replace('\'', "'\\''"))
}

struct Fixture {
    root: PathBuf,
    aws: PathBuf,
    log: PathBuf,
}

impl Fixture {
    fn new(
        head_status: u8,
        head_stdout: &[u8],
        head_stderr: &str,
        list_status: u8,
        list_stdout: &[u8],
        list_stderr: &str,
    ) -> Self {
        static SERIAL: AtomicU64 = AtomicU64::new(0);
        let root = std::env::temp_dir().join(format!(
            "store-scoped-head-{}-{}",
            std::process::id(),
            SERIAL.fetch_add(1, Ordering::Relaxed)
        ));
        // Never remove or reuse an existing directory, including stale test data.
        fs::create_dir(&root).unwrap();
        let aws = root.join("fake-aws");
        let log = root.join("calls.txt");
        for (name, bytes) in [
            ("head.stdout", head_stdout),
            ("head.stderr", head_stderr.as_bytes()),
            ("list.stdout", list_stdout),
            ("list.stderr", list_stderr.as_bytes()),
        ] {
            fs::write(root.join(name), bytes).unwrap();
        }
        let path = |name: &str| quote(root.join(name).to_str().unwrap());
        // The first line of each call is argc, followed by one line per argument;
        // empty keys and spaces remain distinguishable without shell evaluation.
        let script = format!(
            "#!/bin/sh\nset -eu\nprintf '%s\\n' \"$#\" \"$@\" >> {}\n\
             case \"$1:$2\" in\n\
             s3api:head-object) /bin/cat {}; /bin/cat {} >&2; exit {head_status} ;;\n\
             s3api:list-objects-v2) /bin/cat {}; /bin/cat {} >&2; exit {list_status} ;;\n\
             *) printf '%s\\n' 'UNEXPECTED COMMAND: no PUT/GET is permitted' >&2; exit 97 ;;\n\
             esac\n",
            quote(log.to_str().unwrap()),
            path("head.stdout"),
            path("head.stderr"),
            path("list.stdout"),
            path("list.stderr"),
        );
        fs::write(&aws, script).unwrap();
        fs::set_permissions(&aws, fs::Permissions::from_mode(0o700)).unwrap();
        Self { root, aws, log }
    }

    fn forbidden(list: &Value) -> Self {
        Self::new(
            255,
            b"",
            FORBIDDEN,
            0,
            &serde_json::to_vec(list).unwrap(),
            "",
        )
    }

    fn backend(&self, bucket: &str, prefix: &str, endpoint: Option<&str>) -> S3Backend {
        S3Backend {
            bucket: bucket.into(),
            prefix: prefix.into(),
            endpoint: endpoint.map(str::to_owned),
            aws: self.aws.to_str().unwrap().into(),
        }
    }

    fn calls(&self) -> Vec<Vec<String>> {
        let text = fs::read_to_string(&self.log).unwrap();
        let mut lines = text.lines();
        let mut calls = Vec::new();
        while let Some(count) = lines.next() {
            let count: usize = count.parse().unwrap();
            let mut args = Vec::new();
            for _ in 0..count {
                args.push(lines.next().unwrap().to_owned());
            }
            calls.push(args);
        }
        calls
    }

    fn assert_calls(&self, backend: &S3Backend, key: &str, listed: bool) {
        let mut expected = vec![backend.head_args(key)];
        if listed {
            let full_key = if backend.prefix.is_empty() {
                key.to_owned()
            } else {
                format!("{}/{key}", backend.prefix)
            };
            let mut args: Vec<String> = [
                "s3api",
                "list-objects-v2",
                "--bucket",
                &backend.bucket,
                "--prefix",
                &full_key,
                "--max-keys",
                "1",
                "--no-paginate",
                "--output",
                "json",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect();
            if let Some(endpoint) = &backend.endpoint {
                args.extend(["--endpoint-url".into(), endpoint.clone()]);
            }
            expected.push(args);
        }
        let actual = self.calls();
        assert_eq!(actual, expected, "exact argument vectors and call count");
        assert!(actual.len() <= 2);
        assert!(actual
            .iter()
            .all(|call| call[1] == "head-object" || call[1] == "list-objects-v2"));
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn empty_listing(bucket: &str, prefix: &str) -> Value {
    json!({"Name":bucket,"Prefix":prefix,"KeyCount":0,"MaxKeys":1,"IsTruncated":false})
}

fn standard_empty() -> Value {
    empty_listing(BUCKET, &format!("{PREFIX}/{KEY}"))
}

fn assert_original_head_error(error: &str, stderr: &str) {
    assert!(
        error.starts_with(&format!(
            "aws head-object failed (exit status: 255): {stderr}"
        )),
        "the original HEAD failure must remain visible: {error}"
    );
    assert!(
        !error.contains("LIST-ONLY-ERROR"),
        "LIST must not replace HEAD failure: {error}"
    );
}

#[test]
fn exact_empty_listing_accepts_omitted_or_explicit_empty_arrays() {
    for explicit_contents in [false, true] {
        for explicit_prefixes in [false, true] {
            let mut reply = standard_empty();
            if explicit_contents {
                reply["Contents"] = json!([]);
            }
            if explicit_prefixes {
                reply["CommonPrefixes"] = json!([]);
            }
            let fixture = Fixture::forbidden(&reply);
            let backend = fixture.backend(BUCKET, PREFIX, None);
            assert_eq!(backend.head(KEY).unwrap(), None);
            fixture.assert_calls(&backend, KEY, true);
        }
    }
}

#[test]
fn fallback_uses_full_key_without_shell_interpolation_and_preserves_endpoint() {
    for (prefix, key) in [
        ("", KEY),
        ("prefix with spaces/'quote", "sha256/ab/$(literal)"),
    ] {
        let full = if prefix.is_empty() {
            key.into()
        } else {
            format!("{prefix}/{key}")
        };
        let fixture = Fixture::forbidden(&empty_listing(BUCKET, &full));
        let backend = fixture.backend(BUCKET, prefix, Some("http://127.0.0.1:9000/scoped fixture"));
        assert_eq!(backend.head(key).unwrap(), None);
        fixture.assert_calls(&backend, key, true);
    }
}

#[test]
fn existing_object_sibling_only_and_common_prefix_are_inconclusive() {
    for contents in [
        json!([{"Key":format!("{PREFIX}/{KEY}"),"Size":7}]),
        json!([{"Key":format!("{PREFIX}/{KEY}-sibling"),"Size":7}]),
    ] {
        let mut reply = standard_empty();
        reply["KeyCount"] = json!(1);
        reply["Contents"] = contents;
        let fixture = Fixture::forbidden(&reply);
        let backend = fixture.backend(BUCKET, PREFIX, None);
        assert_original_head_error(&backend.head(KEY).unwrap_err(), FORBIDDEN);
        fixture.assert_calls(&backend, KEY, true);
    }
    let mut reply = standard_empty();
    reply["CommonPrefixes"] = json!([{"Prefix":format!("{PREFIX}/{KEY}/")}]);
    let fixture = Fixture::forbidden(&reply);
    let backend = fixture.backend(BUCKET, PREFIX, None);
    assert_original_head_error(&backend.head(KEY).unwrap_err(), FORBIDDEN);
    fixture.assert_calls(&backend, KEY, true);
}

#[test]
fn truncated_missing_mismatched_and_wrongly_typed_responses_preserve_head_error() {
    let mutations = [
        ("Name", json!("other-bucket")),
        ("Prefix", json!(PREFIX)),
        ("Prefix", json!(format!("{PREFIX}/{KEY}/"))),
        ("Name", Value::Null),
        ("KeyCount", json!(1)),
        ("KeyCount", json!("0")),
        ("KeyCount", json!(false)),
        ("KeyCount", json!(-1)),
        ("KeyCount", json!(0.0)),
        ("MaxKeys", json!(0)),
        ("MaxKeys", json!(1000)),
        ("MaxKeys", json!("1")),
        ("IsTruncated", json!(true)),
        ("IsTruncated", json!("false")),
        ("IsTruncated", Value::Null),
        ("Contents", Value::Null),
        ("Contents", json!({})),
        ("Contents", json!([{}])),
        ("CommonPrefixes", Value::Null),
        ("CommonPrefixes", json!({})),
    ];
    for (field, value) in mutations {
        let mut reply = standard_empty();
        reply[field] = value;
        let fixture = Fixture::forbidden(&reply);
        let backend = fixture.backend(BUCKET, PREFIX, None);
        assert_original_head_error(&backend.head(KEY).unwrap_err(), FORBIDDEN);
        fixture.assert_calls(&backend, KEY, true);
    }
    for field in ["Name", "Prefix", "KeyCount", "MaxKeys", "IsTruncated"] {
        let mut reply = standard_empty();
        reply.as_object_mut().unwrap().remove(field);
        let fixture = Fixture::forbidden(&reply);
        let backend = fixture.backend(BUCKET, PREFIX, None);
        assert_original_head_error(&backend.head(KEY).unwrap_err(), FORBIDDEN);
        fixture.assert_calls(&backend, KEY, true);
    }
}

#[test]
fn pagination_and_delimiter_state_are_rejected_even_when_empty_or_null() {
    for field in [
        "ContinuationToken",
        "NextContinuationToken",
        "StartAfter",
        "Delimiter",
    ] {
        for value in [Value::Null, json!(""), json!("opaque-state")] {
            let mut reply = standard_empty();
            reply[field] = value;
            let fixture = Fixture::forbidden(&reply);
            let backend = fixture.backend(BUCKET, PREFIX, None);
            assert_original_head_error(&backend.head(KEY).unwrap_err(), FORBIDDEN);
            fixture.assert_calls(&backend, KEY, true);
        }
    }
}

#[test]
fn malformed_or_duplicate_listing_is_not_absence() {
    let duplicate = format!(
        r#"{{"Name":"{BUCKET}","Prefix":"{PREFIX}/{KEY}","KeyCount":1,"KeyCount":0,"MaxKeys":1,"IsTruncated":false}}"#
    );
    for reply in ["", "null", "[]", "{}", "not JSON", "{", duplicate.as_str()] {
        let fixture = Fixture::new(255, b"", FORBIDDEN, 0, reply.as_bytes(), "");
        let backend = fixture.backend(BUCKET, PREFIX, None);
        assert_original_head_error(&backend.head(KEY).unwrap_err(), FORBIDDEN);
        fixture.assert_calls(&backend, KEY, true);
    }
}

#[test]
fn denied_listing_cannot_turn_a_head_failure_into_absence() {
    let reply = serde_json::to_vec(&standard_empty()).unwrap();
    for stderr in [
        "LIST-ONLY-ERROR: An error occurred (AccessDenied) when calling ListObjectsV2",
        "LIST-ONLY-ERROR: connection reset",
    ] {
        let fixture = Fixture::new(255, b"", FORBIDDEN, 255, &reply, stderr);
        let backend = fixture.backend(BUCKET, PREFIX, None);
        assert_original_head_error(&backend.head(KEY).unwrap_err(), FORBIDDEN);
        fixture.assert_calls(&backend, KEY, true);
    }
}

#[test]
fn checksum_kms_like_403_preserves_error_when_object_is_listed() {
    let stderr = "An error occurred (403) when calling the HeadObject operation: AccessDenied kms:Decrypt required";
    let mut reply = standard_empty();
    reply["KeyCount"] = json!(1);
    reply["Contents"] = json!([{"Key":format!("{PREFIX}/{KEY}"),"Size":7}]);
    let fixture = Fixture::new(
        255,
        b"",
        stderr,
        0,
        &serde_json::to_vec(&reply).unwrap(),
        "",
    );
    let backend = fixture.backend(BUCKET, PREFIX, None);
    assert_original_head_error(&backend.head(KEY).unwrap_err(), stderr);
    fixture.assert_calls(&backend, KEY, true);
}

#[test]
fn successful_and_not_found_heads_do_not_list() {
    let fixture = Fixture::new(0, br#"{"ContentLength":7}"#, "", 97, b"", "unexpected LIST");
    let backend = fixture.backend(BUCKET, PREFIX, None);
    let meta = backend.head(KEY).unwrap().unwrap();
    assert_eq!(meta.bytes, 7);
    fixture.assert_calls(&backend, KEY, false);
    let stderr = "An error occurred (404) when calling the HeadObject operation: Not Found";
    let fixture = Fixture::new(255, b"", stderr, 97, b"", "unexpected LIST");
    let backend = fixture.backend(BUCKET, PREFIX, None);
    assert_eq!(backend.head(KEY).unwrap(), None);
    fixture.assert_calls(&backend, KEY, false);
}

#[test]
fn malformed_successful_head_does_not_try_listing() {
    for reply in [
        b"{}".as_slice(),
        b"null",
        b"not JSON",
        br#"{"ContentLength":7,"ChecksumSHA256":"invalid"}"#,
    ] {
        // A spurious stderr 403 must not change the successful process branch.
        let fixture = Fixture::new(0, reply, FORBIDDEN, 97, b"", "unexpected LIST");
        let backend = fixture.backend(BUCKET, PREFIX, None);
        assert!(backend.head(KEY).is_err());
        fixture.assert_calls(&backend, KEY, false);
    }
}

#[test]
fn other_head_failures_do_not_trigger_listing() {
    for stderr in [
        "Unable to locate credentials",
        "An error occurred (500) when calling the HeadObject operation: InternalError",
        "An error occurred (500) when calling the HeadObject operation: Forbidden by gateway",
        "Forbidden",
        "An error occurred (AccessDenied) when calling the HeadObject operation: Access Denied",
        "Could not connect to endpoint",
    ] {
        let fixture = Fixture::new(255, b"", stderr, 97, b"", "unexpected LIST");
        let backend = fixture.backend(BUCKET, PREFIX, None);
        assert_original_head_error(&backend.head(KEY).unwrap_err(), stderr);
        fixture.assert_calls(&backend, KEY, false);
    }
}

#[test]
fn directory_buckets_and_empty_keys_never_list() {
    for (bucket, prefix, key) in [
        ("example--use2-az1--x-s3", PREFIX, KEY),
        (BUCKET, "", ""),
        (BUCKET, PREFIX, ""),
    ] {
        let fixture = Fixture::forbidden(&standard_empty());
        let backend = fixture.backend(bucket, prefix, None);
        assert_original_head_error(&backend.head(key).unwrap_err(), FORBIDDEN);
        fixture.assert_calls(&backend, key, false);
    }
}
