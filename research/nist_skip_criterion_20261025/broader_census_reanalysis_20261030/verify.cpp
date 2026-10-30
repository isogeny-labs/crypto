#include "io.hpp"
#include <pari/pari.h>
#include <algorithm>
#include <iostream>
#include <map>
#include <set>
struct Record {
    std::vector<std::string> text;
    GEN delta;
    std::vector<int> strict, ramified;
};
static std::string decimal(GEN value) {
    char* raw = GENtostr(value);
    std::string text(raw);
    pari_free(raw);
    return text;
}
static std::string windows(const std::vector<bool>& yes) {
    std::ostringstream out;
    out << '[';
    int begin = -1, emitted = 0;
    for (int cutoff = 100; cutoff <= 5001; ++cutoff) {
        bool ok = cutoff <= 5000 && yes[static_cast<size_t>(cutoff-100)];
        if (ok && begin < 0) begin = cutoff;
        if (!ok && begin >= 0) {
            if (emitted++) out << ',';
            out << '[' << begin << ',' << cutoff-1 << ']';
            begin = -1;
        }
    }
    out << ']';
    return out.str();
}
int main(int argc, char** argv) {
try {
    pari_init(128000000, 1000000);
    if (argc == 2 && std::string(argv[1]) == "--self-test") {
        GEN e = ellinit(mkvec2(stoi(-3), gen_1), stoi(17), DEFAULTPREC);
        GEN g = mkvec2(gen_0, gen_1);
        require(equaliu(ellcard(e,nullptr),20) && lg(ellmul(e,g,stoi(10))) == 2,
                "independent F17 group control");
        require(kronecker(stoi(9),stoi(11)) == 1 && kronecker(stoi(3),stoi(5)) == -1
                && kronecker(gen_0,stoi(7)) == 0, "character controls");
        std::cout << "PASS PARI exhaustive toy order and three characters\n";
        return 0;
    }
    require(argc == 5, "verify INPUTS PRIOR21 OUTPUTS FRESH_RECEIPT");
    auto source = table(argv[1],14), old = table(argv[2],9);
    require(source.size() == 82 && old.size() == 21, "full inventories");
    std::vector<int> primes;
    for (int n = 2; n <= 5000; ++n) if (uisprime(static_cast<ulong>(n))) primes.push_back(n);
    require(primes.size() == 669, "all primes");
    std::vector<Record> records;
    std::set<std::string> ids, prime_cache;
    std::map<std::string,std::vector<size_t>> positions;
    std::ostringstream chars;
    chars << "branch\tell\tcharacter\tstrict\tramified\tinclusive\n";
    for (const auto& row : source) {
        require(ids.insert(row[0]).second, "branch duplicate");
        pari_sp mark = avma;
        GEN p = gp_read_str(row[7].c_str()), b = gp_read_str(row[8].c_str()),
            n = gp_read_str(row[9].c_str()), x = gp_read_str(row[10].c_str()),
            y = gp_read_str(row[11].c_str());
        for (GEN v : {p,n}) {
            std::string key = decimal(v);
            if (prime_cache.insert(key).second) require(isprime(v), "proved primality");
        }
        require(cmpis(p,5000) > 0 && signe(b) >= 0 && cmpii(b,p) < 0
                && signe(x) >= 0 && cmpii(x,p) < 0 && signe(y) >= 0 && cmpii(y,p) < 0,
                "field/coordinate bounds");
        GEN e = ellinit(mkvec2(stoi(-3),b),p,DEFAULTPREC), q = mkvec2(x,y);
        require(lg(e) > 1 && !gequal0(ellisoncurve(e,q)), "nonsingular point equation");
        require(lg(ellmul(e,q,n)) == 2, "independent order-point certificate");
        GEN trace = subii(addis(p,1),n), delta = subii(sqri(trace),shifti(p,2));
        require(signe(delta) < 0 && cmpii(sqri(n),shifti(p,4)) > 0, "Hasse uniqueness");
        Record r{row,gclone(delta),{}, {}};
        int strict = 0, zero = 0;
        for (int ell : primes) {
            if (ell == 2) chars << row[0] << "\t2\tNA_ell2\t0\t0\t0\n";
            else {
                long character = kronecker(delta,stoi(ell));
                strict += character == 1; zero += character == 0;
                chars << row[0] << '\t' << ell << '\t' << character << '\t'
                      << strict << '\t' << zero << '\t' << strict+zero << '\n';
            }
            r.strict.push_back(strict); r.ramified.push_back(zero);
        }
        positions[row[1]].push_back(records.size());
        records.push_back(std::move(r));
        avma = mark;
    }
    require(positions.size() == 69, "69positions");
    std::set<size_t> picked;
    for (const auto& group : positions) {
        size_t selected = group.second.front();
        for (size_t i : group.second) {
            const auto& r = records[i]; const auto& first = records[group.second.front()];
            require(equalii(r.delta,first.delta) && r.text[2] == first.text[2]
                    && r.text[3] == first.text[3] && r.text[4] == first.text[4]
                    && r.text[6] == first.text[6], "all branch invariants");
            if (r.text[5] == "canonical") selected = i;
        }
        picked.insert(selected);
    }
    int published = 0, narrow = 0;
    for (size_t i : picked) { published += records[i].text[3] == "published"; narrow += records[i].text[6] == "1"; }
    require(published == 10 && narrow == 21, "publication/narrow inventories");
    for (const auto& row : old) {
        int matched = 0;
        for (size_t i : picked) if (records[i].text[1] == row[0]) {
            ++matched;
            const auto& c = records[i].text;
            pari_sp mark = avma;
            require(c[6] == "1" && equalii(gp_read_str(c[7].c_str()),gp_read_str(row[5].c_str()))
                    && equalii(gp_read_str(c[9].c_str()),gp_read_str(row[7].c_str())), "prior exact identity");
            avma = mark;
        }
        require(matched == 1, "prior identity inventory");
    }
    auto index = [&](int cutoff) {
        size_t j = 0;
        while (j+1 < primes.size() && primes[j+1] <= cutoff) ++j;
        return j;
    };
    auto value = [&](size_t i,size_t j,bool inclusive) {
        return records[i].strict[j]+(inclusive ? records[i].ramified[j] : 0);
    };
    auto gap = [&](int bits,size_t j,bool inclusive,bool original) {
        int result = 10000; bool found = false;
        for (size_t pub : picked) if (std::stoi(records[pub].text[2]) == bits
                && records[pub].text[3] == "published") {
            for (size_t unpub : picked) if (records[unpub].text[2] == records[pub].text[2]
                    && records[unpub].text[3] == "unpublished"
                    && (!original || records[unpub].text[6] == "1")) {
                found = true;
                result = std::min(result,value(unpub,j,inclusive)-value(pub,j,inclusive));
            }
        }
        require(found,"paired field"); return result;
    };
    auto separates = [&](size_t j,bool inclusive,bool original) {
        bool result = true;
        for (int bits : {192,224,239,256,384}) if (!original || bits != 384)
            if (gap(bits,j,inclusive,original) <= 0) result = false;
        return result;
    };
    std::ostringstream profiles, cuts, branches, violations;
    profiles << "seed_id\tbits\trole\tcategory\tcutoff\tstrict\tramified\tinclusive\n";
    cuts << "cutoff\tnarrow_strict\tnarrow_inclusive\tbroader_strict\tbroader_inclusive\tmargin192\tmargin224\tmargin239\tmargin256\tmargin384\n";
    std::vector<bool> ns,ni,bs,bi;
    for (int cutoff = 100; cutoff <= 5000; ++cutoff) {
        size_t j = index(cutoff);
        bool a = separates(j,false,true), b = separates(j,true,true),
             c = separates(j,false,false), d = separates(j,true,false);
        ns.push_back(a); ni.push_back(b); bs.push_back(c); bi.push_back(d);
        cuts << cutoff << '\t' << a << '\t' << b << '\t' << c << '\t' << d;
        for (int bits : {192,224,239,256,384}) cuts << '\t' << gap(bits,j,false,false);
        cuts << '\n';
        for (size_t i : picked) {
            const auto& r = records[i];
            tsv(profiles,{r.text[1],r.text[2],r.text[3],r.text[4],std::to_string(cutoff),
                          std::to_string(r.strict[j]),std::to_string(r.ramified[j]),
                          std::to_string(value(i,j,true))});
        }
    }
    require(windows(ns) == "[[691,718],[769,772]]" && std::count(ni.begin(),ni.end(),true) == 0,
            "old21positive reproduction control");
    branches << "branch\tseed_id\tbits\trole\tcategory\tside\tselected\tstrict691\tramified691\tinclusive691\tstrict5000\tramified5000\tinclusive5000\tdelta\n";
    for (size_t i = 0; i < records.size(); ++i) {
        const auto& r = records[i]; size_t j=index(691),k=index(5000);
        tsv(branches,{r.text[0],r.text[1],r.text[2],r.text[3],r.text[4],r.text[5],picked.count(i) ? "1" : "0",
                      std::to_string(r.strict[j]),std::to_string(r.ramified[j]),std::to_string(value(i,j,true)),
                      std::to_string(r.strict[k]),std::to_string(r.ramified[k]),std::to_string(value(i,k,true)),decimal(r.delta)});
    }
    violations << "seed_id\tbits\tcategory\tstrict691\tmax_published691\tmargin\n";
    int bad = 0;
    for (size_t i : picked) if (records[i].text[3] == "unpublished") {
        const auto& r = records[i]; int hi = -1;
        for (size_t k : picked) if (records[k].text[2] == r.text[2] && records[k].text[3] == "published")
            hi = std::max(hi,records[k].strict[index(691)]);
        require(hi >= 0,"published comparison");
        int v = r.strict[index(691)];
        if (v <= hi) { ++bad; tsv(violations,{r.text[1],r.text[2],r.text[4],std::to_string(v),std::to_string(hi),std::to_string(v-hi)}); }
    }
    auto nbs=std::count(bs.begin(),bs.end(),true), nbi=std::count(bi.begin(),bi.end(),true);
    std::ostringstream summary;
    summary << "{\"kind\":\"broader-recorded-census/v1\",\"branches\":82,\"seed_positions\":69,\"published_positions\":10,\"unpublished_positions\":59,\"integer_cutoffs\":4901,\"character_cells\":54776,\"profile_rows\":338169,\"branch_discriminant_invariance\":true,\"narrow_strict_intervals\":" << windows(ns)
            << ",\"broader_strict_intervals\":" << windows(bs) << ",\"broader_inclusive_intervals\":" << windows(bi)
            << ",\"broader_strict_success_cutoffs\":" << nbs << ",\"broader_inclusive_success_cutoffs\":" << nbi
            << ",\"strict691_violating_unpublished_positions\":" << bad << ",\"historical_use_established\":false,\"ECDLP_advantage_established\":false}\n";
    const fs::path outputs=argv[3];
    for (const auto& item : std::vector<std::pair<std::string,std::string>>{
            {"characters.tsv",chars.str()},{"profiles.tsv",profiles.str()},{"cutoffs.tsv",cuts.str()},
            {"branches.tsv",branches.str()},{"at691-violations.tsv",violations.str()},{"summary.json",summary.str()}})
        require(read_file(outputs/item.first) == item.second,"independent mismatch: "+item.first);
    require(!fs::exists(argv[4]),"fresh receipt");
    std::ostringstream receipt;
    receipt << "{\"kind\":\"independent-pari-broader-verification/v1\",\"branches\":82,\"seed_positions\":69,\"character_cells\":54776,\"integer_cutoffs\":4901,\"profile_rows\":338169,\"all_six_files_equal\":true,\"distinct_primes_proved\":" << prime_cache.size()
            << ",\"shared\":\"GMP integer family and TSV I/O only\",\"historical_use_established\":false}\n";
    put(argv[4],receipt.str());
    std::cout << "PASS independently reconstructed all six outputs and prime/order-point certificates\n";
    return 0;
} catch (const std::exception& error) { std::cerr << "FAIL: " << error.what() << '\n'; return 1; }
}
