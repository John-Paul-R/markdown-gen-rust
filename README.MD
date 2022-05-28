
# markdown-gen-rust

A collection of CLI tools for generating markdown from my own markup, written in rust

## example usage

File: `./listOfCities.md`
```markdown
[List of Cities]!code.json{{
[
    "New York",
    "Beijing",
    "Paris",
    "Buenos Aires"
]
}}
```

Usage:

```bash
cat ./listOfCities.md | markdown-gen-rust
```

Output:

````
<details>
<summary><code>List of Cities</code></summary>

```json
[
    "New York",
    "Beijing",
    "Paris",
    "Buenos Aires"
]

```

</details>
````

---

This project is in large part a way for me to learn rust. Be warned: bad code ahead!
