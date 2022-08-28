extern crate test;

mod tests {
    use super::*;
    use test::Bencher;
    use crate::{exec, exec_join, exec_push_str};

    // #[test]
    // fn run_with_code_block() {
    //     main();
    // }

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

    #[bench]
    fn bench_with_code_block_push_str(b: &mut Bencher) {
        b.iter(|| exec_push_str(INPUT_STRING));
    }

    #[bench]
    fn bench_with_code_block_join(b: &mut Bencher) {
        b.iter(|| exec_join(INPUT_STRING));
    }

}
