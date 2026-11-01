#!/usr/bin/env bash
# Exact-key/type adversarial tests. Fixtures contain no measured data.
set -euo pipefail
root=$(cd "$(dirname "$0")" && pwd -P)
validator=${JSONSCHEMA:-jsonschema}
command -v "$validator" >/dev/null || exit 2
tmp=$(mktemp -d)
trap 'rm -rf -- "$tmp"' EXIT
h40=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
h64=bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
digest=sha256:cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc

jq -n --arg c "$h40" --arg h "$h64" --arg d "$digest" '
  {schema:"lead15.diem-quartic/v1",record:"run_start",protocol_id:"LEAD15-DT-E5-20261101",
   q:257,curves:6,base_seed:20261023,max_candidates:200000,retain_factor:8,
   runtime_provenance:{isolation:"none",cpu_model:null,available_parallelism:1,
     kubernetes_node:null,kubernetes_job:null,kubernetes_job_uid:null,
     kubernetes_pod:null,kubernetes_pod_uid:null},commit:$c,protocol_sha256:$h,
   binary_sha256:$h,image_digest:$d}' > "$tmp/run-start.json"

jq -n --arg c "$h40" --arg h "$h64" --arg d "$digest" '
  [0,0,0] as $k | [$k,$k,$k,$k] as $k4 |
  def bc($w;$b): {native_weight_gae:$w,gae_price:1,native_band_cost:$b,emitted_total:(1+$b)};
  {schema:"lead15.diem-quartic/v1",record:"instance",protocol_id:"LEAD15-DT-E5-20261101",
   q:257,curve_index:0,stream_retry:0,seed:1,field_modulus:{x3_a:1,constant_b:1},
   ub:$k,uc:$k,quartic:[range(0;15)|0],order_decimal:"28",cofactor:4,ell:7,trace:"0",
   candidate_count:1,rejects:{no_common_cover:0,quartic_core:0,order_or_cofactor:0},
   builder_setup:{streams_attempted:1,candidates_total:1,
     rejects_total:{no_common_cover:0,quartic_core:0,order_or_cofactor:0,duplicate_quartic_streams:0},
     fq_mul:0,fq_inv:0,fq_sqrt:0,hash_lookups:0,hash_inserts:1,rng_draws:0,sort_items:0,
     primality_modular_ops:0,fq_arithmetic:0,lookup_cost:0,native_counter_count:1,
     inversion_price:9,sqrt_price:12,cost_receipts:[bc(0;0),bc(0.1;0.1),bc(1;1)]},
   m2_certificate:{z:$k,target_ordered:$k4},
   m3_certificate:{a:0,b:1,r:$k,r_q:$k,source_ordered:$k4},
   common_branch_certificate:{cross_ratio:$k,mobius:[[$k,$k],[$k,$k]],verified_all_four_points:true},
   commit:$c,protocol_sha256:$h,binary_sha256:$h,image_digest:$d}' > "$tmp/instance.json"

jq -n --arg c "$h40" --arg h "$h64" --arg d "$digest" '
  def ac($m;$w): {fq_arithmetic:1,lookup_cost:0,mod_ell_cost:0,native_counter_count:0,
    native_band_cost:0,attack_total:1,verification_cost_excluded:0,inversion_price:9,
    sqrt_price:12,mod_ell_price:$m,native_weight_gae:$w,gae_price:181};
  {schema:"lead15.diem-quartic/v1",record:"harvest",protocol_id:"LEAD15-DT-E5-20261101",
   q:257,curve_index:0,seed:1,ell:7,n_factor_base:2,pairs:1,
   yield:{raw_full:1,raw_fp:0,raw_pp:0,unique_full:1,unique_fp:0,unique_pp:0,
     raw_fp_per_pair:0,raw_pp_per_pair:0,unique_fp_per_pair:0,unique_pp_per_pair:0},
   large_prime_graph:{vertices:0,edges:0,giant_fraction:0,cycle_count:0,usable_cycles_including_full:1},
   selection_sge:{retained_tree_edges:0,retained_extra_edges:0,materialized_tree_edges:0,
     materialized_full_rows:1,materialized_rows:1,materialized_dependency_dimension:0,
     rows_after_lp_sge:1,total_row_weight_after_lp_sge:1,mean_row_weight_after_lp_sge:1,
     max_row_weight_after_lp_sge:1,mean_pivot_weight_after_rank_sge:1},
   linear_algebra:{rank:1,nullity:1},
   cost_receipts:[ac(1;0),ac(1;0.1),ac(1;1),ac(9;0),ac(9;0.1),ac(9;1)],
   references:{rho_same_instance_relation_only:1,
     rho_formula:"sqrt(pi*ell/2)*74.5 FqM; no native-band counters available, so lower-bound reference only",
     matched_hyperelliptic_2p15:1,
     matched_hyper_formula:"9.967e10*(2^15/2^22)^1.408; q-matched round-4 aggregate extrapolation, not rerun on this exact E",
     quartic_projected_2p15:1,
     quartic_projection_formula:"7.3e6+0.53e6*w, w=measured mean row weight after LP SGE",k4_threshold:1},
   counters:{lines:1,split:1,rel_full:1,rel_fp:0,rel_pp:0,duplicates:0,verified:1,
     verify_failed:0,hash_lookups:0,hash_inserts:0,rng_draws:0,uf_ops:0,root_calls:0,
     normalizations:0,fq_mul_collect:0,fq_inv_collect:0,fq_sqrt_collect:0,fq_mul_fb:0,
     fq_inv_fb:0,fq_sqrt_fb:0,fq_mul_verify:0,fq_inv_verify:0,fq_sqrt_verify:0,
     lp_sge_ops:0,lp_sge_entries:0,rank_sge_ops:0,rank_inversions:0,rank_entries:0,sort_items:0},
   commit:$c,protocol_sha256:$h,binary_sha256:$h,image_digest:$d}' > "$tmp/harvest.json"

jq -n --arg c "$h40" --arg h "$h64" --arg d "$digest" '
  {schema:"lead15.diem-quartic/v1",record:"run_end",protocol_id:"LEAD15-DT-E5-20261101",
   q:257,instances:6,harvests:6,complete:true,commit:$c,protocol_sha256:$h,
   binary_sha256:$h,image_digest:$d}' > "$tmp/run-end.json"

for record in run-start instance harvest run-end; do
  "$validator" -i "$tmp/$record.json" "$root/result.schema.json" >/dev/null
done

expect_reject() {
  local name=$1 source=$2 filter=$3
  jq "$filter" "$tmp/$source.json" > "$tmp/$name.json"
  if "$validator" -i "$tmp/$name.json" "$root/result.schema.json" >/dev/null 2>&1; then
    echo "result-schema-tests: mutation $name was accepted" >&2
    exit 1
  fi
}
expect_reject top-extra harvest '.unregistered=1'
expect_reject nested-extra harvest '.selection_sge.unregistered=1'
expect_reject missing-certificate harvest 'del(.selection_sge.materialized_dependency_dimension)'
expect_reject counter-string harvest '.counters.lines="1"'
expect_reject wrong-cost-order harvest '.cost_receipts[0].mod_ell_price=9'
expect_reject builder-extra instance '.builder_setup.cost_receipts[0].extra=1'
expect_reject malformed-k instance '.m3_certificate.r=[0,0]'
expect_reject fictional-row run-end '.record="formula_reference"'
echo "result schema tests passed"
