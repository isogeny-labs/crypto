// Development integration checks. The expected group arithmetic below comes
// from the separately authored audit; field enumeration is test-only.
#define main independent_paper_audit_entrypoint
#include "audit.cpp"
#undef main
#include "incidence.hpp"
#include "isogeny.hpp"
#include <optional>

namespace integration {
using namespace audit;
using paper_ec::LabelledPools;
using paper_ec::LabelledPoint;
using paper_ec::RootBackend;
struct KnownCase { LabelledPools problem; Word secret = 0; };
struct Witness { std::array<std::size_t, 3> at{}; std::array<int, 3> sign{}; };
struct Truth { std::set<Word> xs; std::vector<Witness> witnesses; bool informative = false; };
struct Panel {
    Runner checks;
    paper_ec::Stats construction, validation, generation, solving, isogeny_checks;
    Word natural_cases = 0, natural_hits = 0, natural_misses = 0, solver_calls = 0;
    Word nonlinear_root_calls = 0, uninformative_controls = 0, isogeny_pairs = 0;
    Word mapped_basis_order_product = 0, codomain_basis_order_product = 0;
    std::vector<Word> codomain_basis_orders;
};
bool prime(Word n) { if (n < 2) return false; for (Word d = 2; d <= n / d; ++d) if (n % d == 0) return false; return true; }
std::pair<Curve,Point> target_curve(Word p) {
    for (Word a = 1; a < 8; ++a) for (Word b = 1; b < 12; ++b) {
        if ((4 * a * a * a + 27 * b * b) % p == 0) continue;
        const Curve c{p, a, b}; const auto ps = points(c);
        if (ps.size() != p && prime(ps.size())) return {c, ps[1]};
    }
    throw std::runtime_error("no bounded public prime-order target fixture");
}
KnownCase make_case(const Curve& c, Point generator, unsigned B, Word seed,
                    paper_ec::Stats& generation) {
    KnownCase out; auto& p = out.problem;
    p.target = c; p.generator = generator; p.subgroup_order = points(c).size();
    out.secret = 1 + (seed * 17 + 3) % (p.subgroup_order - 1);
    p.public_target = times(c, out.secret, generator); // fixture setup, not a solver input scalar
    std::mt19937_64 rng(seed); std::set<Word> used;
    for (auto& pool : p.pools) while (pool.size() < B) {
        const Word alpha = rng() % p.subgroup_order, beta = rng() % p.subgroup_order;
        generation.add("pool.public_draws");
        auto point = paper_ec::point_add(c,
            paper_ec::point_mul(c, alpha, generator, &generation),
            paper_ec::point_mul(c, beta, p.public_target, &generation), &generation);
        if (point.infinity) { generation.add("pool.rejected_infinity"); continue; }
        if (!used.insert(point.x).second) { generation.add("pool.rejected_coordinate"); continue; }
        pool.push_back({point, alpha, beta}); generation.add("pool.accepted_points");
        need(point == sum(c, times(c, alpha, generator), times(c, beta, p.public_target)),
             "public mixed label differs from independent arithmetic");
    }
    return out;
}
Truth truth(const LabelledPools& p) {
    Truth result; const auto& c = p.target; const Word n = p.subgroup_order;
    std::vector<std::size_t> order(p.pools[2].size());
    for (std::size_t i = 0; i < order.size(); ++i) order[i] = i;
    std::sort(order.begin(), order.end(), [&](auto a, auto b) { return p.pools[2][a].point.x < p.pools[2][b].point.x; });
    for (const auto k : order) for (std::size_t i = 0; i < p.pools[0].size(); ++i)
        for (const int s1 : {1,-1}) for (std::size_t j = 0; j < p.pools[1].size(); ++j)
            for (const int s2 : {1,-1}) {
                auto a = p.pools[0][i].point, b = p.pools[1][j].point;
                if (s1 < 0) a = neg(a, c.p);
                if (s2 < 0) b = neg(b, c.p);
                if (!sum(c, sum(c, a, b), p.pools[2][k].point).infinity) continue;
                result.xs.insert(p.pools[2][k].point.x);
                result.witnesses.push_back({{i,j,k},{s1,s2,1}});
                Word B = 0;
                for (std::size_t h = 0; h < 3; ++h) {
                    const auto beta = p.pools[h][std::array<std::size_t,3>{i,j,k}[h]].beta;
                    B = add(B, std::array<int,3>{s1,s2,1}[h] > 0 ? beta : sub(0,beta,n), n);
                }
                if (B) result.informative = true;
            }
    return result;
}
void run_case(Panel& panel, const KnownCase& known, const paper_ec::PreparedDatum& auxiliary,
              bool require_uninformative_skip = false) {
    const auto& p = known.problem; const auto expected = truth(p);
    const std::vector<Word> roots(expected.xs.begin(), expected.xs.end());
    const auto expected_gcd = vanish(roots, p.target.p);
    for (const auto backend : {RootBackend::KnownCoordinates, RootBackend::PaperTrace, RootBackend::PaperDense, RootBackend::Pullback}) {
        paper_ec::Stats counts;
        const auto answer = paper_ec::solve_incidence(p, backend, &auxiliary, &counts);
        need(answer.incidence_gcd == expected_gcd, "entire native incidence GCD support differs from independent signed enumeration");
        need(answer.coordinate_relation == !roots.empty(), "coordinate existence differs");
        need(answer.informative_relation == expected.informative, "informative existence differs");
        if (answer.informative_relation) {
            need(answer.recovered_scalar == known.secret, "recovered scalar differs from test-only truth");
            need(times(p.target, answer.recovered_scalar, p.generator) == p.public_target, "independent original-curve scalar check fails");
            Point sum_point;
            Word A = 0, B = 0;
            for (std::size_t h = 0; h < 3; ++h) {
                const auto entry = p.pools[h][answer.positions[h]];
                need(answer.signs[h] == 1 || answer.signs[h] == -1, "bad witness sign");
                sum_point = sum(p.target, sum_point, answer.signs[h] > 0 ? entry.point : neg(entry.point,p.target.p));
                A = add(A, answer.signs[h] > 0 ? entry.alpha : sub(0,entry.alpha,p.subgroup_order), p.subgroup_order);
                B = add(B, answer.signs[h] > 0 ? entry.beta : sub(0,entry.beta,p.subgroup_order), p.subgroup_order);
            }
            need(sum_point.infinity && B && add(A,mul(B,known.secret,p.subgroup_order),p.subgroup_order)==0, "signed public-label witness check fails");
        }
        if (require_uninformative_skip) need(counts.counts["incidence.uninformative_witnesses"] > 0 && answer.informative_relation,
            "uninformative-first continuation was not exercised");
        if (backend == RootBackend::Pullback) {
            need(counts.counts["translation_calls"] == 0 && counts.counts["pullback_full_translation_calls"] == 0,
                 "pullback incidence unexpectedly reconstructs translated divisors");
        }
        panel.nonlinear_root_calls += counts.counts["incidence.nonlinear_root_backend_calls"];
        panel.solving.merge(counts); ++panel.solver_calls;
    }
}
void isogeny_checks(Panel& panel) {
    const auto original = paper_ec::paper_f83_datum();
    const auto map = paper_ec::two_isogeny(original.curve, original.basis.first, &panel.construction);
    need(map.codomain.a == 37 && map.codomain.b == 74, "classical two-isogeny coefficients differ");
    const auto domain_points = points(original.curve);
    std::set<Point> image;
    for (const auto a : domain_points) {
        auto ia = paper_ec::isogeny_image(map, a, &panel.isogeny_checks);
        need(on(map.codomain, ia), "isogeny codomain equation differs from independent arithmetic");
        image.insert(ia);
        for (const auto b : domain_points) {
            const auto lhs = paper_ec::isogeny_image(map, sum(original.curve,a,b), &panel.isogeny_checks);
            const auto rhs = sum(map.codomain, ia, paper_ec::isogeny_image(map,b,&panel.isogeny_checks));
            need(lhs == rhs, "isogeny is not a homomorphism under independent group law"); ++panel.isogeny_pairs;
        }
    }
    need(image.size()*2 == domain_points.size(), "isogeny rational image index is not two");
    const auto image1 = paper_ec::isogeny_image(map, original.basis.first, &panel.construction);
    const auto image2 = paper_ec::isogeny_image(map, original.basis.second, &panel.construction);
    panel.mapped_basis_order_product = order(map.codomain,image1)*order(map.codomain,image2);
    need(panel.mapped_basis_order_product == 36, "mapped generators unexpectedly form a full codomain basis");
    const auto rebuilt = paper_ec::construct_toy_datum(map.codomain, original.beta, &panel.construction);
    paper_ec::PreparedDatum prepared(rebuilt, &panel.validation);
    need(rebuilt.order == original.order && rebuilt.twist_order == original.twist_order, "isogeny changed rational orders");
    need(rebuilt.twist.a==30 && rebuilt.twist.b==30, "isogeny codomain twist scaling differs");
    panel.codomain_basis_order_product = rebuilt.basis.first_order*rebuilt.basis.second_order;
    panel.codomain_basis_orders = {rebuilt.basis.first_order,rebuilt.basis.second_order,rebuilt.twist_basis.first_order,rebuilt.twist_basis.second_order};
    need(panel.codomain_basis_order_product == 72 && panel.construction.counts["auxiliary.exhaustive_coordinate_pairs"] == 2*83*83,
         "codomain basis completion or charged exhaustive search missing");
    const Word inv_beta = inverse(original.beta,83);
    const Point twist_kernel = affine(mul(original.basis.first.x,inv_beta,83),0);
    const auto twisted_map = paper_ec::two_isogeny(original.twist,twist_kernel,&panel.construction);
    need(twisted_map.codomain.a==rebuilt.twist.a && twisted_map.codomain.b==rebuilt.twist.b,
         "two-isogeny and quadratic twisting do not commute on coefficients");
    for (const auto point : points(original.twist)) {
        if (point.infinity || point==twist_kernel) continue;
        const auto actual = paper_ec::isogeny_image(twisted_map,point,&panel.isogeny_checks);
        // In E(F_p^2), y_E=(beta*y_tw)*sqrt(beta); compare that coefficient.
        const Word x = mul(original.beta,point.x,83), y_coefficient=mul(original.beta,point.y,83);
        const Word inv_den=inverse(sub(x,original.basis.first.x,83),83);
        const Word expected_x=mul(add(x,mul(map.kernel_coefficient,inv_den,83),83),inv_beta,83);
        const Word expected_y=mul(mul(y_coefficient,sub(1,mul(map.kernel_coefficient,mul(inv_den,inv_den,83),83),83),83),inv_beta,83);
        need(actual==affine(expected_x,expected_y), "two-isogeny/twist ordinate coefficient identity fails");
    }
    const Poly f{26,12,7,48,4,58,78,41,32,28,1};
    for (const auto method : {TranslationMethod::Trace,TranslationMethod::Dense})
        check_roots(f,paper_ec::root_find(prepared,f,method,&panel.solving),83);
    check_roots(f,paper_ec::root_find_pullback(prepared,f,&panel.solving),83);
    panel.checks.rejects("reject-nontorsion-isogeny-kernel",[&]{paper_ec::two_isogeny(original.curve,original.basis.second);});
    panel.checks.rejects("refuse-unbounded-exhaustive-datum",[&]{paper_ec::construct_toy_datum({4099,1,1},2);});
}
void incidence_checks(Panel& panel) {
    bool made_uninformative_control=false; std::optional<KnownCase> nonlinear_case;
    for (const Word p : {Word(83),Word(97)}) {
        const auto auxiliary = p==83 ? paper_ec::paper_f83_datum()
            : paper_ec::construct_toy_datum({97,2,3},5,&panel.construction);
        paper_ec::PreparedDatum prepared(auxiliary,&panel.validation);
        const auto target = target_curve(p);
        for (const unsigned B : {2u,3u,4u}) for (Word seed=0; seed<10; ++seed) {
            auto known = make_case(target.first,target.second,B,2026111600+p+seed+17*B,panel.generation);
            const auto expected=truth(known.problem); ++panel.natural_cases;
            if(expected.informative)++panel.natural_hits;else ++panel.natural_misses;
            run_case(panel,known,prepared);
            if (expected.xs.size()>1) nonlinear_case=known;
            if (!made_uninformative_control && expected.xs.size()>1) {
                auto changed=known;const auto w=expected.witnesses.front();const Word n=changed.problem.subgroup_order;
                auto& third=changed.problem.pools[2][w.at[2]];
                const Word log_third=add(third.alpha,mul(changed.secret,third.beta,n),n);
                Word other_beta=0;
                for(std::size_t h=0;h<2;++h){const auto beta=changed.problem.pools[h][w.at[h]].beta;other_beta=add(other_beta,w.sign[h]>0?beta:sub(0,beta,n),n);}
                third.beta=sub(0,other_beta,n);third.alpha=sub(log_third,mul(changed.secret,third.beta,n),n);
                // Explicit correctness-only relabelling uses the toy secret.
                if(truth(changed.problem).informative){run_case(panel,changed,prepared,true);made_uninformative_control=true;++panel.uninformative_controls;}
            }
        }
        auto bad=make_case(target.first,target.second,2,100+p,panel.generation);
        bad.problem.pools[1][0]=bad.problem.pools[0][0];
        panel.checks.rejects("incidence-duplicate-x-p"+std::to_string(p),[&]{paper_ec::solve_incidence(bad.problem,RootBackend::Pullback,&prepared);});
    }
    need(made_uninformative_control && panel.nonlinear_root_calls>0 && panel.natural_hits>0 && panel.natural_misses>0,
         "panel did not exercise a required positive/miss/nonlinear/continuation branch");
    need(nonlinear_case.has_value(),"no nonlinear fixture");
    panel.checks.rejects("incidence-missing-nonlinear-auxiliary",[&]{paper_ec::solve_incidence(nonlinear_case->problem,RootBackend::Pullback,nullptr);});
    paper_ec::PreparedDatum wrong(paper_ec::paper_f83_datum());
    if(nonlinear_case->problem.target.p==97)panel.checks.rejects("incidence-mismatched-auxiliary-field",[&]{paper_ec::solve_incidence(nonlinear_case->problem,RootBackend::Pullback,&wrong);});
}
void output_counts(const paper_ec::Stats& s) {
    std::cout<<'{';bool first=true;for(const auto& entry:s.counts){if(!first)std::cout<<',';first=false;std::cout<<quote(entry.first)<<':'<<entry.second;}std::cout<<'}';
}
} // namespace integration
int main() {
    integration::Panel p;
    p.checks.check("auxiliary-two-isogeny-and-twist",[&]{integration::isogeny_checks(p);});
    p.checks.check("original-curve-incidence-recovery",[&]{integration::incidence_checks(p);});
    const auto failures=std::count_if(p.checks.rows.begin(),p.checks.rows.end(),[](const audit::Row& r){return r.status!="passed";});
    std::cout<<"{\n\"schema\":\"paper-native-integration-audit/v1\",\n\"pcc_standing\":\"unverified\",\n\"status\":"<<audit::quote(failures?"failed":"passed")<<",\n\"timings_collected\":false,\n\"natural_cases\":"<<p.natural_cases<<",\n\"natural_hits\":"<<p.natural_hits<<",\n\"natural_misses\":"<<p.natural_misses<<",\n\"complete_incidence_calls\":"<<p.solver_calls<<",\n\"nonlinear_root_backend_calls\":"<<p.nonlinear_root_calls<<",\n\"uninformative_first_controls\":"<<p.uninformative_controls<<",\n\"isogeny_homomorphism_pairs\":"<<p.isogeny_pairs<<",\n\"mapped_generator_order_product\":"<<p.mapped_basis_order_product<<",\n\"completed_codomain_basis_order_product\":"<<p.codomain_basis_order_product<<",\n\"codomain_basis_orders\":[";
    for(std::size_t i=0;i<p.codomain_basis_orders.size();++i){if(i)std::cout<<',';std::cout<<p.codomain_basis_orders[i];}
    std::cout<<"],\n\"rows\":[";bool first=true;for(const auto& row:p.checks.rows){if(!first)std::cout<<',';first=false;std::cout<<"{\"name\":"<<audit::quote(row.name)<<",\"status\":"<<audit::quote(row.status)<<",\"message\":"<<audit::quote(row.message)<<'}';}
    std::cout<<"],\n\"construction_counts\":";integration::output_counts(p.construction);
    std::cout<<",\n\"validation_counts\":";integration::output_counts(p.validation);
    std::cout<<",\n\"public_pool_generation_counts\":";integration::output_counts(p.generation);
    std::cout<<",\n\"aggregate_solver_counts\":";integration::output_counts(p.solving);
    std::cout<<",\n\"limitations\":[\"Development correctness panel only; no timing, GAE pricing or fitted exponent\",\"Construction is exhaustive and refused above p257; not a general efficient preprocessing algorithm\",\"Validation scalars and secret-based relabelling are used only in explicit correctness controls\",\"Operation counts aggregate unlike calls; they are not an end-to-end performance ratio\"]\n}\n";
    return failures?1:0;
}
