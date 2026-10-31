// FIPS 203/204 NTT and inverse NTT, 256 canonical coefficients per command.
// The public mode selects q=3329 / root=17 or q=8380417 / root=1753.
// Coefficients and twiddles use normal residues, NOT Montgomery encoding.
// This is a reusable arithmetic kernel, not a complete KEM/signature engine.
module lattice_ntt (
    input  logic        clk,
    input  logic        rst_n,
    input  logic        clear,
    input  logic        start,
    input  logic        mode_dsa,
    input  logic        inverse,
    output logic        ready,
    output logic        busy,
    output logic        done,
    output logic        error,
    input  logic        in_valid,
    output logic        in_ready,
    input  logic [31:0] in_coeff,
    output logic        out_valid,
    input  logic        out_ready,
    output logic [31:0] out_coeff
);
  typedef enum logic [3:0] {
    IDLE, LOAD, BUTTERFLY_START, BUTTERFLY_WAIT, SCALE_START, SCALE_WAIT, DRAIN, SCRUB
  } phase_t;
  phase_t phase_q;
  logic dsa_q, inverse_q, report_error_q;
  logic [22:0] coefficients [0:255];
  logic [22:0] modulus, operand_a_q, operand_b_q;
  wire [22:0] dsa_zetas [0:255];
  wire [22:0] kem_zetas [0:255];
  logic [7:0] io_index_q, scrub_index_q, twiddle_q;
  logic [8:0] length_q, block_q, j_q, high_index;
  logic [31:0] mul_a, mul_b, mul_result;
  logic mul_done, mul_error, mul_ready, unused_mul_busy;

  function automatic logic [7:0] bit_reverse(input logic [7:0] value, input int width);
    logic [7:0] r;
    r = '0;
    for (int i = 0; i < width; i++) r = (r << 1) | 8'(value[i]);
    return r;
  endfunction

  // Called only in localparams below: all divisions are elaboration-time constants.
  function automatic logic [22:0] power_const(
      input logic [22:0] base, input logic [7:0] exponent, input logic [22:0] q);
    logic [63:0] p, r;
    p = 64'(base);
    r = 64'd1;
    for (int i = 0; i < 8; i++) begin
      if (exponent[i]) r = (r * p) % 64'(q);
      p = (p * p) % 64'(q);
    end
    return 23'(r);
  endfunction

  for (genvar k = 0; k < 256; k++) begin : twiddles
    localparam logic [22:0] DSA = power_const(23'd1753, bit_reverse(8'(k), 8), 23'd8380417);
    assign dsa_zetas[k] = DSA;
    if (k < 128) begin : kem
      localparam logic [22:0] KEM = power_const(23'd17, bit_reverse(8'(k), 7), 23'd3329);
      assign kem_zetas[k] = KEM;
    end else begin : unused_kem
      assign kem_zetas[k] = 23'b0;
    end
  end

  function automatic logic [22:0] add_mod(
      input logic [22:0] a, input logic [22:0] b, input logic [22:0] q);
    logic [23:0] sum;
    sum = {1'b0, a} + {1'b0, b};
    return 23'(sum >= {1'b0, q} ? sum - {1'b0, q} : sum);
  endfunction

  function automatic logic [22:0] sub_mod(
      input logic [22:0] a, input logic [22:0] b, input logic [22:0] q);
    return a >= b ? a - b : q - (b - a);
  endfunction

  assign modulus = dsa_q ? 23'd8380417 : 23'd3329;
  assign high_index = j_q + length_q;
  assign ready = rst_n && !clear && phase_q == IDLE;
  assign busy = phase_q != IDLE;
  assign in_ready = rst_n && !clear && phase_q == LOAD;
  assign out_valid = rst_n && !clear && phase_q == DRAIN;
  assign out_coeff = out_valid ? 32'(coefficients[io_index_q]) : 32'b0;

  always_comb begin
    mul_a = '0;
    mul_b = '0;
    if (phase_q == BUTTERFLY_START) begin
      mul_a = inverse_q ?
          {9'b0, sub_mod(coefficients[high_index[7:0]], coefficients[j_q[7:0]], modulus)} :
          {9'b0, coefficients[high_index[7:0]]};
      mul_b = dsa_q ? {9'b0, dsa_zetas[twiddle_q]} : {9'b0, kem_zetas[twiddle_q]};
    end else if (phase_q == SCALE_START) begin
      mul_a = 32'(coefficients[io_index_q]);
      // Multiplicative inverses of 256 and 128 in the selected fields.
      mul_b = dsa_q ? 32'd8347681 : 32'd3303;
    end
  end

  lattice_modmul multiplier (
      .clk(clk), .rst_n(rst_n),
      .clear(clear || phase_q == SCRUB ||
             ((phase_q == BUTTERFLY_WAIT || phase_q == SCALE_WAIT) && mul_done)),
      .start(phase_q == BUTTERFLY_START || phase_q == SCALE_START),
      .mode_dsa(dsa_q), .a(mul_a), .b(mul_b), .ready(mul_ready), .busy(unused_mul_busy),
      .done(mul_done), .error(mul_error), .result(mul_result)
  );

  always_ff @(posedge clk or negedge rst_n) begin
    if (!rst_n) begin
      phase_q <= SCRUB;
      dsa_q <= 1'b0;
      inverse_q <= 1'b0;
      report_error_q <= 1'b0;
      operand_a_q <= '0;
      operand_b_q <= '0;
      io_index_q <= '0;
      scrub_index_q <= '0;
      twiddle_q <= '0;
      length_q <= '0;
      block_q <= '0;
      j_q <= '0;
      done <= 1'b0;
      error <= 1'b0;
    end else if (clear) begin
      phase_q <= SCRUB;
      dsa_q <= 1'b0;
      inverse_q <= 1'b0;
      report_error_q <= 1'b0;
      operand_a_q <= '0;
      operand_b_q <= '0;
      io_index_q <= '0;
      scrub_index_q <= '0;
      twiddle_q <= '0;
      length_q <= '0;
      block_q <= '0;
      j_q <= '0;
      done <= 1'b0;
      error <= 1'b0;
    end else begin
      done <= 1'b0;
`ifndef SYNTHESIS
      if (phase_q == IDLE)
        for (int i = 0; i < 256; i++) assert (coefficients[i] == 0);
      if (out_valid) assert (out_coeff < 32'(modulus));
      if (phase_q == BUTTERFLY_START) begin
        assert (high_index < 9'd256);
        assert (twiddle_q != 0);
        assert (dsa_q || twiddle_q < 8'd128);
        assert (mul_ready);
      end
      if (phase_q == SCALE_START) assert (mul_ready);
`endif
      case (phase_q)
        IDLE: if (start) begin
          dsa_q <= mode_dsa;
          inverse_q <= inverse;
          error <= 1'b0;
          io_index_q <= '0;
          phase_q <= LOAD;
        end
        LOAD: if (in_valid) begin
          if (in_coeff >= 32'(modulus)) begin
            report_error_q <= 1'b1;
            scrub_index_q <= '0;
            phase_q <= SCRUB;
          end else begin
            coefficients[io_index_q] <= in_coeff[22:0];
            if (io_index_q == 8'd255) begin
              io_index_q <= '0;
              block_q <= '0;
              j_q <= '0;
              length_q <= inverse_q ? (dsa_q ? 9'd1 : 9'd2) : 9'd128;
              twiddle_q <= inverse_q ? (dsa_q ? 8'd255 : 8'd127) : 8'd1;
              phase_q <= BUTTERFLY_START;
            end else io_index_q <= io_index_q + 8'd1;
          end
        end
        BUTTERFLY_START: begin
          operand_a_q <= coefficients[j_q[7:0]];
          operand_b_q <= coefficients[high_index[7:0]];
          phase_q <= BUTTERFLY_WAIT;
        end
        BUTTERFLY_WAIT: if (mul_done) begin
          if (mul_error || mul_result >= 32'(modulus)) begin
            report_error_q <= 1'b1;
            scrub_index_q <= '0;
            phase_q <= SCRUB;
          end else begin
            coefficients[j_q[7:0]] <= add_mod(operand_a_q,
                inverse_q ? operand_b_q : mul_result[22:0], modulus);
            coefficients[high_index[7:0]] <= inverse_q ? mul_result[22:0] :
                sub_mod(operand_a_q, mul_result[22:0], modulus);
            phase_q <= BUTTERFLY_START;
            if (j_q + 9'd1 < block_q + length_q) j_q <= j_q + 9'd1;
            else begin
              twiddle_q <= inverse_q ? twiddle_q - 8'd1 : twiddle_q + 8'd1;
              if (block_q + (length_q << 1) < 9'd256) begin
                block_q <= block_q + (length_q << 1);
                j_q <= block_q + (length_q << 1);
              end else begin
                block_q <= '0;
                j_q <= '0;
                if (!inverse_q && length_q == (dsa_q ? 9'd1 : 9'd2)) begin
                  io_index_q <= '0;
                  phase_q <= DRAIN;
                end else if (inverse_q && length_q == 9'd128) begin
                  io_index_q <= '0;
                  phase_q <= SCALE_START;
                end else length_q <= inverse_q ? length_q << 1 : length_q >> 1;
              end
            end
          end
        end
        SCALE_START: phase_q <= SCALE_WAIT;
        SCALE_WAIT: if (mul_done) begin
          if (mul_error || mul_result >= 32'(modulus)) begin
            report_error_q <= 1'b1;
            scrub_index_q <= '0;
            phase_q <= SCRUB;
          end else begin
            coefficients[io_index_q] <= mul_result[22:0];
            if (io_index_q == 8'd255) begin
              io_index_q <= '0;
              phase_q <= DRAIN;
            end else begin
              io_index_q <= io_index_q + 8'd1;
              phase_q <= SCALE_START;
            end
          end
        end
        DRAIN: if (out_ready) begin
          coefficients[io_index_q] <= '0;
          if (io_index_q == 8'd255) begin
            dsa_q <= 1'b0;
            inverse_q <= 1'b0;
            operand_a_q <= '0;
            operand_b_q <= '0;
            io_index_q <= '0;
            twiddle_q <= '0;
            length_q <= '0;
            block_q <= '0;
            j_q <= '0;
            phase_q <= IDLE;
            done <= 1'b1;
          end else io_index_q <= io_index_q + 8'd1;
        end
        SCRUB: begin
          coefficients[scrub_index_q] <= '0;
          if (scrub_index_q == 8'd255) begin
            scrub_index_q <= '0;
            operand_a_q <= '0;
            operand_b_q <= '0;
            io_index_q <= '0;
            twiddle_q <= '0;
            length_q <= '0;
            block_q <= '0;
            j_q <= '0;
            dsa_q <= 1'b0;
            inverse_q <= 1'b0;
            done <= report_error_q;
            error <= report_error_q;
            report_error_q <= 1'b0;
            phase_q <= IDLE;
          end else scrub_index_q <= scrub_index_q + 8'd1;
        end
        default: begin
          scrub_index_q <= '0;
          report_error_q <= 1'b1;
          phase_q <= SCRUB;
        end
      endcase
    end
  end

endmodule
