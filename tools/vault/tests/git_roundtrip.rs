//! End-to-end behaviour against real git, with a fake `aws` that enforces the
//! KMS encryption context the way KMS does.

use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

const VAULT: &str = env!("CARGO_BIN_EXE_vault");
const SECRET: &str = "SECRET-FINDING-42: the walk collapses at k=992";

const FAKE_AWS: &str = r#"#!/bin/sh
# Fake `aws kms`: the wrapped blob is "wrapped:" + encryption context, so
# decrypt fails unless the caller presents the context the key was wrapped under.
op="$2"; ctx=""
while [ $# -gt 0 ]; do case "$1" in --encryption-context) ctx="$2"; shift;; esac; shift; done
blob=$(printf 'wrapped:%s' "$ctx" | base64 -w0)
key=$(cat "$FAKE_KMS_KEY")
case "$op" in
  generate-data-key) printf '{"KeyId":"fake","Plaintext":"%s","CiphertextBlob":"%s"}' "$key" "$blob" ;;
  decrypt)
    got=$(cat | base64 -w0)
    [ "$got" = "$blob" ] || { echo "An error occurred (InvalidCiphertextException)" >&2; exit 254; }
    printf '{"KeyId":"fake","Plaintext":"%s"}' "$key" ;;
  *) echo "fake aws: unsupported $op" >&2; exit 2 ;;
esac
"#;

struct Lab {
    _tmp: tempfile::TempDir,
    root: PathBuf,
    aws: PathBuf,
    key_b64: PathBuf,
}

impl Lab {
    fn new() -> Self {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path().to_path_buf();
        let aws = root.join("fake-aws");
        std::fs::write(&aws, FAKE_AWS).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&aws, std::fs::Permissions::from_mode(0o755)).unwrap();
        }
        let key_b64 = root.join("kms-key.b64");
        // 64 bytes, base64.
        let raw: Vec<u8> = (0..64u8)
            .map(|i| i.wrapping_mul(37).wrapping_add(11))
            .collect();
        std::fs::write(&key_b64, b64(&raw)).unwrap();
        Lab {
            _tmp: tmp,
            root,
            aws,
            key_b64,
        }
    }
    fn cmd(&self, dir: &Path, prog: &str, args: &[&str]) -> Output {
        Command::new(prog)
            .args(args)
            .current_dir(dir)
            .env("VAULT_AWS", &self.aws)
            .env("FAKE_KMS_KEY", &self.key_b64)
            .env("GIT_AUTHOR_NAME", "t")
            .env("GIT_AUTHOR_EMAIL", "t@example.invalid")
            .env("GIT_COMMITTER_NAME", "t")
            .env("GIT_COMMITTER_EMAIL", "t@example.invalid")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .stdin(Stdio::null())
            .output()
            .unwrap()
    }
    fn ok(&self, dir: &Path, prog: &str, args: &[&str]) -> String {
        let o = self.cmd(dir, prog, args);
        assert!(
            o.status.success(),
            "{prog} {args:?} failed:\n{}\n{}",
            String::from_utf8_lossy(&o.stdout),
            String::from_utf8_lossy(&o.stderr)
        );
        String::from_utf8_lossy(&o.stdout).into_owned()
    }
    fn fails(&self, dir: &Path, prog: &str, args: &[&str]) -> String {
        let o = self.cmd(dir, prog, args);
        assert!(
            !o.status.success(),
            "{prog} {args:?} unexpectedly succeeded"
        );
        String::from_utf8_lossy(&o.stderr).into_owned()
    }
    fn git(&self, dir: &Path, args: &[&str]) -> String {
        self.ok(dir, "git", args)
    }
    fn vault(&self, dir: &Path, args: &[&str]) -> String {
        self.ok(dir, VAULT, args)
    }
    fn blob(&self, dir: &Path, spec: &str) -> Vec<u8> {
        self.cmd(dir, "git", &["cat-file", "blob", spec]).stdout
    }

    /// A repository with vault adopted and one committed secret.
    fn origin(&self) -> PathBuf {
        let a = self.root.join("origin");
        std::fs::create_dir(&a).unwrap();
        self.git(&a, &["init", "-q", "-b", "main"]);
        self.vault(
            &a,
            &[
                "init",
                "--repo",
                "o/r",
                "--kms-key",
                "alias/test",
                "--protect",
                "research/**",
            ],
        );
        self.vault(&a, &["new-key", "--realm", "findings"]);
        std::fs::write(
            a.join(".gitattributes"),
            "research/** filter=vault-findings diff=vault\n",
        )
        .unwrap();
        std::fs::create_dir(a.join("research")).unwrap();
        std::fs::write(a.join("research/a.md"), SECRET).unwrap();
        std::fs::write(a.join("README.md"), "public\n").unwrap();
        self.git(&a, &["add", "-A"]);
        self.git(&a, &["commit", "-q", "-m", "adopt vault"]);
        a
    }
}

fn b64(data: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.encode(data)
}

fn contains(hay: &[u8], needle: &str) -> bool {
    hay.windows(needle.len()).any(|w| w == needle.as_bytes())
}

#[test]
fn commit_stores_ciphertext_and_worktree_stays_plain() {
    let lab = Lab::new();
    let a = lab.origin();
    let blob = lab.blob(&a, "HEAD:research/a.md");
    assert!(
        blob.starts_with(b"\0AVLT1"),
        "committed blob is not a vault blob"
    );
    assert!(!contains(&blob, "SECRET"));
    assert_eq!(
        std::fs::read_to_string(a.join("research/a.md")).unwrap(),
        SECRET
    );
    // Deterministic clean: nothing shows as modified.
    assert_eq!(lab.git(&a, &["status", "--porcelain"]), "");
    // The whole object store holds no plaintext.
    let all = lab.git(&a, &["log", "-p", "--all", "--no-textconv", "--binary"]);
    assert!(!all.contains("SECRET-FINDING"));
    // textconv shows plaintext to a key holder.
    assert!(lab
        .git(&a, &["show", "HEAD", "--", "research/a.md"])
        .contains("SECRET-FINDING"));
    // The public file is untouched.
    assert_eq!(lab.blob(&a, "HEAD:README.md"), b"public\n");
    // The wrapped key, not the key, is committed.
    assert!(!lab.git(&a, &["ls-files"]).contains(".key"));
}

#[test]
fn fresh_clone_is_locked_until_unlock_then_lock_again() {
    let lab = Lab::new();
    let a = lab.origin();
    let b = lab.root.join("clone");
    lab.git(
        &lab.root,
        &["clone", "-q", a.to_str().unwrap(), b.to_str().unwrap()],
    );
    let locked = std::fs::read(b.join("research/a.md")).unwrap();
    assert!(locked.starts_with(b"\0AVLT1"));

    lab.vault(&b, &["unlock"]);
    assert_eq!(
        std::fs::read_to_string(b.join("research/a.md")).unwrap(),
        SECRET
    );
    assert_eq!(lab.git(&b, &["status", "--porcelain"]), "");

    lab.vault(&b, &["lock"]);
    assert!(std::fs::read(b.join("research/a.md"))
        .unwrap()
        .starts_with(b"\0AVLT1"));
    assert_eq!(lab.git(&b, &["status", "--porcelain"]), "");

    // Fail closed: filter installed, no key, new plaintext cannot be staged.
    std::fs::write(b.join("research/new.md"), "another finding").unwrap();
    let err = lab.fails(&b, "git", &["add", "research/new.md"]);
    assert!(err.contains("refusing to stage"), "{err}");
}

#[test]
fn guard_catches_a_clone_without_the_filter() {
    let lab = Lab::new();
    let a = lab.origin();
    let c = lab.root.join("naive");
    lab.git(
        &lab.root,
        &["clone", "-q", a.to_str().unwrap(), c.to_str().unwrap()],
    );
    // No `vault install`: git stages plaintext silently. The guard must not.
    std::fs::write(c.join("research/leak.md"), "plaintext finding").unwrap();
    lab.git(&c, &["add", "research/leak.md"]);
    let err = lab.fails(&c, VAULT, &["guard", "--staged"]);
    assert!(err.contains("research/leak.md is PLAINTEXT"), "{err}");
    lab.git(&c, &["commit", "-q", "-m", "oops"]);
    let err = lab.fails(&c, VAULT, &["guard", "--range", "origin/main..HEAD"]);
    assert!(err.contains("PLAINTEXT"), "{err}");
    // Dropping the attribute does not help: the protect glob still applies.
    std::fs::write(c.join(".gitattributes"), "").unwrap();
    std::fs::write(c.join("research/leak2.md"), "more").unwrap();
    lab.git(&c, &["add", "-A"]);
    let err = lab.fails(&c, VAULT, &["guard", "--staged"]);
    assert!(
        err.contains("leak2.md is PLAINTEXT") && err.contains("has no vault filter attribute"),
        "{err}"
    );
}

#[test]
fn pre_commit_hook_refuses_a_moved_or_swapped_blob() {
    let lab = Lab::new();
    let a = lab.origin();
    // Swap: put a.md's ciphertext at another path in the index.
    let sha = lab.git(&a, &["rev-parse", "HEAD:research/a.md"]);
    lab.git(
        &a,
        &[
            "update-index",
            "--add",
            "--cacheinfo",
            &format!("100644,{},research/b.md", sha.trim()),
        ],
    );
    let err = lab.fails(&a, "git", &["commit", "-q", "-m", "swap"]);
    assert!(
        err.contains("was encrypted for path \"research/a.md\""),
        "{err}"
    );
    lab.git(&a, &["rm", "-q", "--cached", "research/b.md"]);

    // git mv keeps the old blob; the hook asks for a re-stage, which fixes it.
    lab.git(&a, &["mv", "research/a.md", "research/moved.md"]);
    let err = lab.fails(&a, "git", &["commit", "-q", "-m", "mv"]);
    assert!(err.contains("git add -- research/moved.md"), "{err}");
    lab.git(&a, &["add", "research/moved.md"]);
    lab.git(&a, &["commit", "-q", "-m", "mv"]);
    lab.vault(&a, &["guard", "--range", "HEAD", "--verify"]);
}

#[test]
fn rotation_and_wrong_context() {
    let lab = Lab::new();
    let a = lab.origin();
    lab.vault(&a, &["new-key", "--realm", "findings"]);
    lab.git(&a, &["add", "--renormalize", "--", "."]);
    lab.git(&a, &["add", "-A"]);
    lab.git(&a, &["commit", "-q", "-m", "rotate"]);
    let blob = lab.blob(&a, "HEAD:research/a.md");
    let (realm_len, epoch_at) = (blob[6] as usize, 7 + blob[6] as usize);
    assert_eq!(&blob[7..7 + realm_len], b"findings");
    assert_eq!(&blob[epoch_at..epoch_at + 4], &2u32.to_be_bytes());

    // Both epochs unlock in a clone; history from epoch 1 still reads.
    let b = lab.root.join("clone2");
    lab.git(
        &lab.root,
        &["clone", "-q", a.to_str().unwrap(), b.to_str().unwrap()],
    );
    lab.vault(&b, &["unlock"]);
    assert!(lab
        .git(&b, &["log", "-p", "--", "research/a.md"])
        .contains("SECRET-FINDING"));

    // A wrapped key relabelled for another epoch does not unwrap: KMS checks the context.
    let c = lab.root.join("clone3");
    lab.git(
        &lab.root,
        &["clone", "-q", a.to_str().unwrap(), c.to_str().unwrap()],
    );
    std::fs::copy(
        c.join(".vault/keys/findings.1.kms"),
        c.join(".vault/keys/findings.3.kms"),
    )
    .unwrap();
    let err = lab.fails(&c, VAULT, &["unlock"]);
    assert!(err.contains("findings epoch 3"), "{err}");
}

#[test]
fn seal_and_unseal_artifacts() {
    let lab = Lab::new();
    let a = lab.origin();
    let pt = lab.root.join("report.json");
    let ct = lab.root.join("report.json.vault");
    let back = lab.root.join("report.back.json");
    std::fs::write(&pt, "{\"finding\": 1}").unwrap();
    lab.vault(
        &a,
        &[
            "seal",
            "--realm",
            "findings",
            "--label",
            "ci/report.json",
            pt.to_str().unwrap(),
            ct.to_str().unwrap(),
        ],
    );
    assert!(std::fs::read(&ct).unwrap().starts_with(b"\0AVLT1"));
    lab.vault(
        &a,
        &[
            "unseal",
            "--label",
            "ci/report.json",
            ct.to_str().unwrap(),
            back.to_str().unwrap(),
        ],
    );
    assert_eq!(std::fs::read(&back).unwrap(), std::fs::read(&pt).unwrap());
    lab.fails(
        &a,
        VAULT,
        &[
            "unseal",
            "--label",
            "other",
            ct.to_str().unwrap(),
            back.to_str().unwrap(),
        ],
    );
}
