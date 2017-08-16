pub trait SrlChar {
    fn is_srl_whitespace(self) -> bool;
    fn is_group_start(self) -> bool;
    fn is_group_end(self) -> bool;
    fn is_group_char(self) -> bool;
    fn is_space(self) -> bool;
    fn is_quote(self) -> bool;
    fn is_backslash(self) -> bool;
}

impl SrlChar for char {
    fn is_srl_whitespace(self) -> bool {
        (self == ' ') || (self == ',') || (self == '\n') || (self == '\t')
    }

    fn is_group_start(self) -> bool {
        self == '('
    }

    fn is_group_end(self) -> bool {
        self == ')'
    }

    fn is_group_char(self) -> bool {
        self.is_group_start() || self.is_group_end()
    }

    fn is_space(self) -> bool {
        self == ' '
    }

    fn is_quote(self) -> bool {
        (self == '\'') || (self == '"')
    }

    fn is_backslash(self) -> bool {
        self == '\\'
    }
}
