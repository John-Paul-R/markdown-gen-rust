extern crate core;

use std::io;
use std::io::{Read};

use regex::{Captures, Regex};

enum DetailsContentType {
    Text,
    Code,
}

impl DetailsContentType {
    fn build_props(&self, decoration: &str) -> ContentTypeProps {
        match self {
            DetailsContentType::Text => ContentTypeProps {
                decoration_header: String::new(),
                decoration_footer: String::new(),
            },
            DetailsContentType::Code => ContentTypeProps {
                decoration_header: format!("{}{}", "```", decoration),
                decoration_footer: String::from("```"),
            },
        }
    }
}

struct ContentTypeProps {
    decoration_header: String,
    decoration_footer: String,
}

struct DetailsReplaceProps {
    summary: String,
    content_type_props: ContentTypeProps,
    content_body: String,
}

fn main() {
    let re = Regex::new(r"\[(.+?)](?:!?(\w+)(?:\.(\w+))?)?\{\{\n?([\s\S]+?)?}}").unwrap();
    let mut buf = vec![];
    let _ = io::stdin().lock().read_to_end(&mut buf);

    let input_string = String::from_utf8(buf)
        .unwrap_or_else(|err|
            panic!("Error parsing input to utf8 string: {}", err));

    let mut out_frags: Vec<String> = vec![];
    let mut prev_end_idx: usize = 0;
    for c in re.captures_iter(&input_string) {
        // Range of the full match in the input string
        let range = c.get(0).unwrap().range();

        // Append the non-matched fragment (the range between prev match and this one)
        out_frags.push(input_string[prev_end_idx..range.start].to_owned());
        // Append the matched fragment's newly-built replace text
        out_frags.push(build_details_text(match_to_replace_props(c)));

        prev_end_idx = range.end;
    }
    out_frags.push(input_string[prev_end_idx..].to_owned());

    print!("{}", out_frags.concat());
}

fn match_to_replace_props(c: Captures) -> DetailsReplaceProps {
    let content_type = c.get(2)
        .map(|m| m.as_str())
        .map(match_content_type)
        .unwrap_or(DetailsContentType::Text);

    let decoration = c.get(3)
        .map(|m| m.as_str().to_owned())
        .unwrap_or_else(String::new);

    DetailsReplaceProps {
        summary: c[1].to_string(),
        content_type_props: content_type.build_props(&decoration),
        content_body: c.get(4)
            .map(|m| m.as_str().to_owned())
            .unwrap_or_else(|| panic!("Unwrapped None for matched `content_body`")),
    }
}

fn match_content_type(input_string: &str) -> DetailsContentType {
    match input_string {
        "!code" => DetailsContentType::Code,
        _ => DetailsContentType::Text,
    }
}

fn build_details_text(m: DetailsReplaceProps) -> String {
    return format!(
        "<details>\n<summary><code>{summary}</code></summary>

{content_header}
{content_body}
{content_footer}

</details>",
        summary = m.summary,
        content_header = m.content_type_props.decoration_header,
        content_body = m.content_body,
        content_footer = m.content_type_props.decoration_footer);
}
