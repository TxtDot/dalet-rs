pub fn trim_indent(input: &str) -> String {
    let lines: Vec<&str> = trim_unused(input).lines().collect();

    // Find the minimum indentation of non-empty lines
    let min_indent = lines
        .iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().take_while(|c| c.is_whitespace()).count())
        .min()
        .unwrap_or(0);

    // Trim the leading whitespace from each line by the minimum indentation
    let trimmed_lines: Vec<&str> = lines
        .into_iter()
        .map(|line| {
            if line.len() < min_indent {
                line
            } else {
                &line[min_indent..]
            }
        })
        .collect();

    trimmed_lines.join("\n")
}

pub fn set_indent(input: &str, indent: usize) -> String {
    prepend_indent(&trim_indent(input), indent)
}

pub fn set_spaces(input: &str, spaces: usize) -> String {
    prepend_spaces(&trim_indent(input), spaces)
}

fn trim_unused<'a>(s: &'a str) -> &'a str {
    let mut trim_start = 0;
    let mut been_newlines = false;

    for start_char in s.chars() {
        if !been_newlines
            && (char::is_whitespace(start_char) && start_char != '\n' && start_char != '\r')
        {
            trim_start += 1;
            continue;
        } else if start_char != '\n' && start_char != '\r' {
            break;
        } else {
            been_newlines = true;
            trim_start += 1;
        }
    }

    &s[(trim_start)..].trim_end()
}

pub fn prepend_indent(input: &str, indent: usize) -> String {
    prepend_spaces(input, indent * 4)
}

fn prepend_spaces(input: &str, spaces: usize) -> String {
    let indent = &" ".repeat(spaces);
    let lines: Vec<String> = input
        .lines()
        .map(|line| format!("{}{}", indent, line))
        .collect();

    lines.join("\n")
}
