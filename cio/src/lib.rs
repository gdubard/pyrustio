// lib.rs
use proc_macro::TokenStream;
use quote::quote;
use regex::Regex;
use syn::{parse_macro_input, Expr, LitStr, parse_str};

#[proc_macro]
pub fn printf(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let fmt_str = input.value();
    let re = Regex::new(r"\{([^{}]*?(?:\([^()]*\)[^{}]*)*?)(?::([^{}]*))?}").expect("Invalid regex");
    let mut args = Vec::new();
    let mut final_fmt = String::with_capacity(fmt_str.len());
    let mut last = 0;
    for cap in re.captures_iter(&fmt_str) {
        let m = cap.get(0).unwrap();
        final_fmt.push_str(&fmt_str[last..m.start()]);
        let expr = cap[1].trim();
        let fmt_spec = cap.get(2).map(|f| f.as_str());
        match fmt_spec {
            Some("a") => {
                final_fmt.push_str("{}");
                args.push(parse_str::<Expr>(&format!("format_container(&{})", expr))
                    .expect(&format!("Failed to parse: {}", expr)));
            },
            Some("c") => {
                final_fmt.push_str("{}");
                args.push(parse_str::<Expr>(&format!("format!(\"{{:?}}\", {})", expr))
                    .expect(&format!("Failed to parse: {}", expr)));
            },
            Some("j") => {
                final_fmt.push_str("{:#?}");
                args.push(parse_str::<Expr>(expr).expect(&format!("Failed to parse: {}", expr)));
            },
            Some(spec) => {
                final_fmt.push_str("{}");
                args.push(parse_str::<Expr>(&format!("format!(\"{{:{}}}\", {})", spec, expr))
                    .expect(&format!("Failed to parse: {}", expr)));
            },
            None => {
                final_fmt.push_str("{}");
                args.push(parse_str::<Expr>(expr).expect(&format!("Failed to parse: {}", expr)));
            },
        }
        last = m.end();
    }
    final_fmt.push_str(&fmt_str[last..]);
    quote! {
        {
            fn format_container<T: std::fmt::Debug>(value: &T) -> String {
                let debug_str = format!("{:?}", value);
                if debug_str.starts_with('[') {
                    // Tableaux et vecteurs
                    let depth = count_nesting_depth(&debug_str);
                    format_array(&debug_str, depth)
                } else if debug_str.starts_with('{') {
                    // Maps et structures
                    if !debug_str.contains('\n') && debug_str.len() < 100 {
                        debug_str
                    } else {
                        format!("{:#?}", value)
                    }
                } else {
                    debug_str
                }
            }
            fn count_nesting_depth(s: &str) -> usize {
                let mut depth = 0;
                let mut max_depth = 0;
                let mut in_quotes = false;
                for c in s.chars() {
                    match c {
                        '"' => in_quotes = !in_quotes,
                        '[' if !in_quotes => {
                            depth += 1;
                            max_depth = max_depth.max(depth);
                        },
                        ']' if !in_quotes => depth -= 1,
                        _ => {}
                    }
                }
                max_depth
            }
            fn format_array(debug_str: &str, depth: usize) -> String {
                match depth {
                    1 => debug_str.to_string(),
                    2 => debug_str
                        .replace("[[", "[\n    [")
                        .replace("]]", "]\n]")
                        .replace("], [", "],\n    ["),
                    3 => debug_str
                        .replace("[[[", "[\n    [\n        [")
                        .replace("]]]", "]\n    ]\n]")
                        .replace("]], [[", "]\n    ],\n    [\n        [")
                        .replace("], [", "],\n        ["),
                    4 => debug_str
                        .replace("[[[[", "[\n    [\n        [\n            [")
                        .replace("]]]]", "]\n        ]\n    ]\n]")
                        .replace("]]], [[[", "]\n    ],\n    [\n        [\n            [")
                        .replace("]], [[", "]\n        ],\n        [\n            [")
                        .replace("], [", "],\n            ["),
                    _ => format_complex_array(debug_str)
                }
            }
            fn format_complex_array(debug_str: &str) -> String {
                let mut result = String::new();
                let mut level = 0;
                let mut in_quotes = false;
                for c in debug_str.chars() {
                    match c {
                        '"' => {
                            in_quotes = !in_quotes;
                            result.push(c);
                        },
                        '[' if !in_quotes => {
                            level += 1;
                            result.push(c);
                            if level > 1 {
                                result.push('\n');
                                result.push_str(&"    ".repeat(level - 1));
                            }
                        },
                        ']' if !in_quotes => {
                            level -= 1;
                            if level >= 1 {
                                result.push('\n');
                                result.push_str(&"    ".repeat(level));
                            }
                            result.push(c);
                        },
                        ',' if !in_quotes => {
                            result.push(c);
                            // Ajouter un saut de ligne après la virgule si on est dans un tableau imbriqué
                            if level >= 1 && debug_str.chars().skip(debug_str.find(c).unwrap() + 1)
                                .find(|&c| !c.is_whitespace()).map(|c| c == '[').unwrap_or(false) {
                                result.push('\n');
                                result.push_str(&"    ".repeat(level));
                            }
                        },
                        _ => result.push(c),
                    }
                }

                result
            }
            println!(#final_fmt, #(#args),*);
        }
    }.into()
}

#[proc_macro]
pub fn input(input: TokenStream) -> TokenStream {
    let prompt = parse_macro_input!(input as LitStr);
    let prompt_str = prompt.value();
    quote! {{
        use std::io::{self, Write};
        loop {
            print!(#prompt_str);
            io::stdout().flush().expect("Failed to flush stdout");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let trimmed = input.trim();
            if trimmed.is_empty() {
                println!("Error: Unauthorized empty input.");
                continue;
            }
            match trimmed.parse() {
                Ok(value) => break value,
                Err(e) => {
                    println!("Error: {e}.");
                    continue;
                }
            }
        }
    }}.into()
}