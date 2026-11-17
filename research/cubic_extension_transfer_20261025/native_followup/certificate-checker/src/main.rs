#![allow(dead_code)]
// Reuse the repository's existing checker, rather than trusting the C++ issuer.
#[path = "../../vendor/identity_certificate.rs"]
mod certificate;
mod hash {
    pub mod sha256 {
        #[link(name = "crypto")]
        extern "C" {
            fn SHA256(data: *const u8, len: usize, out: *mut u8) -> *mut u8;
        }
        pub fn sha256(data: &[u8]) -> [u8; 32] {
            let mut output = [0; 32];
            unsafe { SHA256(data.as_ptr(), data.len(), output.as_mut_ptr()); }
            output
        }
    }
}
fn main() {
    let mut records = Vec::new();
    for path in std::env::args().skip(1) {
        let raw = std::fs::read(&path).expect("certificate input");
        let cert: certificate::IdentityCertificate = serde_json::from_slice(&raw).expect("certificate schema");
        let verdict = certificate::check_with_extra(&cert, Some((20261103, 64))).expect("well-formed certificate");
        assert!(matches!(verdict, certificate::Verdict::Accept { extra_points: 64, .. }));
        let mut false_statement = cert.clone();
        false_statement.statement.rhs = certificate::Expr::plus(false_statement.statement.rhs, certificate::Expr::Const(1));
        false_statement.id = certificate::statement_id(&false_statement.statement);
        let rejected = certificate::check(&false_statement).expect("well-formed false identity");
        assert!(matches!(rejected, certificate::Verdict::Reject(_)));
        records.push(serde_json::json!({"path":path,"id":cert.id,"status":"passed","extra_checker_seed":20261103,"extra_points":64,"verdict":format!("{verdict:?}"),"false_identity_rejected":true}));
    }
    assert!(records.len() >= 3);
    println!("{}", serde_json::to_string_pretty(&serde_json::json!({"status":"passed","implementation":"existing Rust identity checker; C++ issuer; same host/session","records":records})).unwrap());
}
