#!/usr/bin/env python3
"""Index recorded outputs and fixed claim scopes; no cryptographic computation."""
from pathlib import Path
import json,re
base=Path(__file__).resolve().parent.parent

def write(path,obj):path.write_text(json.dumps(obj,indent=2)+'\n')
def code(p):return int(p.read_text().strip())if p.exists()else None

def inventory(round_id):
 rows=[]
 for p in sorted((base/str(round_id)/'runs').iterdir()):
  if not p.is_dir():continue
  row={'id':p.name}
  if p.name.startswith('lean'):
   row['stage']='proof';row['compile_exit']=code(p/'compile.exit') if (p/'compile.exit').exists() else code(p/'exit.txt');row['audit_exit']=code(p/'audit.exit')
   audit=(p/'audit.log').read_text()if(p/'audit.log').exists()else''
   count=re.search(r'audited (\d+) theorems.*0 failing, 0 trusted',audit)
   if row['compile_exit']==0 and row['audit_exit']==0 and count:
    row['status']='accepted';row['audited_declarations']=int(count[1])
   elif not any((p/name).exists() for name in ['compile.log','compile-audit.log']):row['status']='gate_blocked_before_compiler'
   else:row['status']='proof_or_runtime_failure'
  elif(p/'native.log').exists():
   row['stage']='native';row['exit']=code(p/'exit.txt');row['build_exit']=code(p/'build.exit')
   log=(p/'native.log').read_text();errors=bool(re.search(r'^\s*\?',log,re.M));row['singular_errors']=errors
   row['markers']=re.findall(r'^([A-Z0-9_]+_ALL_CHECKS_PASS)$',log,re.M)
   if 'corrupt'in p.name:
    assert errors;row['status']='expected_rejection'
   elif row['exit'] is None:row['status']='wrapper_evidence_incomplete'
   elif row['exit'] in [14,124,137]:row['status']='resource_limit'
   elif row['exit']==0 and not errors and row['markers']:row['status']='accepted'
   else:row['status']='implementation_rejection'
  elif(p/'build.exit').exists():
   row.update(stage='build',build_exit=code(p/'build.exit'),status='compiler_failure')
  else:row.update(stage='other',status='unclassified')
  rows.append(row)
 return rows

common={'date_utc':'2026-10-29','client_date':'2026-10-28 America/Los_Angeles','performance_measured':False,'new_IC_relation_rank_claim':False,'full_F2_modular_factor_transfer':False,'merge_authorized':False,'generic_checker_available':False}
r27=dict(common,schema='koblitz.round27/proper-models-and-torsion-v1',round=27)
r27.update(
 complete_divisors={'accepted':True,'transported_charts':20,'lambda_pole_degree':64,'lambda_extra_zero_degree':14,'nu_pole_degree':132,'S_pole_degree':308,'N_pole_degree':462,'original_coordinate_and_registered_divisors_complete_under_written_DVR_bridge':True},
 complete_RR={'accepted':True,'divisor_degree':128,'source_genus':61,'sections':68,'candidate_monomials':1568,'independent_O_jet_coefficients':129,'charts_independently_reconstructed':20,'sigma_trace_rank':34,'quotient_genus':31,'quotient_divisor_degree':64,'quotient_sections':34,'complete_serialized_exports':True,'bulk_list_exports_rejected':True},
 projective_models={'accepted':True,'source_coordinates':68,'source_products':2346,'source_multiplication_rank':196,'source_quadrics':2150,'quotient_coordinates':34,'quotient_products':595,'quotient_multiplication_rank':98,'quotient_quadrics':497,'all_kernel_vectors_and_pivots_recombined':True,'prime_saturated_new_ideals_under_written_RR_and_Prop11_bridge':True,'old_coupled_ideal_saturation_proved':False},
 actual_torsion={'accepted':True,'exact_order':11,'diamond_eigenvalue_mod11':6,'actual_nonprincipal_divisor_from_full_RR':True,'full_membership_rank':68,'kernel_dimension':0,'moved_divisor_positive_degree':64,'principal_presentation_determinant':1477212,'desired_CM_sign_projection_verified':False},
 mixed_polarized_carrier={'dimension':24,'polarization_type_ones':18,'polarization_type_fives':6,'polarization_degree':244140625,'classification':'written geometric deduction from checked etale cyclic10 tower and standard Prym/duality theorems','explicit_integral_correspondence':'(1-T^5)*(5-(1+T^2+T^4+T^6+T^8))','pseudo_idempotent_scalar':10,'actual_nonzero_11_class_lies_in_carrier_under_written_bridge':True,'explicit_theta_sections':False,'old_det19_embedding_constructed':False},
 cusp_reverse={'accepted':True,'arbitrary_formal_cusp_coefficients':True,'prime_to_11_recovery_scalar':134292,'matrix_rows':5,'matrix_columns':5,'all_entries_integral_and_products_checked':True,'arbitrary_ordinary_divisor_evaluator':False},
 node187={'accepted':True,'actual_Miller_functions_with_conjugates':80,'graphs':4,'coefficient_conjugates_each':5,'proper_chart_group_law_lifts':2,'local_nilpotent_unit_certificate':True,'original_global_h_power_membership':False,'complete_simultaneous_arc_constructed':False},
 low_genus={'accepted':True,'particular_quotient_genera_excluded_under_modular_bridge':[7,13],'target_Weil_degree':8,'irreducible_and_non_even':True,'other_low_genus_carriers_excluded':False},
 RR_F32_map_system={'equations':455,'binary_coefficient_terms':646907,'invariant_and_trace_preimage_sections_each':34,'all_source_jet_checks':910,'control_failures':0,'coefficient_field_size':32,'both_ordinary_signs':True,'nonconstant_invariant_adjustments_included':True,'solver_run':'mapsolve27-02','solver_exit':14,'solver_outcome':'4GiB virtual memory cap; no map or unit certificate','universal_nonexistence_proved':False},
 proof={'named_theorems':35,'audited_declarations':35,'failing':0,'trusted_native':0,'accepted_run':'lean09','lean':'4.34.1','mathlib':'d13f23b723b8a846827a245b89c10fc7d3f11612','proper_geometry_formalized':False})
r28=dict(common,schema='koblitz.round28/selected-hecke-images-v1',round=28)
r28.update(adjugate_norms={'accepted':True,'exact_functions':4,'q_coefficients_each':1024,'input_coefficients':123904,'original_M15_norms_match_exactly':True,'dual_determinant_inverse_and_multiplicativity_controls':True},ordinary_images={'accepted':True,'selected_supports':3,'selected_first_degrees':[436,284,83],'selected_second_degrees':[436,284,83],'unresolved_first_degree':12,'unresolved_first_multiplicity':2,'unresolved_cause':'both first adjugates vanish','candidate_cubic_roots_each':3,'complete_scalar_C5_source_fibers':False},proof={'named_theorems':3,'audited_declarations':4,'generated_congruence_theorems':1,'failing':0,'trusted_native':0,'run':'lean28-01','actual_Hecke_norm_specialization_geometry_formalized':False})
r29=dict(common,schema='koblitz.round29/double-image-and-full-level-bridge-v1',round=29)
r29.update(second_adjugate={'accepted':True,'functions':3,'q_coefficients_each':1024,'weights':['1','xinf','1+xinf'],'original_M15_and_R28_coefficients_match_exactly':True},ordinary_images={'accepted':True,'selected_supports_cumulative':4,'all_first_degrees':[436,12,284,83],'all_second_degrees':[436,12,284,83],'multiplicities':[1,2,1,1],'ordinary_product_plus_one_degree':460,'ordinary_asymmetric_plus_one_degree':367,'degree12_distinct_selected_second_points':1,'degree12_selected_multiplicity':2,'degree12_quadratic':'X^2+(beta^11+beta^8+beta^5+beta^3+beta)','degree12_second_x':'beta^11+beta^8+beta^5+beta^3+beta^2+1','complete_ordinary_paired_Hecke_images_under_written_norm_bridge':True,'complete_scalar_C5_source_fibers':False,'complete_reverse_signed_divisors':False},full_level_cubic={'accepted':True,'registered_base_point_degree':14,'irreducible_specialization_degree':3,'residue_field_degree':42,'actual_Tate_point_exact_order':77,'CRT_projectors':[56,22],'diamond_generator':23,'diamond_order':3,'source_genus':61,'X1_77_genus':181,'generic_cubic_irreducibility_under_monic_DVR_bridge':True,'proper_moduli_identification':'written; MODULI_BRIDGE.md','independent_MIT_global_birational_identity_verified':False},proof={'named_theorems':2,'audited_declarations':2,'failing':0,'trusted_native':0,'run':'lean29-01','actual_norm_and_moduli_geometry_formalized':False})
for record in [r27,r28,r29]:
 rid=record['round'];record['runs']=inventory(rid)
 record['flow']={}
 for row in record['runs']:
  key=row['stage']+'_'+row['status'];record['flow'][key]=record['flow'].get(key,0)+1
 if rid==29:
  p=base/'29/runs/dependencies29-01/native.log'
  if p.exists():
   log=p.read_text();matches=re.findall(r'IMAGEDEPENDENCIES29_CASE (\d+) degree=(\d+) id,neg,T,negT offsets\n([^\n]+)',log)
   record['image_dependencies']={'classification':'original E49 closed images only; not colored C5 columns','ordinary_product_plus_one_base_closed_divisor_cancels':True,'base_cancellation_degree':460,'asymmetric_supports_not_matched_by_tested_deck_actions':2,'records':[{'case':int(i),'degree':int(d),'frobenius_offsets_id_neg_T_negT':[int(n)for n in line.split(',')]}for i,d,line in matches]}
 write(base/str(rid)/'results.json',record)
 print('assembled',rid,record['flow'])
