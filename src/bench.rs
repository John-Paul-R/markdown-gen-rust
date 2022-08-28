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

    #[test]
    fn basic_valid() {
        let input = "Testing pre text[clickable thing]{{summary body}}post text";
        let expected_output =
"Testing pre text<details>
<summary><code>clickable thing</code></summary>


summary body


</details>post text";
        let output = exec(input);
        assert_eq!(expected_output, output);
    }

    #[test]
    fn basic_code_valid() {
        let input = "Testing pre text[clickable thing]!code{{summary body}}post text";
        let expected_output =
            "Testing pre text<details>
<summary><code>clickable thing</code></summary>

```
summary body
```

</details>post text";
        let output = exec(input);
        assert_eq!(expected_output, output);
    }

    #[test]
    fn basic_code_with_lang_valid() {
        let input = "Testing pre text[clickable thing]!code.json{{summary body}}post text";
        let expected_output =
            "Testing pre text<details>
<summary><code>clickable thing</code></summary>

```json
summary body
```

</details>post text";
        let output = exec(input);
        assert_eq!(expected_output, output);
    }
}
