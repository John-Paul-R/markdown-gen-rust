extern crate test;

mod tests {
    use super::*;
    use test::Bencher;
    use crate::{exec};


    static INPUT_STRING: &str =
    "This is a test
[I wonder What this does]!code{{
code block body lmao
}}
[I wonder What this does]!code{{
code block body lmao
}}
[I wonder What this does]!code{{
code block body lmao
}}
[I wonder What this does]!code{{
code block body lmao
}}
[I wonder What this does]!code{{
code block body lmao
}}
[I wonder What this does]!code{{
code block body lmao
}}
[I wonder What this does]!code{{
code block body lmao
}}[I wonder What this does]!code{{
code block body lmao
}}[I wonder What this does]!code{{
code block body lmao
}}[I wonder What this does]!code{{
code block body lmao
}}[I wonder What this does]!code{{
code block body lmao
}}
";

    #[bench]
    fn bench_with_code_block(b: &mut Bencher) {
        b.iter(|| exec(INPUT_STRING));
    }

}
