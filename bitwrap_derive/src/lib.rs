extern crate proc_macro;

use proc_macro2::{
    TokenStream,
    Ident,
};

use quote::quote;

use syn::{
    self,
    parse_macro_input,
};


struct BitWrapMacro {
    struct_id: Ident,
    pack_list: TokenStream,
    unpack_list: TokenStream,
    bits: usize,
}


impl BitWrapMacro {
    fn new(ident: &Ident) -> Self {
        Self {
            struct_id: ident.clone(),
            pack_list: TokenStream::default(),
            unpack_list: TokenStream::default(),
            bits: 0,
        }
    }

    fn build_field_bits(&mut self, field: &syn::Field, meta_list: &syn::MetaList) {
        let mut iter = meta_list.nested.iter();

        let mut bits = match iter.next() {
            Some(syn::NestedMeta::Lit(syn::Lit::Int(v))) => v.base10_parse::<usize>().unwrap(),
            _ => panic!("bits first argument should be a number"),
        };

        if bits == 0 || bits > 64 {
            panic!("bits should be in range 1 .. 64");
        }

        let field_ty = &field.ty;
        let field_ident = &field.ident;

        let convert_ty = if bits <= 8 {
            "u8"
        } else if bits <= 16 {
            "u16"
        } else if bits <= 32 {
            "u32"
        } else {
            "u64"
        };
        let ty = Ident::new(convert_ty, proc_macro2::Span::call_site());

        let mut convert_from: Option<&Ident> = None;
        let mut convert_to: Option<&Ident> = None;

        for item in iter {
            match item {
                syn::NestedMeta::Meta(syn::Meta::List(arg)) if arg.path.is_ident("convert") => {
                    let mut nested = arg.nested.iter();

                    if let Some(syn::NestedMeta::Meta(syn::Meta::Path(v))) = nested.next() {
                        convert_from = v.get_ident();
                    } else {
                        panic!("bits convert argument #1 should be a function");
                    }

                    if let Some(syn::NestedMeta::Meta(syn::Meta::Path(v))) = nested.next() {
                        convert_to = v.get_ident();
                    } else {
                        panic!("bits convert argument #2 should be a function");
                    }
                }
                _ => panic!("bits has wrong arguments format"),
            }
        }

        if self.bits == 8 {
            let bytes = (bits + 7) / 8;

            self.pack_list.extend(quote! {
                if #bytes + offset > dst.len() {
                    return #bytes + offset;
                }

                dst[offset] = 0;
            });

            self.unpack_list.extend(quote! {
                if #bytes + offset > src.len() {
                    return #bytes + offset;
                }
            });
        }

        if let Some(v) = convert_from {
            self.pack_list.extend(quote! {
                let value = Self::#v ( self.#field_ident ) as #ty;
            });
        } else {
            self.pack_list.extend(quote! {
                let value = self.#field_ident as #ty;
            });
        }

        self.unpack_list.extend(quote! {
            let mut value: #ty = 0;
        });

        while bits > self.bits {
            let shift = bits - self.bits; // value left shift
            let mask = 0xFFu8 >> (8 - self.bits);

            self.pack_list.extend(quote! {
                dst[offset] |= ((value >> #shift) as u8) & #mask;
                offset += 1;
                dst[offset] = 0;
            });

            self.unpack_list.extend(quote! {
                value |= ((src[offset] & #mask) as #ty) << #shift;
                offset += 1;
            });

            bits -= self.bits;
            self.bits = 8;
        }

        self.bits -= bits;

        let shift = self.bits; // byte right shift
        let mask = 0xFFu8 >> (8 - bits);

        if shift == 0 {
            self.pack_list.extend(quote! {
                dst[offset] |= (value as u8) & #mask;
                offset += 1;
            });

            self.unpack_list.extend(quote! {
                value |= (src[offset] & #mask) as #ty;
                offset += 1;
            });

            self.bits = 8;
        } else {
            self.pack_list.extend(quote! {
                dst[offset] |= ((value as u8) & #mask) << #shift;
            });

            self.unpack_list.extend(quote! {
                value |= ((src[offset] >> #shift) & #mask) as #ty;
            });
        }

        if let Some(v) = convert_to {
            self.unpack_list.extend(quote! {
                self.#field_ident = Self::#v ( value ) as #field_ty;
            });
        } else {
            self.unpack_list.extend(quote! {
                self.#field_ident = value as #field_ty;
            });
        }
    }

    fn build_field_bits_skip(&mut self, meta_list: &syn::MetaList) {
        let mut nested = meta_list.nested.iter();

        let mut bits = match nested.next() {
            Some(syn::NestedMeta::Lit(syn::Lit::Int(v))) => v.base10_parse::<usize>().unwrap(),
            _ => panic!("bits_skip value should be a number"),
        };

        let value = match nested.next() {
            Some(syn::NestedMeta::Lit(syn::Lit::Int(v))) => v.base10_parse::<usize>().unwrap(),
            _ => 0usize,
        };

        if self.bits == 8 {
            let bytes = (bits + 7) / 8;

            self.pack_list.extend(quote! {
                if #bytes + offset > dst.len() {
                    return #bytes + offset;
                }

                dst[offset] = 0;
            });

            self.unpack_list.extend(quote! {
                if #bytes + offset > src.len() {
                    return #bytes + offset;
                }
            });
        }

        while bits > self.bits {
            let shift = bits - self.bits; // value left shift
            let mask = 0xFFu8 >> (8 - self.bits);
            let v = ((value >> shift) as u8) & mask;

            self.pack_list.extend(quote! {
                dst[offset] |= #v;
                offset += 1;
                dst[offset] = 0;
            });

            self.unpack_list.extend(quote! {
                offset += 1;
            });

            bits -= self.bits;
            self.bits = 8;
        }

        self.bits -= bits;

        let shift = self.bits; // byte right shift
        let mask = 0xFFu8 >> (8 - bits);
        let v = ((value as u8) & mask) << shift;

        self.pack_list.extend(quote! {
            dst[offset] |= #v;
        });

        if shift == 0 {
            self.pack_list.extend(quote! {
                offset += 1;
            });

            self.unpack_list.extend(quote! {
                offset += 1;
            });

            self.bits = 8;
        }
    }

    fn build_field_bitwrap(&mut self, field: &syn::Field) {
        assert_eq!(self.bits, 8, "bitwrap not aligned");

        let ident = &field.ident;

        self.pack_list.extend(quote! {
            offset += self.#ident.pack(&mut dst[offset ..]);
        });

        self.unpack_list.extend(quote! {
            offset += self.#ident.unpack(&src[offset ..]);
        });
    }

    fn build_field(&mut self, field: &syn::Field) {
        for attr in field.attrs.iter().filter(|v| v.path.segments.len() == 1) {
            match attr.path.segments[0].ident.to_string().as_str() {
                "bits" => {
                    let meta = attr.parse_meta().unwrap();
                    match &meta {
                        syn::Meta::List(v) => self.build_field_bits(field, v),
                        _ => panic!("bits format mismatch"),
                    }
                }
                "bits_skip" => {
                    let meta = attr.parse_meta().unwrap();
                    match &meta {
                        syn::Meta::List(v) => self.build_field_bits_skip(v),
                        _ => panic!("bits_skip format mismatch"),
                    }
                }
                "bitwrap" => {
                    self.build_field_bitwrap(field)
                }
                _ => {}
            };
        }
    }

    fn build(&mut self, data: &syn::DataStruct) -> TokenStream {
        self.bits = 8;

        let fields = match &data.fields {
            syn::Fields::Named(v) => &v.named,
            syn::Fields::Unnamed(_v) => unimplemented!(),
            syn::Fields::Unit => unimplemented!(),
        };

        for field in fields {
            self.build_field(field);
        }

        assert_eq!(self.bits, 8, "bitwrap not aligned");

        let struct_id = &self.struct_id;
        let pack_list = &self.pack_list;
        let unpack_list = &self.unpack_list;

        quote! {
            impl BitWrap for #struct_id {
                fn pack(&self, dst: &mut [u8]) -> usize {
                    let mut offset: usize = 0;
                    #pack_list
                    offset
                }

                fn unpack(&mut self, src: &[u8]) -> usize {
                    let mut offset: usize = 0;
                    #unpack_list
                    offset
                }
            }
        }
    }
}


#[proc_macro_derive(BitWrap, attributes(bits, bits_skip, bitwrap))]
pub fn bitwrap_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    if let syn::Data::Struct(ref s) = input.data {
        let mut bitwrap = BitWrapMacro::new(&input.ident);
        bitwrap.build(s).into()
    } else {
        panic!("struct required")
    }
}
