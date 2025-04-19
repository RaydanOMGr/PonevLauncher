use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    FnArg, ItemFn, LitStr, PatType, Type, parse::Parse,
    parse::ParseStream, parse_macro_input
};

struct JniArgs {
    path: LitStr,
}

impl Parse for JniArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path = input.parse::<LitStr>()?;
        Ok(JniArgs { path })
    }
}

#[proc_macro_attribute]
pub fn jni(attr: TokenStream, item: TokenStream) -> TokenStream {
    let jni_args = parse_macro_input!(attr as JniArgs);
    let input_fn = parse_macro_input!(item as ItemFn);

    let vis = &input_fn.vis;
    let block = &input_fn.block;
    let return_type = &input_fn.sig.output;

    let java_path = jni_args.path.value();
    let jni_function_name = format!("Java_{}", java_path.replace(".", "_"));
    let jni_fn_ident = format_ident!("{}", jni_function_name);

    let lifetime = quote! { 'local };

    let mut transformed_args = Vec::new();

    for arg in &input_fn.sig.inputs {
        match arg {
            FnArg::Typed(PatType { pat, ty, attrs, .. }) => {
                let pat_clone = pat.clone();
                match &**ty {
                    Type::Path(type_path)
                        if type_path.path.segments.last().map_or(false, |seg| {
                            let name = seg.ident.to_string();
                            name.starts_with("J")
                        }) =>
                    {
                        let ty_str = quote! { #ty<#lifetime> }.to_string();
                        let new_ty: Type = syn::parse_str(&ty_str).unwrap();

                        transformed_args.push(FnArg::Typed(PatType {
                            attrs: attrs.clone(),
                            pat: pat_clone,
                            colon_token: Default::default(),
                            ty: Box::new(new_ty),
                        }));
                    }
                    _ => {
                        transformed_args.push(arg.clone());
                    }
                }
            }
            _ => transformed_args.push(arg.clone()),
        }
    }

    let output = quote! {
        #[unsafe(no_mangle)]
        #vis extern "C" fn #jni_fn_ident<#lifetime>(
            #(#transformed_args),*
        ) #return_type {
            #block
        }
    };

    output.into()
}
