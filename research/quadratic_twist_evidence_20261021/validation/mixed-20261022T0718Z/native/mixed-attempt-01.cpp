// Exhaustive finite mixed-component relation experiment; no timing claims.
#define TWIST_FULL_NO_MAIN
#include "full.cpp"
#include <functional>

using Row = std::vector<I>;
Json point_json(Point P) { return nums({P.x, P.y}); }
Json row_json(const std::set<Row> &rows) {
  auto j = arr();
  for (const auto &row : rows) j.items.push_back(nums(row));
  return j;
}
I row_rank(const std::set<Row> &rows, I r, I width) {
  LinearSystem sys{r, width, {}};
  for (const auto &row : rows) sys.insert(row, 0);
  return I(sys.rows.size());
}
I scalar_json(const Json &j) {
  return j.kind == Json::String ? std::stoll(j.string()) : j.integer();
}
struct ModelNames {
  std::map<std::tuple<I,I,I>,std::string> ids;
  explicit ModelNames(const Json &registry) {
    for (const auto &m : registry.at("curves").items) {
      const auto &p = m.at("params");
      if (m.at("family").string() != "prime" || !p.fields.count("p") ||
          !p.fields.count("a") || !p.fields.count("b") ||
          p.at("a").kind == Json::Array || p.at("b").kind == Json::Array) continue;
      ids[{scalar_json(p.at("p")), scalar_json(p.at("a")), scalar_json(p.at("b"))}] = m.at("slug").string();
    }
  }
  std::string get(I p, I a, I b) const {
    auto i = ids.find({p,a,b});
    require(i != ids.end(), "unregistered model");
    return i->second;
  }
};
Point generator(const Curve &C, const std::vector<Point> &points, I r) {
  for (auto P : points)
    if (!P.infinity() && C.scalar(r,P).infinity()) return P;
  throw std::runtime_error("missing subgroup generator");
}

Json subgroup(const Curve &L, Point P, Point T, I r) {
  require(!P.infinity() && !T.infinity() && L.scalar(r,P).infinity() &&
          L.scalar(r,T).infinity(), "subgroup orders");
  require(L.frob(P) == P && L.frob(T) == L.neg(T), "Frobenius generators");
  std::set<Point> product;
  for (I a=0;a<r;++a) for (I b=0;b<r;++b)
    product.insert(L.add(L.scalar(a,P),L.scalar(b,T)));
  require(I(product.size()) == r*r, "independent eigenspaces");
  require(!L.add(P,L.neg(T)).infinity(), "false identified-log control");
  I B=(r-1)/2, width=2*B;
  struct Letter { Point p; I plus, minus, col, sign; };
  std::vector<Letter> alphabet;
  for (I component=0;component<2;++component)
    for (I j=1;j<=B;++j) for (I sign : {I(1),I(-1)})
      alphabet.push_back({L.scalar(sign*j,component ? T:P),
          component ? 0:sign*j,component ? sign*j:0,component*B+j-1,sign});
  std::set<Row> all,pure_plus,pure_minus;
  auto levels=arr();
  for (I length=3;length<=6;++length) {
    std::vector<I> words(size_t(length+1)),accepted(size_t(length+1)),zero(size_t(length+1));
    std::set<Row> exact;
    Row row(size_t(width),0);
    std::vector<size_t> word;
    std::function<void(I,I,I,I)> visit = [&](I depth,I a,I b,I plus_letters) {
      if (depth == length) {
        ++words[size_t(plus_letters)];
        if (mod(a,r) || mod(b,r)) return;
        Point actual;
        for (auto i:word) actual=L.add(actual,alphabet[i].p);
        require(actual.infinity(), "residue acceptance disagrees with actual point addition");
        ++accepted[size_t(plus_letters)];
        bool is_zero=std::all_of(row.begin(),row.end(),[](I x){return x==0;});
        zero[size_t(plus_letters)]+=is_zero;
        exact.insert(row); all.insert(row);
        if (plus_letters == length) pure_plus.insert(row);
        if (plus_letters == 0) pure_minus.insert(row);
        return;
      }
      for (size_t i=0;i<alphabet.size();++i) {
        const auto &v=alphabet[i]; row[size_t(v.col)]+=v.sign; word.push_back(i);
        visit(depth+1,a+v.plus,b+v.minus,plus_letters+(v.plus!=0));
        word.pop_back(); row[size_t(v.col)]-=v.sign;
      }
    };
    visit(0,0,0,0);
    I rp=row_rank(pure_plus,r,width),rt=row_rank(pure_minus,r,width),ra=row_rank(all,r,width);
    require(ra==rp+rt, "mixed rows add relation rank");
    levels.items.push_back(obj({{"length",num(length)},{"candidate_words_by_plus_arity",nums(words)},
      {"zero_words_by_plus_arity",nums(accepted)},{"zero_coefficient_words_by_plus_arity",nums(zero)},
      {"distinct_rows_exact_length",num(I(exact.size()))},{"all_rank_up_to_length",num(ra)},
      {"pure_plus_rank_up_to_length",num(rp)},{"pure_minus_rank_up_to_length",num(rt)}}));
  }
  return obj({{"r",num(r)},{"P",point_json(P)},{"T",point_json(T)},
    {"basis_size_per_component",num(B)},{"product_table_size",num(I(product.size()))},
    {"identified_log_false_relation",nums({1,-1})},{"identified_log_control_rejected",truth()},
    {"levels",levels},{"all_rows",row_json(all)},
    {"pure_plus_rows",row_json(pure_plus)},{"pure_minus_rows",row_json(pure_minus)}});
}

int main(int argc,char **argv) {
  try {
    require(argc==3,"usage: mixed REGISTRY OUTPUT");
    const auto registry_bytes=read(argv[1]); ModelNames names(parse(registry_bytes));
    auto models=arr(),totals=obj(); I model_count=0,case_count=0,pair_count=0,exceptions=0;
    for (I p : {5,7,11,13,17,19}) {
      I d=nonsquare(p); Field k(p),K(p,d);
      for (I a=0;a<p;++a) for (I b=0;b<p;++b) {
        if (!mod(4*a*a*a+27*b*b,p)) continue;
        Curve E(k,a,b),D(k,a,b,d),L(K,a,b); auto ep=E.points(),dp=D.points();
        std::vector<Point> tp; for (auto P:dp) tp.push_back(image(P,K));
        I rational_sums=0,nonzero_exceptions=0;
        for (auto A:ep) for (auto B:tp) {
          ++pair_count; auto R=L.add(A,B); bool rational=L.frob(R)==R;
          require(rational == L.scalar(2,B).infinity(),"mixed rational sum counterexample");
          if (rational) {++rational_sums;if(!B.infinity()){++nonzero_exceptions;++exceptions;}}
        }
        auto cases=arr();
        for (auto [r,e]:factors(std::gcd(I(ep.size()),I(dp.size())))) {
          (void)e; if(r==2) continue;
          cases.items.push_back(subgroup(L,generator(E,ep,r),image(generator(D,dp,r),K),r));
          ++case_count;
        }
        // Both controls stay on the valid source curve.  A square-parameter
        // image is rational; a common nonzero 2-torsion point survives mixed sums.
        Json two, square;
        for(auto B:ep) {
          if(!B.infinity() && L.scalar(2,B).infinity() && two.kind==Json::Null)
            two=point_json(B);
          if(!B.infinity() && !L.scalar(2,B).infinity() && square.kind==Json::Null) {
            require(L.frob(B)==B && L.frob(B)!=L.neg(B),"square-parameter control");
            square=point_json(B); // d=1,u=1,x/y unchanged.
          }
        }
        models.items.push_back(obj({{"p",num(p)},{"a",num(a)},{"b",num(b)},{"d",num(d)},
          {"curve",str(names.get(p,a,b))},{"twist_normalized",str(names.get(p,a*d*d%p,b*d*d%p*d%p))},
          {"curve_order",num(I(ep.size()))},{"twist_order",num(I(dp.size()))},
          {"mixed_pairs",num(I(ep.size()*dp.size()))},{"rational_mixed_sums",num(rational_sums)},
          {"nonzero_two_torsion_mixed_sums",num(nonzero_exceptions)},
          {"two_torsion_control",two},{"square_parameter_control",square},{"subgroups",cases}}));
        ++model_count;
      }
      std::cout<<"PASS p="<<p<<", cumulative models="<<model_count<<", subgroups="<<case_count<<'\n'<<std::flush;
    }
    require(model_count==942 && case_count==274 && exceptions>0,"declared coverage/control census");
    auto output=obj({{"schema",str("quadratic-twist-mixed-relations/v1")},{"status",str("passed")},
      {"registry_sha256",str(digest(registry_bytes))},{"models",models},
      {"totals",obj({{"models",num(model_count)},{"shared_odd_prime_subgroups",num(case_count)},
        {"mixed_pairs",num(pair_count)},{"nonzero_two_torsion_mixed_sums",num(exceptions)},
        {"mixed_component_counterexamples",num(0)},{"rank_gain_cases",num(0)}})}});
    std::ofstream(argv[2])<<dump(output)<<'\n';
  } catch(const std::exception &e) {std::cerr<<"FAIL: "<<e.what()<<'\n';return 1;}
}
