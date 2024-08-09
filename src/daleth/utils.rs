pub fn trim_indent(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

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

    trim_newline(&trimmed_lines.join("\n")).to_owned()
}

pub fn set_indent(input: &str, indent: usize) -> String {
    prepend_indent(&trim_indent(input), &"    ".repeat(indent))
}

fn trim_newline<'a>(s: &'a str) -> &'a str {
    let mut trim_start = 0;

    for start_char in s.chars() {
        if start_char != '\n' && start_char != '\r' {
            break;
        }

        trim_start += 1;
    }

    &s[(trim_start)..].trim_end()
}

fn prepend_indent(input: &str, indent: &str) -> String {
    let lines: Vec<String> = input
        .lines()
        .map(|line| format!("{}{}", indent, line))
        .collect();

    lines.join("\n")
}
