use proc_macro::{TokenStream, TokenTree};
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, FnArg, ItemFn};

enum AttrParam {
    Name,
    Type,
}

struct VarargConfig {
    macro_name: TokenStream2,
    prefix: TokenStream2,
}

#[proc_macro_attribute]
pub fn vararg(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);
    let name = func.sig.ident.clone();
    let args = func.sig.inputs.clone();
    let (first, last) = args.into_iter().fold((vec![], None), |mut t, e| match t.1 {
        None => (t.0, Some(e)),
        Some(c) => {
            t.0.push(c);
            (t.0, Some(e))
        }
    });
    let last: FnArg = last.expect("Function with 'vararg' attribute has no args");
    if let FnArg::Receiver(_) = last {
        panic!("Function with 'vararg' attribute has only self arg")
    }

    let (first_args, first_uses): (String, String) = (0..first.len())
        .map(|i| (format!("$arg{}:expr, ", i), format!("$arg{}, ", i)))
        .unzip();
    let mut need_comma: Option<TokenStream2> = Some(",".parse().unwrap());
    // don't want to use itertools (bc it's usage can leak it's use_std feature), and
    // don't want to make heap reallocs in cycle when concat strings, and
    // don't want to do special calculation of str len, and alloc exact size of memory
    // so use strip_suffix with empty string corner case
    let first_args = first_args.strip_suffix(", ").unwrap_or_else(|| {
        need_comma = None;
        ""
    });
    let first_args: TokenStream2 = first_args.parse().unwrap();
    let first_uses: TokenStream2 = first_uses.parse().unwrap();

    let VarargConfig { macro_name, prefix } = attrs_parse(attrs, name.to_token_stream());

    let result = quote! {
        #func
        macro_rules! #macro_name {
            (#first_args) => {#name(#first_uses #prefix [])};
            (#first_args #need_comma $($lasts:expr),*) => {#name(#first_uses #prefix [$($lasts),*])};
        }
    };
    result.to_token_stream().into()
}

fn attrs_parse(attrs: TokenStream, default_name: TokenStream2) -> VarargConfig {
    let attr_format_panic_msg = "vararg attribute params list must be comma-separated list of \
                params in 'ident = ident' format. \n\
                For example: `#[vararg(name = macro_name, type = vec)]`";

    if attrs.is_empty() {
        return VarargConfig {
            macro_name: default_name,
            prefix: "".parse().unwrap(),
        };
    }

    let mut macro_name = default_name;
    let mut prefix: TokenStream2 = "".parse().unwrap();

    let v = attrs.into_iter().collect::<Vec<_>>();
    for chunk in v.chunks(4) {
        let p = match &chunk[0] {
            TokenTree::Ident(i) => match i.to_string().as_str() {
                "name" => AttrParam::Name,
                "type" => AttrParam::Type,
                _ => panic!("vararg attribute params currently can be 'name' or 'type' only"),
            },
            _ => panic!("{}", attr_format_panic_msg),
        };
        match &chunk[1] {
            TokenTree::Punct(p) => {
                if p.as_char() != '=' {
                    panic!("{}", attr_format_panic_msg)
                }
            }
            _ => panic!("{}", attr_format_panic_msg),
        }
        match &chunk[2] {
            TokenTree::Ident(i) => match (p, i.to_string().as_str()) {
                (AttrParam::Name, i) => macro_name = i.parse().unwrap(),
                (AttrParam::Type, "vec") => prefix = "vec!".parse().unwrap(),
                (AttrParam::Type, "array") => prefix = "".parse().unwrap(),
                (AttrParam::Type, "slice") => prefix = "&".parse().unwrap(),
                (AttrParam::Type, _) => panic!("type param must be 'vec', 'array' or 'slice'"),
            },
            _ => panic!("{}", attr_format_panic_msg),
        }
    }

    VarargConfig { macro_name, prefix }
}
