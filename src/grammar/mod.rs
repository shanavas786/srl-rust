use pest::prelude::*;

impl_rdp! {
    grammar! {
        digit = _{ ['0'..'9'] }
        single_digit = { digit }
        number = @{ digit+ }
        lower_alpha = { ['a'..'z'] }
        upper_alpha = { ['A'..'Z'] }
        whitespace = _{ [" "] | ["\t"] | ["\u{000C}"] | ["\r"] | ["\n"] | [","] }

        escape_sequence = _{ ["\\\\"] | ["\\\""] | ["\\\'"] | ["\\n"] | ["\\r"] | ["\\t"] }
        literal_char = _{ escape_sequence | (!["\""] ~ any) }
        string_literal = @{ (["\""] | ["'"]) ~ literal_char* ~ (["\""] | ["'"]) }

        exactly = { [i"exactly"] ~ number ~ [i"times"] }
        once = { [i"once"] }
        twice = { [i"twice"] }
        between_x_y = { [i"between"] ~ number ~ [i"and"] ~ number ~ [i"times"]? }
        optional = { [i"optional"] }
        once_or_more = { [i"once"] ~ [i"or"] ~ [i"more"] }
        never_or_more = { [i"never"] ~ [i"or"] ~ [i"more"] }
        atleast_x = { [i"atleast"] ~ number ~ [i"times"] }

        quantifer = _{ exactly | once | twice | between_x_y | optional |
                       once_or_more | never_or_more | atleast_x }

        begin_with = { ([i"begin"] | [i"start"]) ~ [i"with"] }
        must_end = { [i"must"] ~ [i"end"] }
        anchor = _{  begin_with | must_end }

        case_insensitive = { [i"case"] ~ [i"insensitive"] }
        multiline = { [i"multiline"] }
        all_lazy = { [i"all"] ~ [i"lazy"] }
        flag = _{ case_insensitive | multiline | all_lazy }

        literally = { [i"literally"] ~ string_literal }
        oneof = { [i"one"] ~ [i"of"]~ string_literal }
        letter = { [i"letter"] ~ ([i"from"] ~ lower_alpha ~ [i"to"] ~ lower_alpha)? }
        upperletter = { [i"upper"] ~ [i"case"] ~ [i"letter"] ~ ([i"from"] ~ upper_alpha ~ [i"to"] ~ upper_alpha)? }
        anycharacter = { [i"any"] ~ [i"character"] }
        nocharacter = { [i"no"] ~ [i"character"] }
        chardigit = { [i"digit"] ~ ([i"from"] ~ single_digit ~ [i"to"] ~ single_digit)? }
        anything = { [i"anything"] }
        newline = { [i"new"] ~ [i"line"] }
        space = { [i"whitespace"] }
        tab = { [i"tab"] }
        nospace = { [i"no"] ~ [i"whitespace"] }
        raw = { [i"raw"] ~ string_literal }

        character = { literally |
                      oneof |
                      letter |
                      upperletter |
                      anycharacter |
                      nocharacter |
                      chardigit |
                      anything |
                      newline |
                      space |
                      nospace |
                      raw |
                      group |
                      capture |
                      anyof |
                      until |
                      (character ~ quantifer) |
                      (character ~ anchor) |
                      (character ~ quantifer ~ anchor) |
                      (anchor ~ character) |
                      (anchor ~ anchor) |
                      (character ~ flag) |
                      (character ~ lookaround) |
                      (lookaround ~ character) |
                      (character+) }

        group = { string_literal |
                  (["("] ~ character ~ [")"]) }
        capture = { [i"capture"] ~ group  ~ ([i"as"] ~ string_literal)? }
        anyof = { ([i"any"] | [i"either"]) ~ [i"of"] ~ group }
        until = { [i"until"] ~ group }

        iffollowedby = { [i"if"] ~ [i"followed"] ~ [i"by"] ~ group }
        ifnotfollowedby = { [i"if"] ~ [i"not"] ~ [i"followed"] ~ [i"by"] ~ group }
        ifalreadyhad = { [i"if"] ~ [i"already"] ~ [i"had"] ~ group }
        ifnotalreadyhad = { [i"if"] ~ [i"not"] ~ [i"already"] ~ [i"had"] ~ group }
        lookaround = _{ iffollowedby | ifnotfollowedby | ifalreadyhad | ifnotfollowedby }

        srl = _{ soi ~ character? ~ eoi }
    }

}

#[cfg(test)]
mod tests;
