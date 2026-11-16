// Additional input coverage using the independently authored verifier unchanged.
// This includes a positive characteristic-p torsion case, not just psi_p
// evaluations on groups of order coprime to p. No timings are collected.
#define main independent_paper_audit_entrypoint
#include "audit.cpp"
#undef main

int main() {
    audit::Runner checks;
    std::mt19937_64 seed(2026111607);
    checks.check("anomalous-auxiliary-positive-characteristic-torsion", [&] {
        const auto datum = audit::datum({5,3,2});
        audit::need(datum.order==5 && datum.twist_order==7, "edge datum has wrong rational orders");
        paper_ec::PreparedDatum prepared(datum,&checks.setup);
        for (const auto point : audit::points(datum.curve)) {
            if (point.infinity) continue;
            audit::need(audit::times(datum.curve,5,point).infinity, "missing actual rational p-torsion");
        }
        audit::group_checks(checks,datum.curve);
        audit::group_checks(checks,datum.twist);
        audit::psi_checks(checks,datum.curve);
        audit::psi_checks(checks,datum.twist);
        audit::root_checks(checks,datum,seed);
    });
    audit::output(checks,std::cout);
    return std::any_of(checks.rows.begin(),checks.rows.end(),
        [](const audit::Row& row){return row.status!="passed";})?1:0;
}
