// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(unused)]

fn main() {
    print!("Hello, World!");
    println!();
    termimad::print_inline("*Hello, World!*");
    println!();
    termimad::print_inline("**Hello, World!**");
    let now = std::time::Instant::now();
    let output = termimad::term_text(
        r#"
# My Project!

Here's an example: 
```rust
fn hello() -> Option<String> {
    println!("Hello, World!");
    None
}
```

## List
- Item 1
- Item 2
- Item 3
    - Item 3.1
    - Item 3.2
- Item 4

## From Readme
|:-:|:-:|-
|**feature**|**supported**|**details**|
|-:|:-:|-
| tables | yes | pipe based, with or without alignments
| italic, bold | yes | star based |
| inline code | yes | `with backquotes` (it works in tables too)
| code bloc | yes |with tabs or code fences
| syntax coloring | no |
| crossed text |  ~~not yet~~ | wait... now it works `~~like this~~`
| horizontal rule | yes | Use 3 or more dashes (`---`)
| lists | yes|* unordered lists supported
|  | |* ordered lists *not* supported
| quotes |  yes |> What a wonderful time to be alive!
| links | no | (but your terminal already handles raw URLs)
|-
"#,
    );
    let elapsed = now.elapsed();

    println!("{}", output);
    println!("{:#?}", output);
    println!("Markdown parsing took {:?}", elapsed);
}
