extern crate proc_macro;

use quote::ToTokens;
use syn::Type;
use {
    proc_macro2::{
        Ident,
        Literal,
        TokenStream,
        TokenTree,
        token_stream::IntoIter,
    },
    quote::quote,
    syn::{
        self,
        parse_macro_input,
    },
};

struct BitWrapMacro {
    struct_id: Ident,
    pack_list: TokenStream,
    len_list: TokenStream,
    unpack_list: TokenStream,
    bits: usize,
}

// convert TokenTree literal to usize
#[inline]
fn literal_to_usize(v: &Literal) -> Option<usize> {
    syn::LitInt::from(v.clone()).base10_parse::<usize>().ok()
}


// push attribute option tokens to TokenStream
fn extend_token_stream(stream: &mut TokenStream, iter: &mut IntoIter) {
    for item in iter {
        match item {
            TokenTree::Punct(v) if v.as_char() == ',' => break,
            v => stream.extend(quote! { #v }),
        }
    }
}

const BASIC_TYPE_LIST: &[&str] = &["u8", "u16", "u32", "u64", "u128", "i8", "i16", "i32", "i64", "i128", "isize", "usize", "f32", "f64"];

fn is_basic_type(t: &Type) -> bool {
    let ty = t.to_token_stream().to_string().replace(" ", "");
    (*BASIC_TYPE_LIST).contains(&ty.as_str())
}

fn bits_type(bits: usize) -> Ident {
    Ident::new(
        if bits <= 8 {
            "u8"
        } else if bits <= 16 {
            "u16"
        } else if bits <= 32 {
            "u32"
        } else if bits <= 64 {
            "u64"
        } else {
            "u128"
        },
        proc_macro2::Span::call_site()
    )
}

impl BitWrapMacro {
    fn new(ident: &Ident) -> Self {
        Self {
            struct_id: ident.clone(),
            pack_list: TokenStream::default(),
            len_list: TokenStream::default(),
            unpack_list: TokenStream::default(),
            bits: 0,
        }
    }

    fn assert_align(&self) {
        assert_eq!(self.bits, 8, "bitwrap not aligned");
    }

    fn macro_make_bits(&mut self, ty: &Ident, bits: usize, pack_le: bool, unpack_le: bool) {
        let mut bits = bits;

        if (pack_le || unpack_le) && (bits < 16 || bits % 8 != 0 || self.bits % 8 != 0){
            panic!("edition le need complete u16/u32/u64/u128 byte")
        }

        self.unpack_list.extend(quote! {
            let mut value: #ty = 0;
        });

        let mut tmp_index = 0 as usize;
        while bits > self.bits {
            let mut shift_pack = bits - self.bits; // value left shift
            let mut shift_unpack = shift_pack; // value left shift
            let mask = 0xFFu8 >> (8 - self.bits);

            if pack_le  {
                shift_pack = tmp_index * 8;
            }

            self.pack_list.extend(quote! {
                dst[offset] |= ((value >> #shift_pack) as u8) & #mask;
                offset += 1;
                dst[offset] = 0;
            });

            if unpack_le  {
                shift_unpack = tmp_index * 8;
            }

            self.unpack_list.extend(quote! {
                value |= ((src[offset] & #mask) as #ty) << #shift_unpack;
                offset += 1;
            });
            bits -= self.bits;
            self.bits = 8;
            tmp_index +=1;
        }

        self.bits -= bits;

        let mut shift_pack = self.bits; // byte right shift
        let mut shift_unpack = shift_pack; // byte right shift
        let mask = 0xFFu8 >> (8 - bits);

        if shift_pack == 0 {
            if pack_le  {
                shift_pack = tmp_index * 8;
                self.pack_list.extend(quote! {
                    dst[offset] |= ((value >> #shift_pack) as u8) & #mask;
                    offset += 1;
                });
            } else {
                self.pack_list.extend(quote! {
                    dst[offset] |= (value as u8) & #mask;
                    offset += 1;
                });
            }

            if unpack_le  {
                shift_unpack = tmp_index * 8;
                self.unpack_list.extend(quote! {
                    value |= ((src[offset] & #mask) as #ty) << #shift_unpack;
                    offset += 1;
                });
            } else {
                self.unpack_list.extend(quote! {
                    value |= (src[offset] & #mask) as #ty;
                    offset += 1;
                });
            }

            self.bits = 8;
        } else {
            self.pack_list.extend(quote! {
                dst[offset] |= ((value as u8) & #mask) << #shift_pack;
            });
            self.unpack_list.extend(quote! {
                value |= ((src[offset] >> #shift_unpack) & #mask) as #ty;
            });
        }
    }

    fn build_bitfield_array(&mut self, field: &syn::Field) {
        self.assert_align();

        let field_ident = &field.ident;

        self.pack_list.extend(quote! {
            if dst.len() >= limit {
                let tmp = self.#field_ident.pack()?;
                let tmp_len = tmp.len();
                dst[offset..offset+tmp_len].clone_from_slice(tmp.as_slice());
                offset += tmp_len;
            } else {
                return Err(bitwrap_extra::BitWrapError);
            }
        });

        self.unpack_list.extend(quote! {
            if src.len() >= limit {
                offset += self.#field_ident.unpack(&src[offset .. limit])?;
            } else {
                return Err(bitwrap_extra::BitWrapError);
            }
        });
    }

    fn build_bitfield_nested(&mut self, field: &syn::Field) {
        let field_ty = &field.ty;
        let field_ident = &field.ident;

        if let syn::Type::Array(_) = field_ty {
            self.len_list.extend(quote! {
                length += self.#field_ident.len() as usize;
            });
            // [u8; N]
            self.pack_list.extend(quote! {
                let next = offset + self.#field_ident.len();
                if dst.len() >= next {
                    dst[offset .. next].clone_from_slice(&self.#field_ident);
                    offset = next;
                } else {
                    return Err(bitwrap_extra::BitWrapError);
                }
            });

            self.unpack_list.extend(quote! {
                let next = offset + self.#field_ident.len();
                if src.len() >= next {
                    self.#field_ident.clone_from_slice(&src[offset .. next]);
                    offset = next;
                } else {
                    return Err(bitwrap_extra::BitWrapError);
                }
            });
        } else {
            self.len_list.extend(quote! {
                length += self.#field_ident.len() as usize;
            });
            // Any object with BitWrap implementation
            self.pack_list.extend(quote! {
                let limit = dst.len();
            });

            self.unpack_list.extend(quote! {
                let limit = src.len();
            });

            self.build_bitfield_array(field);
        }
    }

    fn build_bitfield(&mut self, field: &syn::Field, tokens: &TokenStream) {
        // nested bitfield (attribute without arguments)
        if tokens.is_empty() {
            self.build_bitfield_nested(field);
            return;
        }

        let field_ty = &field.ty;
        let field_ident = &field.ident;

        let tokens = tokens.clone();
        let tree = tokens.into_iter().next().unwrap();
        let group = match tree {
            TokenTree::Group(v) => v.stream(),
            _ => unreachable!(),
        };
        let mut iter = group.into_iter();

        // check first_token
        let first_token = iter.next().unwrap();
        let bits = match first_token {
            TokenTree::Literal(v) => {
                literal_to_usize(&v).unwrap_or(0)
            }
            TokenTree::Ident(v) => {
                // self.len_list.extend(quote! {
                //     length += ( #v ) as usize;
                // });
                self.len_list.extend(quote! {
                    length += self.#field_ident.len() as usize;
                });
                self.pack_list.extend(quote! {
                    let limit = offset + ( #v ) as usize;
                });

                self.unpack_list.extend(quote! {
                    let limit = offset + ( #v ) as usize;
                });

                self.build_bitfield_array(field);
                return;
            }
            _ => panic!("bitfield argument #1 has wrong type")
        };

        if bits == 0 || bits > 128 {
            panic!("bitfield argument #1 should be a number in range 1 ..= 128");
        }

        let mut field_name = TokenStream::new();
        let mut field_value = TokenStream::new();
        let mut pack_le_value = TokenStream::new();
        let mut unpack_le_value = TokenStream::new();

        // check buffer len
        if self.bits == 8 {
            let bytes = (bits + 7) / 8;

            self.pack_list.extend(quote! {
                if #bytes + offset > dst.len() {
                    return Err(bitwrap_extra::BitWrapError);
                }

                dst[offset] = 0;
            });

            self.unpack_list.extend(quote! {
                if #bytes + offset > src.len() {
                    return Err(bitwrap_extra::BitWrapError);
                }
            });
        }

        // get type to store bits
        let mut ty = bits_type(bits);
        let ty1 = field_ty.to_token_stream().to_string().replace(" ", "");
        let basic_type = is_basic_type(field_ty);
        // println!("basic_type {} {} {} {}", basic_type, ty, ty1, field_ident.to_token_stream().to_string());
        if basic_type {
            ty = Ident::new(ty1.as_str(), proc_macro2::Span::call_site());
        }

        // parse attributes
        while let Some(item) = iter.next() {
            match item {
                TokenTree::Punct(v) if v.as_char() == ',' => continue,
                TokenTree::Ident(v) => {
                    // skip '=' token after ident in attribute options
                    match iter.next() {
                        Some(TokenTree::Punct(v)) if v.as_char() == '=' => {}
                        _ => panic!("unexpected token")
                    }

                    match v.to_string().as_str() {
                        "name" => {
                            extend_token_stream(&mut field_name, &mut iter);
                        }
                        "value" => {
                            extend_token_stream(&mut field_value, &mut iter);
                        }
                        "pack" => {
                            extend_token_stream(&mut pack_le_value, &mut iter);
                        }
                        "unpack" => {
                            extend_token_stream(&mut unpack_le_value, &mut iter);
                        }

                        v => panic!("bitfield has unexpected argument: {}", v),
                    }
                }
                _ => panic!("bitfield has wrong format"),
            }
        }

        let pack_le = if !pack_le_value.is_empty() && pack_le_value.to_string().trim().to_uppercase() == "LE" { true } else { false };
        let unpack_le = if !unpack_le_value.is_empty() && unpack_le_value.to_string().trim().to_uppercase() == "LE" { true } else { false };

        if !basic_type {
            // 不是基础类型 单独处理
            self.len_list.extend(quote! {
                // #field_ty
                length += (#bits) as usize;
            });
            self.pack_list.extend(quote! {
                    let limit = offset + ( #bits / 8usize ) as usize;
                });

            self.unpack_list.extend(quote! {
                    let limit = offset + ( #bits / 8usize ) as usize;
                });

            self.build_bitfield_array(field);
            return;
        }

        self.len_list.extend(quote! {
            length += ( #bits ) as usize;
        });

        if !field_name.is_empty() {
            //  name + value

            if field_value.is_empty() {
                panic!("value is required for named filed");
            }

            self.len_list.extend(quote! {
                let #field_name = ( #field_value ) as usize ;
                length += #field_name * 8 as usize;
            });

            self.pack_list.extend(quote! {
                let value = ( #field_value ) as #ty ;
                let #field_name = value ;
            });

            // TODO: skip if name started with _

            self.macro_make_bits(&ty, bits, pack_le, unpack_le);

            self.unpack_list.extend(quote! {
                let #field_name = value ;
            });

        }

        // set default conversion field -> bits
        let field_ident_val = field_ident.clone().unwrap();
        let field_name_val = field_name.to_string();
        //println!("build_bitfield_array_flag {:?} {:?} ", field_ident_val.to_string(), field_name_val);
        if field_name.is_empty() || field_ident_val == field_name_val {
            match field_ty {
                syn::Type::Path(v) if v.path.is_ident("bool") => {
                    self.pack_list.extend(quote! {
                    let value: #ty = if self.#field_ident { 1 } else { 0 } ;
                });
                }
                _ => {
                    self.pack_list.extend(quote! {
                    let value: #ty = #ty::try_from(self.#field_ident)? ;
                });
                }
            }
        }

        if field_name.is_empty() {
            self.macro_make_bits(&ty, bits, pack_le, unpack_le);
        }

        // set default conversion bits -> field
        if field_name.is_empty() || field_ident_val == field_name_val {
            match field_ty {
                syn::Type::Path(v) if v.path.is_ident("bool") => {
                    self.unpack_list.extend(quote! {
                    self.#field_ident = value != 0 ;
                });
                }
                _ => {
                    self.unpack_list.extend(quote! {
                    self.#field_ident = #field_ty::try_from(value)? ;
                });
                }
            }
        }

    }

    fn build_field(&mut self, field: &syn::Field) {
        let bf = Ident::new("bitfield", proc_macro2::Span::call_site());
        for attr in field.attrs.iter().filter(|v| v.path.segments.len() == 1) {
            if attr.path.segments[0].ident == bf {
                self.build_bitfield(field, &attr.tokens);
            }
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

        self.assert_align();

        let struct_id = &self.struct_id;
        let pack_list = &self.pack_list;
        let len_list = &self.len_list;
        let unpack_list = &self.unpack_list;

        quote! {
            impl bitwrap_extra::BitWrapExt for #struct_id {

                fn len(&self) -> usize {
                    let mut length: usize = 0;
                    #len_list
                    (length / 8) as usize
                }

                fn pack(&self) -> Result<Vec<u8>, bitwrap_extra::BitWrapError> {
                    use core::convert::TryFrom as _;
                    let len = self.len() as usize;
                    let mut dst = vec![0 as u8; len];
                    let mut offset: usize = 0;
                    #pack_list
                    Ok(dst)
                }

                fn unpack(&mut self, src: &[u8]) -> Result<usize, bitwrap_extra::BitWrapError> {
                    use core::convert::TryFrom as _;
                    let mut offset: usize = 0;
                    #unpack_list
                    Ok(offset)
                }

            }
        }
    }
}


#[proc_macro_derive(BitWrap, attributes(bitfield))]
pub fn bitwrap_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    if let syn::Data::Struct(ref s) = input.data {
        let mut bitwrap = BitWrapMacro::new(&input.ident);
        bitwrap.build(s).into()
    } else {
        panic!("struct required")
    }
}
