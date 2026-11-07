mod owned_identity { pub(super) struct Capsule {
 pub schema:&'static str,
 pub capture_binding_sha256:String,
 pub public_sha256:String,
 pub factor_sha256:String,
 pub launch_sha256:String,
 pub binding_sha256:String,
 pub manifest_sha256:String,
 pub raw_execution_context_sha256:String,
 pub run_sha256:String,
 pub original_math_replayed_here:bool,
 pub cell_files_revalidated_here:bool,
} }
mod capture { pub(super) struct CommittedEventPosition {
    pub binding_sha256: String,
    pub event_count: u64,
    pub byte_end: u64,
    pub last_event_sha256: String,
} }
struct Snapshot { index:u64, decoded:Option<wire::Attempt> }
