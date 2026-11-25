#include "rank_arithmetic.hpp"
int main(int argc,char** argv) {
    try {
        require(argc==5,"usage: classify BASELINE.json AUDIT.json TARGET.json OUTPUT.json");
        const auto data=read_json(argv[1]),audit=read_json(argv[2]),target=read_json(argv[3]);
        const size_t width=data.at("projected_base").arr.size();
        auto row_for=[&](const Json& sparse) {
            Row row(width);
            for(const auto& entry:sparse.arr) row.at(size_t(entry.arr.at(0).integer()))+=mpz_class(entry.arr.at(1).scalar);
            return row;
        };
        Matrix old;
        for(const auto& row:data.at("rows").arr) old.push_back(row_for(row));
        require(width==225&&old.size()==6&&matrix_rank(old)==6,"original rank binding");
        require(target.at("baseline_principal_checks").integer()==6&&target.at("mutated_baseline_rejected").scalar=="true","target baseline/negative controls");
        const auto& rows=audit.at("signed_rows").arr;
        const auto& decisions=target.at("classifications").arr;
        require(rows.size()==decisions.size()&&audit.at("halftrace_alias_rows").arr.empty(),"complete ordered classification binding");
        Matrix all=old,principal=old;
        int zero=0,nonzero=0;
        for(size_t i=0;i<rows.size();++i) {
            const auto row=row_for(rows[i]);all.push_back(row);
            require(decisions[i].scalar=="true"||decisions[i].scalar=="false","explicit target decision");
            if(decisions[i].scalar=="true") {principal.push_back(row);++zero;}
            else ++nonzero;
        }
        require(zero==target.at("new_principal_rows").integer()&&nonzero==target.at("new_nonprincipal_norm_rows").integer(),"classification count binding");
        const size_t all_rank=matrix_rank(all),principal_rank=matrix_rank(principal);
        require(all_rank==size_t(audit.at("all_norm_rank").integer()),"all-norm rank agreement");
        std::ofstream out(argv[4]);require(bool(out),"fresh classification output");
        out << "{\"standing\":\"producer-unverified\",\"baseline_rank\":6,\"norm_candidates\":" << rows.size()
            << ",\"principal_candidates\":" << zero << ",\"nonprincipal_norm_candidates\":" << nonzero
            << ",\"all_norm_rank\":" << all_rank << ",\"principal_rank\":" << principal_rank
            << ",\"principal_gain\":" << principal_rank-6 << ",\"new_curve_points\":0}\n";
        require(bool(out),"complete classification output");
    } catch(const std::exception& error) { std::cerr << "REJECT " << error.what() << '\n';return 1; }
}
