// The archived experiment requires x86_64 Valgrind client registers.
// compat changes only the leading documentation comment form for include!.
#[cfg(target_arch = "x86_64")]
include!("../../compat/ct_dynamic_audit.rs");

#[cfg(not(target_arch = "x86_64"))]
fn main() {
    eprintln!("ct_dynamic_audit requires x86_64 Valgrind client registers");
    std::process::exit(2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn prepared_copy_preserves_every_archived_code_byte() {
        use sha2::{Digest, Sha256};
        let original = include_bytes!("../../research/ct_dynamic_audit_20261102/src/main.rs");
        assert_eq!(
            format!("{:x}", Sha256::digest(original)),
            "34791996bd59e7fcab68f3edb1eee7f1fef2424ba1f676f18afa6311df06aac3"
        );
        let source = std::str::from_utf8(original).unwrap();
        let mut header = true;
        let expected: String = source
            .split_inclusive('\n')
            .map(|line| {
                if header && line.starts_with("//!") {
                    format!("// {}", &line[3..])
                } else {
                    header = false;
                    line.to_owned()
                }
            })
            .collect();
        assert_eq!(
            expected.as_bytes(),
            include_bytes!("../../compat/ct_dynamic_audit.rs")
        );
    }
}
