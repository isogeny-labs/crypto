// Exact development controls, not a scaling experiment or ECDLP solver.
#include <gmpxx.h>
#include <bit>
#include <iostream>
#include <stdexcept>
#include <vector>

using Q = mpq_class;
using Matrix = std::vector<std::vector<Q>>;

static void require(bool condition, const char* message) {
    if (!condition) throw std::runtime_error(message);
}

static Matrix product(const Matrix& a, const Matrix& b) {
    const std::size_t n = a.size();
    Matrix c(n, std::vector<Q>(n));
    for (std::size_t i = 0; i < n; ++i)
        for (std::size_t j = 0; j < n; ++j)
            for (std::size_t k = 0; k < n; ++k) c[i][j] += a[i][k] * b[k][j];
    return c;
}

static Q determinant(Matrix a) {
    Q result = 1;
    for (std::size_t j = 0; j < a.size(); ++j) {
        std::size_t pivot = j;
        while (pivot < a.size() && a[pivot][j] == 0) ++pivot;
        if (pivot == a.size()) return 0;
        if (pivot != j) { std::swap(a[pivot], a[j]); result = -result; }
        const Q value = a[j][j];
        result *= value;
        for (std::size_t i = j + 1; i < a.size(); ++i) {
            const Q factor = a[i][j] / value;
            for (std::size_t k = j + 1; k < a.size(); ++k) a[i][k] -= factor * a[j][k];
        }
    }
    return result;
}

static Q exterior_entry(const Matrix& a, unsigned y, unsigned x) {
    if (std::popcount(y) != std::popcount(x)) return 0;
    Matrix minor;
    for (unsigned i = 0; i < a.size(); ++i) if ((y >> i) & 1U) {
        std::vector<Q> row;
        for (unsigned j = 0; j < a.size(); ++j) if ((x >> j) & 1U) row.push_back(a[i][j]);
        minor.push_back(row);
    }
    return determinant(minor);
}

static Matrix exterior(const Matrix& a) {
    const unsigned size = 1U << a.size();
    Matrix result(size, std::vector<Q>(size));
    for (unsigned y = 0; y < size; ++y)
        for (unsigned x = 0; x < size; ++x) result[y][x] = exterior_entry(a, y, x);
    return result;
}

static Matrix boolean_kernel(const Matrix& a) {
    const unsigned size = 1U << a.size();
    Matrix result(size, std::vector<Q>(size));
    for (unsigned x = 0; x < size; ++x)
        for (unsigned y = 0; y < size; ++y) {
            Q entry = 1;
            for (unsigned i = 0; i < a.size(); ++i) {
                Q value = 0;
                for (unsigned j = 0; j < a.size(); ++j) if ((x >> j) & 1U) value += a[i][j];
                entry *= ((y >> i) & 1U) ? value : Q(1 - value);
            }
            result[y][x] = entry;
        }
    return result;
}

static Matrix fixture(unsigned n, unsigned seed) {
    Matrix result(n, std::vector<Q>(n));
    for (unsigned i = 0; i < n; ++i)
        for (unsigned j = 0; j < n; ++j)
            result[i][j] = static_cast<int>((7U*i + 11U*j + 3U*seed) % 13U) - 6;
    return result;
}

// Coefficients of the source W_delta(t+alpha*z), calculated as an exact
// rational product after canceling the single indicated numerator factor.
static std::vector<Q> cauchy_chart(int length, int delta, int t, int alpha) {
    require(-length <= delta && delta < length, "delta outside interpolation grid");
    Q denominator = 1;
    for (int node = -length; node < length; ++node) if (node != 0) denominator *= -node;
    std::vector<Q> result{Q(1) / denominator};
    for (int node = -length; node < length; ++node) if (node != delta) {
        std::vector<Q> next(result.size() + 1);
        for (std::size_t j = 0; j < result.size(); ++j) {
            next[j] += (t - node) * result[j];
            next[j+1] += alpha * result[j];
        }
        result = next;
    }
    return result;
}

int main() {
    try {
        const Matrix w{{1,1},{0,1}}, v{{1,0},{1,1}};
        const Q composed = product(boolean_kernel(w), boolean_kernel(v))[3][3];
        const Q direct = boolean_kernel(product(w,v))[3][3];
        require(composed == 4 && direct == 6, "Boolean nonfunctoriality witness");
        std::cout << "boolean_composition\t" << composed << '\t' << direct << '\n';

        unsigned exterior_checks = 0, monomial_checks = 0;
        for (unsigned n = 1; n <= 4; ++n) for (unsigned seed = 0; seed < 12; ++seed) {
            const Matrix a = fixture(n,seed), b = fixture(n,seed+4);
            const Matrix lhs = product(exterior(a),exterior(b)), rhs = exterior(product(a,b));
            Matrix monomial(n,std::vector<Q>(n));
            for (unsigned i = 0; i < n; ++i) monomial[i][(i+seed)%n] = (i%2 == 0) ? Q(2) : Q(-1);
            const Matrix klhs = product(boolean_kernel(monomial),boolean_kernel(b));
            const Matrix krhs = boolean_kernel(product(monomial,b));
            for (unsigned y = 0; y < lhs.size(); ++y) for (unsigned x = 0; x < lhs.size(); ++x) {
                require(lhs[y][x] == rhs[y][x], "exterior composition"); ++exterior_checks;
                require(klhs[y][x] == krhs[y][x], "monomial Boolean positive control"); ++monomial_checks;
            }
        }
        std::cout << "exterior_composition_entries\t" << exterior_checks << '\n';
        std::cout << "monomial_positive_entries\t" << monomial_checks << '\n';

        const auto crossing = cauchy_chart(4,2,1,8);
        const auto local = cauchy_chart(4,1,1,8);
        require(crossing[0] == 0 && crossing[1] == Q(40,3), "crossing chart coefficient");
        require(local[0] == Q(-5,3), "local chart value");
        const Q passive = crossing[1]*local[0];
        require(passive == Q(-200,9), "global-parity passive crossing");
        require((std::popcount(0U)+std::popcount(0U))%2 == 0 &&
                (std::popcount(2U)+std::popcount(1U))%2 == 0, "code endpoints");
        std::cout << "parity_code_passive_coefficient\t" << passive << '\n';

        unsigned swap_entries = 0;
        for (unsigned n = 1; n <= 5; ++n) {
            const unsigned size = 1U << n;
            Matrix swap(2*n,std::vector<Q>(2*n)), h(size,std::vector<Q>(size));
            for (unsigned j = 0; j < n; ++j) { swap[2*j][2*j+1]=1; swap[2*j+1][2*j]=1; }
            for (unsigned a = 0; a < size; ++a) for (unsigned b = 0; b < size; ++b) {
                unsigned x=0, y=0;
                for (unsigned j = 0; j < n; ++j) {
                    x |= ((a>>j)&1U)<<(2*j); x |= ((b>>j)&1U)<<(2*j+1);
                    y |= ((a>>j)&1U)<<(2*j+1); y |= ((b>>j)&1U)<<(2*j);
                }
                h[a][b] = (std::popcount(a&b)%2 == 0) ? Q(1) : Q(-1);
                require(exterior_entry(swap,y,x) == h[a][b], "fermionic swap sign"); ++swap_entries;
            }
            const Matrix hh = product(h,h);
            for (unsigned a = 0; a < size; ++a) for (unsigned b = 0; b < size; ++b)
                require(hh[a][b] == ((a == b) ? Q(size) : Q(0)), "Walsh full-rank certificate");
            std::cout << "swap_sign_rank\t" << n << '\t' << size << '\n';
        }
        std::cout << "swap_sign_entries\t" << swap_entries << '\n';

        const Matrix swap{{0,1},{1,0}};
        require(exterior(product(swap,swap))[1][1] == 1, "full intermediate state");
        require(exterior(Matrix{{0}})[1][1] == 0, "restricted intermediate state");
        std::cout << "restricted_composition\t1\t0\n";

        unsigned affine_minors = 0;
        for (unsigned seed = 0; seed < 8; ++seed) {
            Matrix base = fixture(4,seed);
            std::vector<Matrix> values;
            for (int z = 0; z <= 3; ++z) {
                Matrix update = base;
                for (unsigned i=0;i<4;++i) for (unsigned j=0;j<4;++j)
                    update[i][j] += z * (static_cast<int>(i)+1) * (static_cast<int>(j)-2);
                values.push_back(exterior(update));
            }
            for (unsigned y=0;y<16;++y) for (unsigned x=0;x<16;++x)
                if (std::popcount(y) == std::popcount(x)) {
                    require(values[2][y][x]-2*values[1][y][x]+values[0][y][x] == 0, "rank-one affine second difference");
                    require(values[3][y][x]-3*values[1][y][x]+2*values[0][y][x] == 0, "rank-one affine third sample");
                    ++affine_minors;
                }
        }
        std::cout << "rank_one_affine_minors\t" << affine_minors << '\n';
        std::cout << "PASS\n";
    } catch (const std::exception& e) {
        std::cerr << "FAIL " << e.what() << '\n';
        return 1;
    }
}
