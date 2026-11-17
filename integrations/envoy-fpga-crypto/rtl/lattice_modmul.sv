// Canonical modular multiplication for ML-KEM (3329) and ML-DSA (8380417).
// Barrett reciprocal reduction uses mu=floor(2^48/q). For canonical a,b,
// a*b < q^2 < 2^48, so the provisional remainder is in [0,2*q).
// There is one fixed correction, no divider and no data-dependent iteration.
module lattice_modmul (
    input  logic        clk,
    input  logic        rst_n,
    input  logic        clear,
    input  logic        start,
    input  logic        mode_dsa,
    input  logic [31:0] a,
    input  logic [31:0] b,
    output logic        ready,
    output logic        busy,
    output logic        done,
    output logic        error,
    output logic [31:0] result
);
  typedef enum logic [1:0] {IDLE, RECIPROCAL, REDUCE, CORRECT} phase_t;
  phase_t phase_q;
  logic [22:0] modulus_q;
  logic [36:0] reciprocal_q;
  logic [45:0] product_q, remainder_q;
  logic [34:0] quotient_q;
  logic [82:0] reciprocal_product;
  logic [22:0] modulus_in;
  // The low fractional bits are intentionally discarded by Barrett reduction.
  logic [47:0] unused_fraction;
  assign modulus_in = mode_dsa ? 23'd8380417 : 23'd3329;
  assign reciprocal_product = 83'(product_q) * 83'(reciprocal_q);
  assign unused_fraction = reciprocal_product[47:0];
  assign ready = rst_n && !clear && phase_q == IDLE;
  assign busy = phase_q != IDLE;

  always_ff @(posedge clk or negedge rst_n) begin
    if (!rst_n) begin
      phase_q <= IDLE;
      modulus_q <= '0;
      reciprocal_q <= '0;
      product_q <= '0;
      remainder_q <= '0;
      quotient_q <= '0;
      result <= '0;
      error <= 1'b0;
      done <= 1'b0;
    end else if (clear) begin
      phase_q <= IDLE;
      modulus_q <= '0;
      reciprocal_q <= '0;
      product_q <= '0;
      remainder_q <= '0;
      quotient_q <= '0;
      result <= '0;
      error <= 1'b0;
      done <= 1'b0;
    end else begin
      done <= 1'b0;
`ifndef SYNTHESIS
      if (phase_q == CORRECT) assert (remainder_q < 46'(modulus_q) * 46'd2);
`endif
      case (phase_q)
        IDLE: if (start) begin
          result <= '0;
          error <= 1'b0;
          if (a >= 32'(modulus_in) || b >= 32'(modulus_in)) begin
            error <= 1'b1;
            done <= 1'b1;
          end else begin
            modulus_q <= modulus_in;
            reciprocal_q <= mode_dsa ? 37'd33587228 : 37'd84552411147;
            product_q <= 46'(a[22:0]) * 46'(b[22:0]);
            phase_q <= RECIPROCAL;
          end
        end
        RECIPROCAL: begin
          quotient_q <= reciprocal_product[82:48];
          phase_q <= REDUCE;
        end
        REDUCE: begin
          remainder_q <= product_q - 46'(quotient_q) * 46'(modulus_q);
          phase_q <= CORRECT;
        end
        CORRECT: begin
          result <= 32'(remainder_q >= 46'(modulus_q) ?
                         remainder_q - 46'(modulus_q) : remainder_q);
          modulus_q <= '0;
          reciprocal_q <= '0;
          product_q <= '0;
          remainder_q <= '0;
          quotient_q <= '0;
          phase_q <= IDLE;
          done <= 1'b1;
        end
        default: phase_q <= IDLE;
      endcase
    end
  end

endmodule
