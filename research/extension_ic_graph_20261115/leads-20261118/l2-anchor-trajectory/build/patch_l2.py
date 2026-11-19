"""Make the L2 copy of Run50's deferred collector (tooling; the science change is
listed here in full). Input: the manifest-checked 11c9a031 file
jv_extension_deferred_pair_dlp.rs (sha256 a3ad4923...). Output: jv_l2_deferred_pair_dlp.rs.

Edits (exact string replacements, each asserted to occur once):
  1. Add two module statics: L2_EXTEND_PERMILLE (0 = Run50's behaviour, unchanged)
     and L2_LOG (per-complete-batch records).
  2. At the end of every complete batch (after Run50's own checkpoint push), push a
     record: B, sources, projections, graph stats, counts, and the collection
     tally priced by phase (snapshot since collection start; pricing reads only).
  3. Replace the rank-stop `break` by: on the first certified batch, push a
     rank-stop snapshot (everything Run50 records at its stop); then break exactly
     as Run50 when L2_EXTEND_PERMILLE == 0, else keep collecting complete batches
     until sources >= rank-stop sources * permille / 1000. Nothing else changes:
     the rank tracker is not due after the kernel (Tracker::due), so no further
     linear algebra runs, and the kernel used afterwards is the rank-stop kernel.
"""
import hashlib, sys
src, dst = sys.argv[1], sys.argv[2]
s = open(src, "rb").read()
assert hashlib.sha256(s).hexdigest() == "a3ad49239001eda72f344ab34f94f0e78e802867e6afaae81f9ca1b9756f8d34"
s = s.decode()

def rep(old, new):
    global s
    assert s.count(old) == 1, old
    s = s.replace(old, new)

rep("use std::collections::{BTreeMap, BTreeSet, HashMap};\n",
"""use std::collections::{BTreeMap, BTreeSet, HashMap};
/// L2 lane: 0 keeps Run50's stop; N > 0 continues to N/1000 x rank-stop sources.
pub static L2_EXTEND_PERMILLE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
/// L2 lane: one record per complete batch, plus one rank-stop snapshot.
pub static L2_LOG: std::sync::Mutex<Vec<Value>> = std::sync::Mutex::new(Vec::new());
""")
rep("""        if rank.kernel.is_some() {
            stop = "sparse homogeneous rank certified at complete batch".into();
            break;
        }
""", """        {
            let tally = work::snapshot().since(&collection_before);
            let widths = work::widths(spec.p, spec.l);
            let priced: BTreeMap<&str, String> = Phase::IC
                .iter()
                .map(|&p| (p.name(), work::price(tally.phase(p), &widths).expect("registered widths").to_string()))
                .collect();
            let mut c = counts.clone();
            c.vertices = graph.stats().vertices;
            c.rank = rank.rank_lower_bound();
            let ready = rank.kernel.is_some();
            let mut log = L2_LOG.lock().expect("l2 log");
            log.push(json!({"kind":"batch","b_count":end,"sources":sources.len(),"projections":projections.len(),
                "rank_ready":ready,"graph_stats":graph.stats(),"counts":c,"phase_bitops":priced,
                "splits":splits.len(),"screen_calls":screen_calls,"screen_refusals":screen_refusals.len()}));
            if ready && l2_rank_stop_sources.is_none() {
                l2_rank_stop_sources = Some(sources.len());
                log.push(json!({"kind":"rank_stop","b_count":end,"sources":sources.len(),"projections":projections.len(),
                    "graph_stats":graph.stats(),"counts":c,"phase_bitops":priced,"stop":"sparse homogeneous rank certified at complete batch",
                    "rank_checks":rank.checkpoints,"last_checkpoint":checkpoints.last(),"checkpoints_len":checkpoints.len(),
                    "splits":splits.len(),"screen_calls":screen_calls,"screen_refusals":screen_refusals.len()}));
            }
        }
        if rank.kernel.is_some() {
            let permille = L2_EXTEND_PERMILLE.load(std::sync::atomic::Ordering::SeqCst) as u128;
            let s0 = l2_rank_stop_sources.expect("rank stop recorded") as u128;
            if permille == 0 {
                stop = "sparse homogeneous rank certified at complete batch".into();
                break;
            }
            if sources.len() as u128 * 1000 >= s0 * permille {
                stop = "l2: source target reached at complete batch after rank stop".into();
                break;
            }
        }
""")
rep("""    let mut stop = "B budget exhausted".to_string();
    'collect:""", """    let mut stop = "B budget exhausted".to_string();
    let mut l2_rank_stop_sources: Option<usize> = None;
    'collect:""")
open(dst, "w").write(s)
print("patched", hashlib.sha256(s.encode()).hexdigest())
