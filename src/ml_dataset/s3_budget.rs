//! Native, admitted S3 transport for dataset workers only.
//!
//! Every SDK send has one attempt (`max_attempts(1)`); the bounded retry loop
//! creates a new durable debit. The final pre-transmit interceptor checks the
//! actual method, endpoint, key, conditional header and credential deadline.
//! The explicitly configured Smithy connector disables pooling: Hyper's
//! retry-on-cancel path requires a reused connection, so it cannot reissue a
//! request below the admission hook. There is no CLI, LIST, multipart, proxy,
//! endpoint discovery, default credential chain, IMDS or S3 Express route.
//!
//! Primary implementation references (reviewed 2026-10-28):
//! https://docs.aws.amazon.com/sdk-for-rust/latest/dg/interceptors.html
//! https://docs.rs/aws-smithy-runtime-api/1.19.2/aws_smithy_runtime_api/client/interceptors/trait.Intercept.html
//! https://docs.rs/aws-smithy-http-client/1.5.2/src/aws_smithy_http_client/client.rs.html
//! https://docs.rs/hyper-util/latest/src/hyper_util/client/legacy/client.rs.html
//!
//! The byte ceiling is an admitted object/response-body ceiling, not a claim
//! about TLS overhead, DNS traffic, kernel buffers, or account-wide billing.
//! Remote activation fences accidental reuse of a lost/recreated journal;
//! copied live SQLite state or a malicious caller requires external fencing.

use std::collections::BTreeMap;
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use aws_credential_types::Credentials;
use aws_sdk_s3::config::{BehaviorVersion, Region, RequestChecksumCalculation};
use aws_sdk_s3::primitives::ByteStream;
use aws_smithy_runtime_api::box_error::BoxError;
use aws_smithy_runtime_api::client::http::SharedHttpClient;
use aws_smithy_runtime_api::client::interceptors::context::{
    BeforeDeserializationInterceptorContextRef, BeforeTransmitInterceptorContextMut,
};
use aws_smithy_runtime_api::client::interceptors::Intercept;
use aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
use aws_smithy_types::config_bag::ConfigBag;
use aws_smithy_types::retry::RetryConfig;
use aws_smithy_types::timeout::TimeoutConfig;
use base64::Engine;
use tokio::io::AsyncReadExt;

use super::budget::{
    AttemptOutcome, AttemptTicket, BudgetScope, BudgetSession, HttpMethod, HttpOperation,
};
use crate::hash::sha256::{sha256, Sha256};
use crate::store::backend::{Backend, Meta, RemoteSpec, SINGLE_PUT_MAX};
use crate::store::{Config, Store};

const REGION: &str = "us-east-2";
const S3_ENDPOINT: &str = "https://s3.us-east-2.amazonaws.com";
const STS_ENDPOINT: &str = "https://sts.us-east-2.amazonaws.com";
const STS_BODY_LIMIT: u64 = 65_536;
const ATTEMPTS: usize = 3;
pub(super) const AMBIGUOUS_HEAD_DENIED: &str = "dataset-budget-head-denied-403";
static OPERATION_SERIAL: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Debug, PartialEq, Eq)]
struct ExpectedObject {
    sha256: String,
    bytes: u64,
}

struct CredentialsState {
    credentials: Credentials,
    expiration: Option<SystemTime>,
}

enum CredentialSource {
    Static,
    WebIdentity {
        role_arn: String,
        token_file: PathBuf,
    },
}

/// The context is deliberately opaque: callers cannot obtain an unmetered SDK
/// client or replace its endpoint/connector. Store adapters share this context.
pub struct S3BudgetContext {
    session: Arc<BudgetSession>,
    expected: Mutex<BTreeMap<(String, String), ExpectedObject>>,
    runtime: tokio::runtime::Runtime,
    http_client: SharedHttpClient,
    credentials: Mutex<Option<CredentialsState>>,
    source: CredentialSource,
}

impl S3BudgetContext {
    #[cfg(test)]
    pub(super) fn test_context(
        session: Arc<BudgetSession>,
        http_client: SharedHttpClient,
    ) -> Arc<Self> {
        Arc::new(Self {
            session,
            expected: Mutex::new(BTreeMap::new()),
            runtime: tokio::runtime::Builder::new_multi_thread()
                .worker_threads(1)
                .enable_all()
                .build()
                .unwrap(),
            http_client,
            credentials: Mutex::new(Some(CredentialsState {
                credentials: Credentials::new(
                    "offline-key",
                    "offline-secret",
                    None,
                    None,
                    "offline-test",
                ),
                expiration: None,
            })),
            source: CredentialSource::Static,
        })
    }
    /// Read explicit static credentials or the explicitly admitted IRSA pair.
    /// No credential contents or token files are printed or persisted.
    pub fn from_env(session: Arc<BudgetSession>) -> Result<Arc<Self>, String> {
        for name in [
            "HTTP_PROXY",
            "HTTPS_PROXY",
            "ALL_PROXY",
            "http_proxy",
            "https_proxy",
            "all_proxy",
            "AWS_ENDPOINT_URL",
            "AWS_ENDPOINT_URL_S3",
            "AWS_ENDPOINT_URL_STS",
        ] {
            if std::env::var_os(name).is_some_and(|v| !v.is_empty()) {
                return Err(format!("budgeted transport rejects {name}; endpoint/proxy traffic must not bypass admission"));
            }
        }
        let static_key = std::env::var("AWS_ACCESS_KEY_ID").ok();
        let static_secret = std::env::var("AWS_SECRET_ACCESS_KEY").ok();
        let role = std::env::var("AWS_ROLE_ARN").ok();
        let token_file = std::env::var_os("AWS_WEB_IDENTITY_TOKEN_FILE");
        let (source, credentials) = match (static_key, static_secret, role, token_file) {
            (Some(key), Some(secret), None, None) if !key.is_empty() && !secret.is_empty() => {
                if session.grant().request.credential_role_arn.is_some() {
                    return Err("this grant requires its bound IRSA role; static credentials are not admitted".into());
                }
                let token = std::env::var("AWS_SESSION_TOKEN").ok();
                let expiration = std::env::var("CRYPTO_DATASET_AWS_EXPIRATION_UNIX").ok()
                    .map(|v| v.parse::<u64>().map_err(|_| "invalid explicit credential expiration".to_string()))
                    .transpose()?.map(|v| UNIX_EPOCH.checked_add(Duration::from_secs(v)).ok_or("credential expiration overflow"))
                    .transpose()?;
                if token.is_some() && expiration.is_none() {
                    return Err("temporary static credentials require CRYPTO_DATASET_AWS_EXPIRATION_UNIX".into());
                }
                (CredentialSource::Static, Some(CredentialsState { credentials:Credentials::new(key,secret,token,expiration,"dataset-explicit"),expiration }))
            }
            (None, None, Some(role_arn), Some(token_file)) => {
                if session.grant().request.credential_role_arn.as_deref() != Some(role_arn.as_str()) {
                    return Err("IRSA role differs from the explicit budget grant".into());
                }
                (CredentialSource::WebIdentity { role_arn, token_file:token_file.into() }, None)
            }
            _ => return Err("provide exactly one complete explicit static credential tuple or admitted IRSA role/token pair".into()),
        };
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .map_err(|_| "create budget transport runtime")?;
        let http_client = aws_smithy_http_client::Builder::new()
            .pool_max_idle_per_host(0)
            .tls_provider(aws_smithy_http_client::tls::Provider::Rustls(
                aws_smithy_http_client::tls::rustls_provider::CryptoMode::Ring,
            ))
            .build_https();
        Ok(Arc::new(Self {
            session,
            expected: Mutex::new(BTreeMap::new()),
            runtime,
            http_client,
            credentials: Mutex::new(credentials),
            source,
        }))
    }

    pub fn session(&self) -> &Arc<BudgetSession> {
        &self.session
    }

    /// Publication must call this even when Store::push skipped an existing
    /// size-matching object. HEAD metadata alone cannot prove stored content.
    pub fn verify_registered(
        &self,
        bucket: &str,
        key: &str,
        scope: BudgetScope,
    ) -> Result<(), String> {
        let expected = self.expected(bucket, key)?;
        let reservation =
            self.session
                .reserve_object(bucket, key, &expected.sha256, expected.bytes, &scope)?;
        let ticket = self.get_body(bucket, key, &expected, &scope, &mut std::io::sink())?;
        self.session
            .verify_object(&reservation, &ticket, &expected.sha256, expected.bytes)
    }

    /// Bind exact object content before any HEAD/GET. Scope belongs to each
    /// Store adapter, so identical CAS objects can be reused by different cells.
    pub fn expect_object(
        &self,
        bucket: &str,
        key: &str,
        digest: &str,
        bytes: u64,
        _scope: BudgetScope,
    ) -> Result<(), String> {
        self.session.validate_remote(bucket, key)?;
        validate_key(key)?;
        crate::store::index::validate_sha256(digest)?;
        if bytes > SINGLE_PUT_MAX {
            return Err("budgeted single-object limit exceeded".into());
        }
        let expected = ExpectedObject {
            sha256: digest.into(),
            bytes,
        };
        let mut objects = self
            .expected
            .lock()
            .map_err(|_| "poisoned expected-object registry")?;
        match objects.get(&(bucket.into(), key.into())) {
            Some(previous) if previous != &expected => {
                Err("immutable object registration changed hash or length".into())
            }
            _ => {
                objects.insert((bucket.into(), key.into()), expected);
                Ok(())
            }
        }
    }

    fn expected(&self, bucket: &str, key: &str) -> Result<ExpectedObject, String> {
        self.session.validate_remote(bucket, key)?;
        self.expected
            .lock()
            .map_err(|_| "poisoned expected-object registry")?
            .get(&(bucket.into(), key.into()))
            .cloned()
            .ok_or_else(|| "register exact object digest and length before HEAD/GET".into())
    }

    fn credentials(&self) -> Result<(Credentials, Option<SystemTime>), String> {
        let mut cached = self
            .credentials
            .lock()
            .map_err(|_| "poisoned credential state")?;
        let refresh = cached.as_ref().is_none_or(|c| {
            c.expiration
                .is_some_and(|t| t <= SystemTime::now() + Duration::from_secs(60))
        });
        if refresh {
            match &self.source {
                CredentialSource::Static => {
                    return Err(
                        "explicit credentials expired or are within the renewal margin".into(),
                    )
                }
                CredentialSource::WebIdentity {
                    role_arn,
                    token_file,
                } => {
                    *cached = Some(self.refresh_irsa(role_arn, token_file)?);
                }
            }
        }
        let value = cached.as_ref().ok_or("credentials unavailable")?;
        Ok((value.credentials.clone(), value.expiration))
    }

    fn refresh_irsa(&self, role_arn: &str, token_file: &Path) -> Result<CredentialsState, String> {
        let mut bytes = Vec::new();
        fs::File::open(token_file)
            .map_err(|_| "cannot open IRSA token file")?
            .take(STS_BODY_LIMIT + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| "cannot read IRSA token file")?;
        if bytes.is_empty() || bytes.len() as u64 > STS_BODY_LIMIT {
            return Err("IRSA token exceeds bounded input".into());
        }
        let token = String::from_utf8(bytes).map_err(|_| "IRSA token is not UTF-8")?;
        let client = aws_sdk_sts::Client::from_conf(
            aws_sdk_sts::Config::builder()
                .behavior_version(BehaviorVersion::latest())
                .region(Region::new(REGION))
                .endpoint_url(STS_ENDPOINT)
                .http_client(self.http_client.clone())
                .retry_config(RetryConfig::standard().with_max_attempts(1))
                .timeout_config(timeouts())
                .build(),
        );
        let operation = HttpOperation {
            operation_id: operation_id(),
            method: HttpMethod::StsWebIdentity,
            bucket: "sts.us-east-2.amazonaws.com".into(),
            key: hex::encode(sha256(role_arn.as_bytes())),
            expected_sha256: None,
            expected_bytes: STS_BODY_LIMIT,
            scope: BudgetScope::Finalization,
        };
        for attempt in 0..ATTEMPTS {
            let gate = AttemptGate::new(self.session.clone(), operation.clone(), None, None);
            let result = self.runtime.block_on(
                client
                    .assume_role_with_web_identity()
                    .role_arn(role_arn)
                    .role_session_name("crypto-dataset-budgeted")
                    .duration_seconds(900)
                    .web_identity_token(&token)
                    .customize()
                    .interceptor(gate.clone())
                    .send(),
            );
            if let Ok(output) = result {
                let value = output
                    .credentials()
                    .ok_or("STS response omitted credentials")?;
                let seconds = u64::try_from(value.expiration().secs())
                    .map_err(|_| "STS credential expiration precedes epoch")?;
                let expiration = UNIX_EPOCH
                    .checked_add(Duration::from_secs(seconds))
                    .ok_or("STS expiration overflow")?;
                if expiration <= SystemTime::now() + Duration::from_secs(60) {
                    gate.finish(0, Some("STS returned expired credentials"))?;
                    return Err("STS returned credentials outside usable lifetime".into());
                }
                let observed = gate
                    .content_length()?
                    .ok_or("STS response omitted its admitted content length")?;
                gate.finish(observed, None)?;
                return Ok(CredentialsState {
                    credentials: Credentials::new(
                        value.access_key_id(),
                        value.secret_access_key(),
                        Some(value.session_token().into()),
                        Some(expiration),
                        "dataset-admitted-irsa",
                    ),
                    expiration: Some(expiration),
                });
            }
            let retry = gate.retryable()?;
            gate.finish(0, Some("STS request failed; token and response omitted"))?;
            if !retry || attempt + 1 == ATTEMPTS {
                return Err(gate.failure("admitted STS request")?);
            }
        }
        Err("STS attempt limit reached".into())
    }

    fn client(&self) -> Result<(aws_sdk_s3::Client, Option<SystemTime>), String> {
        let (credentials, expiration) = self.credentials()?;
        let config = aws_sdk_s3::Config::builder()
            .behavior_version(BehaviorVersion::latest())
            .region(Region::new(REGION))
            .endpoint_url(S3_ENDPOINT)
            .force_path_style(true)
            .disable_multi_region_access_points(true)
            .disable_s3_express_session_auth(true)
            .http_client(self.http_client.clone())
            .credentials_provider(credentials)
            .request_checksum_calculation(RequestChecksumCalculation::WhenRequired)
            .retry_config(RetryConfig::standard().with_max_attempts(1))
            .timeout_config(timeouts())
            .build();
        Ok((aws_sdk_s3::Client::from_conf(config), expiration))
    }

    /// Only the explicit activation command calls this. A conflicting remote
    /// nonce is checked by a fresh full GET and cannot activate a recreated DB.
    pub fn activate(&self) -> Result<(), String> {
        let payload = self.session.activation_payload()?;
        let digest = hex::encode(sha256(&payload));
        let (bucket, key) = self.session.activation_location();
        let expected = ExpectedObject {
            sha256: digest.clone(),
            bytes: payload.len() as u64,
        };
        self.expect_object(
            bucket,
            key,
            &digest,
            expected.bytes,
            BudgetScope::Finalization,
        )?;
        let reservation = self.session.reserve_object(
            bucket,
            key,
            &digest,
            expected.bytes,
            &BudgetScope::Finalization,
        )?;
        self.put_body(
            bucket,
            key,
            &expected,
            &BudgetScope::Finalization,
            PutSource::Bytes(payload),
        )?;
        let ticket = self.get_body(
            bucket,
            key,
            &expected,
            &BudgetScope::Finalization,
            &mut std::io::sink(),
        )?;
        self.session
            .verify_object(&reservation, &ticket, &digest, expected.bytes)?;
        self.session
            .confirm_activation(&ticket, &digest, expected.bytes)
    }

    fn operation(
        &self,
        method: HttpMethod,
        bucket: &str,
        key: &str,
        expected: &ExpectedObject,
        scope: &BudgetScope,
    ) -> HttpOperation {
        HttpOperation {
            operation_id: operation_id(),
            method,
            bucket: bucket.into(),
            key: key.into(),
            expected_sha256: Some(expected.sha256.clone()),
            expected_bytes: if method == HttpMethod::Head {
                0
            } else {
                expected.bytes
            },
            scope: scope.clone(),
        }
    }

    fn head_object(
        &self,
        bucket: &str,
        key: &str,
        expected: &ExpectedObject,
        scope: &BudgetScope,
    ) -> Result<Option<Meta>, String> {
        let (client, expiration) = self.client()?;
        let operation = self.operation(HttpMethod::Head, bucket, key, expected, scope);
        for attempt in 0..ATTEMPTS {
            let gate = AttemptGate::new(
                self.session.clone(),
                operation.clone(),
                expiration,
                Some(expected.bytes),
            );
            let result = self.runtime.block_on(
                client
                    .head_object()
                    .bucket(bucket)
                    .key(key)
                    .customize()
                    .interceptor(gate.clone())
                    .send(),
            );
            if let Ok(output) = result {
                let length = output.content_length().and_then(|v| u64::try_from(v).ok());
                if length != Some(expected.bytes) {
                    gate.finish(0, Some("HEAD length differs from registered object"))?;
                    return Err("HEAD length differs from registered object".into());
                }
                let mut reported_sha256 = None;
                if let Some(checksum) = output.checksum_sha256() {
                    if let Err(error) = verify_checksum_header(checksum, &expected.sha256) {
                        gate.finish(0, Some(&error))?;
                        return Err(error);
                    }
                    reported_sha256 = Some(expected.sha256.clone());
                }
                if let Some(value) = output.metadata().and_then(|v| v.get("sha256")) {
                    if value != &expected.sha256 {
                        gate.finish(0, Some("HEAD digest metadata differs"))?;
                        return Err("HEAD digest metadata differs".into());
                    }
                    reported_sha256 = Some(value.clone());
                }
                gate.finish(0, None)?;
                return Ok(Some(Meta {
                    bytes: expected.bytes,
                    sha256: reported_sha256,
                }));
            }
            let status = gate.status()?;
            let retry = gate.retryable()?;
            gate.finish(0, Some("HEAD request failed"))?;
            if status == Some(404) {
                return Ok(None);
            }
            if status == Some(403) {
                // No LIST absence probe: an independently charged GET can prove
                // presence only. A denied GET remains an error, never absence.
                if let Err(error) =
                    self.get_body(bucket, key, expected, scope, &mut std::io::sink())
                {
                    if error.contains("HTTP status Some(403)")
                        || error.contains("HTTP status Some(404)")
                    {
                        return Err(format!("{AMBIGUOUS_HEAD_DENIED}: presence remains unknown after charged GET; immutable conditional PUT and full verification are required"));
                    }
                    return Err(error);
                }
                return Ok(Some(Meta {
                    bytes: expected.bytes,
                    sha256: Some(expected.sha256.clone()),
                }));
            }
            if !retry || attempt + 1 == ATTEMPTS {
                return Err(gate.failure("HEAD")?);
            }
        }
        Err("HEAD attempt limit reached".into())
    }

    fn put_body(
        &self,
        bucket: &str,
        key: &str,
        expected: &ExpectedObject,
        scope: &BudgetScope,
        source: PutSource,
    ) -> Result<(), String> {
        let (client, expiration) = self.client()?;
        let operation = self.operation(HttpMethod::Put, bucket, key, expected, scope);
        let checksum = crate::store::backend::hex_to_b64(&expected.sha256)?;
        for attempt in 0..ATTEMPTS {
            let gate =
                AttemptGate::new(self.session.clone(), operation.clone(), expiration, Some(0));
            let body = match &source {
                PutSource::Bytes(bytes) => ByteStream::from(bytes.clone()),
                PutSource::File(path) => self
                    .runtime
                    .block_on(ByteStream::from_path(path))
                    .map_err(|_| "open bounded PUT source")?,
            };
            let result = self.runtime.block_on(
                client
                    .put_object()
                    .bucket(bucket)
                    .key(key)
                    .if_none_match("*")
                    .content_length(expected.bytes as i64)
                    .checksum_sha256(&checksum)
                    .metadata("sha256", &expected.sha256)
                    .body(body)
                    .customize()
                    .interceptor(gate.clone())
                    .send(),
            );
            if result.is_ok() {
                gate.finish(0, None)?;
                return Ok(());
            }
            let status = gate.status()?;
            let retry = gate.retryable()?;
            gate.finish(0, Some("conditional PUT did not confirm creation"))?;
            // A 412 never establishes equality. The caller must perform a new
            // full GET against the registered digest before returning success.
            if status == Some(412) {
                return Ok(());
            }
            if !retry || attempt + 1 == ATTEMPTS {
                return Err(gate.failure("conditional PUT")?);
            }
        }
        Err("PUT attempt limit reached".into())
    }

    fn get_body(
        &self,
        bucket: &str,
        key: &str,
        expected: &ExpectedObject,
        scope: &BudgetScope,
        dest: &mut dyn Write,
    ) -> Result<AttemptTicket, String> {
        let (client, expiration) = self.client()?;
        let operation = self.operation(HttpMethod::Get, bucket, key, expected, scope);
        for attempt in 0..ATTEMPTS {
            let gate = AttemptGate::new(
                self.session.clone(),
                operation.clone(),
                expiration,
                Some(expected.bytes),
            );
            let response = self.runtime.block_on(
                client
                    .get_object()
                    .bucket(bucket)
                    .key(key)
                    .customize()
                    .interceptor(gate.clone())
                    .send(),
            );
            if let Ok(output) = response {
                let mut reader = output.body.into_async_read();
                let mut hash = Sha256::new();
                let mut observed = 0_u64;
                let result = self
                    .runtime
                    .block_on(async {
                        tokio::time::timeout(Duration::from_secs(300), async {
                            let mut buffer = [0_u8; 65_536];
                            loop {
                                // Content-Length was checked before SDK deserialization;
                                // never request more than the admitted remaining bytes.
                                if observed == expected.bytes {
                                    break;
                                }
                                let want =
                                    (expected.bytes - observed).min(buffer.len() as u64) as usize;
                                let n = reader
                                    .read(&mut buffer[..want])
                                    .await
                                    .map_err(|_| "GET body transport failed")?;
                                if n == 0 {
                                    return Err("GET body shorter than registered length");
                                }
                                observed += n as u64;
                                hash.update(&buffer[..n]);
                                dest.write_all(&buffer[..n])
                                    .map_err(|_| "write verified GET destination")?;
                            }
                            let mut end = [0_u8; 1];
                            if reader
                                .read(&mut end)
                                .await
                                .map_err(|_| "GET body completion failed")?
                                != 0
                            {
                                return Err("GET body exceeded registered length");
                            }
                            if hex::encode(hash.finalize()) != expected.sha256 {
                                return Err("GET body SHA-256 differs from registered digest");
                            }
                            Ok(())
                        })
                        .await
                    })
                    .unwrap_or(Err("GET body exceeded its wall-clock timeout"));
                match result {
                    Ok(()) => {
                        return gate
                            .finish(observed, None)?
                            .ok_or_else(|| "GET was never durably admitted".into())
                    }
                    Err(error) => {
                        gate.finish(observed, Some(error))?;
                        return Err(error.into());
                    }
                }
            }
            let retry = gate.retryable()?;
            gate.finish(0, Some("GET request failed before body verification"))?;
            if !retry || attempt + 1 == ATTEMPTS {
                return Err(gate.failure("GET")?);
            }
        }
        Err("GET attempt limit reached".into())
    }
}

enum PutSource {
    Bytes(Vec<u8>),
    File(PathBuf),
}

/// Construct one scope-bound adapter. The same context can serve different
/// authorized publication prefixes without an ambient thread-local budget.
pub fn open_store(
    cfg: Config,
    context: Arc<S3BudgetContext>,
    scope: BudgetScope,
) -> Result<Store, String> {
    if cfg.endpoint.is_some() {
        return Err("budgeted S3 refuses endpoint overrides".into());
    }
    let Some(RemoteSpec::S3 { bucket, prefix }) = &cfg.remote else {
        return Err("native budgeted S3 requires an S3 remote".into());
    };
    let backend = BudgetedS3Backend {
        context,
        bucket: bucket.clone(),
        prefix: prefix.clone(),
        scope,
    };
    Store::open_with_backend(cfg, Box::new(backend))
}

struct BudgetedS3Backend {
    context: Arc<S3BudgetContext>,
    bucket: String,
    prefix: String,
    scope: BudgetScope,
}

impl BudgetedS3Backend {
    fn key(&self, key: &str) -> Result<String, String> {
        validate_key(key)?;
        let key = if self.prefix.is_empty() {
            key.into()
        } else {
            format!("{}/{key}", self.prefix)
        };
        self.context.session.validate_remote(&self.bucket, &key)?;
        Ok(key)
    }
}

impl Backend for BudgetedS3Backend {
    fn describe(&self) -> String {
        format!("budgeted native s3://{}/{}", self.bucket, self.prefix)
    }
    fn head(&self, key: &str) -> Result<Option<Meta>, String> {
        let key = self.key(key)?;
        let expected = self.context.expected(&self.bucket, &key)?;
        let mut meta = self
            .context
            .head_object(&self.bucket, &key, &expected, &self.scope)?;
        if let Some(meta) = meta.as_mut() {
            if meta.sha256.is_none() {
                // HEAD may omit digest metadata. Establish it from a bounded,
                // charged full GET before satisfying Store's digest contract.
                self.context.get_body(
                    &self.bucket,
                    &key,
                    &expected,
                    &self.scope,
                    &mut std::io::sink(),
                )?;
                meta.sha256 = Some(expected.sha256.clone());
            }
        }
        Ok(meta)
    }
    fn put(&self, key: &str, src: &Path, digest: &str) -> Result<bool, String> {
        let key = self.key(key)?;
        let (actual, bytes) = crate::store::hash_file(src)?;
        if actual != digest {
            return Err("PUT source differs from its declared SHA-256".into());
        }
        self.context
            .expect_object(&self.bucket, &key, digest, bytes, self.scope.clone())?;
        let expected = self.context.expected(&self.bucket, &key)?;
        let reservation =
            self.context
                .session
                .reserve_object(&self.bucket, &key, digest, bytes, &self.scope)?;
        self.context.put_body(
            &self.bucket,
            &key,
            &expected,
            &self.scope,
            PutSource::File(src.into()),
        )?;
        let ticket = self.context.get_body(
            &self.bucket,
            &key,
            &expected,
            &self.scope,
            &mut std::io::sink(),
        )?;
        self.context
            .session
            .verify_object(&reservation, &ticket, digest, bytes)?;
        // The budgeted transport writes unconditionally and verifies the bytes
        // it read back; it does not distinguish a writer that won a race, so it
        // reports that this call performed the store.
        Ok(true)
    }
    fn get(&self, key: &str, dest: &Path) -> Result<(), String> {
        let key = self.key(key)?;
        let expected = self.context.expected(&self.bucket, &key)?;
        let (mut file, created) = empty_destination(dest)?;
        let result = self
            .context
            .get_body(&self.bucket, &key, &expected, &self.scope, &mut file)
            .and_then(|_| {
                file.sync_all()
                    .map_err(|e| format!("sync GET destination: {e}"))
            });
        if result.is_err() {
            if created {
                let _ = fs::remove_file(dest);
            } else {
                let _ = file.set_len(0);
            }
        }
        drop(file);
        result
    }
    fn list(&self, _prefix: &str) -> Result<Vec<(String, u64)>, String> {
        Err("LIST is disabled by the finite dataset request policy".into())
    }
}

#[derive(Default, Debug)]
struct AttemptState {
    ticket: Option<AttemptTicket>,
    status: Option<u16>,
    request_id: Option<String>,
    content_length: Option<u64>,
    denied: Option<String>,
    finished: bool,
}

#[derive(Clone)]
struct AttemptGate {
    session: Arc<BudgetSession>,
    operation: HttpOperation,
    expiration: Option<SystemTime>,
    response_length: Option<u64>,
    state: Arc<Mutex<AttemptState>>,
}

impl std::fmt::Debug for AttemptGate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AttemptGate")
            .field("operation", &self.operation)
            .finish_non_exhaustive()
    }
}

impl AttemptGate {
    fn new(
        session: Arc<BudgetSession>,
        operation: HttpOperation,
        expiration: Option<SystemTime>,
        response_length: Option<u64>,
    ) -> Self {
        Self {
            session,
            operation,
            expiration,
            response_length,
            state: Arc::new(Mutex::new(AttemptState::default())),
        }
    }
    fn status(&self) -> Result<Option<u16>, String> {
        Ok(self
            .state
            .lock()
            .map_err(|_| "poisoned HTTP admission")?
            .status)
    }
    fn content_length(&self) -> Result<Option<u64>, String> {
        Ok(self
            .state
            .lock()
            .map_err(|_| "poisoned HTTP admission")?
            .content_length)
    }
    fn retryable(&self) -> Result<bool, String> {
        let state = self.state.lock().map_err(|_| "poisoned HTTP admission")?;
        Ok(state.ticket.is_some()
            && state.denied.is_none()
            && (state.status.is_none()
                || state
                    .status
                    .is_some_and(|s| matches!(s, 408 | 429 | 500 | 502 | 503 | 504))))
    }
    fn finish(&self, observed: u64, error: Option<&str>) -> Result<Option<AttemptTicket>, String> {
        let mut state = self.state.lock().map_err(|_| "poisoned HTTP admission")?;
        let ticket = state.ticket.clone();
        if let Some(ticket) = &ticket {
            if state.finished {
                return Err("HTTP attempt outcome already finalized".into());
            }
            self.session.finish_attempt(
                ticket,
                &AttemptOutcome {
                    status: state.status,
                    request_id: state.request_id.clone(),
                    observed_bytes: observed,
                    error: error.map(str::to_owned).or_else(|| state.denied.clone()),
                },
            )?;
            state.finished = true;
        }
        Ok(ticket)
    }
    fn failure(&self, label: &str) -> Result<String, String> {
        let state = self.state.lock().map_err(|_| "poisoned HTTP admission")?;
        Ok(if let Some(reason) = &state.denied {
            format!("{label} denied before completion: {reason}")
        } else {
            format!(
                "{label} failed (HTTP status {:?}; ambiguous attempts remain charged)",
                state.status
            )
        })
    }
}

impl Intercept for AttemptGate {
    fn name(&self) -> &'static str {
        "DatasetDurableAttemptAdmission"
    }
    fn modify_before_transmit(
        &self,
        context: &mut BeforeTransmitInterceptorContextMut<'_>,
        _runtime: &RuntimeComponents,
        _cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        let mut state = self.state.lock().map_err(|_| "poisoned HTTP admission")?;
        let validate = || -> Result<(), String> {
            if state.ticket.is_some() {
                return Err("SDK attempted an unplanned internal retry".into());
            }
            if self
                .expiration
                .is_some_and(|v| v <= SystemTime::now() + Duration::from_secs(5))
            {
                return Err("credential lifetime ended before transmit".into());
            }
            let request = context.request();
            let (method, url) = if self.operation.method == HttpMethod::StsWebIdentity {
                ("POST", format!("{STS_ENDPOINT}/"))
            } else {
                let method = match self.operation.method {
                    HttpMethod::Head => "HEAD",
                    HttpMethod::Get => "GET",
                    HttpMethod::Put => "PUT",
                    HttpMethod::StsWebIdentity => unreachable!(),
                };
                (
                    method,
                    format!(
                        "{S3_ENDPOINT}/{}/{}",
                        self.operation.bucket,
                        encode_key(&self.operation.key)
                    ),
                )
            };
            let (actual, query) = request.uri().split_once('?').unwrap_or((request.uri(), ""));
            let expected_query = match self.operation.method {
                HttpMethod::Head => "x-id=HeadObject",
                HttpMethod::Get => "x-id=GetObject",
                HttpMethod::Put => "x-id=PutObject",
                HttpMethod::StsWebIdentity => "",
            };
            if request.method() != method
                || actual != url
                || (!query.is_empty() && query != expected_query)
            {
                return Err(
                    "serialized HTTP request escaped its admitted endpoint/method/key".into(),
                );
            }
            if self.operation.method == HttpMethod::Put
                && request.headers().get("if-none-match") != Some("*")
            {
                return Err("immutable PUT conditional header missing".into());
            }
            Ok(())
        };
        if let Err(error) = validate() {
            state.denied = Some(error.clone());
            return Err(error.into());
        }
        match self.session.before_http_attempt(&self.operation) {
            Ok(ticket) => {
                state.ticket = Some(ticket);
                Ok(())
            }
            Err(error) => {
                state.denied = Some(error.clone());
                Err(error.into())
            }
        }
    }
    fn read_after_transmit(
        &self,
        context: &BeforeDeserializationInterceptorContextRef<'_>,
        _runtime: &RuntimeComponents,
        _cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        let mut state = self.state.lock().map_err(|_| "poisoned HTTP admission")?;
        let response = context.response();
        let status = response.status().as_u16();
        state.status = Some(status);
        state.request_id = response
            .headers()
            .get("x-amz-request-id")
            .or_else(|| response.headers().get("x-amzn-requestid"))
            .map(|v| v.chars().take(512).collect());
        state.content_length = response
            .headers()
            .get("content-length")
            .and_then(|v| v.parse::<u64>().ok());
        // Do not consume arbitrary XML error bodies. Retry decisions use the
        // saved status, and each manual retry obtains another durable ticket.
        if !(200..300).contains(&status) {
            return Err("HTTP status rejected before response body consumption".into());
        }
        let violation = if self.operation.method == HttpMethod::StsWebIdentity {
            state.content_length.is_none_or(|n| n > STS_BODY_LIMIT)
        } else if let Some(length) = self.response_length {
            state.content_length != Some(length)
        } else {
            false
        };
        if violation {
            state.denied =
                Some("response content length exceeds or differs from admitted bound".into());
            return Err("response content length violates admitted bound".into());
        }
        Ok(())
    }
}

fn validate_key(key: &str) -> Result<(), String> {
    if key.is_empty()
        || key.len() > 1024
        || key.starts_with('/')
        || key.ends_with('/')
        || key.contains("//")
        || key.split('/').any(|v| v == "." || v == "..")
        || key.bytes().any(|v| v.is_ascii_control() || v == b'\\')
    {
        return Err("unsafe exact S3 object key".into());
    }
    Ok(())
}

fn empty_destination(path: &Path) -> Result<(fs::File, bool), String> {
    let mut fresh = OpenOptions::new();
    fresh.create_new(true).write(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        fresh.custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC);
    }
    match fresh.open(path) {
        Ok(file) => Ok((file, true)),
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            let metadata =
                fs::symlink_metadata(path).map_err(|e| format!("inspect GET destination: {e}"))?;
            if !metadata.is_file() || metadata.len() != 0 {
                return Err("GET destination must be absent or an empty regular file".into());
            }
            let mut existing = OpenOptions::new();
            existing.write(true);
            #[cfg(unix)]
            {
                use std::os::unix::fs::OpenOptionsExt;
                existing.custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC);
            }
            let file = existing
                .open(path)
                .map_err(|e| format!("open empty GET destination: {e}"))?;
            let metadata = file
                .metadata()
                .map_err(|e| format!("inspect opened GET destination: {e}"))?;
            if !metadata.is_file() || metadata.len() != 0 {
                return Err("GET destination changed or is nonempty".into());
            }
            Ok((file, false))
        }
        Err(error) => Err(format!("create GET destination: {error}")),
    }
}

fn encode_key(key: &str) -> String {
    let mut encoded = String::new();
    for byte in key.bytes() {
        if byte.is_ascii_alphanumeric() || b"-_.~/".contains(&byte) {
            encoded.push(byte as char);
        } else {
            use std::fmt::Write as _;
            let _ = write!(encoded, "%{byte:02X}");
        }
    }
    encoded
}

fn verify_checksum_header(value: &str, digest: &str) -> Result<(), String> {
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(value)
        .map_err(|_| "invalid remote checksum header")?;
    if hex::encode(bytes) != digest {
        return Err("remote checksum differs from registered digest".into());
    }
    Ok(())
}

fn operation_id() -> String {
    format!(
        "http-{}-{}",
        std::process::id(),
        OPERATION_SERIAL.fetch_add(1, Ordering::Relaxed)
    )
}

fn timeouts() -> TimeoutConfig {
    TimeoutConfig::builder()
        .connect_timeout(Duration::from_secs(15))
        .read_timeout(Duration::from_secs(60))
        .operation_timeout(Duration::from_secs(300))
        .operation_attempt_timeout(Duration::from_secs(300))
        .build()
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::super::budget::{CampaignLedger, RemoteScope, Resources};
    use super::*;
    use aws_smithy_runtime_api::client::http::{
        HttpClient, HttpConnector, HttpConnectorFuture, HttpConnectorSettings, SharedHttpConnector,
    };
    use aws_smithy_runtime_api::client::orchestrator::{HttpRequest, HttpResponse};
    use aws_smithy_runtime_api::client::result::ConnectorError;
    use aws_smithy_types::body::SdkBody;
    use std::collections::VecDeque;

    enum Reply {
        Http {
            method: &'static str,
            status: u16,
            body: Vec<u8>,
            length: u64,
        },
        Ambiguous {
            method: &'static str,
        },
    }
    #[derive(Clone)]
    struct FakeWire {
        session: Arc<BudgetSession>,
        replies: Arc<Mutex<VecDeque<Reply>>>,
        calls: Arc<AtomicU64>,
    }
    impl std::fmt::Debug for FakeWire {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("FakeWire(no network)")
        }
    }
    impl FakeWire {
        fn push(&self, method: &'static str, status: u16, body: &[u8]) {
            self.replies.lock().unwrap().push_back(Reply::Http {
                method,
                status,
                body: body.into(),
                length: body.len() as u64,
            });
        }
        fn calls(&self) -> u64 {
            self.calls.load(Ordering::SeqCst)
        }
    }
    impl HttpClient for FakeWire {
        fn http_connector(
            &self,
            _settings: &HttpConnectorSettings,
            _components: &RuntimeComponents,
        ) -> SharedHttpConnector {
            SharedHttpConnector::new(self.clone())
        }
    }
    impl HttpConnector for FakeWire {
        fn call(&self, request: HttpRequest) -> HttpConnectorFuture {
            let already_charged = self.session.snapshot().unwrap()["charged_attempts"]
                .as_u64()
                .unwrap();
            let previous = self.calls.fetch_add(1, Ordering::SeqCst);
            assert!(
                already_charged > previous,
                "wire boundary reached before durable debit"
            );
            let reply = self
                .replies
                .lock()
                .unwrap()
                .pop_front()
                .expect("unexpected HTTP request");
            let method = match &reply {
                Reply::Http { method, .. } | Reply::Ambiguous { method } => *method,
            };
            assert_eq!(request.method(), method);
            if method == "PUT" {
                assert_eq!(request.headers().get("if-none-match"), Some("*"));
            }
            match reply {
                Reply::Http {
                    status,
                    body,
                    length,
                    ..
                } => {
                    let mut response =
                        HttpResponse::new(status.try_into().unwrap(), SdkBody::from(body));
                    response
                        .headers_mut()
                        .insert("content-length", length.to_string());
                    response
                        .headers_mut()
                        .insert("x-amz-request-id", "fake-request");
                    HttpConnectorFuture::ready(Ok(response))
                }
                Reply::Ambiguous { .. } => HttpConnectorFuture::ready(Err(ConnectorError::io(
                    std::io::Error::other("ambiguous send result").into(),
                ))),
            }
        }
    }

    struct Fixture {
        context: Arc<S3BudgetContext>,
        wire: FakeWire,
        root: PathBuf,
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
    fn fixture(work_reads: u64, work_download: u64) -> Fixture {
        fixture_with_role(work_reads, work_download, None)
    }
    fn fixture_with_role(work_reads: u64, work_download: u64, role: Option<&str>) -> Fixture {
        let root = std::env::temp_dir().join(format!(
            "dataset-budget-wire-{}-{}",
            std::process::id(),
            OPERATION_SERIAL.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&root).unwrap();
        let (mut policy, mut request) = super::super::budget::test_policy_request(&root);
        policy.allowed_remotes = vec![RemoteScope {
            bucket: "setec-astronomy-artifacts".into(),
            prefix: "budget-wire/".into(),
        }];
        policy.coordination_prefix = "budget-wire/control/".into();
        let owner = request.owner.clone();
        let finalization = Resources {
            money_micro_usd: 10,
            output_bytes: 4096,
            put_attempts: 5,
            read_attempts: 5,
            download_bytes: if role.is_some() {
                4096 + 4 * STS_BODY_LIMIT
            } else {
                4096
            },
            local_bytes: 4096,
        };
        let resources = finalization
            .checked_add(&Resources {
                money_micro_usd: 100,
                output_bytes: 4096,
                put_attempts: 10,
                read_attempts: work_reads,
                download_bytes: work_download,
                local_bytes: 4096,
            })
            .unwrap();
        request.resources = resources;
        request.finalization = finalization;
        request.credential_role_arn = role.map(str::to_owned);
        let ledger =
            CampaignLedger::create(Path::new(&policy.allocator_ledger_path), &policy).unwrap();
        let grant = ledger.reserve(&request).unwrap();
        let session = Arc::new(
            BudgetSession::create(&root.join("worker.sqlite"), &policy, &grant, &owner).unwrap(),
        );
        let wire = FakeWire {
            session: session.clone(),
            replies: Arc::new(Mutex::new(VecDeque::new())),
            calls: Arc::new(AtomicU64::new(0)),
        };
        let context = Arc::new(S3BudgetContext {
            session,
            expected: Mutex::new(BTreeMap::new()),
            runtime: tokio::runtime::Builder::new_multi_thread()
                .worker_threads(1)
                .enable_all()
                .build()
                .unwrap(),
            http_client: SharedHttpClient::new(wire.clone()),
            credentials: Mutex::new(Some(CredentialsState {
                credentials: Credentials::new(
                    "offline-key",
                    "offline-secret",
                    None,
                    None,
                    "offline-test",
                ),
                expiration: None,
            })),
            source: CredentialSource::Static,
        });
        Fixture {
            context,
            wire,
            root,
        }
    }
    fn activate(fixture: &Fixture) {
        let payload = fixture.context.session.activation_payload().unwrap();
        fixture.wire.push("PUT", 200, b"");
        fixture.wire.push("GET", 200, &payload);
        fixture.context.activate().unwrap();
        assert_eq!(
            fixture.context.session.snapshot().unwrap()["phase"],
            "active"
        );
    }
    fn expected(body: &[u8]) -> ExpectedObject {
        ExpectedObject {
            sha256: hex::encode(sha256(body)),
            bytes: body.len() as u64,
        }
    }

    #[test]
    fn native_sdk_retry_is_debited_and_exhaustion_never_reaches_wire() {
        let fixture = fixture(2, 6);
        activate(&fixture);
        fixture
            .wire
            .push("GET", 503, b"error body must not become an artifact");
        fixture.wire.push("GET", 200, b"abc");
        let object = expected(b"abc");
        fixture
            .context
            .get_body(
                "setec-astronomy-artifacts",
                "budget-wire/object",
                &object,
                &BudgetScope::Work,
                &mut Vec::new(),
            )
            .unwrap();
        assert_eq!(fixture.wire.calls(), 4);
        let error = fixture
            .context
            .get_body(
                "setec-astronomy-artifacts",
                "budget-wire/object",
                &object,
                &BudgetScope::Work,
                &mut Vec::new(),
            )
            .unwrap_err();
        assert!(error.contains("budget exceeded"), "{error}");
        assert_eq!(fixture.wire.calls(), 4);
        let snapshot = fixture.context.session.snapshot().unwrap();
        assert_eq!(snapshot["work_reserved_or_charged"]["read_attempts"], 2);
        assert_eq!(snapshot["work_reserved_or_charged"]["download_bytes"], 6);
    }

    #[test]
    fn native_sdk_pending_session_and_unknown_object_send_nothing() {
        let fixture = fixture(3, 30);
        let object = expected(b"abc");
        assert!(fixture
            .context
            .get_body(
                "setec-astronomy-artifacts",
                "budget-wire/object",
                &object,
                &BudgetScope::Work,
                &mut Vec::new()
            )
            .is_err());
        assert!(fixture
            .context
            .expected("setec-astronomy-artifacts", "budget-wire/unregistered")
            .is_err());
        assert_eq!(fixture.wire.calls(), 0);
        assert_eq!(
            fixture.context.session.snapshot().unwrap()["charged_attempts"],
            0
        );
    }

    #[test]
    fn native_sdk_length_mismatch_is_rejected_before_body_copy() {
        let fixture = fixture(1, 3);
        activate(&fixture);
        fixture.wire.push("GET", 200, b"four");
        let mut destination = Vec::new();
        let error = fixture
            .context
            .get_body(
                "setec-astronomy-artifacts",
                "budget-wire/object",
                &expected(b"abc"),
                &BudgetScope::Work,
                &mut destination,
            )
            .unwrap_err();
        assert!(error.contains("content length"), "{error}");
        assert!(destination.is_empty());
        assert_eq!(fixture.wire.calls(), 3);
        assert_eq!(
            fixture.context.session.snapshot().unwrap()["work_reserved_or_charged"]
                ["download_bytes"],
            3
        );
    }

    #[test]
    fn native_sdk_head_forbidden_uses_charged_get_without_list() {
        let fixture = fixture(2, 3);
        activate(&fixture);
        fixture.wire.push("HEAD", 403, b"");
        fixture.wire.push("GET", 200, b"abc");
        let meta = fixture
            .context
            .head_object(
                "setec-astronomy-artifacts",
                "budget-wire/object",
                &expected(b"abc"),
                &BudgetScope::Work,
            )
            .unwrap()
            .unwrap();
        assert_eq!(meta.bytes, 3);
        assert_eq!(fixture.wire.calls(), 4);
        assert_eq!(
            fixture.context.session.snapshot().unwrap()["work_reserved_or_charged"]
                ["read_attempts"],
            2
        );
    }

    #[test]
    fn native_sdk_ambiguous_put_then_precondition_requires_fresh_readback() {
        let fixture = fixture(1, 3);
        activate(&fixture);
        let object = expected(b"abc");
        let reservation = fixture
            .context
            .session
            .reserve_object(
                "setec-astronomy-artifacts",
                "budget-wire/object",
                &object.sha256,
                3,
                &BudgetScope::Work,
            )
            .unwrap();
        fixture
            .wire
            .replies
            .lock()
            .unwrap()
            .push_back(Reply::Ambiguous { method: "PUT" });
        fixture.wire.push("PUT", 412, b"");
        fixture.wire.push("GET", 200, b"abc");
        fixture
            .context
            .put_body(
                "setec-astronomy-artifacts",
                "budget-wire/object",
                &object,
                &BudgetScope::Work,
                PutSource::Bytes(b"abc".to_vec()),
            )
            .unwrap();
        let ticket = fixture
            .context
            .get_body(
                "setec-astronomy-artifacts",
                "budget-wire/object",
                &object,
                &BudgetScope::Work,
                &mut Vec::new(),
            )
            .unwrap();
        fixture
            .context
            .session
            .verify_object(&reservation, &ticket, &object.sha256, 3)
            .unwrap();
        let snapshot = fixture.context.session.snapshot().unwrap();
        assert_eq!(snapshot["work_reserved_or_charged"]["put_attempts"], 2);
        assert_eq!(snapshot["work_reserved_or_charged"]["output_bytes"], 3);
        assert_eq!(snapshot["outcome_unknown_attempts"], 1);
        assert_eq!(fixture.wire.calls(), 5);
    }

    #[test]
    fn native_sdk_conflicting_activation_nonce_never_activates() {
        let fixture = fixture(1, 3);
        let mut wrong = fixture.context.session.activation_payload().unwrap();
        let index = wrong.len() / 2;
        wrong[index] ^= 1;
        fixture.wire.push("PUT", 412, b"");
        fixture.wire.push("GET", 200, &wrong);
        let error = fixture.context.activate().unwrap_err();
        assert!(error.contains("SHA-256"), "{error}");
        assert_eq!(
            fixture.context.session.snapshot().unwrap()["phase"],
            "activation_pending"
        );
        assert_eq!(fixture.wire.calls(), 2);
    }

    #[test]
    fn native_sdk_checksum_failure_never_verifies_object() {
        let fixture = fixture(1, 3);
        activate(&fixture);
        let object = expected(b"abc");
        let reservation = fixture
            .context
            .session
            .reserve_object(
                "setec-astronomy-artifacts",
                "budget-wire/object",
                &object.sha256,
                3,
                &BudgetScope::Work,
            )
            .unwrap();
        fixture.wire.push("GET", 200, b"xyz");
        assert!(fixture
            .context
            .get_body(
                "setec-astronomy-artifacts",
                "budget-wire/object",
                &object,
                &BudgetScope::Work,
                &mut Vec::new()
            )
            .is_err());
        assert_eq!(
            fixture.context.session.snapshot().unwrap()["verified_unique_output_bytes"],
            fixture.context.session.activation_payload().unwrap().len() as u64
        );
        assert_eq!(reservation.bytes, 3);
    }

    #[test]
    fn native_sdk_irsa_bootstrap_is_admitted_before_activation() {
        let role = "arn:aws:iam::123456789012:role/offline-dataset";
        let fixture = fixture_with_role(0, 0, Some(role));
        let path = fixture.root.join("fake-token");
        fs::write(&path, b"offline-fake-web-identity-token").unwrap();
        let response = br#"<AssumeRoleWithWebIdentityResponse xmlns="https://sts.amazonaws.com/doc/2011-06-15/"><AssumeRoleWithWebIdentityResult><Credentials><AccessKeyId>offline-key</AccessKeyId><SecretAccessKey>offline-secret</SecretAccessKey><SessionToken>offline-token</SessionToken><Expiration>2035-01-01T00:00:00Z</Expiration></Credentials></AssumeRoleWithWebIdentityResult><ResponseMetadata><RequestId>offline-sts</RequestId></ResponseMetadata></AssumeRoleWithWebIdentityResponse>"#;
        fixture.wire.push("POST", 200, response);
        let credentials = fixture.context.refresh_irsa(role, &path).unwrap();
        assert!(credentials.expiration.unwrap() > SystemTime::now());
        let snapshot = fixture.context.session.snapshot().unwrap();
        assert_eq!(snapshot["phase"], "activation_pending");
        assert_eq!(snapshot["sts_attempts"], 1);
        assert_eq!(
            snapshot["finalization_charged"]["download_bytes"],
            STS_BODY_LIMIT
        );
        assert_eq!(fixture.wire.calls(), 1);
        let rendered = snapshot.to_string();
        assert!(!rendered.contains("offline-secret"));
        assert!(!rendered.contains("offline-fake-web-identity-token"));
    }

    #[test]
    fn native_sdk_irsa_oversize_response_is_rejected_before_credentials() {
        let role = "arn:aws:iam::123456789012:role/offline-dataset";
        let fixture = fixture_with_role(0, 0, Some(role));
        let path = fixture.root.join("fake-token");
        fs::write(&path, b"offline-fake-token").unwrap();
        fixture.wire.replies.lock().unwrap().push_back(Reply::Http {
            method: "POST",
            status: 200,
            body: Vec::new(),
            length: STS_BODY_LIMIT + 1,
        });
        let result = fixture.context.refresh_irsa(role, &path);
        assert!(result.is_err());
        assert_eq!(fixture.wire.calls(), 1);
        assert_eq!(
            fixture.context.session.snapshot().unwrap()["finalization_charged"]["download_bytes"],
            STS_BODY_LIMIT
        );
    }

    #[test]
    fn native_sdk_expired_credentials_cannot_reach_wire() {
        let fixture = fixture(1, 3);
        activate(&fixture);
        let object = expected(b"abc");
        let operation = fixture.context.operation(
            HttpMethod::Get,
            "setec-astronomy-artifacts",
            "budget-wire/object",
            &object,
            &BudgetScope::Work,
        );
        let gate = AttemptGate::new(
            fixture.context.session.clone(),
            operation,
            Some(UNIX_EPOCH),
            Some(3),
        );
        let (client, _) = fixture.context.client().unwrap();
        let result = fixture.context.runtime.block_on(
            client
                .get_object()
                .bucket("setec-astronomy-artifacts")
                .key("budget-wire/object")
                .customize()
                .interceptor(gate.clone())
                .send(),
        );
        assert!(result.is_err());
        assert!(gate.failure("GET").unwrap().contains("credential lifetime"));
        assert_eq!(fixture.wire.calls(), 2);
        assert_eq!(
            fixture.context.session.snapshot().unwrap()["work_reserved_or_charged"]
                ["read_attempts"],
            0
        );
    }

    #[test]
    fn native_sdk_precreated_empty_destination_and_unsafe_paths() {
        use std::os::unix::fs::symlink;
        let fixture = fixture(1, 3);
        activate(&fixture);
        let object = expected(b"abc");
        fixture
            .context
            .expect_object(
                "setec-astronomy-artifacts",
                "budget-wire/object",
                &object.sha256,
                3,
                BudgetScope::Work,
            )
            .unwrap();
        let backend = BudgetedS3Backend {
            context: fixture.context.clone(),
            bucket: "setec-astronomy-artifacts".into(),
            prefix: "budget-wire".into(),
            scope: BudgetScope::Work,
        };
        let destination = fixture.root.join("existing-empty");
        fs::write(&destination, b"").unwrap();
        fixture.wire.push("GET", 200, b"abc");
        backend.get("object", &destination).unwrap();
        assert_eq!(fs::read(&destination).unwrap(), b"abc");
        assert!(backend.get("object", &destination).is_err());
        let empty = fixture.root.join("empty-target");
        fs::write(&empty, b"").unwrap();
        let linked = fixture.root.join("symlink-destination");
        symlink(&empty, &linked).unwrap();
        assert!(backend.get("object", &linked).is_err());
        assert!(fs::read(empty).unwrap().is_empty());
        assert_eq!(fixture.wire.calls(), 3);
    }

    #[test]
    fn native_sdk_head_ambiguity_marker_requires_observed_denial() {
        let fixture = fixture(2, 3);
        activate(&fixture);
        fixture.wire.push("HEAD", 403, b"");
        fixture.wire.push("GET", 403, b"");
        let error = fixture
            .context
            .head_object(
                "setec-astronomy-artifacts",
                "budget-wire/object",
                &expected(b"abc"),
                &BudgetScope::Work,
            )
            .unwrap_err();
        assert!(error.starts_with(AMBIGUOUS_HEAD_DENIED));
        let quota_error = fixture
            .context
            .head_object(
                "setec-astronomy-artifacts",
                "budget-wire/object",
                &expected(b"abc"),
                &BudgetScope::Work,
            )
            .unwrap_err();
        assert!(!quota_error.starts_with(AMBIGUOUS_HEAD_DENIED));
        assert_eq!(fixture.wire.calls(), 4);
    }

    #[test]
    fn native_sdk_put_success_body_must_be_empty() {
        let fixture = fixture(0, 0);
        activate(&fixture);
        let object = expected(b"abc");
        fixture
            .context
            .session
            .reserve_object(
                "setec-astronomy-artifacts",
                "budget-wire/object",
                &object.sha256,
                3,
                &BudgetScope::Work,
            )
            .unwrap();
        fixture.wire.push("PUT", 200, b"unexpected body");
        let error = fixture
            .context
            .put_body(
                "setec-astronomy-artifacts",
                "budget-wire/object",
                &object,
                &BudgetScope::Work,
                PutSource::Bytes(b"abc".to_vec()),
            )
            .unwrap_err();
        assert!(error.contains("content length"));
        assert_eq!(fixture.wire.calls(), 3);
    }
}
