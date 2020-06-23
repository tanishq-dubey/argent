pub fn brace_expand(input: &str) -> &str {
    /*
     * https://www.gnu.org/software/bash/manual/html_node/Brace-Expansion.html
     *
     * Patterns to be expanded take the form of
     *  - preamble (optional)
     *  - either (between braces)
     *    - a series of comma seperated strings
     *    - a sequence expression
     *  - postscript (optional)
     *
     * A sequence expression has the format
     *  {x..y[..increment]}
     *  x and y are either integers or single characters
     *  increment is an integer
     *
     * Any incorrectly formed brace expression is left unchanged
     *
     * Examples:
     * {aa,bb,cc,dd}  => aa bb cc dd
     * {0..12}        => 0 1 2 3 4 5 6 7 8 9 10 11 12
     * {1..10..2}     => 1 3 5 7 9
     * {g..a}         => g f e d c b a
     * {A..1}         => A @ ? > = < ; : 9 8 7 6 5 4 3 2 1
     * {-1..A}        => -1..A
     * a{1..10..2}b     => a1b a3b a5b a7b a9b
     * ^preamble  ^postscript
     */

    /*
     * We should first get any preamble/postscript:
     *  Format: abc{someexpansion}def
     *  where abc is preamble and def is postscript
     */

    let f_pass: Vec<&str> = input.splitn(2, "{").collect::<Vec<&str>>();
    if f_pass.len() != 2 {
        return input;
    }

    let mut preamble: &str = "";
    let mut postscript: &str = "";
    match f_pass.get(0) {
        None => return input, // There is no left bracket, so just return
        Some(v) => {
            preamble = v;
        }
    }
    // At this point, we have a left bracket, check for right by doing a split
    match f_pass.get(1) {
        None => return input, // There is no left bracket, so just return
        Some(v) => {
            let s_pass: Vec<&str> = v.split("}").collect::<Vec<&str>>();
            if s_pass.len() < 2 {
                return input
            }
        }
    }

    println!("{:?}", do_expand("a", "g"));
    println!("{:?}", do_expand("g", "a"));
    println!("{:?}", do_expand("A", "1"));
    return ""
}

fn do_expand(start: &str, end: &str) -> Vec<String> {
}

fn do_expand_numeric(start: i64, end: i64) -> Vec<String> {
    let nums =(start..=end)
        .map(|c| c.to_string())
        .collect::<Vec<String>>();
    return nums
}

fn do_expand_char(start: u8, end: u8) -> Vec<char> {
    if start > end {
        let values = (end..=start)
            .map(|c| c as char)
            .collect::<Vec<_>>();
        let rev_val = values.into_iter().rev().collect::<Vec<char>>();
        return rev_val;
    }

    let values = (start..=end)
        .map(|c| c as char)
        .collect::<Vec<_>>();
    return values;
}
