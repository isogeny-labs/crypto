/* Exact, exhaustive checks for the guide's tiny public examples.
 * This is teaching code, not a constant-time cryptographic implementation.
 * stdout is a JSON fixture consumed by the document renderer.
 */
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct { int a, b; } Element;
typedef struct { int p, d; bool quadratic; } Field;
typedef struct { Element x, y; bool infinity; } Point;
typedef struct { Point points[512]; int count; } PointSet;
typedef struct { int b, count, degree, first, exponent; } Twist;

static void check(bool condition, const char *message) {
    if (!condition) { fprintf(stderr, "FAIL: %s\n", message); exit(EXIT_FAILURE); }
}
static int mod(int a, int p) { int r = a % p; return r < 0 ? r + p : r; }
static Element element(int a, int b, Field f) {
    return (Element){mod(a, f.p), mod(b, f.p)};
}
static bool equal(Element a, Element b) { return a.a == b.a && a.b == b.b; }
static Element add(Element a, Element b, Field f) {
    return element(a.a + b.a, a.b + b.b, f);
}
static Element neg(Element a, Field f) { return element(-a.a, -a.b, f); }
static Element sub(Element a, Element b, Field f) { return add(a, neg(b, f), f); }
static Element mul(Element a, Element b, Field f) {
    return element(a.a*b.a + f.d*a.b*b.b, a.a*b.b + a.b*b.a, f);
}
static int size(Field f) { return f.quadratic ? f.p*f.p : f.p; }
static Element power(Element a, int n, Field f) {
    Element result = {1, 0};
    check(n >= 0, "power exponent must be nonnegative");
    while (n) {
        if (n & 1) result = mul(result, a, f);
        a = mul(a, a, f); n >>= 1;
    }
    return result;
}
static Element inverse(Element a, Field f) {
    check(!equal(a, (Element){0, 0}), "zero denominator");
    return power(a, size(f)-2, f);
}
static Element divide(Element a, Element b, Field f) { return mul(a, inverse(b, f), f); }
static Element from_index(int i, Field f) { return element(i % f.p, i / f.p, f); }
static Point identity(void) { return (Point){{0, 0}, {0, 0}, true}; }
static bool same(Point p, Point q) {
    if (p.infinity || q.infinity) return p.infinity == q.infinity;
    return equal(p.x, q.x) && equal(p.y, q.y);
}
static Point negative(Point p, Field f) { if (!p.infinity) p.y = neg(p.y, f); return p; }
static Point point_add(Point p, Point q, Element a, Field f) {
    if (p.infinity) return q;
    if (q.infinity) return p;
    if (equal(p.x, q.x) && equal(add(p.y, q.y, f), (Element){0, 0})) return identity();
    Element numerator, denominator;
    if (same(p, q)) {
        numerator = add(mul((Element){3, 0}, mul(p.x, p.x, f), f), a, f);
        denominator = mul((Element){2, 0}, p.y, f);
    } else {
        numerator = sub(q.y, p.y, f); denominator = sub(q.x, p.x, f);
    }
    Element slope = divide(numerator, denominator, f);
    Element x = sub(sub(mul(slope, slope, f), p.x, f), q.x, f);
    Element y = sub(mul(slope, sub(p.x, x, f), f), p.y, f);
    return (Point){x, y, false};
}
static Point scalar_mul(int n, Point p, Element a, Field f) {
    Point result = identity();
    while (n) {
        if (n & 1) result = point_add(result, p, a, f);
        p = point_add(p, p, a, f); n >>= 1;
    }
    return result;
}
static bool on_curve(Point p, Element a, Element b, Field f) {
    if (p.infinity) return true;
    Element rhs = add(add(power(p.x, 3, f), mul(a, p.x, f), f), b, f);
    return equal(mul(p.y, p.y, f), rhs);
}
static PointSet points(Field f, Element a, Element b) {
    PointSet result = {.count = 1}; result.points[0] = identity();
    for (int x = 0; x < size(f); ++x) {
        for (int y = 0; y < size(f); ++y) {
            Point p = {from_index(x, f), from_index(y, f), false};
            if (on_curve(p, a, b, f)) {
                check(result.count < 512, "point-set capacity exceeded");
                result.points[result.count++] = p;
            }
        }
    }
    return result;
}
static int order(Point p, Element a, Field f, int bound) {
    Point q = identity();
    for (int n = 1; n <= bound; ++n) {
        q = point_add(q, p, a, f);
        if (q.infinity) return n;
    }
    check(false, "point order exceeds group size"); return 0;
}
static int gcd(int a, int b) { while (b) { int c = a % b; a = b; b = c; } return a; }
static int int_power(int a, int n) { int result = 1; while (n--) result *= a; return result; }
static void check_field(Field f) {
    check(equal(power((Element){f.d, 0}, (f.p-1)/2, f), (Element){f.p-1, 0}),
          "quadratic field polynomial must be irreducible");
    for (int i = 1; i < size(f); ++i) {
        Element a = from_index(i, f);
        check(equal(mul(a, inverse(a, f), f), (Element){1, 0}), "field inverse");
    }
}
static Point psi(Point p, Element cx, Element cy, Field f) {
    if (p.infinity) return p;
    return (Point){mul(cx, power(p.x, f.p, f), f),
                   mul(cy, power(p.y, f.p, f), f), false};
}
static void print_affine(PointSet set) {
    putchar('[');
    for (int i = 1; i < set.count; ++i) {
        if (i > 1) putchar(',');
        printf("[%d,%d]", set.points[i].x.a, set.points[i].y.a);
    }
    putchar(']');
}

int main(void) {
    const Element zero = {0, 0};
    Field f7 = {7, 0, false}, f13 = {13, 0, false};
    Field f101 = {101, 0, false}, f5 = {5, 0, false};
    const int expected_counts[] = {12, 13, 9, 4, 3, 7};
    const int expected_degrees[] = {1, 6, 3, 2, 3, 6};
    const int expected_first[] = {2, 1, 3, 2, 1, 1};
    const int expected_exponents[] = {6, 13, 3, 2, 3, 7};
    Twist sextic[6];
    for (int i = 0; i < 6; ++i) {
        int b = power((Element){3, 0}, i, f7).a;
        PointSet set = points(f7, zero, (Element){b, 0});
        int exponent = 1;
        for (int j = 0; j < set.count; ++j) {
            int n = order(set.points[j], zero, f7, set.count);
            if (n > exponent) exponent = n;
        }
        int degree = 0;
        for (int m = 1; m <= 6; ++m) {
            int q_minus_one = int_power(7, m)-1;
            if (equal(power((Element){b, 0}, q_minus_one/gcd(6, q_minus_one), f7),
                      (Element){1, 0})) { degree = m; break; }
        }
        sextic[i] = (Twist){b, set.count, degree, set.count/exponent, exponent};
        check(set.count == expected_counts[i], "F7 point count");
        check(degree == expected_degrees[i], "F7 exact extension degree");
        check(sextic[i].first == expected_first[i] && exponent == expected_exponents[i],
              "F7 group invariants");
    }
    for (int i = 0; i < 3; ++i)
        check(sextic[i].count + sextic[i+3].count == 16, "quadratic count identity");
    const int f7_quartic_a[] = {1, 3}, f7_quartic_exponents[] = {8, 4};
    for (int i = 0; i < 2; ++i) {
        Element a = {f7_quartic_a[i], 0};
        PointSet set = points(f7, a, zero);
        int exponent = 1;
        for (int j = 0; j < set.count; ++j) {
            int n = order(set.points[j], a, f7, set.count);
            if (n > exponent) exponent = n;
        }
        check(set.count == 8 && exponent == f7_quartic_exponents[i],
              "F7 equal-count quartic forms have distinct groups");
    }
    const int root_cases[][4] = {{7, 2, 3, 2}, {7, 3, 3, 3}, {13, 4, 2, 4}};
    for (int i = 0; i < 3; ++i) {
        int p = root_cases[i][0], n = root_cases[i][1], degree = 0;
        Field f = {p, 0, false};
        Element c = {root_cases[i][2], 0};
        for (int m = 1; m <= n; ++m) {
            int q_minus_one = int_power(p, m)-1;
            if (equal(power(c, q_minus_one/gcd(n, q_minus_one), f), (Element){1, 0})) {
                degree = m; break;
            }
        }
        check(degree == root_cases[i][3], "displayed binomial root extension degree");
    }
    const int quartic_counts[] = {20, 10, 8, 18};
    const int quartic_degrees[] = {1, 4, 2, 4};
    for (int i = 0; i < 4; ++i) {
        Element a = power((Element){2, 0}, i, f13);
        check(points(f13, a, zero).count == quartic_counts[i], "F13 quartic count");
        int degree = 0;
        for (int m = 1; m <= 4; ++m) {
            int q_minus_one = int_power(13, m)-1;
            if (equal(power(a, q_minus_one/gcd(4, q_minus_one), f13), (Element){1, 0})) {
                degree = m; break;
            }
        }
        check(degree == quartic_degrees[i], "F13 exact extension degree");
    }
    check(points(f101, (Element){2, 0}, (Element){3, 0}).count == 96, "F101 base count");
    check(points(f101, (Element){8, 0}, (Element){24, 0}).count == 108, "F101 twist count");
    for (int b = 1; b <= 4; ++b)
        check(points(f5, zero, (Element){b, 0}).count == 6, "F5 j=0 count");
    Field f49 = {7, 3, true}, f121 = {11, 10, true}, f11 = {11, 0, false};
    check_field(f49); check_field(f121);
    check(points(f49, zero, (Element){1, 0}).count == 48, "F49 original count");
    check(points(f49, zero, (Element){6, 0}).count == 48, "F49 twist count");
    check(points(f11, (Element){1, 0}, (Element){6, 0}).count == 13, "GLS base count");
    check(points(f121, (Element){1, 0}, (Element){6, 0}).count == 143, "GLS extension count");
    Element delta = {1, 1};
    check(equal(power(delta, 60, f121), (Element){10, 0}), "GLS nonsquare parameter");
    Element a = power(delta, 2, f121), b = mul((Element){6, 0}, power(delta, 3, f121), f121);
    check(equal(a, (Element){0, 2}) && equal(b, (Element){10, 1}), "GLS twist coefficients");
    PointSet gls = points(f121, a, b);
    check(gls.count == 101, "GLS twist count");
    Element cx = power(inverse(delta, f121), 10, f121);
    Element cy = power(inverse(delta, f121), 15, f121);
    check(91*91 % 101 == 100, "GLS eigenvalue square");
    for (int i = 0; i < gls.count; ++i) {
        Point p = gls.points[i], q = psi(p, cx, cy, f121), minus = negative(p, f121);
        check(on_curve(q, a, b, f121), "GLS preserves twist");
        check(same(psi(q, cx, cy, f121), minus), "GLS square is negation");
        check(same(q, scalar_mul(91, p, a, f121)), "GLS eigenvalue on all points");
        if (!p.infinity) {
            Point orbit[] = {p, q, minus, psi(minus, cx, cy, f121)};
            for (int j = 0; j < 4; ++j) for (int k = j+1; k < 4; ++k)
                check(!same(orbit[j], orbit[k]), "GLS nonidentity orbit size four");
        }
    }
    /* Emit only after every mathematical check has passed. */
    printf("{\n  \"sextic_F7\": [\n");
    for (int i = 0; i < 6; ++i) {
        Twist t = sextic[i];
        printf("    {\"i\":%d,\"b\":%d,\"count\":%d,\"degree\":%d,\"invariants\":[%d,%d]}%s\n",
               i, t.b, t.count, t.degree, t.first, t.exponent, i == 5 ? "" : ",");
    }
    printf("  ],\n  \"affine_F7\": {\n    \"1\":");
    print_affine(points(f7, zero, (Element){1, 0}));
    printf(",\n    \"6\":"); print_affine(points(f7, zero, (Element){6, 0}));
    printf("\n  },\n  \"quadratic_fibers_F7\": [\n");
    for (int x = 0; x < 7; ++x) {
        int rhs = mod(x*x*x+1, 7), ny = 0, nv = 0;
        for (int v = 0; v < 7; ++v) {
            if (v*v % 7 == rhs) ++ny;
            if (3*v*v % 7 == rhs) ++nv;
        }
        check(ny + nv == 2, "quadratic fiber balance");
        printf("    {\"x\":%d,\"rhs\":%d,\"ny\":%d,\"nv\":%d}%s\n", x, rhs, ny, nv, x == 6 ? "" : ",");
    }
    printf("  ],\n  \"quadratic_F101\":[96,108],\n  \"quartic_F13\":[20,10,8,18],\n");
    printf("  \"F49_counts\":[48,48],\n  \"GLS_example\":{\"base_F11\":13,\"extension_F121\":143,");
    printf("\"twist_F121\":101,\"eigenvalue\":91,\"checked_points\":101,\"psi_squared_is_negation\":true}\n}\n");
    check(!ferror(stdout), "writing verification fixture");
    return EXIT_SUCCESS;
}
