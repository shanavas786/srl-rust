use pest::prelude::*;

impl_rdp! {
    grammar! {
        digit = _{ ['0'..'9'] }
        number = @{ digit+ }
        lower_apha = _{ ['a'..'z'] }
        upper_apha = _{ ['A'..'Z'] }
        alpha = @{ (lower_apha | upper_apha)* }
        whitespace = _{ [" "] | ["\t"] | ["\u{000C}"] | ["\r"] | ["\n"] | [","] }

        escape_sequence = _{ ["\\\\"] | ["\\\""] | ["\\\'"] | ["\\n"] | ["\\r"] | ["\\t"] }
        literal_char = { escape_sequence | (!["\""] ~ any) }
        string_literal = @{ ["b\""] ~ literal_char* ~ ["\""] }

        exactly = { [i"exactly"] ~ number ~ [i"times"] }
        once = { [i"once"] }
        twice = { [i"twice"] }
        between_x_y = { [i"between"] ~ number ~ [i"and"] ~ number ~ [i"times"]? }
        optional = { [i"optional"] }
        once_or_more = { [i"once"] ~ [i"or"] ~ [i"more"] }
        never_or_more = { [i"two"] ~ [i"or"] ~ [i"more"] }
        atleast_x = { [i"atleast"] ~ number ~ [i"times"] }

        quantifer = _{ exactly | once | twice | between_x_y | optional |
                       once_or_more | never_or_more | atleast_x }

        anchor = { (([i"begin"] | [i"start"]) ~ [i"with"]) |
                      ([i"must"] ~ [i"end"]) }

        flag = { ([i"case"] ~ [i"insensitive"]) |
                   [i"multiline"] |
                   ([i"all"] ~ [i"lazy"]) }

        literally = { [i"literally"] ~ string_literal }
        oneof = { [i"one"] ~ [i"of"]~ string_literal }
        letter = { [i"letter"] ~ ([i"from"] ~ lower_apha ~ [i"to"] ~ lower_apha)? }
        upperletter = { [i"upper"] ~ [i"letter"] ~ ([i"from"] ~ upper_apha ~ [i"to"] ~ upper_apha)? }
        anycharacter = { [i"any"] ~ [i"character"] }
        nocharacter = { [i"no"] ~ [i"character"] }
        chardigit = { [i"digit"] ~ ([i"from"] ~ digit ~ [i"to"] ~ digit)? }
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
                      until }


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

        expression = { (character ~ quantifer) |
                        (character ~ anchor) |
                        (character ~ quantifer ~ anchor) |
                        (anchor ~ character) |
                        (anchor ~ anchor) |
                        (character ~ flag) |
                        (character ~ lookaround) |
                        (lookaround ~ character) |
                        (character ~ character) }
    }
}
