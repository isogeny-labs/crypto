// Byte-stream SHAKE128/SHAKE256 with command lengths and ready/valid backpressure.
// Exactly input_length bytes are absorbed and output_length bytes are squeezed.
// start is accepted only when ready. clear cancels and wipes the current command.
module shake_stream (
    input  logic        clk,
    input  logic        rst_n,
    input  logic        clear,
    input  logic        start,
    input  logic        mode_256,
    input  logic [31:0] input_length,
    input  logic [31:0] output_length,
    output logic        ready,
    output logic        busy,
    output logic        done,
    input  logic        in_valid,
    output logic        in_ready,
    input  logic [7:0]  in_byte,
    output logic        out_valid,
    input  logic        out_ready,
    output logic [7:0]  out_byte
);
  typedef enum logic [2:0] {IDLE, ABSORB, PAD, PERM_START, PERM_WAIT, SQUEEZE} phase_t;
  phase_t phase_q, after_perm_q;
  logic mode_q;
  logic [7:0] position_q, rate;
  logic [31:0] input_left_q, output_left_q;
  logic [7:0] sponge_q [0:199];
  logic [1599:0] perm_state, perm_result;
  logic perm_done, perm_ready, unused_perm_busy;

  assign rate = mode_q ? 8'd136 : 8'd168;
  assign ready = rst_n && !clear && phase_q == IDLE;
  assign busy = phase_q != IDLE;
  assign in_ready = rst_n && !clear && phase_q == ABSORB;
  assign out_valid = rst_n && !clear && phase_q == SQUEEZE && output_left_q != 0;
  assign out_byte = out_valid ? sponge_q[position_q] : 8'b0;
  // Byte-addressed write enables avoid a wide variable-part-select update mux.
  for (genvar i = 0; i < 200; i++) begin : pack_state
    assign perm_state[8*i +: 8] = sponge_q[i];
  end

  // A completed permutation is copied into sponge_q and immediately erased here.
  keccak_f1600 permutation (
      .clk(clk), .rst_n(rst_n),
      .clear(clear || (phase_q == PERM_WAIT && perm_done)),
      .start(phase_q == PERM_START), .state_in(perm_state),
      .ready(perm_ready), .busy(unused_perm_busy), .done(perm_done), .state_out(perm_result)
  );

  always_ff @(posedge clk or negedge rst_n) begin
    if (!rst_n) begin
      phase_q <= IDLE;
      after_perm_q <= IDLE;
      mode_q <= 1'b0;
      position_q <= '0;
      input_left_q <= '0;
      output_left_q <= '0;
      for (int i = 0; i < 200; i++) sponge_q[i] <= '0;
      done <= 1'b0;
    end else if (clear) begin
      phase_q <= IDLE;
      after_perm_q <= IDLE;
      mode_q <= 1'b0;
      position_q <= '0;
      input_left_q <= '0;
      output_left_q <= '0;
      for (int i = 0; i < 200; i++) sponge_q[i] <= '0;
      done <= 1'b0;
    end else begin
      done <= 1'b0;
`ifndef SYNTHESIS
      assert (position_q < rate);
      if (phase_q == IDLE)
        for (int i = 0; i < 200; i++) assert (sponge_q[i] == 0);
      if (phase_q == ABSORB) assert (input_left_q != 0);
      if (phase_q == SQUEEZE) assert (output_left_q != 0);
      if (phase_q == PERM_START) assert (perm_ready);
`endif
      case (phase_q)
        IDLE: if (start) begin
          mode_q <= mode_256;
          input_left_q <= input_length;
          output_left_q <= output_length;
          position_q <= '0;
          for (int i = 0; i < 200; i++) sponge_q[i] <= '0;
          phase_q <= input_length == 0 ? PAD : ABSORB;
        end
        ABSORB: if (in_valid) begin
          sponge_q[position_q] <= sponge_q[position_q] ^ in_byte;
          input_left_q <= input_left_q - 32'd1;
          if (position_q == rate - 8'd1) begin
            position_q <= '0;
            after_perm_q <= input_left_q == 32'd1 ? PAD : ABSORB;
            phase_q <= PERM_START;
          end else begin
            position_q <= position_q + 8'd1;
            if (input_left_q == 32'd1) phase_q <= PAD;
          end
        end
        PAD: begin
          if (position_q == rate - 8'd1) sponge_q[position_q] <= sponge_q[position_q] ^ 8'h9f;
          else begin
            sponge_q[position_q] <= sponge_q[position_q] ^ 8'h1f;
            sponge_q[rate - 8'd1] <= sponge_q[rate - 8'd1] ^ 8'h80;
          end
          after_perm_q <= SQUEEZE;
          phase_q <= PERM_START;
        end
        PERM_START: phase_q <= PERM_WAIT;
        PERM_WAIT: if (perm_done) begin
          for (int i = 0; i < 200; i++) sponge_q[i] <= perm_result[8*i +: 8];
          position_q <= '0;
          phase_q <= after_perm_q;
          if (after_perm_q == SQUEEZE && output_left_q == 0) begin
            for (int i = 0; i < 200; i++) sponge_q[i] <= '0;
            mode_q <= 1'b0;
            after_perm_q <= IDLE;
            phase_q <= IDLE;
            done <= 1'b1;
          end
        end
        SQUEEZE: if (out_ready) begin
          output_left_q <= output_left_q - 32'd1;
          if (output_left_q == 32'd1) begin
            for (int i = 0; i < 200; i++) sponge_q[i] <= '0;
            position_q <= '0;
            mode_q <= 1'b0;
            after_perm_q <= IDLE;
            phase_q <= IDLE;
            done <= 1'b1;
          end else if (position_q == rate - 8'd1) begin
            position_q <= '0;
            after_perm_q <= SQUEEZE;
            phase_q <= PERM_START;
          end else position_q <= position_q + 8'd1;
        end
        default: begin
          phase_q <= IDLE;
          after_perm_q <= IDLE;
          mode_q <= 1'b0;
          position_q <= '0;
          input_left_q <= '0;
          output_left_q <= '0;
          for (int i = 0; i < 200; i++) sponge_q[i] <= '0;
        end
      endcase
    end
  end

endmodule
