use std::io;
use std::io::{Read};
use std::ops::{Range};

use regex::{Captures, Regex};

enum DetailsContentType {
    Text,
    Code,
}

// This setup has not translated to rust very well. Can be done better.
impl DetailsContentType {
    fn build_props(&self, decoration: &str) -> ContentTypeProps {
        match self {
            DetailsContentType::Text => ContentTypeProps {
                decoration_header: format!("{}{}", "```", decoration),
                decoration_footer: String::from("```"),
            },
            DetailsContentType::Code => ContentTypeProps {
                decoration_header: String::from("```"),
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

struct DetailsRegexMatch {
    range: Range<usize>,
    replace_text: String,
}

fn main() {
    let re = Regex::new(r"(?:\[(.+?)\])(?:(!\w+)(?:\.(\w+))?)?(?:\{\{\n?([\s\S]+?)?}})").unwrap();
    let mut buf = vec![];
    let lines = io::stdin().lock().read_to_end(&mut buf);
    drop(lines);

    let err_prefix = String::from("Error parsing input to utf8 string: ");
    let input_string = &String::from_utf8(buf)
        .unwrap_or_else(|err| err_prefix + &*err.to_string());

    let captures = re.captures_iter(input_string);

    let mut matches_vec: Vec<DetailsRegexMatch> = Vec::new();

    for c in captures {
        matches_vec.push(DetailsRegexMatch {
            range: c.get(0).unwrap().range(),
            replace_text: build_details_text(match_to_replace_props(c))
        });
    }

    let mut out_frags: Vec<String> = Vec::new();
    let mut prev_end_idx: usize = 0;
    for m in matches_vec {
        out_frags.push(input_string[prev_end_idx..m.range.start].to_owned());
        out_frags.push(m.replace_text);
        prev_end_idx = m.range.end;
    }

    println!("{}", out_frags.concat());

}

fn match_to_replace_props(c: Captures) -> DetailsReplaceProps {
    let content_type = c.get(2)
        .map(|s| if s.as_str().eq("!code") { DetailsContentType::Code } else { DetailsContentType::Text })
        .unwrap_or(DetailsContentType::Text);
    let decoration = c.get(3).map(|m| m.as_str().to_owned()).unwrap_or_else(String::new);
    DetailsReplaceProps {
        summary: c[1].to_string(),
        content_type_props: content_type.build_props(&decoration),
        content_body: c.get(4).map(|m| m.as_str().to_owned()).unwrap_or_else(|| String::from("Unwrapped None")),
    }
}

fn build_details_text(m: DetailsReplaceProps) -> String {
    return format!(
        "<details>\n<summary><code>{summary}</code></summary>

{content_header}
{content_body}
{content_footer}

</details>",
        summary=m.summary,
        content_header= m.content_type_props.decoration_header,
        content_body=m.content_body,
        content_footer= m.content_type_props.decoration_footer);
}