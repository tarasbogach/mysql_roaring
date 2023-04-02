use quote::{format_ident, quote};
use proc_macro::TokenStream;

fn uc_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[proc_macro]
pub fn maps_op(_input: TokenStream) -> TokenStream {
    let mut output = quote!();
    for bit_size in vec!["32", "64"] {
        for nullsafe in vec![true, false] {
            for count in vec![true, false] {
                for op in vec!["or", "xor", "and"] {
                    let struct_name = format_ident!("Roaring{}{}{}{}",  bit_size,  if nullsafe {"Nullsafe"} else {""}, uc_first(op), if count {"Count"} else {""});
                    let struct_fields = if count {quote!()} else {quote!(vec: Vec<u8>)};
                    let map_type_name = format_ident!("Map{}", bit_size);
                    let op_fn_name = format_ident!("bit{}_assign", op);
                    let return_type = if count {quote!(i64)} else  { quote!(&'a [u8]) };
                    let return_type = if nullsafe {quote!(#return_type)} else {quote!(Option<#return_type>)};
                    let maybe_null = if nullsafe {quote!()} else {quote!(cfg.set_maybe_null(true);)};
                    let on_null_arg = if nullsafe {quote!(())} else {quote!(return Ok(None))};
                    let on_null_agg = if nullsafe {quote!(<#map_type_name>::new())} else {quote!(return Ok(None))};
                    let bytes_result = quote!(&self.vec[..]);
                    let bytes_result = if nullsafe {quote!(#bytes_result)} else {quote!(Some(#bytes_result))};
                    let count_result = quote!(agg.len() as i64);
                    let count_result = if nullsafe {quote!(#count_result)} else {quote!(Some(#count_result))};
                    let count_null_result = if nullsafe {quote!(0i64)} else {quote!(None)};
                    let result = if count {
                      quote!{
                        match maybe_agg {
                            Some(agg) => Ok(#count_result),
                            None => Ok(#count_null_result)
                        }
                      }
                    } else {
                      quote!{
                        let agg = match maybe_agg {
                            Some(agg) => agg,
                            None => #on_null_agg,
                        };
                        let mut bytes = Vec::with_capacity(agg.serialized_size());
                        if let Ok(_) = agg.serialize_into(&mut bytes) {
                            self.vec = bytes;
                            Ok(#bytes_result)
                        } else {
                            Err(ProcessError)
                        }
                      }
                    };
                    // let null_result = if nullsafe {quote!(bytes)} else {quote!(None)};
                    // let null_result = if nullsafe {quote!(bytes)} else {quote!(None)};
                    output.extend(quote!{
                        #[derive(Default)]
                        struct #struct_name {
                            #struct_fields
                        }

                        #[register]
                        impl BasicUdf for #struct_name {
                            type Returns<'a> = #return_type;

                            fn init(cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
                                if args.len() < 2 {
                                    return Err(format!("Expected two or more CLOB (bitmap) or NULL arguments; Got {} arguments.", args.len()));
                                }
                                for arg in args.iter() {
                                    if let SqlResult::String(_) = arg.value() {} else {
                                        return Err(format!("Only CLOB (bitmap) or NULL arguments are allowed."));
                                    }
                                }
                                #maybe_null
                                Ok(Self::default())
                            }

                            fn process<'a>(&'a mut self, _cfg: &UdfCfg<Process>, args: &ArgList<Process>, _error: Option<NonZeroU8>) -> Result<Self::Returns<'a>, ProcessError> {
                                let mut maybe_agg: Option<#map_type_name> = None;
                                for arg in args.iter() {
                                    match arg.value().as_bytes() {
                                        None => #on_null_arg,
                                        Some(bytes) => {
                                            if let Ok(map) = <#map_type_name>::deserialize_from(bytes) {
                                                match maybe_agg {
                                                    None => maybe_agg = Some(map),
                                                    Some(ref mut agg) => agg.#op_fn_name(map),
                                                }
                                            } else {
                                                return Err(ProcessError);
                                            }
                                        }
                                    }
                                }
                                #result
                            }
                        }
                    })
                }
            }
        }
    }
    output.into()
}

#[proc_macro]
pub fn map_contains(_input: TokenStream) -> TokenStream {
    let mut output = quote!();
    for bit_size in vec!["32", "64"] {
        for nullsafe in vec![true, false] {
            let struct_name = format_ident!("Roaring{}{}Contains",  bit_size,  if nullsafe {"Nullsafe"} else {""});
            let cast_function_name = format_ident!("to_u{}", bit_size);
            let map_type_name = format_ident!("Map{}", bit_size);
            let return_type = if nullsafe {quote!(i64)} else {quote!(Option<i64>)};
            let null_result = if nullsafe {quote!(0i64)} else {quote!(None)};
            let result = quote!(map.contains(value).into());
            let result = if nullsafe {quote!(#result)} else {quote!(Some(#result))};
            let maybe_null_cnf = if nullsafe {quote!(_cfg)} else {quote!{cfg}};
            let maybe_null = if nullsafe {quote!()} else {quote!{cfg.set_maybe_null(true);}};
            output.extend(quote! {
                #[derive(Default)]
                struct #struct_name;

                #[register]
                impl BasicUdf for #struct_name {
                    type Returns<'a> = #return_type;

                    fn init(#maybe_null_cnf: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
                        if args.len() < 2 {
                            return Err(format!("Expected two arguments; Got {} arguments.", args.len()));
                        }
                        if !args.get(0).unwrap().value().is_string() {
                            return Err(format!("First argument mast be CLOB (bitmap) or NULL."));
                        }
                        if !args.get(1).unwrap().value().is_int() {} else {
                            return Err(format!("Second argument mast be INT or NULL."));
                        }
                        #maybe_null
                        Ok(Self::default())
                    }

                    fn process<'a>(&'a mut self, _cfg: &UdfCfg<Process>, args: &ArgList<Process>, _error: Option<NonZeroU8>) -> Result<Self::Returns<'a>, ProcessError> {
                        let arg0 = args.get(0).unwrap().value();
                        let arg1 = args.get(1).unwrap().value();
                        let bytes = arg0.as_bytes();
                        let value = arg1.as_int();
                        if let (Some(bytes), Some(value)) = (bytes, value) {
                            if let (Ok(map), Some(value)) = (#map_type_name::deserialize_from(bytes), value.#cast_function_name()) {
                                Ok(#result)
                            } else {
                                Err(ProcessError)
                            }
                        } else {
                            Ok(#null_result)
                        }
                    }
                }
            });
        }
    }
    output.into()
}

#[proc_macro]
pub fn map_count(_input: TokenStream) -> TokenStream {
    let mut output = quote!();
    for bit_size in vec!["32", "64"] {
        for nullsafe in vec![true, false] {
            let struct_name = format_ident!("Roaring{}{}Count",  bit_size,  if nullsafe {"Nullsafe"} else {""});
            let map_type_name = format_ident!("Map{}", bit_size);
            let return_type = if nullsafe {quote!(i64)} else {quote!(Option<i64>)};
            let null_result = if nullsafe {quote!(0i64)} else {quote!(None)};
            let result = quote!(map.len() as i64);
            let result = if nullsafe {quote!(#result)} else {quote!(Some(#result))};
            let maybe_null_cnf = if nullsafe {quote!(_cfg)} else {quote!{cfg}};
            let maybe_null = if nullsafe {quote!()} else {quote!{cfg.set_maybe_null(true);}};
            output.extend(quote! {
                #[derive(Default)]
                struct #struct_name;

                #[register]
                impl BasicUdf for #struct_name {
                    type Returns<'a> = #return_type;

                    fn init(#maybe_null_cnf: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
                        #maybe_null
                        if args.len() != 1 {
                            return Err(format!("Expected one arguments; Got {} arguments.", args.len()));
                        }
                        if !args.get(0).unwrap().value().is_string() {
                            return Err(format!("First argument mast be CLOB (bitmap) or NULL."));
                        }
                        Ok(Self::default())
                    }

                    fn process<'a>(&'a mut self, _cfg: &UdfCfg<Process>, args: &ArgList<Process>, _error: Option<NonZeroU8>) -> Result<Self::Returns<'a>, ProcessError> {
                        let arg = args.get(0).unwrap().value();
                        let bytes = arg.as_bytes();
                        if let Some(bytes) = bytes {
                            if let Ok(map) = <#map_type_name>::deserialize_from(bytes) {
                                Ok(#result)
                            } else {
                                Err(ProcessError)
                            }
                        } else {
                            Ok(#null_result)
                        }
                    }
                }
            });
        }
    }
    output.into()
}

#[proc_macro]
pub fn map_json(_input: TokenStream) -> TokenStream {
    let mut output = quote!();
    for bit_size in vec!["32", "64"] {
        for nullsafe in vec![true, false] {
            let struct_name = format_ident!("Roaring{}{}Json",  bit_size,  if nullsafe {"Nullsafe"} else {""});
            let map_type_name = format_ident!("Map{}", bit_size);
            let return_type = if nullsafe {quote!(&'a [u8])} else {quote!(Option<&'a [u8]>)};
            let null_result = if nullsafe {
                quote!{
                    self.vec = Vec::from("[]");
                    Ok(&self.vec[0..self.vec.len()])
                }
            } else {
                quote!(Ok(None))
            };
            let result = quote!(&self.vec[0..self.vec.len()]);
            let result = if nullsafe {quote!(#result)} else {quote!(Some(#result))};
            let maybe_null_cnf = if nullsafe {quote!(_cfg)} else {quote!{cfg}};
            let maybe_null = if nullsafe {quote!()} else {quote!{cfg.set_maybe_null(true);}};
            output.extend(quote! {
                #[derive(Default)]
                struct #struct_name {
                    vec: Vec<u8>
                }

                #[register]
                impl BasicUdf for #struct_name {
                    type Returns<'a> = #return_type;

                    fn init(#maybe_null_cnf: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
                        #maybe_null
                        if args.len() != 1 {
                            return Err(format!("Expected one arguments; Got {} arguments.", args.len()));
                        }
                        if !args.get(0).unwrap().value().is_string() {
                            return Err(format!("First argument mast be CLOB (bitmap) or NULL."));
                        }
                        Ok(Self::default())
                    }

                    fn process<'a>(&'a mut self, _cfg: &UdfCfg<Process>, args: &ArgList<Process>, _error: Option<NonZeroU8>) -> Result<Self::Returns<'a>, ProcessError> {
                        let arg = args.get(0).unwrap().value();
                        let bytes = arg.as_bytes();
                        if let Some(bytes) = bytes {
                            if let Ok(map) = <#map_type_name>::deserialize_from(bytes) {
                                let mut count = map.len() as usize;
                                let capacity = ((10usize+1)*count)+2; // "[" + (log10(2^32) + ",") * map.len() + "]";
                                let mut vec: Vec<u8> = Vec::with_capacity(capacity);
                                const COMA: &[u8] = b",";
                                vec.extend(b"[");
                                for item in map.iter() {
                                    vec.extend(item.to_string().as_bytes());
                                    count-=1;
                                    if count != 0 {
                                        vec.extend(COMA);
                                    }
                                }
                                vec.extend(b"]");
                                self.vec = vec;
                                Ok(#result)
                            } else {
                                Err(ProcessError)
                            }
                        } else {
                            #null_result
                        }
                    }
                }
            });
        }
    }
    output.into()
}

