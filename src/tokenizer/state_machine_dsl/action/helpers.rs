macro_rules! action_helper {
    ( @emit_lex_result_with_raw_inclusive | $self:ident |> $token:expr ) => {
        debug!(@trace_raw $self, $self.pos + 1);
        action_helper!(@emit_lex_result |$self|> $token, Some(&$self.buffer[$self.raw_start..=$self.pos]));
    };

    ( @emit_lex_result_with_raw_exclusive | $self:ident |> $token:expr ) => {
        debug!(@trace_raw $self, $self.pos);
        action_helper!(@emit_lex_result |$self|> $token, Some(&$self.buffer[$self.raw_start..$self.pos]));
    };

    ( @emit_lex_result | $self:ident |> $token:expr, $raw:expr ) => {
        let res = LexResult {
            shallow_token: $token,
            raw: $raw,
        };

        ($self.token_handler)(res);
    };

    ( @set_token_part_range | $self:ident |> $part:ident ) => {
        $part.start = $self.token_part_start;
        $part.end = $self.pos - $self.raw_start;
    };
}