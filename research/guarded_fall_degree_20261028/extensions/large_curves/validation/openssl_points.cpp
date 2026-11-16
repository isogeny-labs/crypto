// Independent native interoperability check for the fixed public SEC 2 points.
// Compile: c++ -std=c++17 -O2 -Wall -Wextra -Werror openssl_points.cpp -lcrypto
// This program uses only the public scalars 1..28. It generates no keys and
// performs no search, timing comparison, or fall-degree computation.
// API reference: https://docs.openssl.org/3.5/man3/EC_POINT_add/

#include <openssl/bn.h>
#include <openssl/crypto.h>
#include <openssl/ec.h>
#include <openssl/err.h>
#include <openssl/obj_mac.h>

#include <array>
#include <cstdlib>
#include <iostream>
#include <memory>
#include <sstream>
#include <stdexcept>
#include <string>
#include <string_view>

namespace {
using Bn = std::unique_ptr<BIGNUM, decltype(&BN_free)>;
using Context = std::unique_ptr<BN_CTX, decltype(&BN_CTX_free)>;
using Group = std::unique_ptr<EC_GROUP, decltype(&EC_GROUP_free)>;
using Point = std::unique_ptr<EC_POINT, decltype(&EC_POINT_free)>;

std::string diagnostic(const char *operation) {
    std::ostringstream text;
    text << operation;
    for (unsigned long code = ERR_get_error(); code != 0;
         code = ERR_get_error()) {
        std::array<char, 256> buffer{};
        ERR_error_string_n(code, buffer.data(), buffer.size());
        text << ": " << buffer.data();
    }
    return text.str();
}

void require(bool condition, const char *operation) {
    if (!condition) {
        throw std::runtime_error(diagnostic(operation));
    }
}

Bn number() {
    Bn result(BN_new(), BN_free);
    require(result != nullptr, "BN_new failed");
    return result;
}

struct OpenSslTextFree {
    void operator()(char *text) const { OPENSSL_free(text); }
};

std::string decimal(const BIGNUM *value) {
    require(value != nullptr && BN_is_negative(value) == 0,
            "unexpected negative or absent coordinate");
    std::unique_ptr<char, OpenSslTextFree> text(BN_bn2dec(value));
    require(text != nullptr, "BN_bn2dec failed");
    return std::string(text.get());
}

void string(std::ostream &output, std::string_view value) {
    constexpr char hex[] = "0123456789abcdef";
    output << '"';
    for (unsigned char c : value) {
        if (c == '"' || c == '\\') {
            output << '\\' << static_cast<char>(c);
        } else if (c < 0x20) {
            output << "\\u00" << hex[c >> 4] << hex[c & 0xf];
        } else {
            output << static_cast<char>(c);
        }
    }
    output << '"';
}

struct NamedCurve {
    const char *standard_name;
    int nid;
    int field_bits;
};

void emit_curve(std::ostream &output, const NamedCurve &named, BN_CTX *context) {
    Group group(EC_GROUP_new_by_curve_name(named.nid), EC_GROUP_free);
    require(group != nullptr, "EC_GROUP_new_by_curve_name failed");
    require(EC_GROUP_get_curve_name(group.get()) == named.nid,
            "unexpected named-group identifier");
    require(EC_GROUP_get_degree(group.get()) == named.field_bits,
            "unexpected named-group field degree");
    const EC_POINT *generator = EC_GROUP_get0_generator(group.get());
    require(generator != nullptr, "named group has no generator");
    require(EC_POINT_is_at_infinity(group.get(), generator) == 0,
            "named generator is infinity");
    require(EC_POINT_is_on_curve(group.get(), generator, context) == 1,
            "named generator is off curve or cannot be checked");

    Bn p = number();
    Bn a = number();
    Bn b = number();
    require(EC_GROUP_get_curve(group.get(), p.get(), a.get(), b.get(), context) ==
                1,
            "EC_GROUP_get_curve failed");
    output << "{\"standard_name\":";
    string(output, named.standard_name);
    output << ",\"field_bits\":" << named.field_bits << ",\"p_decimal\":";
    string(output, decimal(p.get()));
    output << ",\"a_decimal\":";
    string(output, decimal(a.get()));
    output << ",\"b_decimal\":";
    string(output, decimal(b.get()));
    output << ",\"multiples_1_through_28\":[";

    Point point(EC_POINT_new(group.get()), EC_POINT_free);
    require(point != nullptr, "EC_POINT_new failed");
    Bn scalar = number();
    Bn x = number();
    Bn y = number();
    for (unsigned int multiple = 1; multiple <= 28; ++multiple) {
        require(BN_set_word(scalar.get(), multiple) == 1,
                "BN_set_word failed");
        require(EC_POINT_mul(group.get(), point.get(), scalar.get(), nullptr,
                             nullptr, context) == 1,
                "EC_POINT_mul failed");
        require(EC_POINT_is_at_infinity(group.get(), point.get()) == 0,
                "public small multiple is infinity");
        require(EC_POINT_is_on_curve(group.get(), point.get(), context) == 1,
                "public small multiple is off curve or cannot be checked");
        require(EC_POINT_get_affine_coordinates(group.get(), point.get(),
                                                x.get(), y.get(), context) == 1,
                "EC_POINT_get_affine_coordinates failed");
        if (multiple != 1) {
            output << ',';
        }
        output << "{\"multiple\":" << multiple << ",\"x_decimal\":";
        string(output, decimal(x.get()));
        output << ",\"y_decimal\":";
        string(output, decimal(y.get()));
        output << '}';
    }
    output << "]}";
}
} // namespace

int main() {
    try {
        Context context(BN_CTX_new(), BN_CTX_free);
        require(context != nullptr, "BN_CTX_new failed");
        constexpr std::array<NamedCurve, 4> curves{{
            {"secp256k1", NID_secp256k1, 256},
            {"secp256r1", NID_X9_62_prime256v1, 256},
            {"secp384r1", NID_secp384r1, 384},
            {"secp521r1", NID_secp521r1, 521},
        }};
        // Buffer the complete document so a failed arithmetic check never
        // produces a partial stdout record that resembles a valid receipt.
        std::ostringstream output;
        output << "{\"schema\":\"openssl-standard-public-points/v1\","
                  "\"openssl_version\":";
        const char *version = OpenSSL_version(OPENSSL_VERSION);
        require(version != nullptr, "OpenSSL_version failed");
        string(output, version);
        output << ",\"coordinate_encoding\":\"canonical decimal strings\","
                  "\"scope\":\"Independent named-group public-point "
                  "interoperability check; no keys, searches or timing "
                  "measurements.\",\"curves\":[";
        bool first = true;
        for (const NamedCurve &curve : curves) {
            if (!first) {
                output << ',';
            }
            first = false;
            emit_curve(output, curve, context.get());
        }
        output << "]}\n";
        std::cout << output.str();
        require(static_cast<bool>(std::cout), "stdout write failed");
        return EXIT_SUCCESS;
    } catch (const std::exception &error) {
        std::cerr << "openssl_points: " << error.what() << '\n';
        return EXIT_FAILURE;
    }
}
