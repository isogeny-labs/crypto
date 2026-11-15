# Copy the fitted exponent of S (fit.alpha, fit.alpha_ci95) and the bound id
# from the 12 records (jq -s over docs/bounds/records/cheon-ecbench-wide-*.json).
# No computation beyond selection.
[ .[] | { family: (if .fit_options.oracle | test("n-1") then "n-1" else "n+1" end),
          arm: (if .method.id | startswith("rho") then "rho.negation" else .method.id end),
          w: (.fit_options.weight // 0), alpha: .fit.alpha,
          lo: .fit.alpha_ci95[0], hi: .fit.alpha_ci95[1], bound_id } ]
| sort_by(.family, .w, .arm)
