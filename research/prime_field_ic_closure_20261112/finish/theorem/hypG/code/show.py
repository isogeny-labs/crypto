import json, sys
K = ['m','k','p','n','g','j0','sec','B','Ndec_targets','min_dweak_nondegen','min_dweak_dec_nondegen','min_ddet','min_ddet_nondegen','proven_floor_k_m_1','weak_floor_k_m_1_ok','det_floor_k_m_1_ok','det_line_floor_ok','uncertainty_ok','RM_analogue_violations','max_t_original_G','max_t_all_targets_good_amended','Delta_values','n_degenerate_abscissas','max_nV_nondegen']
for f in sys.argv[1:]:
    d = json.load(open(f))['summary']; print({k: d.get(k) for k in K})
