//! Bounded source adapters. The verifier owns all logical admissions and hashes.
use super::{
    budget::Budget,
    io::Root,
    schema::{validate_ref, ObjectRef},
};
use anyhow::{ensure, Context, Result};
use std::{fs::File, io::Read, path::Path, process::Command, time::Duration};

pub(crate) struct AdmittedRef<'a> {
    reference: &'a ObjectRef,
}
impl<'a> AdmittedRef<'a> {
    pub(crate) fn new(
        reference: &'a ObjectRef,
        cap: u64,
        cas: bool,
        budget: &mut Budget,
    ) -> Result<Self> {
        validate_ref(reference, cap, cas)?;
        budget.admit_object(reference.bytes)?;
        Ok(Self { reference })
    }
}
pub(crate) trait ObjectReader {
    fn open(
        &mut self,
        object: &AdmittedRef<'_>,
        restored_path: &str,
        budget: &mut Budget,
    ) -> Result<Box<dyn Read>>;
    fn actual_remote(&self) -> bool {
        false
    }
}
pub(crate) struct LocalSource {
    root: Root,
    restored: bool,
}
impl LocalSource {
    pub(crate) fn mirror(path: &Path) -> Result<Self> {
        Ok(Self {
            root: Root::open(path)?,
            restored: false,
        })
    }
    pub(crate) fn restored(root: &Root) -> Result<Self> {
        Ok(Self {
            root: root.try_clone()?,
            restored: true,
        })
    }
}
impl ObjectReader for LocalSource {
    fn open(
        &mut self,
        object: &AdmittedRef<'_>,
        restored_path: &str,
        budget: &mut Budget,
    ) -> Result<Box<dyn Read>> {
        let key = if self.restored {
            restored_path
        } else {
            &object.reference.key
        };
        budget.charge(
            key.len()
                .checked_mul(8)
                .and_then(|n| n.checked_add(1024))
                .context("partial source path reservation")?,
            "source path/reader",
        )?;
        let file = self.root.open_file(key)?;
        ensure!(
            file.metadata()?.len() == object.reference.bytes,
            "partial source declared size mismatch"
        );
        Ok(Box::new(file))
    }
}

pub(crate) trait CommandRunner {
    fn run(
        &mut self,
        command: &mut Command,
        guard: Option<(&Path, u64)>,
    ) -> Result<crate::transport::CommandOutput>;
    fn actual_remote(&self) -> bool {
        false
    }
}
struct AwsRunner;
impl CommandRunner for AwsRunner {
    fn run(
        &mut self,
        command: &mut Command,
        guard: Option<(&Path, u64)>,
    ) -> Result<crate::transport::CommandOutput> {
        crate::transport::run_bounded(command, Duration::from_secs(60), 65536, guard)
    }
    fn actual_remote(&self) -> bool {
        true
    }
}
pub(crate) struct RemoteSource {
    prefix: String,
    runner: Box<dyn CommandRunner>,
}
impl RemoteSource {
    pub(crate) fn new(remote: &str, budget: &mut Budget) -> Result<Self> {
        budget.charge(
            remote
                .len()
                .checked_mul(8)
                .and_then(|n| n.checked_add(1024))
                .context("remote prefix reservation")?,
            "remote prefix",
        )?;
        Ok(Self {
            prefix: crate::transport::remote_prefix(remote)?,
            runner: Box::new(AwsRunner),
        })
    }
    #[cfg(test)]
    pub(crate) fn mocked(
        remote: &str,
        runner: Box<dyn CommandRunner>,
        budget: &mut Budget,
    ) -> Result<Self> {
        let mut source = Self::new(remote, budget)?;
        source.runner = runner;
        Ok(source)
    }
    fn command() -> Command {
        let mut command = Command::new("aws");
        command
            .env("AWS_MAX_ATTEMPTS", "1")
            .env("AWS_PAGER", "")
            .env("AWS_CLI_AUTO_PROMPT", "off");
        command.args(["--output", "json", "--no-cli-pager", "--no-cli-auto-prompt"]);
        command
    }
}
struct TemporaryRead {
    file: File,
    _directory: tempfile::TempDir,
}
impl Read for TemporaryRead {
    fn read(&mut self, out: &mut [u8]) -> std::io::Result<usize> {
        self.file.read(out)
    }
}
impl ObjectReader for RemoteSource {
    fn actual_remote(&self) -> bool {
        self.runner.actual_remote()
    }
    fn open(
        &mut self,
        object: &AdmittedRef<'_>,
        _: &str,
        budget: &mut Budget,
    ) -> Result<Box<dyn Read>> {
        const OUTPUT: usize = 65536;
        let reference = object.reference;
        let size = self
            .prefix
            .len()
            .checked_add(reference.key.len())
            .and_then(|n| n.checked_add(1))
            .context("remote key size")?;
        budget.charge(
            size.checked_mul(16)
                .and_then(|n| n.checked_add(16384 + 8 * OUTPUT))
                .context("remote command reservation")?,
            "two bounded command outputs, arguments and temp paths",
        )?;
        let key = if self.prefix.is_empty() {
            reference.key.clone()
        } else {
            format!("{}/{}", self.prefix, reference.key)
        };
        let mut head = Self::command();
        head.args([
            "--region",
            "us-east-2",
            "s3api",
            "head-object",
            "--bucket",
            "setec-astronomy-artifacts",
            "--key",
            &key,
        ]);
        let out = self.runner.run(&mut head, None)?;
        crate::transport::require_success(&out, "partial HEAD")?;
        let metadata: serde_json::Value = super::io::typed(&out.stdout, budget)?;
        ensure!(
            metadata["ContentLength"].as_u64() == Some(reference.bytes),
            "partial HEAD length mismatch"
        );
        let etag = metadata["ETag"]
            .as_str()
            .context("partial HEAD ETag missing")?;
        ensure!(
            !etag.is_empty() && etag.len() <= 1024 && !etag.chars().any(char::is_control),
            "partial HEAD ETag grammar"
        );
        let directory = tempfile::Builder::new()
            .prefix("consumer-partial-read-")
            .tempdir_in("/var/tmp")?;
        let root = Root::open(directory.path())?;
        root.space(reference.bytes)?;
        let destination = directory.path().join("object");
        let mut get = Self::command();
        get.args([
            "--region",
            "us-east-2",
            "s3api",
            "get-object",
            "--bucket",
            "setec-astronomy-artifacts",
            "--key",
            &key,
            "--if-match",
            etag,
        ])
        .arg(&destination);
        let out = self
            .runner
            .run(&mut get, Some((&destination, reference.bytes)))?;
        crate::transport::require_success(&out, "partial conditioned GET")?;
        let file = root.open_file("object")?;
        ensure!(
            file.metadata()?.len() == reference.bytes,
            "partial GET length mismatch"
        );
        // The caller consumes and hashes this exact opened descriptor. No retry,
        // receipt promotion or credential fallback exists in this adapter.
        Ok(Box::new(TemporaryRead {
            file,
            _directory: directory,
        }))
    }
}
