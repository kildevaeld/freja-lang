#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;
#[macro_use]
extern crate serde;
pub mod ast {
    use std::fmt;
    #[structural_match]
    pub struct Location(pub usize, pub usize);
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_Location: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Location {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_tuple_struct(
                    __serializer,
                    "Location",
                    0 + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeTupleStruct::serialize_field(
                    &mut __serde_state,
                    &self.0,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeTupleStruct::serialize_field(
                    &mut __serde_state,
                    &self.1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeTupleStruct::end(__serde_state)
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Location {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Location(ref __self_0_0, ref __self_0_1) => {
                    let mut debug_trait_builder = f.debug_tuple("Location");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    let _ = debug_trait_builder.field(&&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for Location {
        #[inline]
        fn clone(&self) -> Location {
            match *self {
                Location(ref __self_0_0, ref __self_0_1) => Location(
                    ::std::clone::Clone::clone(&(*__self_0_0)),
                    ::std::clone::Clone::clone(&(*__self_0_1)),
                ),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for Location {
        #[inline]
        fn eq(&self, other: &Location) -> bool {
            match *other {
                Location(ref __self_1_0, ref __self_1_1) => match *self {
                    Location(ref __self_0_0, ref __self_0_1) => {
                        (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Location) -> bool {
            match *other {
                Location(ref __self_1_0, ref __self_1_1) => match *self {
                    Location(ref __self_0_0, ref __self_0_1) => {
                        (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1)
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for Location {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<usize>;
                let _: ::std::cmp::AssertParamIsEq<usize>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::hash::Hash for Location {
        fn hash<__H: ::std::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                Location(ref __self_0_0, ref __self_0_1) => {
                    ::std::hash::Hash::hash(&(*__self_0_0), state);
                    ::std::hash::Hash::hash(&(*__self_0_1), state)
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_Location: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Location {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<Location>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Location;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "tuple struct Location")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<usize>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"tuple struct Location with 2 elements",
                                    ));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<usize>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        1usize,
                                        &"tuple struct Location with 2 elements",
                                    ));
                                }
                            };
                        _serde::export::Ok(Location(__field0, __field1))
                    }
                }
                _serde::Deserializer::deserialize_tuple_struct(
                    __deserializer,
                    "Location",
                    2usize,
                    __Visitor {
                        marker: _serde::export::PhantomData::<Location>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[structural_match]
    pub enum TokenType {
        This,
        Identifier,
        OpAdditive,
        OpMultiplicative,
        ShiftOperator,
        EqualityOperator,
        RelationalOperator,
        BitwiseOrOperator,
        BitwiseAndOperator,
        LogicalOrOperator,
        LogicalAndOperator,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_TokenType: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for TokenType {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    TokenType::This => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "TokenType",
                        0u32,
                        "This",
                    ),
                    TokenType::Identifier => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "TokenType",
                        1u32,
                        "Identifier",
                    ),
                    TokenType::OpAdditive => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "TokenType",
                        2u32,
                        "OpAdditive",
                    ),
                    TokenType::OpMultiplicative => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "TokenType",
                        3u32,
                        "OpMultiplicative",
                    ),
                    TokenType::ShiftOperator => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "TokenType",
                        4u32,
                        "ShiftOperator",
                    ),
                    TokenType::EqualityOperator => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "TokenType",
                        5u32,
                        "EqualityOperator",
                    ),
                    TokenType::RelationalOperator => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "TokenType",
                        6u32,
                        "RelationalOperator",
                    ),
                    TokenType::BitwiseOrOperator => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "TokenType",
                        7u32,
                        "BitwiseOrOperator",
                    ),
                    TokenType::BitwiseAndOperator => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "TokenType",
                        8u32,
                        "BitwiseAndOperator",
                    ),
                    TokenType::LogicalOrOperator => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "TokenType",
                        9u32,
                        "LogicalOrOperator",
                    ),
                    TokenType::LogicalAndOperator => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "TokenType",
                        10u32,
                        "LogicalAndOperator",
                    ),
                }
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for TokenType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&TokenType::This,) => {
                    let mut debug_trait_builder = f.debug_tuple("This");
                    debug_trait_builder.finish()
                }
                (&TokenType::Identifier,) => {
                    let mut debug_trait_builder = f.debug_tuple("Identifier");
                    debug_trait_builder.finish()
                }
                (&TokenType::OpAdditive,) => {
                    let mut debug_trait_builder = f.debug_tuple("OpAdditive");
                    debug_trait_builder.finish()
                }
                (&TokenType::OpMultiplicative,) => {
                    let mut debug_trait_builder = f.debug_tuple("OpMultiplicative");
                    debug_trait_builder.finish()
                }
                (&TokenType::ShiftOperator,) => {
                    let mut debug_trait_builder = f.debug_tuple("ShiftOperator");
                    debug_trait_builder.finish()
                }
                (&TokenType::EqualityOperator,) => {
                    let mut debug_trait_builder = f.debug_tuple("EqualityOperator");
                    debug_trait_builder.finish()
                }
                (&TokenType::RelationalOperator,) => {
                    let mut debug_trait_builder = f.debug_tuple("RelationalOperator");
                    debug_trait_builder.finish()
                }
                (&TokenType::BitwiseOrOperator,) => {
                    let mut debug_trait_builder = f.debug_tuple("BitwiseOrOperator");
                    debug_trait_builder.finish()
                }
                (&TokenType::BitwiseAndOperator,) => {
                    let mut debug_trait_builder = f.debug_tuple("BitwiseAndOperator");
                    debug_trait_builder.finish()
                }
                (&TokenType::LogicalOrOperator,) => {
                    let mut debug_trait_builder = f.debug_tuple("LogicalOrOperator");
                    debug_trait_builder.finish()
                }
                (&TokenType::LogicalAndOperator,) => {
                    let mut debug_trait_builder = f.debug_tuple("LogicalAndOperator");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for TokenType {
        #[inline]
        fn clone(&self) -> TokenType {
            match (&*self,) {
                (&TokenType::This,) => TokenType::This,
                (&TokenType::Identifier,) => TokenType::Identifier,
                (&TokenType::OpAdditive,) => TokenType::OpAdditive,
                (&TokenType::OpMultiplicative,) => TokenType::OpMultiplicative,
                (&TokenType::ShiftOperator,) => TokenType::ShiftOperator,
                (&TokenType::EqualityOperator,) => TokenType::EqualityOperator,
                (&TokenType::RelationalOperator,) => TokenType::RelationalOperator,
                (&TokenType::BitwiseOrOperator,) => TokenType::BitwiseOrOperator,
                (&TokenType::BitwiseAndOperator,) => TokenType::BitwiseAndOperator,
                (&TokenType::LogicalOrOperator,) => TokenType::LogicalOrOperator,
                (&TokenType::LogicalAndOperator,) => TokenType::LogicalAndOperator,
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for TokenType {
        #[inline]
        fn eq(&self, other: &TokenType) -> bool {
            {
                let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for TokenType {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::hash::Hash for TokenType {
        fn hash<__H: ::std::hash::Hasher>(&self, state: &mut __H) -> () {
            match (&*self,) {
                _ => ::std::hash::Hash::hash(
                    &unsafe { ::std::intrinsics::discriminant_value(self) },
                    state,
                ),
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_TokenType: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for TokenType {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __field8,
                    __field9,
                    __field10,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "variant identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            3u64 => _serde::export::Ok(__Field::__field3),
                            4u64 => _serde::export::Ok(__Field::__field4),
                            5u64 => _serde::export::Ok(__Field::__field5),
                            6u64 => _serde::export::Ok(__Field::__field6),
                            7u64 => _serde::export::Ok(__Field::__field7),
                            8u64 => _serde::export::Ok(__Field::__field8),
                            9u64 => _serde::export::Ok(__Field::__field9),
                            10u64 => _serde::export::Ok(__Field::__field10),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"variant index 0 <= i < 11",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "This" => _serde::export::Ok(__Field::__field0),
                            "Identifier" => _serde::export::Ok(__Field::__field1),
                            "OpAdditive" => _serde::export::Ok(__Field::__field2),
                            "OpMultiplicative" => _serde::export::Ok(__Field::__field3),
                            "ShiftOperator" => _serde::export::Ok(__Field::__field4),
                            "EqualityOperator" => _serde::export::Ok(__Field::__field5),
                            "RelationalOperator" => _serde::export::Ok(__Field::__field6),
                            "BitwiseOrOperator" => _serde::export::Ok(__Field::__field7),
                            "BitwiseAndOperator" => _serde::export::Ok(__Field::__field8),
                            "LogicalOrOperator" => _serde::export::Ok(__Field::__field9),
                            "LogicalAndOperator" => _serde::export::Ok(__Field::__field10),
                            _ => _serde::export::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            )),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"This" => _serde::export::Ok(__Field::__field0),
                            b"Identifier" => _serde::export::Ok(__Field::__field1),
                            b"OpAdditive" => _serde::export::Ok(__Field::__field2),
                            b"OpMultiplicative" => _serde::export::Ok(__Field::__field3),
                            b"ShiftOperator" => _serde::export::Ok(__Field::__field4),
                            b"EqualityOperator" => _serde::export::Ok(__Field::__field5),
                            b"RelationalOperator" => _serde::export::Ok(__Field::__field6),
                            b"BitwiseOrOperator" => _serde::export::Ok(__Field::__field7),
                            b"BitwiseAndOperator" => _serde::export::Ok(__Field::__field8),
                            b"LogicalOrOperator" => _serde::export::Ok(__Field::__field9),
                            b"LogicalAndOperator" => _serde::export::Ok(__Field::__field10),
                            _ => {
                                let __value = &_serde::export::from_utf8_lossy(__value);
                                _serde::export::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<TokenType>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = TokenType;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "enum TokenType")
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match match _serde::de::EnumAccess::variant(__data) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            (__Field::__field0, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                _serde::export::Ok(TokenType::This)
                            }
                            (__Field::__field1, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                _serde::export::Ok(TokenType::Identifier)
                            }
                            (__Field::__field2, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                _serde::export::Ok(TokenType::OpAdditive)
                            }
                            (__Field::__field3, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                _serde::export::Ok(TokenType::OpMultiplicative)
                            }
                            (__Field::__field4, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                _serde::export::Ok(TokenType::ShiftOperator)
                            }
                            (__Field::__field5, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                _serde::export::Ok(TokenType::EqualityOperator)
                            }
                            (__Field::__field6, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                _serde::export::Ok(TokenType::RelationalOperator)
                            }
                            (__Field::__field7, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                _serde::export::Ok(TokenType::BitwiseOrOperator)
                            }
                            (__Field::__field8, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                _serde::export::Ok(TokenType::BitwiseAndOperator)
                            }
                            (__Field::__field9, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                _serde::export::Ok(TokenType::LogicalOrOperator)
                            }
                            (__Field::__field10, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                _serde::export::Ok(TokenType::LogicalAndOperator)
                            }
                        }
                    }
                }
                const VARIANTS: &'static [&'static str] = &[
                    "This",
                    "Identifier",
                    "OpAdditive",
                    "OpMultiplicative",
                    "ShiftOperator",
                    "EqualityOperator",
                    "RelationalOperator",
                    "BitwiseOrOperator",
                    "BitwiseAndOperator",
                    "LogicalOrOperator",
                    "LogicalAndOperator",
                ];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "TokenType",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<TokenType>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[serde(tag = "type")]
    #[structural_match]
    pub struct Token {
        pub kind: TokenType,
        pub location: Location,
        pub value: String,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_Token: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Token {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Token",
                    true as usize + 1 + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "type",
                    "Token",
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "kind",
                    &self.kind,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "location",
                    &self.location,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "value",
                    &self.value,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Token {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Token {
                    kind: ref __self_0_0,
                    location: ref __self_0_1,
                    value: ref __self_0_2,
                } => {
                    let mut debug_trait_builder = f.debug_struct("Token");
                    let _ = debug_trait_builder.field("kind", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("location", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("value", &&(*__self_0_2));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for Token {
        #[inline]
        fn eq(&self, other: &Token) -> bool {
            match *other {
                Token {
                    kind: ref __self_1_0,
                    location: ref __self_1_1,
                    value: ref __self_1_2,
                } => match *self {
                    Token {
                        kind: ref __self_0_0,
                        location: ref __self_0_1,
                        value: ref __self_0_2,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Token) -> bool {
            match *other {
                Token {
                    kind: ref __self_1_0,
                    location: ref __self_1_1,
                    value: ref __self_1_2,
                } => match *self {
                    Token {
                        kind: ref __self_0_0,
                        location: ref __self_0_1,
                        value: ref __self_0_2,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for Token {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<TokenType>;
                let _: ::std::cmp::AssertParamIsEq<Location>;
                let _: ::std::cmp::AssertParamIsEq<String>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::hash::Hash for Token {
        fn hash<__H: ::std::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                Token {
                    kind: ref __self_0_0,
                    location: ref __self_0_1,
                    value: ref __self_0_2,
                } => {
                    ::std::hash::Hash::hash(&(*__self_0_0), state);
                    ::std::hash::Hash::hash(&(*__self_0_1), state);
                    ::std::hash::Hash::hash(&(*__self_0_2), state)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for Token {
        #[inline]
        fn clone(&self) -> Token {
            match *self {
                Token {
                    kind: ref __self_0_0,
                    location: ref __self_0_1,
                    value: ref __self_0_2,
                } => Token {
                    kind: ::std::clone::Clone::clone(&(*__self_0_0)),
                    location: ::std::clone::Clone::clone(&(*__self_0_1)),
                    value: ::std::clone::Clone::clone(&(*__self_0_2)),
                },
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_Token: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Token {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 3",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "kind" => _serde::export::Ok(__Field::__field0),
                            "location" => _serde::export::Ok(__Field::__field1),
                            "value" => _serde::export::Ok(__Field::__field2),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"kind" => _serde::export::Ok(__Field::__field0),
                            b"location" => _serde::export::Ok(__Field::__field1),
                            b"value" => _serde::export::Ok(__Field::__field2),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<Token>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Token;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct Token")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<TokenType>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct Token with 3 elements",
                                ));
                            }
                        };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<Location>(&mut __seq)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Token with 3 elements",
                                    ));
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Token with 3 elements",
                                    ));
                                }
                            };
                        _serde::export::Ok(Token {
                            kind: __field0,
                            location: __field1,
                            value: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<TokenType> = _serde::export::None;
                        let mut __field1: _serde::export::Option<Location> = _serde::export::None;
                        let mut __field2: _serde::export::Option<String> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "kind",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<TokenType>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "location",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Location>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "value",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("kind") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("location") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::export::Some(__field2) => __field2,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("value") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(Token {
                            kind: __field0,
                            location: __field1,
                            value: __field2,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["kind", "location", "value"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Token",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<Token>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    impl Token {
        pub fn new(location: Location, kind: TokenType, value: &str) -> Token {
            Token {
                location,
                value: value.to_owned(),
                kind,
            }
        }
    }
    impl fmt::Display for Token {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_fmt(::std::fmt::Arguments::new_v1(
                &[""],
                &match (&self.value,) {
                    (arg0,) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)],
                },
            ))
        }
    }
    pub trait StmtVisitor<R> {
        fn visit_program_stmt(&mut self, e: &ProgramStmt) -> R;
        fn visit_var_stmt(&mut self, e: &VarStmt) -> R;
        fn visit_varlist_stmt(&mut self, e: &VarListStmt) -> R;
        fn visit_expr_stmt(&mut self, e: &ExprStmt) -> R;
        fn visit_func_stmt(&mut self, e: &FuncStmt) -> R;
        fn visit_class_stmt(&mut self, e: &ClassStmt) -> R;
        fn visit_block_stmt(&mut self, e: &BlockStmt) -> R;
        fn visit_if_stmt(&mut self, e: &IfStmt) -> R;
        fn visit_for_stmt(&mut self, e: &ForStmt) -> R;
        fn visit_return_stmt(&mut self, e: &ReturnStmt) -> R;
        fn visit_continue_stmt(&mut self, e: &ContinueStmt) -> R;
        fn visit_break_stmt(&mut self, e: &BreakStmt) -> R;
    }
    pub trait ExprVisitor<R> {
        fn visit_assign_expr(&mut self, e: &AssignExpr) -> R;
        fn visit_call_expr(&mut self, e: &CallExpr) -> R;
        fn visit_literal_expr(&mut self, e: &LiteralExpr) -> R;
        fn visit_binary_expr(&mut self, e: &BinaryExpr) -> R;
        fn visit_member_expr(&mut self, e: &MemberExpr) -> R;
        fn visit_lookup_expr(&mut self, e: &LookupExpr) -> R;
        fn visit_arguments_expr(&mut self, e: &ArgumentsExpr) -> R;
        fn visit_logical_expr(&mut self, e: &LogicalExpr) -> R;
    }
    #[serde(tag = "type", content = "value")]
    pub enum Operator {
        Add,
        Sub,
        Mul,
        Div,
        BitwiseXor,
        BitwiseAnd,
        BitwiseOr,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_Operator: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Operator {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    Operator::Add => {
                        let mut __struct =
                            match _serde::Serializer::serialize_struct(__serializer, "Operator", 1)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "type",
                            "Add",
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    Operator::Sub => {
                        let mut __struct =
                            match _serde::Serializer::serialize_struct(__serializer, "Operator", 1)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "type",
                            "Sub",
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    Operator::Mul => {
                        let mut __struct =
                            match _serde::Serializer::serialize_struct(__serializer, "Operator", 1)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "type",
                            "Mul",
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    Operator::Div => {
                        let mut __struct =
                            match _serde::Serializer::serialize_struct(__serializer, "Operator", 1)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "type",
                            "Div",
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    Operator::BitwiseXor => {
                        let mut __struct =
                            match _serde::Serializer::serialize_struct(__serializer, "Operator", 1)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "type",
                            "BitwiseXor",
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    Operator::BitwiseAnd => {
                        let mut __struct =
                            match _serde::Serializer::serialize_struct(__serializer, "Operator", 1)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "type",
                            "BitwiseAnd",
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    Operator::BitwiseOr => {
                        let mut __struct =
                            match _serde::Serializer::serialize_struct(__serializer, "Operator", 1)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "type",
                            "BitwiseOr",
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_Operator: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Operator {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "variant identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            3u64 => _serde::export::Ok(__Field::__field3),
                            4u64 => _serde::export::Ok(__Field::__field4),
                            5u64 => _serde::export::Ok(__Field::__field5),
                            6u64 => _serde::export::Ok(__Field::__field6),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"variant index 0 <= i < 7",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "Add" => _serde::export::Ok(__Field::__field0),
                            "Sub" => _serde::export::Ok(__Field::__field1),
                            "Mul" => _serde::export::Ok(__Field::__field2),
                            "Div" => _serde::export::Ok(__Field::__field3),
                            "BitwiseXor" => _serde::export::Ok(__Field::__field4),
                            "BitwiseAnd" => _serde::export::Ok(__Field::__field5),
                            "BitwiseOr" => _serde::export::Ok(__Field::__field6),
                            _ => _serde::export::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            )),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"Add" => _serde::export::Ok(__Field::__field0),
                            b"Sub" => _serde::export::Ok(__Field::__field1),
                            b"Mul" => _serde::export::Ok(__Field::__field2),
                            b"Div" => _serde::export::Ok(__Field::__field3),
                            b"BitwiseXor" => _serde::export::Ok(__Field::__field4),
                            b"BitwiseAnd" => _serde::export::Ok(__Field::__field5),
                            b"BitwiseOr" => _serde::export::Ok(__Field::__field6),
                            _ => {
                                let __value = &_serde::export::from_utf8_lossy(__value);
                                _serde::export::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                const VARIANTS: &'static [&'static str] = &[
                    "Add",
                    "Sub",
                    "Mul",
                    "Div",
                    "BitwiseXor",
                    "BitwiseAnd",
                    "BitwiseOr",
                ];
                struct __Seed<'de> {
                    field: __Field,
                    marker: _serde::export::PhantomData<Operator>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::DeserializeSeed<'de> for __Seed<'de> {
                    type Value = Operator;
                    fn deserialize<__D>(
                        self,
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self::Value, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        match self.field {
                            __Field::__field0 => match _serde::Deserializer::deserialize_any(
                                __deserializer,
                                _serde::private::de::UntaggedUnitVisitor::new("Operator", "Add"),
                            ) {
                                _serde::export::Ok(()) => _serde::export::Ok(Operator::Add),
                                _serde::export::Err(__err) => _serde::export::Err(__err),
                            },
                            __Field::__field1 => match _serde::Deserializer::deserialize_any(
                                __deserializer,
                                _serde::private::de::UntaggedUnitVisitor::new("Operator", "Sub"),
                            ) {
                                _serde::export::Ok(()) => _serde::export::Ok(Operator::Sub),
                                _serde::export::Err(__err) => _serde::export::Err(__err),
                            },
                            __Field::__field2 => match _serde::Deserializer::deserialize_any(
                                __deserializer,
                                _serde::private::de::UntaggedUnitVisitor::new("Operator", "Mul"),
                            ) {
                                _serde::export::Ok(()) => _serde::export::Ok(Operator::Mul),
                                _serde::export::Err(__err) => _serde::export::Err(__err),
                            },
                            __Field::__field3 => match _serde::Deserializer::deserialize_any(
                                __deserializer,
                                _serde::private::de::UntaggedUnitVisitor::new("Operator", "Div"),
                            ) {
                                _serde::export::Ok(()) => _serde::export::Ok(Operator::Div),
                                _serde::export::Err(__err) => _serde::export::Err(__err),
                            },
                            __Field::__field4 => match _serde::Deserializer::deserialize_any(
                                __deserializer,
                                _serde::private::de::UntaggedUnitVisitor::new(
                                    "Operator",
                                    "BitwiseXor",
                                ),
                            ) {
                                _serde::export::Ok(()) => _serde::export::Ok(Operator::BitwiseXor),
                                _serde::export::Err(__err) => _serde::export::Err(__err),
                            },
                            __Field::__field5 => match _serde::Deserializer::deserialize_any(
                                __deserializer,
                                _serde::private::de::UntaggedUnitVisitor::new(
                                    "Operator",
                                    "BitwiseAnd",
                                ),
                            ) {
                                _serde::export::Ok(()) => _serde::export::Ok(Operator::BitwiseAnd),
                                _serde::export::Err(__err) => _serde::export::Err(__err),
                            },
                            __Field::__field6 => match _serde::Deserializer::deserialize_any(
                                __deserializer,
                                _serde::private::de::UntaggedUnitVisitor::new(
                                    "Operator",
                                    "BitwiseOr",
                                ),
                            ) {
                                _serde::export::Ok(()) => _serde::export::Ok(Operator::BitwiseOr),
                                _serde::export::Err(__err) => _serde::export::Err(__err),
                            },
                        }
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<Operator>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Operator;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(
                            __formatter,
                            "adjacently tagged enum Operator",
                        )
                    }
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        match {
                            let mut __rk: _serde::export::Option<
                                _serde::private::de::TagOrContentField,
                            > = _serde::export::None;
                            while let _serde::export::Some(__k) =
                                match _serde::de::MapAccess::next_key_seed(
                                    &mut __map,
                                    _serde::private::de::TagContentOtherFieldVisitor {
                                        tag: "type",
                                        content: "value",
                                    },
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            {
                                match __k {
                                    _serde::private::de::TagContentOtherField::Other => {
                                        match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        };
                                        continue;
                                    }
                                    _serde::private::de::TagContentOtherField::Tag => {
                                        __rk = _serde::export::Some(
                                            _serde::private::de::TagOrContentField::Tag,
                                        );
                                        break;
                                    }
                                    _serde::private::de::TagContentOtherField::Content => {
                                        __rk = _serde::export::Some(
                                            _serde::private::de::TagOrContentField::Content,
                                        );
                                        break;
                                    }
                                }
                            }
                            __rk
                        } {
                            _serde::export::Some(_serde::private::de::TagOrContentField::Tag) => {
                                let __field = match _serde::de::MapAccess::next_value(&mut __map) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                match {
                                    let mut __rk: _serde::export::Option<
                                        _serde::private::de::TagOrContentField,
                                    > = _serde::export::None;
                                    while let _serde::export::Some(__k) =
                                        match _serde::de::MapAccess::next_key_seed(
                                            &mut __map,
                                            _serde::private::de::TagContentOtherFieldVisitor {
                                                tag: "type",
                                                content: "value",
                                            },
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        }
                                    {
                                        match __k {
                                            _serde::private::de::TagContentOtherField::Other => {
                                                match _serde::de::MapAccess::next_value::<
                                                    _serde::de::IgnoredAny,
                                                >(
                                                    &mut __map
                                                ) {
                                                    _serde::export::Ok(__val) => __val,
                                                    _serde::export::Err(__err) => {
                                                        return _serde::export::Err(__err);
                                                    }
                                                };
                                                continue;
                                            }
                                            _serde::private::de::TagContentOtherField::Tag => {
                                                __rk = _serde::export::Some(
                                                    _serde::private::de::TagOrContentField::Tag,
                                                );
                                                break;
                                            }
                                            _serde::private::de::TagContentOtherField::Content => {
                                                __rk = _serde::export::Some(
                                                    _serde::private::de::TagOrContentField::Content,
                                                );
                                                break;
                                            }
                                        }
                                    }
                                    __rk
                                } {
                                    _serde::export::Some(
                                        _serde::private::de::TagOrContentField::Tag,
                                    ) => _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                    ),
                                    _serde::export::Some(
                                        _serde::private::de::TagOrContentField::Content,
                                    ) => {
                                        let __ret = match _serde::de::MapAccess::next_value_seed(
                                            &mut __map,
                                            __Seed {
                                                field: __field,
                                                marker: _serde::export::PhantomData,
                                                lifetime: _serde::export::PhantomData,
                                            },
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        };
                                        match {
                                            let mut __rk: _serde::export::Option<
                                                _serde::private::de::TagOrContentField,
                                            > = _serde::export::None;
                                            while let _serde :: export :: Some ( __k ) = match _serde :: de :: MapAccess :: next_key_seed ( & mut __map , _serde :: private :: de :: TagContentOtherFieldVisitor { tag : "type" , content : "value" , } ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } { match __k { _serde :: private :: de :: TagContentOtherField :: Other => { match _serde :: de :: MapAccess :: next_value :: < _serde :: de :: IgnoredAny > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ; continue ; } _serde :: private :: de :: TagContentOtherField :: Tag => { __rk = _serde :: export :: Some ( _serde :: private :: de :: TagOrContentField :: Tag ) ; break ; } _serde :: private :: de :: TagContentOtherField :: Content => { __rk = _serde :: export :: Some ( _serde :: private :: de :: TagOrContentField :: Content ) ; break ; } } }
                                            __rk
                                        } {
                                            _serde::export::Some(
                                                _serde::private::de::TagOrContentField::Tag,
                                            ) => _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "type",
                                                ),
                                            ),
                                            _serde::export::Some(
                                                _serde::private::de::TagOrContentField::Content,
                                            ) => _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "value",
                                                ),
                                            ),
                                            _serde::export::None => _serde::export::Ok(__ret),
                                        }
                                    }
                                    _serde::export::None => match __field {
                                        __Field::__field0 => _serde::export::Ok(Operator::Add),
                                        __Field::__field1 => _serde::export::Ok(Operator::Sub),
                                        __Field::__field2 => _serde::export::Ok(Operator::Mul),
                                        __Field::__field3 => _serde::export::Ok(Operator::Div),
                                        __Field::__field4 => {
                                            _serde::export::Ok(Operator::BitwiseXor)
                                        }
                                        __Field::__field5 => {
                                            _serde::export::Ok(Operator::BitwiseAnd)
                                        }
                                        __Field::__field6 => {
                                            _serde::export::Ok(Operator::BitwiseOr)
                                        }
                                    },
                                }
                            }
                            _serde::export::Some(
                                _serde::private::de::TagOrContentField::Content,
                            ) => {
                                let __content = match _serde::de::MapAccess::next_value::<
                                    _serde::private::de::Content,
                                >(&mut __map)
                                {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                match {
                                    let mut __rk: _serde::export::Option<
                                        _serde::private::de::TagOrContentField,
                                    > = _serde::export::None;
                                    while let _serde::export::Some(__k) =
                                        match _serde::de::MapAccess::next_key_seed(
                                            &mut __map,
                                            _serde::private::de::TagContentOtherFieldVisitor {
                                                tag: "type",
                                                content: "value",
                                            },
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        }
                                    {
                                        match __k {
                                            _serde::private::de::TagContentOtherField::Other => {
                                                match _serde::de::MapAccess::next_value::<
                                                    _serde::de::IgnoredAny,
                                                >(
                                                    &mut __map
                                                ) {
                                                    _serde::export::Ok(__val) => __val,
                                                    _serde::export::Err(__err) => {
                                                        return _serde::export::Err(__err);
                                                    }
                                                };
                                                continue;
                                            }
                                            _serde::private::de::TagContentOtherField::Tag => {
                                                __rk = _serde::export::Some(
                                                    _serde::private::de::TagOrContentField::Tag,
                                                );
                                                break;
                                            }
                                            _serde::private::de::TagContentOtherField::Content => {
                                                __rk = _serde::export::Some(
                                                    _serde::private::de::TagOrContentField::Content,
                                                );
                                                break;
                                            }
                                        }
                                    }
                                    __rk
                                } {
                                    _serde::export::Some(
                                        _serde::private::de::TagOrContentField::Tag,
                                    ) => {
                                        let __deserializer = _serde :: private :: de :: ContentDeserializer :: < __A :: Error > :: new ( __content ) ;
                                        let __ret = match match match _serde :: de :: MapAccess :: next_value ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } { __Field :: __field0 => match _serde :: Deserializer :: deserialize_any ( __deserializer , _serde :: private :: de :: UntaggedUnitVisitor :: new ( "Operator" , "Add" ) ) { _serde :: export :: Ok ( ( ) ) => _serde :: export :: Ok ( Operator :: Add ) , _serde :: export :: Err ( __err ) => _serde :: export :: Err ( __err ) , } , __Field :: __field1 => match _serde :: Deserializer :: deserialize_any ( __deserializer , _serde :: private :: de :: UntaggedUnitVisitor :: new ( "Operator" , "Sub" ) ) { _serde :: export :: Ok ( ( ) ) => _serde :: export :: Ok ( Operator :: Sub ) , _serde :: export :: Err ( __err ) => _serde :: export :: Err ( __err ) , } , __Field :: __field2 => match _serde :: Deserializer :: deserialize_any ( __deserializer , _serde :: private :: de :: UntaggedUnitVisitor :: new ( "Operator" , "Mul" ) ) { _serde :: export :: Ok ( ( ) ) => _serde :: export :: Ok ( Operator :: Mul ) , _serde :: export :: Err ( __err ) => _serde :: export :: Err ( __err ) , } , __Field :: __field3 => match _serde :: Deserializer :: deserialize_any ( __deserializer , _serde :: private :: de :: UntaggedUnitVisitor :: new ( "Operator" , "Div" ) ) { _serde :: export :: Ok ( ( ) ) => _serde :: export :: Ok ( Operator :: Div ) , _serde :: export :: Err ( __err ) => _serde :: export :: Err ( __err ) , } , __Field :: __field4 => match _serde :: Deserializer :: deserialize_any ( __deserializer , _serde :: private :: de :: UntaggedUnitVisitor :: new ( "Operator" , "BitwiseXor" ) ) { _serde :: export :: Ok ( ( ) ) => _serde :: export :: Ok ( Operator :: BitwiseXor ) , _serde :: export :: Err ( __err ) => _serde :: export :: Err ( __err ) , } , __Field :: __field5 => match _serde :: Deserializer :: deserialize_any ( __deserializer , _serde :: private :: de :: UntaggedUnitVisitor :: new ( "Operator" , "BitwiseAnd" ) ) { _serde :: export :: Ok ( ( ) ) => _serde :: export :: Ok ( Operator :: BitwiseAnd ) , _serde :: export :: Err ( __err ) => _serde :: export :: Err ( __err ) , } , __Field :: __field6 => match _serde :: Deserializer :: deserialize_any ( __deserializer , _serde :: private :: de :: UntaggedUnitVisitor :: new ( "Operator" , "BitwiseOr" ) ) { _serde :: export :: Ok ( ( ) ) => _serde :: export :: Ok ( Operator :: BitwiseOr ) , _serde :: export :: Err ( __err ) => _serde :: export :: Err ( __err ) , } , } { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ;
                                        match {
                                            let mut __rk: _serde::export::Option<
                                                _serde::private::de::TagOrContentField,
                                            > = _serde::export::None;
                                            while let _serde :: export :: Some ( __k ) = match _serde :: de :: MapAccess :: next_key_seed ( & mut __map , _serde :: private :: de :: TagContentOtherFieldVisitor { tag : "type" , content : "value" , } ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } { match __k { _serde :: private :: de :: TagContentOtherField :: Other => { match _serde :: de :: MapAccess :: next_value :: < _serde :: de :: IgnoredAny > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ; continue ; } _serde :: private :: de :: TagContentOtherField :: Tag => { __rk = _serde :: export :: Some ( _serde :: private :: de :: TagOrContentField :: Tag ) ; break ; } _serde :: private :: de :: TagContentOtherField :: Content => { __rk = _serde :: export :: Some ( _serde :: private :: de :: TagOrContentField :: Content ) ; break ; } } }
                                            __rk
                                        } {
                                            _serde::export::Some(
                                                _serde::private::de::TagOrContentField::Tag,
                                            ) => _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "type",
                                                ),
                                            ),
                                            _serde::export::Some(
                                                _serde::private::de::TagOrContentField::Content,
                                            ) => _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "value",
                                                ),
                                            ),
                                            _serde::export::None => _serde::export::Ok(__ret),
                                        }
                                    }
                                    _serde::export::Some(
                                        _serde::private::de::TagOrContentField::Content,
                                    ) => _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("value"),
                                    ),
                                    _serde::export::None => _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::missing_field("type"),
                                    ),
                                }
                            }
                            _serde::export::None => _serde::export::Err(
                                <__A::Error as _serde::de::Error>::missing_field("type"),
                            ),
                        }
                    }
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        match match _serde::de::SeqAccess::next_element(&mut __seq) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__field) => {
                                match match _serde::de::SeqAccess::next_element_seed(
                                    &mut __seq,
                                    __Seed {
                                        field: __field,
                                        marker: _serde::export::PhantomData,
                                        lifetime: _serde::export::PhantomData,
                                    },
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                } {
                                    _serde::export::Some(__ret) => _serde::export::Ok(__ret),
                                    _serde::export::None => _serde::export::Err(
                                        _serde::de::Error::invalid_length(1, &self),
                                    ),
                                }
                            }
                            _serde::export::None => {
                                _serde::export::Err(_serde::de::Error::invalid_length(0, &self))
                            }
                        }
                    }
                }
                const FIELDS: &'static [&'static str] = &["type", "value"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Operator",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<Operator>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Operator {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&Operator::Add,) => {
                    let mut debug_trait_builder = f.debug_tuple("Add");
                    debug_trait_builder.finish()
                }
                (&Operator::Sub,) => {
                    let mut debug_trait_builder = f.debug_tuple("Sub");
                    debug_trait_builder.finish()
                }
                (&Operator::Mul,) => {
                    let mut debug_trait_builder = f.debug_tuple("Mul");
                    debug_trait_builder.finish()
                }
                (&Operator::Div,) => {
                    let mut debug_trait_builder = f.debug_tuple("Div");
                    debug_trait_builder.finish()
                }
                (&Operator::BitwiseXor,) => {
                    let mut debug_trait_builder = f.debug_tuple("BitwiseXor");
                    debug_trait_builder.finish()
                }
                (&Operator::BitwiseAnd,) => {
                    let mut debug_trait_builder = f.debug_tuple("BitwiseAnd");
                    debug_trait_builder.finish()
                }
                (&Operator::BitwiseOr,) => {
                    let mut debug_trait_builder = f.debug_tuple("BitwiseOr");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for Operator {
        #[inline]
        fn clone(&self) -> Operator {
            match (&*self,) {
                (&Operator::Add,) => Operator::Add,
                (&Operator::Sub,) => Operator::Sub,
                (&Operator::Mul,) => Operator::Mul,
                (&Operator::Div,) => Operator::Div,
                (&Operator::BitwiseXor,) => Operator::BitwiseXor,
                (&Operator::BitwiseAnd,) => Operator::BitwiseAnd,
                (&Operator::BitwiseOr,) => Operator::BitwiseOr,
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for Operator {
        #[inline]
        fn eq(&self, other: &Operator) -> bool {
            {
                let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
    }
    #[serde(tag = "type", content = "value")]
    pub enum Number {
        Double(f64),
        Integer(i64),
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_Number: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Number {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    Number::Double(ref __field0) => {
                        let mut __struct =
                            match _serde::Serializer::serialize_struct(__serializer, "Number", 2) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "type",
                            "Double",
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "value",
                            __field0,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    Number::Integer(ref __field0) => {
                        let mut __struct =
                            match _serde::Serializer::serialize_struct(__serializer, "Number", 2) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "type",
                            "Integer",
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "value",
                            __field0,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_Number: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Number {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "variant identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"variant index 0 <= i < 2",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "Double" => _serde::export::Ok(__Field::__field0),
                            "Integer" => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            )),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"Double" => _serde::export::Ok(__Field::__field0),
                            b"Integer" => _serde::export::Ok(__Field::__field1),
                            _ => {
                                let __value = &_serde::export::from_utf8_lossy(__value);
                                _serde::export::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                const VARIANTS: &'static [&'static str] = &["Double", "Integer"];
                struct __Seed<'de> {
                    field: __Field,
                    marker: _serde::export::PhantomData<Number>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::DeserializeSeed<'de> for __Seed<'de> {
                    type Value = Number;
                    fn deserialize<__D>(
                        self,
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self::Value, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        match self.field {
                            __Field::__field0 => _serde::export::Result::map(
                                <f64 as _serde::Deserialize>::deserialize(__deserializer),
                                Number::Double,
                            ),
                            __Field::__field1 => _serde::export::Result::map(
                                <i64 as _serde::Deserialize>::deserialize(__deserializer),
                                Number::Integer,
                            ),
                        }
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<Number>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Number;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(
                            __formatter,
                            "adjacently tagged enum Number",
                        )
                    }
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        match {
                            let mut __rk: _serde::export::Option<
                                _serde::private::de::TagOrContentField,
                            > = _serde::export::None;
                            while let _serde::export::Some(__k) =
                                match _serde::de::MapAccess::next_key_seed(
                                    &mut __map,
                                    _serde::private::de::TagContentOtherFieldVisitor {
                                        tag: "type",
                                        content: "value",
                                    },
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            {
                                match __k {
                                    _serde::private::de::TagContentOtherField::Other => {
                                        match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        };
                                        continue;
                                    }
                                    _serde::private::de::TagContentOtherField::Tag => {
                                        __rk = _serde::export::Some(
                                            _serde::private::de::TagOrContentField::Tag,
                                        );
                                        break;
                                    }
                                    _serde::private::de::TagContentOtherField::Content => {
                                        __rk = _serde::export::Some(
                                            _serde::private::de::TagOrContentField::Content,
                                        );
                                        break;
                                    }
                                }
                            }
                            __rk
                        } {
                            _serde::export::Some(_serde::private::de::TagOrContentField::Tag) => {
                                let __field = match _serde::de::MapAccess::next_value(&mut __map) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                match {
                                    let mut __rk: _serde::export::Option<
                                        _serde::private::de::TagOrContentField,
                                    > = _serde::export::None;
                                    while let _serde::export::Some(__k) =
                                        match _serde::de::MapAccess::next_key_seed(
                                            &mut __map,
                                            _serde::private::de::TagContentOtherFieldVisitor {
                                                tag: "type",
                                                content: "value",
                                            },
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        }
                                    {
                                        match __k {
                                            _serde::private::de::TagContentOtherField::Other => {
                                                match _serde::de::MapAccess::next_value::<
                                                    _serde::de::IgnoredAny,
                                                >(
                                                    &mut __map
                                                ) {
                                                    _serde::export::Ok(__val) => __val,
                                                    _serde::export::Err(__err) => {
                                                        return _serde::export::Err(__err);
                                                    }
                                                };
                                                continue;
                                            }
                                            _serde::private::de::TagContentOtherField::Tag => {
                                                __rk = _serde::export::Some(
                                                    _serde::private::de::TagOrContentField::Tag,
                                                );
                                                break;
                                            }
                                            _serde::private::de::TagContentOtherField::Content => {
                                                __rk = _serde::export::Some(
                                                    _serde::private::de::TagOrContentField::Content,
                                                );
                                                break;
                                            }
                                        }
                                    }
                                    __rk
                                } {
                                    _serde::export::Some(
                                        _serde::private::de::TagOrContentField::Tag,
                                    ) => _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                    ),
                                    _serde::export::Some(
                                        _serde::private::de::TagOrContentField::Content,
                                    ) => {
                                        let __ret = match _serde::de::MapAccess::next_value_seed(
                                            &mut __map,
                                            __Seed {
                                                field: __field,
                                                marker: _serde::export::PhantomData,
                                                lifetime: _serde::export::PhantomData,
                                            },
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        };
                                        match {
                                            let mut __rk: _serde::export::Option<
                                                _serde::private::de::TagOrContentField,
                                            > = _serde::export::None;
                                            while let _serde :: export :: Some ( __k ) = match _serde :: de :: MapAccess :: next_key_seed ( & mut __map , _serde :: private :: de :: TagContentOtherFieldVisitor { tag : "type" , content : "value" , } ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } { match __k { _serde :: private :: de :: TagContentOtherField :: Other => { match _serde :: de :: MapAccess :: next_value :: < _serde :: de :: IgnoredAny > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ; continue ; } _serde :: private :: de :: TagContentOtherField :: Tag => { __rk = _serde :: export :: Some ( _serde :: private :: de :: TagOrContentField :: Tag ) ; break ; } _serde :: private :: de :: TagContentOtherField :: Content => { __rk = _serde :: export :: Some ( _serde :: private :: de :: TagOrContentField :: Content ) ; break ; } } }
                                            __rk
                                        } {
                                            _serde::export::Some(
                                                _serde::private::de::TagOrContentField::Tag,
                                            ) => _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "type",
                                                ),
                                            ),
                                            _serde::export::Some(
                                                _serde::private::de::TagOrContentField::Content,
                                            ) => _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "value",
                                                ),
                                            ),
                                            _serde::export::None => _serde::export::Ok(__ret),
                                        }
                                    }
                                    _serde::export::None => _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::missing_field("value"),
                                    ),
                                }
                            }
                            _serde::export::Some(
                                _serde::private::de::TagOrContentField::Content,
                            ) => {
                                let __content = match _serde::de::MapAccess::next_value::<
                                    _serde::private::de::Content,
                                >(&mut __map)
                                {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                match {
                                    let mut __rk: _serde::export::Option<
                                        _serde::private::de::TagOrContentField,
                                    > = _serde::export::None;
                                    while let _serde::export::Some(__k) =
                                        match _serde::de::MapAccess::next_key_seed(
                                            &mut __map,
                                            _serde::private::de::TagContentOtherFieldVisitor {
                                                tag: "type",
                                                content: "value",
                                            },
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        }
                                    {
                                        match __k {
                                            _serde::private::de::TagContentOtherField::Other => {
                                                match _serde::de::MapAccess::next_value::<
                                                    _serde::de::IgnoredAny,
                                                >(
                                                    &mut __map
                                                ) {
                                                    _serde::export::Ok(__val) => __val,
                                                    _serde::export::Err(__err) => {
                                                        return _serde::export::Err(__err);
                                                    }
                                                };
                                                continue;
                                            }
                                            _serde::private::de::TagContentOtherField::Tag => {
                                                __rk = _serde::export::Some(
                                                    _serde::private::de::TagOrContentField::Tag,
                                                );
                                                break;
                                            }
                                            _serde::private::de::TagContentOtherField::Content => {
                                                __rk = _serde::export::Some(
                                                    _serde::private::de::TagOrContentField::Content,
                                                );
                                                break;
                                            }
                                        }
                                    }
                                    __rk
                                } {
                                    _serde::export::Some(
                                        _serde::private::de::TagOrContentField::Tag,
                                    ) => {
                                        let __deserializer = _serde :: private :: de :: ContentDeserializer :: < __A :: Error > :: new ( __content ) ;
                                        let __ret =
                                            match match match _serde::de::MapAccess::next_value(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            } {
                                                __Field::__field0 => _serde::export::Result::map(
                                                    <f64 as _serde::Deserialize>::deserialize(
                                                        __deserializer,
                                                    ),
                                                    Number::Double,
                                                ),
                                                __Field::__field1 => _serde::export::Result::map(
                                                    <i64 as _serde::Deserialize>::deserialize(
                                                        __deserializer,
                                                    ),
                                                    Number::Integer,
                                                ),
                                            } {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            };
                                        match {
                                            let mut __rk: _serde::export::Option<
                                                _serde::private::de::TagOrContentField,
                                            > = _serde::export::None;
                                            while let _serde :: export :: Some ( __k ) = match _serde :: de :: MapAccess :: next_key_seed ( & mut __map , _serde :: private :: de :: TagContentOtherFieldVisitor { tag : "type" , content : "value" , } ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } { match __k { _serde :: private :: de :: TagContentOtherField :: Other => { match _serde :: de :: MapAccess :: next_value :: < _serde :: de :: IgnoredAny > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ; continue ; } _serde :: private :: de :: TagContentOtherField :: Tag => { __rk = _serde :: export :: Some ( _serde :: private :: de :: TagOrContentField :: Tag ) ; break ; } _serde :: private :: de :: TagContentOtherField :: Content => { __rk = _serde :: export :: Some ( _serde :: private :: de :: TagOrContentField :: Content ) ; break ; } } }
                                            __rk
                                        } {
                                            _serde::export::Some(
                                                _serde::private::de::TagOrContentField::Tag,
                                            ) => _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "type",
                                                ),
                                            ),
                                            _serde::export::Some(
                                                _serde::private::de::TagOrContentField::Content,
                                            ) => _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "value",
                                                ),
                                            ),
                                            _serde::export::None => _serde::export::Ok(__ret),
                                        }
                                    }
                                    _serde::export::Some(
                                        _serde::private::de::TagOrContentField::Content,
                                    ) => _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("value"),
                                    ),
                                    _serde::export::None => _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::missing_field("type"),
                                    ),
                                }
                            }
                            _serde::export::None => _serde::export::Err(
                                <__A::Error as _serde::de::Error>::missing_field("type"),
                            ),
                        }
                    }
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        match match _serde::de::SeqAccess::next_element(&mut __seq) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__field) => {
                                match match _serde::de::SeqAccess::next_element_seed(
                                    &mut __seq,
                                    __Seed {
                                        field: __field,
                                        marker: _serde::export::PhantomData,
                                        lifetime: _serde::export::PhantomData,
                                    },
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                } {
                                    _serde::export::Some(__ret) => _serde::export::Ok(__ret),
                                    _serde::export::None => _serde::export::Err(
                                        _serde::de::Error::invalid_length(1, &self),
                                    ),
                                }
                            }
                            _serde::export::None => {
                                _serde::export::Err(_serde::de::Error::invalid_length(0, &self))
                            }
                        }
                    }
                }
                const FIELDS: &'static [&'static str] = &["type", "value"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Number",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<Number>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Number {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&Number::Double(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Double");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Number::Integer(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Integer");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for Number {
        #[inline]
        fn clone(&self) -> Number {
            match (&*self,) {
                (&Number::Double(ref __self_0),) => {
                    Number::Double(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Number::Integer(ref __self_0),) => {
                    Number::Integer(::std::clone::Clone::clone(&(*__self_0)))
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for Number {
        #[inline]
        fn eq(&self, other: &Number) -> bool {
            {
                let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&Number::Double(ref __self_0), &Number::Double(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Number::Integer(ref __self_0), &Number::Integer(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        _ => unsafe { ::std::intrinsics::unreachable() },
                    }
                } else {
                    false
                }
            }
        }
        #[inline]
        fn ne(&self, other: &Number) -> bool {
            {
                let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&Number::Double(ref __self_0), &Number::Double(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Number::Integer(ref __self_0), &Number::Integer(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        _ => unsafe { ::std::intrinsics::unreachable() },
                    }
                } else {
                    true
                }
            }
        }
    }
    #[serde(tag = "type", content = "value")]
    pub enum Literal {
        String(String),
        Number(Number),
        Boolean(bool),
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_Literal: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Literal {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    Literal::String(ref __field0) => {
                        let mut __struct = match _serde::Serializer::serialize_struct(
                            __serializer,
                            "Literal",
                            2,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "type",
                            "String",
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "value",
                            __field0,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    Literal::Number(ref __field0) => {
                        let mut __struct = match _serde::Serializer::serialize_struct(
                            __serializer,
                            "Literal",
                            2,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "type",
                            "Number",
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "value",
                            __field0,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    Literal::Boolean(ref __field0) => {
                        let mut __struct = match _serde::Serializer::serialize_struct(
                            __serializer,
                            "Literal",
                            2,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "type",
                            "Boolean",
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "value",
                            __field0,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_Literal: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Literal {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "variant identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"variant index 0 <= i < 3",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "String" => _serde::export::Ok(__Field::__field0),
                            "Number" => _serde::export::Ok(__Field::__field1),
                            "Boolean" => _serde::export::Ok(__Field::__field2),
                            _ => _serde::export::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            )),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"String" => _serde::export::Ok(__Field::__field0),
                            b"Number" => _serde::export::Ok(__Field::__field1),
                            b"Boolean" => _serde::export::Ok(__Field::__field2),
                            _ => {
                                let __value = &_serde::export::from_utf8_lossy(__value);
                                _serde::export::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                const VARIANTS: &'static [&'static str] = &["String", "Number", "Boolean"];
                struct __Seed<'de> {
                    field: __Field,
                    marker: _serde::export::PhantomData<Literal>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::DeserializeSeed<'de> for __Seed<'de> {
                    type Value = Literal;
                    fn deserialize<__D>(
                        self,
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self::Value, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        match self.field {
                            __Field::__field0 => _serde::export::Result::map(
                                <String as _serde::Deserialize>::deserialize(__deserializer),
                                Literal::String,
                            ),
                            __Field::__field1 => _serde::export::Result::map(
                                <Number as _serde::Deserialize>::deserialize(__deserializer),
                                Literal::Number,
                            ),
                            __Field::__field2 => _serde::export::Result::map(
                                <bool as _serde::Deserialize>::deserialize(__deserializer),
                                Literal::Boolean,
                            ),
                        }
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<Literal>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Literal;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(
                            __formatter,
                            "adjacently tagged enum Literal",
                        )
                    }
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        match {
                            let mut __rk: _serde::export::Option<
                                _serde::private::de::TagOrContentField,
                            > = _serde::export::None;
                            while let _serde::export::Some(__k) =
                                match _serde::de::MapAccess::next_key_seed(
                                    &mut __map,
                                    _serde::private::de::TagContentOtherFieldVisitor {
                                        tag: "type",
                                        content: "value",
                                    },
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            {
                                match __k {
                                    _serde::private::de::TagContentOtherField::Other => {
                                        match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        };
                                        continue;
                                    }
                                    _serde::private::de::TagContentOtherField::Tag => {
                                        __rk = _serde::export::Some(
                                            _serde::private::de::TagOrContentField::Tag,
                                        );
                                        break;
                                    }
                                    _serde::private::de::TagContentOtherField::Content => {
                                        __rk = _serde::export::Some(
                                            _serde::private::de::TagOrContentField::Content,
                                        );
                                        break;
                                    }
                                }
                            }
                            __rk
                        } {
                            _serde::export::Some(_serde::private::de::TagOrContentField::Tag) => {
                                let __field = match _serde::de::MapAccess::next_value(&mut __map) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                match {
                                    let mut __rk: _serde::export::Option<
                                        _serde::private::de::TagOrContentField,
                                    > = _serde::export::None;
                                    while let _serde::export::Some(__k) =
                                        match _serde::de::MapAccess::next_key_seed(
                                            &mut __map,
                                            _serde::private::de::TagContentOtherFieldVisitor {
                                                tag: "type",
                                                content: "value",
                                            },
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        }
                                    {
                                        match __k {
                                            _serde::private::de::TagContentOtherField::Other => {
                                                match _serde::de::MapAccess::next_value::<
                                                    _serde::de::IgnoredAny,
                                                >(
                                                    &mut __map
                                                ) {
                                                    _serde::export::Ok(__val) => __val,
                                                    _serde::export::Err(__err) => {
                                                        return _serde::export::Err(__err);
                                                    }
                                                };
                                                continue;
                                            }
                                            _serde::private::de::TagContentOtherField::Tag => {
                                                __rk = _serde::export::Some(
                                                    _serde::private::de::TagOrContentField::Tag,
                                                );
                                                break;
                                            }
                                            _serde::private::de::TagContentOtherField::Content => {
                                                __rk = _serde::export::Some(
                                                    _serde::private::de::TagOrContentField::Content,
                                                );
                                                break;
                                            }
                                        }
                                    }
                                    __rk
                                } {
                                    _serde::export::Some(
                                        _serde::private::de::TagOrContentField::Tag,
                                    ) => _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                    ),
                                    _serde::export::Some(
                                        _serde::private::de::TagOrContentField::Content,
                                    ) => {
                                        let __ret = match _serde::de::MapAccess::next_value_seed(
                                            &mut __map,
                                            __Seed {
                                                field: __field,
                                                marker: _serde::export::PhantomData,
                                                lifetime: _serde::export::PhantomData,
                                            },
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        };
                                        match {
                                            let mut __rk: _serde::export::Option<
                                                _serde::private::de::TagOrContentField,
                                            > = _serde::export::None;
                                            while let _serde :: export :: Some ( __k ) = match _serde :: de :: MapAccess :: next_key_seed ( & mut __map , _serde :: private :: de :: TagContentOtherFieldVisitor { tag : "type" , content : "value" , } ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } { match __k { _serde :: private :: de :: TagContentOtherField :: Other => { match _serde :: de :: MapAccess :: next_value :: < _serde :: de :: IgnoredAny > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ; continue ; } _serde :: private :: de :: TagContentOtherField :: Tag => { __rk = _serde :: export :: Some ( _serde :: private :: de :: TagOrContentField :: Tag ) ; break ; } _serde :: private :: de :: TagContentOtherField :: Content => { __rk = _serde :: export :: Some ( _serde :: private :: de :: TagOrContentField :: Content ) ; break ; } } }
                                            __rk
                                        } {
                                            _serde::export::Some(
                                                _serde::private::de::TagOrContentField::Tag,
                                            ) => _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "type",
                                                ),
                                            ),
                                            _serde::export::Some(
                                                _serde::private::de::TagOrContentField::Content,
                                            ) => _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "value",
                                                ),
                                            ),
                                            _serde::export::None => _serde::export::Ok(__ret),
                                        }
                                    }
                                    _serde::export::None => _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::missing_field("value"),
                                    ),
                                }
                            }
                            _serde::export::Some(
                                _serde::private::de::TagOrContentField::Content,
                            ) => {
                                let __content = match _serde::de::MapAccess::next_value::<
                                    _serde::private::de::Content,
                                >(&mut __map)
                                {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                match {
                                    let mut __rk: _serde::export::Option<
                                        _serde::private::de::TagOrContentField,
                                    > = _serde::export::None;
                                    while let _serde::export::Some(__k) =
                                        match _serde::de::MapAccess::next_key_seed(
                                            &mut __map,
                                            _serde::private::de::TagContentOtherFieldVisitor {
                                                tag: "type",
                                                content: "value",
                                            },
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        }
                                    {
                                        match __k {
                                            _serde::private::de::TagContentOtherField::Other => {
                                                match _serde::de::MapAccess::next_value::<
                                                    _serde::de::IgnoredAny,
                                                >(
                                                    &mut __map
                                                ) {
                                                    _serde::export::Ok(__val) => __val,
                                                    _serde::export::Err(__err) => {
                                                        return _serde::export::Err(__err);
                                                    }
                                                };
                                                continue;
                                            }
                                            _serde::private::de::TagContentOtherField::Tag => {
                                                __rk = _serde::export::Some(
                                                    _serde::private::de::TagOrContentField::Tag,
                                                );
                                                break;
                                            }
                                            _serde::private::de::TagContentOtherField::Content => {
                                                __rk = _serde::export::Some(
                                                    _serde::private::de::TagOrContentField::Content,
                                                );
                                                break;
                                            }
                                        }
                                    }
                                    __rk
                                } {
                                    _serde::export::Some(
                                        _serde::private::de::TagOrContentField::Tag,
                                    ) => {
                                        let __deserializer = _serde :: private :: de :: ContentDeserializer :: < __A :: Error > :: new ( __content ) ;
                                        let __ret =
                                            match match match _serde::de::MapAccess::next_value(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            } {
                                                __Field::__field0 => _serde::export::Result::map(
                                                    <String as _serde::Deserialize>::deserialize(
                                                        __deserializer,
                                                    ),
                                                    Literal::String,
                                                ),
                                                __Field::__field1 => _serde::export::Result::map(
                                                    <Number as _serde::Deserialize>::deserialize(
                                                        __deserializer,
                                                    ),
                                                    Literal::Number,
                                                ),
                                                __Field::__field2 => _serde::export::Result::map(
                                                    <bool as _serde::Deserialize>::deserialize(
                                                        __deserializer,
                                                    ),
                                                    Literal::Boolean,
                                                ),
                                            } {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            };
                                        match {
                                            let mut __rk: _serde::export::Option<
                                                _serde::private::de::TagOrContentField,
                                            > = _serde::export::None;
                                            while let _serde :: export :: Some ( __k ) = match _serde :: de :: MapAccess :: next_key_seed ( & mut __map , _serde :: private :: de :: TagContentOtherFieldVisitor { tag : "type" , content : "value" , } ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } { match __k { _serde :: private :: de :: TagContentOtherField :: Other => { match _serde :: de :: MapAccess :: next_value :: < _serde :: de :: IgnoredAny > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ; continue ; } _serde :: private :: de :: TagContentOtherField :: Tag => { __rk = _serde :: export :: Some ( _serde :: private :: de :: TagOrContentField :: Tag ) ; break ; } _serde :: private :: de :: TagContentOtherField :: Content => { __rk = _serde :: export :: Some ( _serde :: private :: de :: TagOrContentField :: Content ) ; break ; } } }
                                            __rk
                                        } {
                                            _serde::export::Some(
                                                _serde::private::de::TagOrContentField::Tag,
                                            ) => _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "type",
                                                ),
                                            ),
                                            _serde::export::Some(
                                                _serde::private::de::TagOrContentField::Content,
                                            ) => _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "value",
                                                ),
                                            ),
                                            _serde::export::None => _serde::export::Ok(__ret),
                                        }
                                    }
                                    _serde::export::Some(
                                        _serde::private::de::TagOrContentField::Content,
                                    ) => _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("value"),
                                    ),
                                    _serde::export::None => _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::missing_field("type"),
                                    ),
                                }
                            }
                            _serde::export::None => _serde::export::Err(
                                <__A::Error as _serde::de::Error>::missing_field("type"),
                            ),
                        }
                    }
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        match match _serde::de::SeqAccess::next_element(&mut __seq) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__field) => {
                                match match _serde::de::SeqAccess::next_element_seed(
                                    &mut __seq,
                                    __Seed {
                                        field: __field,
                                        marker: _serde::export::PhantomData,
                                        lifetime: _serde::export::PhantomData,
                                    },
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                } {
                                    _serde::export::Some(__ret) => _serde::export::Ok(__ret),
                                    _serde::export::None => _serde::export::Err(
                                        _serde::de::Error::invalid_length(1, &self),
                                    ),
                                }
                            }
                            _serde::export::None => {
                                _serde::export::Err(_serde::de::Error::invalid_length(0, &self))
                            }
                        }
                    }
                }
                const FIELDS: &'static [&'static str] = &["type", "value"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Literal",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<Literal>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Literal {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&Literal::String(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("String");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Literal::Number(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Number");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Literal::Boolean(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Boolean");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for Literal {
        #[inline]
        fn clone(&self) -> Literal {
            match (&*self,) {
                (&Literal::String(ref __self_0),) => {
                    Literal::String(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Literal::Number(ref __self_0),) => {
                    Literal::Number(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Literal::Boolean(ref __self_0),) => {
                    Literal::Boolean(::std::clone::Clone::clone(&(*__self_0)))
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for Literal {
        #[inline]
        fn eq(&self, other: &Literal) -> bool {
            {
                let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&Literal::String(ref __self_0), &Literal::String(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Literal::Number(ref __self_0), &Literal::Number(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Literal::Boolean(ref __self_0), &Literal::Boolean(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        _ => unsafe { ::std::intrinsics::unreachable() },
                    }
                } else {
                    false
                }
            }
        }
        #[inline]
        fn ne(&self, other: &Literal) -> bool {
            {
                let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&Literal::String(ref __self_0), &Literal::String(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Literal::Number(ref __self_0), &Literal::Number(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Literal::Boolean(ref __self_0), &Literal::Boolean(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        _ => unsafe { ::std::intrinsics::unreachable() },
                    }
                } else {
                    true
                }
            }
        }
    }
    #[serde(tag = "type", content = "value")]
    pub enum Argument {
        Regular(String),
        Rest(String),
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_Argument: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Argument {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    Argument::Regular(ref __field0) => {
                        let mut __struct =
                            match _serde::Serializer::serialize_struct(__serializer, "Argument", 2)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "type",
                            "Regular",
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "value",
                            __field0,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    Argument::Rest(ref __field0) => {
                        let mut __struct =
                            match _serde::Serializer::serialize_struct(__serializer, "Argument", 2)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "type",
                            "Rest",
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "value",
                            __field0,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_Argument: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Argument {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "variant identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"variant index 0 <= i < 2",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "Regular" => _serde::export::Ok(__Field::__field0),
                            "Rest" => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            )),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"Regular" => _serde::export::Ok(__Field::__field0),
                            b"Rest" => _serde::export::Ok(__Field::__field1),
                            _ => {
                                let __value = &_serde::export::from_utf8_lossy(__value);
                                _serde::export::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                const VARIANTS: &'static [&'static str] = &["Regular", "Rest"];
                struct __Seed<'de> {
                    field: __Field,
                    marker: _serde::export::PhantomData<Argument>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::DeserializeSeed<'de> for __Seed<'de> {
                    type Value = Argument;
                    fn deserialize<__D>(
                        self,
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self::Value, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        match self.field {
                            __Field::__field0 => _serde::export::Result::map(
                                <String as _serde::Deserialize>::deserialize(__deserializer),
                                Argument::Regular,
                            ),
                            __Field::__field1 => _serde::export::Result::map(
                                <String as _serde::Deserialize>::deserialize(__deserializer),
                                Argument::Rest,
                            ),
                        }
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<Argument>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Argument;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(
                            __formatter,
                            "adjacently tagged enum Argument",
                        )
                    }
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        match {
                            let mut __rk: _serde::export::Option<
                                _serde::private::de::TagOrContentField,
                            > = _serde::export::None;
                            while let _serde::export::Some(__k) =
                                match _serde::de::MapAccess::next_key_seed(
                                    &mut __map,
                                    _serde::private::de::TagContentOtherFieldVisitor {
                                        tag: "type",
                                        content: "value",
                                    },
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            {
                                match __k {
                                    _serde::private::de::TagContentOtherField::Other => {
                                        match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        };
                                        continue;
                                    }
                                    _serde::private::de::TagContentOtherField::Tag => {
                                        __rk = _serde::export::Some(
                                            _serde::private::de::TagOrContentField::Tag,
                                        );
                                        break;
                                    }
                                    _serde::private::de::TagContentOtherField::Content => {
                                        __rk = _serde::export::Some(
                                            _serde::private::de::TagOrContentField::Content,
                                        );
                                        break;
                                    }
                                }
                            }
                            __rk
                        } {
                            _serde::export::Some(_serde::private::de::TagOrContentField::Tag) => {
                                let __field = match _serde::de::MapAccess::next_value(&mut __map) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                match {
                                    let mut __rk: _serde::export::Option<
                                        _serde::private::de::TagOrContentField,
                                    > = _serde::export::None;
                                    while let _serde::export::Some(__k) =
                                        match _serde::de::MapAccess::next_key_seed(
                                            &mut __map,
                                            _serde::private::de::TagContentOtherFieldVisitor {
                                                tag: "type",
                                                content: "value",
                                            },
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        }
                                    {
                                        match __k {
                                            _serde::private::de::TagContentOtherField::Other => {
                                                match _serde::de::MapAccess::next_value::<
                                                    _serde::de::IgnoredAny,
                                                >(
                                                    &mut __map
                                                ) {
                                                    _serde::export::Ok(__val) => __val,
                                                    _serde::export::Err(__err) => {
                                                        return _serde::export::Err(__err);
                                                    }
                                                };
                                                continue;
                                            }
                                            _serde::private::de::TagContentOtherField::Tag => {
                                                __rk = _serde::export::Some(
                                                    _serde::private::de::TagOrContentField::Tag,
                                                );
                                                break;
                                            }
                                            _serde::private::de::TagContentOtherField::Content => {
                                                __rk = _serde::export::Some(
                                                    _serde::private::de::TagOrContentField::Content,
                                                );
                                                break;
                                            }
                                        }
                                    }
                                    __rk
                                } {
                                    _serde::export::Some(
                                        _serde::private::de::TagOrContentField::Tag,
                                    ) => _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                    ),
                                    _serde::export::Some(
                                        _serde::private::de::TagOrContentField::Content,
                                    ) => {
                                        let __ret = match _serde::de::MapAccess::next_value_seed(
                                            &mut __map,
                                            __Seed {
                                                field: __field,
                                                marker: _serde::export::PhantomData,
                                                lifetime: _serde::export::PhantomData,
                                            },
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        };
                                        match {
                                            let mut __rk: _serde::export::Option<
                                                _serde::private::de::TagOrContentField,
                                            > = _serde::export::None;
                                            while let _serde :: export :: Some ( __k ) = match _serde :: de :: MapAccess :: next_key_seed ( & mut __map , _serde :: private :: de :: TagContentOtherFieldVisitor { tag : "type" , content : "value" , } ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } { match __k { _serde :: private :: de :: TagContentOtherField :: Other => { match _serde :: de :: MapAccess :: next_value :: < _serde :: de :: IgnoredAny > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ; continue ; } _serde :: private :: de :: TagContentOtherField :: Tag => { __rk = _serde :: export :: Some ( _serde :: private :: de :: TagOrContentField :: Tag ) ; break ; } _serde :: private :: de :: TagContentOtherField :: Content => { __rk = _serde :: export :: Some ( _serde :: private :: de :: TagOrContentField :: Content ) ; break ; } } }
                                            __rk
                                        } {
                                            _serde::export::Some(
                                                _serde::private::de::TagOrContentField::Tag,
                                            ) => _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "type",
                                                ),
                                            ),
                                            _serde::export::Some(
                                                _serde::private::de::TagOrContentField::Content,
                                            ) => _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "value",
                                                ),
                                            ),
                                            _serde::export::None => _serde::export::Ok(__ret),
                                        }
                                    }
                                    _serde::export::None => _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::missing_field("value"),
                                    ),
                                }
                            }
                            _serde::export::Some(
                                _serde::private::de::TagOrContentField::Content,
                            ) => {
                                let __content = match _serde::de::MapAccess::next_value::<
                                    _serde::private::de::Content,
                                >(&mut __map)
                                {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                match {
                                    let mut __rk: _serde::export::Option<
                                        _serde::private::de::TagOrContentField,
                                    > = _serde::export::None;
                                    while let _serde::export::Some(__k) =
                                        match _serde::de::MapAccess::next_key_seed(
                                            &mut __map,
                                            _serde::private::de::TagContentOtherFieldVisitor {
                                                tag: "type",
                                                content: "value",
                                            },
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        }
                                    {
                                        match __k {
                                            _serde::private::de::TagContentOtherField::Other => {
                                                match _serde::de::MapAccess::next_value::<
                                                    _serde::de::IgnoredAny,
                                                >(
                                                    &mut __map
                                                ) {
                                                    _serde::export::Ok(__val) => __val,
                                                    _serde::export::Err(__err) => {
                                                        return _serde::export::Err(__err);
                                                    }
                                                };
                                                continue;
                                            }
                                            _serde::private::de::TagContentOtherField::Tag => {
                                                __rk = _serde::export::Some(
                                                    _serde::private::de::TagOrContentField::Tag,
                                                );
                                                break;
                                            }
                                            _serde::private::de::TagContentOtherField::Content => {
                                                __rk = _serde::export::Some(
                                                    _serde::private::de::TagOrContentField::Content,
                                                );
                                                break;
                                            }
                                        }
                                    }
                                    __rk
                                } {
                                    _serde::export::Some(
                                        _serde::private::de::TagOrContentField::Tag,
                                    ) => {
                                        let __deserializer = _serde :: private :: de :: ContentDeserializer :: < __A :: Error > :: new ( __content ) ;
                                        let __ret =
                                            match match match _serde::de::MapAccess::next_value(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            } {
                                                __Field::__field0 => _serde::export::Result::map(
                                                    <String as _serde::Deserialize>::deserialize(
                                                        __deserializer,
                                                    ),
                                                    Argument::Regular,
                                                ),
                                                __Field::__field1 => _serde::export::Result::map(
                                                    <String as _serde::Deserialize>::deserialize(
                                                        __deserializer,
                                                    ),
                                                    Argument::Rest,
                                                ),
                                            } {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            };
                                        match {
                                            let mut __rk: _serde::export::Option<
                                                _serde::private::de::TagOrContentField,
                                            > = _serde::export::None;
                                            while let _serde :: export :: Some ( __k ) = match _serde :: de :: MapAccess :: next_key_seed ( & mut __map , _serde :: private :: de :: TagContentOtherFieldVisitor { tag : "type" , content : "value" , } ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } { match __k { _serde :: private :: de :: TagContentOtherField :: Other => { match _serde :: de :: MapAccess :: next_value :: < _serde :: de :: IgnoredAny > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ; continue ; } _serde :: private :: de :: TagContentOtherField :: Tag => { __rk = _serde :: export :: Some ( _serde :: private :: de :: TagOrContentField :: Tag ) ; break ; } _serde :: private :: de :: TagContentOtherField :: Content => { __rk = _serde :: export :: Some ( _serde :: private :: de :: TagOrContentField :: Content ) ; break ; } } }
                                            __rk
                                        } {
                                            _serde::export::Some(
                                                _serde::private::de::TagOrContentField::Tag,
                                            ) => _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "type",
                                                ),
                                            ),
                                            _serde::export::Some(
                                                _serde::private::de::TagOrContentField::Content,
                                            ) => _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "value",
                                                ),
                                            ),
                                            _serde::export::None => _serde::export::Ok(__ret),
                                        }
                                    }
                                    _serde::export::Some(
                                        _serde::private::de::TagOrContentField::Content,
                                    ) => _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("value"),
                                    ),
                                    _serde::export::None => _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::missing_field("type"),
                                    ),
                                }
                            }
                            _serde::export::None => _serde::export::Err(
                                <__A::Error as _serde::de::Error>::missing_field("type"),
                            ),
                        }
                    }
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        match match _serde::de::SeqAccess::next_element(&mut __seq) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__field) => {
                                match match _serde::de::SeqAccess::next_element_seed(
                                    &mut __seq,
                                    __Seed {
                                        field: __field,
                                        marker: _serde::export::PhantomData,
                                        lifetime: _serde::export::PhantomData,
                                    },
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                } {
                                    _serde::export::Some(__ret) => _serde::export::Ok(__ret),
                                    _serde::export::None => _serde::export::Err(
                                        _serde::de::Error::invalid_length(1, &self),
                                    ),
                                }
                            }
                            _serde::export::None => {
                                _serde::export::Err(_serde::de::Error::invalid_length(0, &self))
                            }
                        }
                    }
                }
                const FIELDS: &'static [&'static str] = &["type", "value"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Argument",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<Argument>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Argument {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&Argument::Regular(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Regular");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Argument::Rest(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Rest");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for Argument {
        #[inline]
        fn clone(&self) -> Argument {
            match (&*self,) {
                (&Argument::Regular(ref __self_0),) => {
                    Argument::Regular(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Argument::Rest(ref __self_0),) => {
                    Argument::Rest(::std::clone::Clone::clone(&(*__self_0)))
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for Argument {
        #[inline]
        fn eq(&self, other: &Argument) -> bool {
            {
                let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&Argument::Regular(ref __self_0), &Argument::Regular(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Argument::Rest(ref __self_0), &Argument::Rest(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        _ => unsafe { ::std::intrinsics::unreachable() },
                    }
                } else {
                    false
                }
            }
        }
        #[inline]
        fn ne(&self, other: &Argument) -> bool {
            {
                let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&Argument::Regular(ref __self_0), &Argument::Regular(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Argument::Rest(ref __self_0), &Argument::Rest(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        _ => unsafe { ::std::intrinsics::unreachable() },
                    }
                } else {
                    true
                }
            }
        }
    }
    #[serde(tag = "type")]
    pub enum Stmt {
        Program(ProgramStmt),
        Var(VarStmt),
        VarList(VarListStmt),
        Expr(ExprStmt),
        Func(FuncStmt),
        Class(ClassStmt),
        Block(BlockStmt),
        If(IfStmt),
        For(ForStmt),
        Return(ReturnStmt),
        Continue(ContinueStmt),
        Break(BreakStmt),
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_Stmt: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Stmt {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    Stmt::Program(ref __field0) => _serde::private::ser::serialize_tagged_newtype(
                        __serializer,
                        "Stmt",
                        "Program",
                        "type",
                        "Program",
                        __field0,
                    ),
                    Stmt::Var(ref __field0) => _serde::private::ser::serialize_tagged_newtype(
                        __serializer,
                        "Stmt",
                        "Var",
                        "type",
                        "Var",
                        __field0,
                    ),
                    Stmt::VarList(ref __field0) => _serde::private::ser::serialize_tagged_newtype(
                        __serializer,
                        "Stmt",
                        "VarList",
                        "type",
                        "VarList",
                        __field0,
                    ),
                    Stmt::Expr(ref __field0) => _serde::private::ser::serialize_tagged_newtype(
                        __serializer,
                        "Stmt",
                        "Expr",
                        "type",
                        "Expr",
                        __field0,
                    ),
                    Stmt::Func(ref __field0) => _serde::private::ser::serialize_tagged_newtype(
                        __serializer,
                        "Stmt",
                        "Func",
                        "type",
                        "Func",
                        __field0,
                    ),
                    Stmt::Class(ref __field0) => _serde::private::ser::serialize_tagged_newtype(
                        __serializer,
                        "Stmt",
                        "Class",
                        "type",
                        "Class",
                        __field0,
                    ),
                    Stmt::Block(ref __field0) => _serde::private::ser::serialize_tagged_newtype(
                        __serializer,
                        "Stmt",
                        "Block",
                        "type",
                        "Block",
                        __field0,
                    ),
                    Stmt::If(ref __field0) => _serde::private::ser::serialize_tagged_newtype(
                        __serializer,
                        "Stmt",
                        "If",
                        "type",
                        "If",
                        __field0,
                    ),
                    Stmt::For(ref __field0) => _serde::private::ser::serialize_tagged_newtype(
                        __serializer,
                        "Stmt",
                        "For",
                        "type",
                        "For",
                        __field0,
                    ),
                    Stmt::Return(ref __field0) => _serde::private::ser::serialize_tagged_newtype(
                        __serializer,
                        "Stmt",
                        "Return",
                        "type",
                        "Return",
                        __field0,
                    ),
                    Stmt::Continue(ref __field0) => _serde::private::ser::serialize_tagged_newtype(
                        __serializer,
                        "Stmt",
                        "Continue",
                        "type",
                        "Continue",
                        __field0,
                    ),
                    Stmt::Break(ref __field0) => _serde::private::ser::serialize_tagged_newtype(
                        __serializer,
                        "Stmt",
                        "Break",
                        "type",
                        "Break",
                        __field0,
                    ),
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_Stmt: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Stmt {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __field8,
                    __field9,
                    __field10,
                    __field11,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "variant identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            3u64 => _serde::export::Ok(__Field::__field3),
                            4u64 => _serde::export::Ok(__Field::__field4),
                            5u64 => _serde::export::Ok(__Field::__field5),
                            6u64 => _serde::export::Ok(__Field::__field6),
                            7u64 => _serde::export::Ok(__Field::__field7),
                            8u64 => _serde::export::Ok(__Field::__field8),
                            9u64 => _serde::export::Ok(__Field::__field9),
                            10u64 => _serde::export::Ok(__Field::__field10),
                            11u64 => _serde::export::Ok(__Field::__field11),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"variant index 0 <= i < 12",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "Program" => _serde::export::Ok(__Field::__field0),
                            "Var" => _serde::export::Ok(__Field::__field1),
                            "VarList" => _serde::export::Ok(__Field::__field2),
                            "Expr" => _serde::export::Ok(__Field::__field3),
                            "Func" => _serde::export::Ok(__Field::__field4),
                            "Class" => _serde::export::Ok(__Field::__field5),
                            "Block" => _serde::export::Ok(__Field::__field6),
                            "If" => _serde::export::Ok(__Field::__field7),
                            "For" => _serde::export::Ok(__Field::__field8),
                            "Return" => _serde::export::Ok(__Field::__field9),
                            "Continue" => _serde::export::Ok(__Field::__field10),
                            "Break" => _serde::export::Ok(__Field::__field11),
                            _ => _serde::export::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            )),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"Program" => _serde::export::Ok(__Field::__field0),
                            b"Var" => _serde::export::Ok(__Field::__field1),
                            b"VarList" => _serde::export::Ok(__Field::__field2),
                            b"Expr" => _serde::export::Ok(__Field::__field3),
                            b"Func" => _serde::export::Ok(__Field::__field4),
                            b"Class" => _serde::export::Ok(__Field::__field5),
                            b"Block" => _serde::export::Ok(__Field::__field6),
                            b"If" => _serde::export::Ok(__Field::__field7),
                            b"For" => _serde::export::Ok(__Field::__field8),
                            b"Return" => _serde::export::Ok(__Field::__field9),
                            b"Continue" => _serde::export::Ok(__Field::__field10),
                            b"Break" => _serde::export::Ok(__Field::__field11),
                            _ => {
                                let __value = &_serde::export::from_utf8_lossy(__value);
                                _serde::export::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                const VARIANTS: &'static [&'static str] = &[
                    "Program", "Var", "VarList", "Expr", "Func", "Class", "Block", "If", "For",
                    "Return", "Continue", "Break",
                ];
                let __tagged = match _serde::Deserializer::deserialize_any(
                    __deserializer,
                    _serde::private::de::TaggedContentVisitor::<__Field>::new("type"),
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match __tagged.tag {
                    __Field::__field0 => _serde::export::Result::map(
                        <ProgramStmt as _serde::Deserialize>::deserialize(
                            _serde::private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                        ),
                        Stmt::Program,
                    ),
                    __Field::__field1 => _serde::export::Result::map(
                        <VarStmt as _serde::Deserialize>::deserialize(
                            _serde::private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                        ),
                        Stmt::Var,
                    ),
                    __Field::__field2 => _serde::export::Result::map(
                        <VarListStmt as _serde::Deserialize>::deserialize(
                            _serde::private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                        ),
                        Stmt::VarList,
                    ),
                    __Field::__field3 => _serde::export::Result::map(
                        <ExprStmt as _serde::Deserialize>::deserialize(
                            _serde::private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                        ),
                        Stmt::Expr,
                    ),
                    __Field::__field4 => _serde::export::Result::map(
                        <FuncStmt as _serde::Deserialize>::deserialize(
                            _serde::private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                        ),
                        Stmt::Func,
                    ),
                    __Field::__field5 => _serde::export::Result::map(
                        <ClassStmt as _serde::Deserialize>::deserialize(
                            _serde::private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                        ),
                        Stmt::Class,
                    ),
                    __Field::__field6 => _serde::export::Result::map(
                        <BlockStmt as _serde::Deserialize>::deserialize(
                            _serde::private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                        ),
                        Stmt::Block,
                    ),
                    __Field::__field7 => _serde::export::Result::map(
                        <IfStmt as _serde::Deserialize>::deserialize(
                            _serde::private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                        ),
                        Stmt::If,
                    ),
                    __Field::__field8 => _serde::export::Result::map(
                        <ForStmt as _serde::Deserialize>::deserialize(
                            _serde::private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                        ),
                        Stmt::For,
                    ),
                    __Field::__field9 => _serde::export::Result::map(
                        <ReturnStmt as _serde::Deserialize>::deserialize(
                            _serde::private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                        ),
                        Stmt::Return,
                    ),
                    __Field::__field10 => _serde::export::Result::map(
                        <ContinueStmt as _serde::Deserialize>::deserialize(
                            _serde::private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                        ),
                        Stmt::Continue,
                    ),
                    __Field::__field11 => _serde::export::Result::map(
                        <BreakStmt as _serde::Deserialize>::deserialize(
                            _serde::private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                        ),
                        Stmt::Break,
                    ),
                }
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Stmt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&Stmt::Program(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Program");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Stmt::Var(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Var");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Stmt::VarList(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("VarList");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Stmt::Expr(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Expr");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Stmt::Func(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Func");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Stmt::Class(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Class");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Stmt::Block(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Block");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Stmt::If(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("If");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Stmt::For(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("For");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Stmt::Return(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Return");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Stmt::Continue(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Continue");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Stmt::Break(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Break");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for Stmt {
        #[inline]
        fn clone(&self) -> Stmt {
            match (&*self,) {
                (&Stmt::Program(ref __self_0),) => {
                    Stmt::Program(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Stmt::Var(ref __self_0),) => Stmt::Var(::std::clone::Clone::clone(&(*__self_0))),
                (&Stmt::VarList(ref __self_0),) => {
                    Stmt::VarList(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Stmt::Expr(ref __self_0),) => {
                    Stmt::Expr(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Stmt::Func(ref __self_0),) => {
                    Stmt::Func(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Stmt::Class(ref __self_0),) => {
                    Stmt::Class(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Stmt::Block(ref __self_0),) => {
                    Stmt::Block(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Stmt::If(ref __self_0),) => Stmt::If(::std::clone::Clone::clone(&(*__self_0))),
                (&Stmt::For(ref __self_0),) => Stmt::For(::std::clone::Clone::clone(&(*__self_0))),
                (&Stmt::Return(ref __self_0),) => {
                    Stmt::Return(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Stmt::Continue(ref __self_0),) => {
                    Stmt::Continue(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Stmt::Break(ref __self_0),) => {
                    Stmt::Break(::std::clone::Clone::clone(&(*__self_0)))
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for Stmt {
        #[inline]
        fn eq(&self, other: &Stmt) -> bool {
            {
                let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&Stmt::Program(ref __self_0), &Stmt::Program(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Stmt::Var(ref __self_0), &Stmt::Var(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Stmt::VarList(ref __self_0), &Stmt::VarList(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Stmt::Expr(ref __self_0), &Stmt::Expr(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Stmt::Func(ref __self_0), &Stmt::Func(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Stmt::Class(ref __self_0), &Stmt::Class(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Stmt::Block(ref __self_0), &Stmt::Block(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Stmt::If(ref __self_0), &Stmt::If(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Stmt::For(ref __self_0), &Stmt::For(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Stmt::Return(ref __self_0), &Stmt::Return(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Stmt::Continue(ref __self_0), &Stmt::Continue(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Stmt::Break(ref __self_0), &Stmt::Break(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        _ => unsafe { ::std::intrinsics::unreachable() },
                    }
                } else {
                    false
                }
            }
        }
        #[inline]
        fn ne(&self, other: &Stmt) -> bool {
            {
                let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&Stmt::Program(ref __self_0), &Stmt::Program(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Stmt::Var(ref __self_0), &Stmt::Var(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Stmt::VarList(ref __self_0), &Stmt::VarList(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Stmt::Expr(ref __self_0), &Stmt::Expr(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Stmt::Func(ref __self_0), &Stmt::Func(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Stmt::Class(ref __self_0), &Stmt::Class(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Stmt::Block(ref __self_0), &Stmt::Block(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Stmt::If(ref __self_0), &Stmt::If(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Stmt::For(ref __self_0), &Stmt::For(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Stmt::Return(ref __self_0), &Stmt::Return(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Stmt::Continue(ref __self_0), &Stmt::Continue(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Stmt::Break(ref __self_0), &Stmt::Break(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        _ => unsafe { ::std::intrinsics::unreachable() },
                    }
                } else {
                    true
                }
            }
        }
    }
    impl Stmt {
        pub fn accept<R>(&self, visitor: &mut StmtVisitor<R>) -> R {
            match self {
                Stmt::Program(s) => visitor.visit_program_stmt(&s),
                Stmt::Var(s) => visitor.visit_var_stmt(&s),
                Stmt::VarList(s) => visitor.visit_varlist_stmt(&s),
                Stmt::Expr(s) => visitor.visit_expr_stmt(&s),
                Stmt::Func(s) => visitor.visit_func_stmt(&s),
                Stmt::Class(s) => visitor.visit_class_stmt(&s),
                Stmt::Block(s) => visitor.visit_block_stmt(&s),
                Stmt::If(s) => visitor.visit_if_stmt(&s),
                Stmt::For(s) => visitor.visit_for_stmt(&s),
                Stmt::Return(s) => visitor.visit_return_stmt(&s),
                Stmt::Continue(s) => visitor.visit_continue_stmt(&s),
                Stmt::Break(s) => visitor.visit_break_stmt(&s),
            }
        }
    }
    #[serde(tag = "type")]
    pub enum Expr {
        Assign(AssignExpr),
        Call(CallExpr),
        Literal(LiteralExpr),
        Binary(BinaryExpr),
        Member(MemberExpr),
        Lookup(LookupExpr),
        Arguments(ArgumentsExpr),
        Logical(LogicalExpr),
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_Expr: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Expr {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    Expr::Assign(ref __field0) => _serde::private::ser::serialize_tagged_newtype(
                        __serializer,
                        "Expr",
                        "Assign",
                        "type",
                        "Assign",
                        __field0,
                    ),
                    Expr::Call(ref __field0) => _serde::private::ser::serialize_tagged_newtype(
                        __serializer,
                        "Expr",
                        "Call",
                        "type",
                        "Call",
                        __field0,
                    ),
                    Expr::Literal(ref __field0) => _serde::private::ser::serialize_tagged_newtype(
                        __serializer,
                        "Expr",
                        "Literal",
                        "type",
                        "Literal",
                        __field0,
                    ),
                    Expr::Binary(ref __field0) => _serde::private::ser::serialize_tagged_newtype(
                        __serializer,
                        "Expr",
                        "Binary",
                        "type",
                        "Binary",
                        __field0,
                    ),
                    Expr::Member(ref __field0) => _serde::private::ser::serialize_tagged_newtype(
                        __serializer,
                        "Expr",
                        "Member",
                        "type",
                        "Member",
                        __field0,
                    ),
                    Expr::Lookup(ref __field0) => _serde::private::ser::serialize_tagged_newtype(
                        __serializer,
                        "Expr",
                        "Lookup",
                        "type",
                        "Lookup",
                        __field0,
                    ),
                    Expr::Arguments(ref __field0) => {
                        _serde::private::ser::serialize_tagged_newtype(
                            __serializer,
                            "Expr",
                            "Arguments",
                            "type",
                            "Arguments",
                            __field0,
                        )
                    }
                    Expr::Logical(ref __field0) => _serde::private::ser::serialize_tagged_newtype(
                        __serializer,
                        "Expr",
                        "Logical",
                        "type",
                        "Logical",
                        __field0,
                    ),
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_Expr: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Expr {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "variant identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            3u64 => _serde::export::Ok(__Field::__field3),
                            4u64 => _serde::export::Ok(__Field::__field4),
                            5u64 => _serde::export::Ok(__Field::__field5),
                            6u64 => _serde::export::Ok(__Field::__field6),
                            7u64 => _serde::export::Ok(__Field::__field7),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"variant index 0 <= i < 8",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "Assign" => _serde::export::Ok(__Field::__field0),
                            "Call" => _serde::export::Ok(__Field::__field1),
                            "Literal" => _serde::export::Ok(__Field::__field2),
                            "Binary" => _serde::export::Ok(__Field::__field3),
                            "Member" => _serde::export::Ok(__Field::__field4),
                            "Lookup" => _serde::export::Ok(__Field::__field5),
                            "Arguments" => _serde::export::Ok(__Field::__field6),
                            "Logical" => _serde::export::Ok(__Field::__field7),
                            _ => _serde::export::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            )),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"Assign" => _serde::export::Ok(__Field::__field0),
                            b"Call" => _serde::export::Ok(__Field::__field1),
                            b"Literal" => _serde::export::Ok(__Field::__field2),
                            b"Binary" => _serde::export::Ok(__Field::__field3),
                            b"Member" => _serde::export::Ok(__Field::__field4),
                            b"Lookup" => _serde::export::Ok(__Field::__field5),
                            b"Arguments" => _serde::export::Ok(__Field::__field6),
                            b"Logical" => _serde::export::Ok(__Field::__field7),
                            _ => {
                                let __value = &_serde::export::from_utf8_lossy(__value);
                                _serde::export::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                const VARIANTS: &'static [&'static str] = &[
                    "Assign",
                    "Call",
                    "Literal",
                    "Binary",
                    "Member",
                    "Lookup",
                    "Arguments",
                    "Logical",
                ];
                let __tagged = match _serde::Deserializer::deserialize_any(
                    __deserializer,
                    _serde::private::de::TaggedContentVisitor::<__Field>::new("type"),
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match __tagged.tag {
                    __Field::__field0 => _serde::export::Result::map(
                        <AssignExpr as _serde::Deserialize>::deserialize(
                            _serde::private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                        ),
                        Expr::Assign,
                    ),
                    __Field::__field1 => _serde::export::Result::map(
                        <CallExpr as _serde::Deserialize>::deserialize(
                            _serde::private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                        ),
                        Expr::Call,
                    ),
                    __Field::__field2 => _serde::export::Result::map(
                        <LiteralExpr as _serde::Deserialize>::deserialize(
                            _serde::private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                        ),
                        Expr::Literal,
                    ),
                    __Field::__field3 => _serde::export::Result::map(
                        <BinaryExpr as _serde::Deserialize>::deserialize(
                            _serde::private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                        ),
                        Expr::Binary,
                    ),
                    __Field::__field4 => _serde::export::Result::map(
                        <MemberExpr as _serde::Deserialize>::deserialize(
                            _serde::private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                        ),
                        Expr::Member,
                    ),
                    __Field::__field5 => _serde::export::Result::map(
                        <LookupExpr as _serde::Deserialize>::deserialize(
                            _serde::private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                        ),
                        Expr::Lookup,
                    ),
                    __Field::__field6 => _serde::export::Result::map(
                        <ArgumentsExpr as _serde::Deserialize>::deserialize(
                            _serde::private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                        ),
                        Expr::Arguments,
                    ),
                    __Field::__field7 => _serde::export::Result::map(
                        <LogicalExpr as _serde::Deserialize>::deserialize(
                            _serde::private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                        ),
                        Expr::Logical,
                    ),
                }
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Expr {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&Expr::Assign(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Assign");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Expr::Call(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Call");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Expr::Literal(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Literal");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Expr::Binary(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Binary");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Expr::Member(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Member");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Expr::Lookup(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Lookup");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Expr::Arguments(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Arguments");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Expr::Logical(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Logical");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for Expr {
        #[inline]
        fn clone(&self) -> Expr {
            match (&*self,) {
                (&Expr::Assign(ref __self_0),) => {
                    Expr::Assign(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Expr::Call(ref __self_0),) => {
                    Expr::Call(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Expr::Literal(ref __self_0),) => {
                    Expr::Literal(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Expr::Binary(ref __self_0),) => {
                    Expr::Binary(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Expr::Member(ref __self_0),) => {
                    Expr::Member(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Expr::Lookup(ref __self_0),) => {
                    Expr::Lookup(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Expr::Arguments(ref __self_0),) => {
                    Expr::Arguments(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Expr::Logical(ref __self_0),) => {
                    Expr::Logical(::std::clone::Clone::clone(&(*__self_0)))
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for Expr {
        #[inline]
        fn eq(&self, other: &Expr) -> bool {
            {
                let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&Expr::Assign(ref __self_0), &Expr::Assign(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Expr::Call(ref __self_0), &Expr::Call(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Expr::Literal(ref __self_0), &Expr::Literal(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Expr::Binary(ref __self_0), &Expr::Binary(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Expr::Member(ref __self_0), &Expr::Member(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Expr::Lookup(ref __self_0), &Expr::Lookup(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Expr::Arguments(ref __self_0), &Expr::Arguments(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Expr::Logical(ref __self_0), &Expr::Logical(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        _ => unsafe { ::std::intrinsics::unreachable() },
                    }
                } else {
                    false
                }
            }
        }
        #[inline]
        fn ne(&self, other: &Expr) -> bool {
            {
                let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&Expr::Assign(ref __self_0), &Expr::Assign(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Expr::Call(ref __self_0), &Expr::Call(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Expr::Literal(ref __self_0), &Expr::Literal(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Expr::Binary(ref __self_0), &Expr::Binary(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Expr::Member(ref __self_0), &Expr::Member(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Expr::Lookup(ref __self_0), &Expr::Lookup(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Expr::Arguments(ref __self_0), &Expr::Arguments(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Expr::Logical(ref __self_0), &Expr::Logical(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        _ => unsafe { ::std::intrinsics::unreachable() },
                    }
                } else {
                    true
                }
            }
        }
    }
    impl Expr {
        pub fn accept<R>(&self, visitor: &mut ExprVisitor<R>) -> R {
            match self {
                Expr::Assign(s) => visitor.visit_assign_expr(&s),
                Expr::Call(s) => visitor.visit_call_expr(&s),
                Expr::Literal(s) => visitor.visit_literal_expr(&s),
                Expr::Binary(s) => visitor.visit_binary_expr(&s),
                Expr::Member(s) => visitor.visit_member_expr(&s),
                Expr::Lookup(s) => visitor.visit_lookup_expr(&s),
                Expr::Arguments(s) => visitor.visit_arguments_expr(&s),
                Expr::Logical(s) => visitor.visit_logical_expr(&s),
            }
        }
    }
    pub struct ProgramStmt {
        pub statements: Vec<Box<Stmt>>,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_ProgramStmt: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ProgramStmt {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "ProgramStmt",
                    false as usize + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "statements",
                    &self.statements,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_ProgramStmt: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ProgramStmt {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 1",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "statements" => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"statements" => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<ProgramStmt>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ProgramStmt;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct ProgramStmt")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            Vec<Box<Stmt>>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct ProgramStmt with 1 element",
                                ));
                            }
                        };
                        _serde::export::Ok(ProgramStmt {
                            statements: __field0,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<Vec<Box<Stmt>>> =
                            _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "statements",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Vec<Box<Stmt>>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("statements") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(ProgramStmt {
                            statements: __field0,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["statements"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "ProgramStmt",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<ProgramStmt>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ProgramStmt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                ProgramStmt {
                    statements: ref __self_0_0,
                } => {
                    let mut debug_trait_builder = f.debug_struct("ProgramStmt");
                    let _ = debug_trait_builder.field("statements", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for ProgramStmt {
        #[inline]
        fn clone(&self) -> ProgramStmt {
            match *self {
                ProgramStmt {
                    statements: ref __self_0_0,
                } => ProgramStmt {
                    statements: ::std::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for ProgramStmt {
        #[inline]
        fn eq(&self, other: &ProgramStmt) -> bool {
            match *other {
                ProgramStmt {
                    statements: ref __self_1_0,
                } => match *self {
                    ProgramStmt {
                        statements: ref __self_0_0,
                    } => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &ProgramStmt) -> bool {
            match *other {
                ProgramStmt {
                    statements: ref __self_1_0,
                } => match *self {
                    ProgramStmt {
                        statements: ref __self_0_0,
                    } => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl ProgramStmt {
        pub fn new(statements: Vec<Box<Stmt>>) -> ProgramStmt {
            ProgramStmt { statements }
        }
    }
    pub struct VarStmt {
        pub name: Token,
        pub initializer: Option<Expr>,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_VarStmt: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for VarStmt {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "VarStmt",
                    false as usize + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "name",
                    &self.name,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "initializer",
                    &self.initializer,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_VarStmt: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for VarStmt {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 2",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::export::Ok(__Field::__field0),
                            "initializer" => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::export::Ok(__Field::__field0),
                            b"initializer" => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<VarStmt>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = VarStmt;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct VarStmt")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<Token>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct VarStmt with 2 elements",
                                    ));
                                }
                            };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<Option<Expr>>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct VarStmt with 2 elements",
                                ));
                            }
                        };
                        _serde::export::Ok(VarStmt {
                            name: __field0,
                            initializer: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<Token> = _serde::export::None;
                        let mut __field1: _serde::export::Option<Option<Expr>> =
                            _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "name",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Token>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "initializer",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<Expr>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("name") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("initializer") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(VarStmt {
                            name: __field0,
                            initializer: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["name", "initializer"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "VarStmt",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<VarStmt>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for VarStmt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                VarStmt {
                    name: ref __self_0_0,
                    initializer: ref __self_0_1,
                } => {
                    let mut debug_trait_builder = f.debug_struct("VarStmt");
                    let _ = debug_trait_builder.field("name", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("initializer", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for VarStmt {
        #[inline]
        fn clone(&self) -> VarStmt {
            match *self {
                VarStmt {
                    name: ref __self_0_0,
                    initializer: ref __self_0_1,
                } => VarStmt {
                    name: ::std::clone::Clone::clone(&(*__self_0_0)),
                    initializer: ::std::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for VarStmt {
        #[inline]
        fn eq(&self, other: &VarStmt) -> bool {
            match *other {
                VarStmt {
                    name: ref __self_1_0,
                    initializer: ref __self_1_1,
                } => match *self {
                    VarStmt {
                        name: ref __self_0_0,
                        initializer: ref __self_0_1,
                    } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &VarStmt) -> bool {
            match *other {
                VarStmt {
                    name: ref __self_1_0,
                    initializer: ref __self_1_1,
                } => match *self {
                    VarStmt {
                        name: ref __self_0_0,
                        initializer: ref __self_0_1,
                    } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    impl VarStmt {
        pub fn new(name: Token, initializer: Option<Expr>) -> VarStmt {
            VarStmt { name, initializer }
        }
    }
    pub struct VarListStmt {
        pub variables: Vec<VarStmt>,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_VarListStmt: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for VarListStmt {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "VarListStmt",
                    false as usize + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "variables",
                    &self.variables,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_VarListStmt: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for VarListStmt {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 1",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "variables" => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"variables" => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<VarListStmt>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = VarListStmt;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct VarListStmt")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<Vec<VarStmt>>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct VarListStmt with 1 element",
                                ));
                            }
                        };
                        _serde::export::Ok(VarListStmt {
                            variables: __field0,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<Vec<VarStmt>> =
                            _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "variables",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Vec<VarStmt>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("variables") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(VarListStmt {
                            variables: __field0,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["variables"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "VarListStmt",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<VarListStmt>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for VarListStmt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                VarListStmt {
                    variables: ref __self_0_0,
                } => {
                    let mut debug_trait_builder = f.debug_struct("VarListStmt");
                    let _ = debug_trait_builder.field("variables", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for VarListStmt {
        #[inline]
        fn clone(&self) -> VarListStmt {
            match *self {
                VarListStmt {
                    variables: ref __self_0_0,
                } => VarListStmt {
                    variables: ::std::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for VarListStmt {
        #[inline]
        fn eq(&self, other: &VarListStmt) -> bool {
            match *other {
                VarListStmt {
                    variables: ref __self_1_0,
                } => match *self {
                    VarListStmt {
                        variables: ref __self_0_0,
                    } => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &VarListStmt) -> bool {
            match *other {
                VarListStmt {
                    variables: ref __self_1_0,
                } => match *self {
                    VarListStmt {
                        variables: ref __self_0_0,
                    } => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl VarListStmt {
        pub fn new(variables: Vec<VarStmt>) -> VarListStmt {
            VarListStmt { variables }
        }
    }
    pub struct ExprStmt {
        pub expression: Expr,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_ExprStmt: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ExprStmt {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "ExprStmt",
                    false as usize + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "expression",
                    &self.expression,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_ExprStmt: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ExprStmt {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 1",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "expression" => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"expression" => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<ExprStmt>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ExprStmt;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct ExprStmt")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<Expr>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct ExprStmt with 1 element",
                                    ));
                                }
                            };
                        _serde::export::Ok(ExprStmt {
                            expression: __field0,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<Expr> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "expression",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Expr>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("expression") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(ExprStmt {
                            expression: __field0,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["expression"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "ExprStmt",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<ExprStmt>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ExprStmt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                ExprStmt {
                    expression: ref __self_0_0,
                } => {
                    let mut debug_trait_builder = f.debug_struct("ExprStmt");
                    let _ = debug_trait_builder.field("expression", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for ExprStmt {
        #[inline]
        fn clone(&self) -> ExprStmt {
            match *self {
                ExprStmt {
                    expression: ref __self_0_0,
                } => ExprStmt {
                    expression: ::std::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for ExprStmt {
        #[inline]
        fn eq(&self, other: &ExprStmt) -> bool {
            match *other {
                ExprStmt {
                    expression: ref __self_1_0,
                } => match *self {
                    ExprStmt {
                        expression: ref __self_0_0,
                    } => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &ExprStmt) -> bool {
            match *other {
                ExprStmt {
                    expression: ref __self_1_0,
                } => match *self {
                    ExprStmt {
                        expression: ref __self_0_0,
                    } => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl ExprStmt {
        pub fn new(expression: Expr) -> ExprStmt {
            ExprStmt { expression }
        }
    }
    pub struct FuncStmt {
        pub name: Token,
        pub body: Box<Stmt>,
        pub parameters: Vec<Argument>,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_FuncStmt: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for FuncStmt {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "FuncStmt",
                    false as usize + 1 + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "name",
                    &self.name,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "body",
                    &self.body,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "parameters",
                    &self.parameters,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_FuncStmt: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for FuncStmt {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 3",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::export::Ok(__Field::__field0),
                            "body" => _serde::export::Ok(__Field::__field1),
                            "parameters" => _serde::export::Ok(__Field::__field2),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::export::Ok(__Field::__field0),
                            b"body" => _serde::export::Ok(__Field::__field1),
                            b"parameters" => _serde::export::Ok(__Field::__field2),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<FuncStmt>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = FuncStmt;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct FuncStmt")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<Token>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct FuncStmt with 3 elements",
                                    ));
                                }
                            };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<Box<Stmt>>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct FuncStmt with 3 elements",
                                ));
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                            Vec<Argument>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct FuncStmt with 3 elements",
                                ));
                            }
                        };
                        _serde::export::Ok(FuncStmt {
                            name: __field0,
                            body: __field1,
                            parameters: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<Token> = _serde::export::None;
                        let mut __field1: _serde::export::Option<Box<Stmt>> = _serde::export::None;
                        let mut __field2: _serde::export::Option<Vec<Argument>> =
                            _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "name",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Token>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "body",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Box<Stmt>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "parameters",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Vec<Argument>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("name") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("body") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::export::Some(__field2) => __field2,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("parameters") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(FuncStmt {
                            name: __field0,
                            body: __field1,
                            parameters: __field2,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["name", "body", "parameters"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "FuncStmt",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<FuncStmt>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for FuncStmt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                FuncStmt {
                    name: ref __self_0_0,
                    body: ref __self_0_1,
                    parameters: ref __self_0_2,
                } => {
                    let mut debug_trait_builder = f.debug_struct("FuncStmt");
                    let _ = debug_trait_builder.field("name", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("body", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("parameters", &&(*__self_0_2));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for FuncStmt {
        #[inline]
        fn clone(&self) -> FuncStmt {
            match *self {
                FuncStmt {
                    name: ref __self_0_0,
                    body: ref __self_0_1,
                    parameters: ref __self_0_2,
                } => FuncStmt {
                    name: ::std::clone::Clone::clone(&(*__self_0_0)),
                    body: ::std::clone::Clone::clone(&(*__self_0_1)),
                    parameters: ::std::clone::Clone::clone(&(*__self_0_2)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for FuncStmt {
        #[inline]
        fn eq(&self, other: &FuncStmt) -> bool {
            match *other {
                FuncStmt {
                    name: ref __self_1_0,
                    body: ref __self_1_1,
                    parameters: ref __self_1_2,
                } => match *self {
                    FuncStmt {
                        name: ref __self_0_0,
                        body: ref __self_0_1,
                        parameters: ref __self_0_2,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &FuncStmt) -> bool {
            match *other {
                FuncStmt {
                    name: ref __self_1_0,
                    body: ref __self_1_1,
                    parameters: ref __self_1_2,
                } => match *self {
                    FuncStmt {
                        name: ref __self_0_0,
                        body: ref __self_0_1,
                        parameters: ref __self_0_2,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                    }
                },
            }
        }
    }
    impl FuncStmt {
        pub fn new(name: Token, body: Box<Stmt>, parameters: Vec<Argument>) -> FuncStmt {
            FuncStmt {
                name,
                body,
                parameters,
            }
        }
    }
    pub struct ClassStmt {
        pub name: Token,
        pub members: Vec<Box<Stmt>>,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_ClassStmt: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ClassStmt {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "ClassStmt",
                    false as usize + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "name",
                    &self.name,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "members",
                    &self.members,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_ClassStmt: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ClassStmt {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 2",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::export::Ok(__Field::__field0),
                            "members" => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::export::Ok(__Field::__field0),
                            b"members" => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<ClassStmt>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ClassStmt;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct ClassStmt")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<Token>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct ClassStmt with 2 elements",
                                    ));
                                }
                            };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            Vec<Box<Stmt>>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct ClassStmt with 2 elements",
                                ));
                            }
                        };
                        _serde::export::Ok(ClassStmt {
                            name: __field0,
                            members: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<Token> = _serde::export::None;
                        let mut __field1: _serde::export::Option<Vec<Box<Stmt>>> =
                            _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "name",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Token>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "members",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Vec<Box<Stmt>>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("name") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("members") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(ClassStmt {
                            name: __field0,
                            members: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["name", "members"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "ClassStmt",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<ClassStmt>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ClassStmt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                ClassStmt {
                    name: ref __self_0_0,
                    members: ref __self_0_1,
                } => {
                    let mut debug_trait_builder = f.debug_struct("ClassStmt");
                    let _ = debug_trait_builder.field("name", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("members", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for ClassStmt {
        #[inline]
        fn clone(&self) -> ClassStmt {
            match *self {
                ClassStmt {
                    name: ref __self_0_0,
                    members: ref __self_0_1,
                } => ClassStmt {
                    name: ::std::clone::Clone::clone(&(*__self_0_0)),
                    members: ::std::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for ClassStmt {
        #[inline]
        fn eq(&self, other: &ClassStmt) -> bool {
            match *other {
                ClassStmt {
                    name: ref __self_1_0,
                    members: ref __self_1_1,
                } => match *self {
                    ClassStmt {
                        name: ref __self_0_0,
                        members: ref __self_0_1,
                    } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &ClassStmt) -> bool {
            match *other {
                ClassStmt {
                    name: ref __self_1_0,
                    members: ref __self_1_1,
                } => match *self {
                    ClassStmt {
                        name: ref __self_0_0,
                        members: ref __self_0_1,
                    } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    impl ClassStmt {
        pub fn new(name: Token, members: Vec<Box<Stmt>>) -> ClassStmt {
            ClassStmt { name, members }
        }
    }
    pub struct BlockStmt {
        pub statements: Vec<Box<Stmt>>,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_BlockStmt: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for BlockStmt {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "BlockStmt",
                    false as usize + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "statements",
                    &self.statements,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_BlockStmt: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for BlockStmt {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 1",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "statements" => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"statements" => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<BlockStmt>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = BlockStmt;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct BlockStmt")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            Vec<Box<Stmt>>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct BlockStmt with 1 element",
                                ));
                            }
                        };
                        _serde::export::Ok(BlockStmt {
                            statements: __field0,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<Vec<Box<Stmt>>> =
                            _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "statements",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Vec<Box<Stmt>>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("statements") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(BlockStmt {
                            statements: __field0,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["statements"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "BlockStmt",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<BlockStmt>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for BlockStmt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                BlockStmt {
                    statements: ref __self_0_0,
                } => {
                    let mut debug_trait_builder = f.debug_struct("BlockStmt");
                    let _ = debug_trait_builder.field("statements", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for BlockStmt {
        #[inline]
        fn clone(&self) -> BlockStmt {
            match *self {
                BlockStmt {
                    statements: ref __self_0_0,
                } => BlockStmt {
                    statements: ::std::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for BlockStmt {
        #[inline]
        fn eq(&self, other: &BlockStmt) -> bool {
            match *other {
                BlockStmt {
                    statements: ref __self_1_0,
                } => match *self {
                    BlockStmt {
                        statements: ref __self_0_0,
                    } => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &BlockStmt) -> bool {
            match *other {
                BlockStmt {
                    statements: ref __self_1_0,
                } => match *self {
                    BlockStmt {
                        statements: ref __self_0_0,
                    } => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl BlockStmt {
        pub fn new(statements: Vec<Box<Stmt>>) -> BlockStmt {
            BlockStmt { statements }
        }
    }
    pub struct IfStmt {
        pub test: Expr,
        pub consequent: Box<Stmt>,
        pub alternative: Option<Box<Stmt>>,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_IfStmt: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for IfStmt {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "IfStmt",
                    false as usize + 1 + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "test",
                    &self.test,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "consequent",
                    &self.consequent,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "alternative",
                    &self.alternative,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_IfStmt: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for IfStmt {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 3",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "test" => _serde::export::Ok(__Field::__field0),
                            "consequent" => _serde::export::Ok(__Field::__field1),
                            "alternative" => _serde::export::Ok(__Field::__field2),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"test" => _serde::export::Ok(__Field::__field0),
                            b"consequent" => _serde::export::Ok(__Field::__field1),
                            b"alternative" => _serde::export::Ok(__Field::__field2),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<IfStmt>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = IfStmt;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct IfStmt")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<Expr>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct IfStmt with 3 elements",
                                    ));
                                }
                            };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<Box<Stmt>>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct IfStmt with 3 elements",
                                ));
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                            Option<Box<Stmt>>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct IfStmt with 3 elements",
                                ));
                            }
                        };
                        _serde::export::Ok(IfStmt {
                            test: __field0,
                            consequent: __field1,
                            alternative: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<Expr> = _serde::export::None;
                        let mut __field1: _serde::export::Option<Box<Stmt>> = _serde::export::None;
                        let mut __field2: _serde::export::Option<Option<Box<Stmt>>> =
                            _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "test",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Expr>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "consequent",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Box<Stmt>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "alternative",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<Box<Stmt>>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("test") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("consequent") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::export::Some(__field2) => __field2,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("alternative") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(IfStmt {
                            test: __field0,
                            consequent: __field1,
                            alternative: __field2,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["test", "consequent", "alternative"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "IfStmt",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<IfStmt>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for IfStmt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                IfStmt {
                    test: ref __self_0_0,
                    consequent: ref __self_0_1,
                    alternative: ref __self_0_2,
                } => {
                    let mut debug_trait_builder = f.debug_struct("IfStmt");
                    let _ = debug_trait_builder.field("test", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("consequent", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("alternative", &&(*__self_0_2));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for IfStmt {
        #[inline]
        fn clone(&self) -> IfStmt {
            match *self {
                IfStmt {
                    test: ref __self_0_0,
                    consequent: ref __self_0_1,
                    alternative: ref __self_0_2,
                } => IfStmt {
                    test: ::std::clone::Clone::clone(&(*__self_0_0)),
                    consequent: ::std::clone::Clone::clone(&(*__self_0_1)),
                    alternative: ::std::clone::Clone::clone(&(*__self_0_2)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for IfStmt {
        #[inline]
        fn eq(&self, other: &IfStmt) -> bool {
            match *other {
                IfStmt {
                    test: ref __self_1_0,
                    consequent: ref __self_1_1,
                    alternative: ref __self_1_2,
                } => match *self {
                    IfStmt {
                        test: ref __self_0_0,
                        consequent: ref __self_0_1,
                        alternative: ref __self_0_2,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &IfStmt) -> bool {
            match *other {
                IfStmt {
                    test: ref __self_1_0,
                    consequent: ref __self_1_1,
                    alternative: ref __self_1_2,
                } => match *self {
                    IfStmt {
                        test: ref __self_0_0,
                        consequent: ref __self_0_1,
                        alternative: ref __self_0_2,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                    }
                },
            }
        }
    }
    impl IfStmt {
        pub fn new(test: Expr, consequent: Box<Stmt>, alternative: Option<Box<Stmt>>) -> IfStmt {
            IfStmt {
                test,
                consequent,
                alternative,
            }
        }
    }
    pub struct ForStmt {
        pub element: Token,
        pub index: Option<Token>,
        pub iterator: Expr,
        pub body: Box<Stmt>,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_ForStmt: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ForStmt {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "ForStmt",
                    false as usize + 1 + 1 + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "element",
                    &self.element,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "index",
                    &self.index,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "iterator",
                    &self.iterator,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "body",
                    &self.body,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_ForStmt: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ForStmt {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            3u64 => _serde::export::Ok(__Field::__field3),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 4",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "element" => _serde::export::Ok(__Field::__field0),
                            "index" => _serde::export::Ok(__Field::__field1),
                            "iterator" => _serde::export::Ok(__Field::__field2),
                            "body" => _serde::export::Ok(__Field::__field3),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"element" => _serde::export::Ok(__Field::__field0),
                            b"index" => _serde::export::Ok(__Field::__field1),
                            b"iterator" => _serde::export::Ok(__Field::__field2),
                            b"body" => _serde::export::Ok(__Field::__field3),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<ForStmt>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ForStmt;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct ForStmt")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<Token>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct ForStmt with 4 elements",
                                    ));
                                }
                            };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            Option<Token>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct ForStmt with 4 elements",
                                ));
                            }
                        };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<Expr>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct ForStmt with 4 elements",
                                    ));
                                }
                            };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<Box<Stmt>>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    3usize,
                                    &"struct ForStmt with 4 elements",
                                ));
                            }
                        };
                        _serde::export::Ok(ForStmt {
                            element: __field0,
                            index: __field1,
                            iterator: __field2,
                            body: __field3,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<Token> = _serde::export::None;
                        let mut __field1: _serde::export::Option<Option<Token>> =
                            _serde::export::None;
                        let mut __field2: _serde::export::Option<Expr> = _serde::export::None;
                        let mut __field3: _serde::export::Option<Box<Stmt>> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "element",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Token>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "index",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<Token>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "iterator",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Expr>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::export::Option::is_some(&__field3) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "body",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Box<Stmt>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("element") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("index") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::export::Some(__field2) => __field2,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("iterator") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::export::Some(__field3) => __field3,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("body") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(ForStmt {
                            element: __field0,
                            index: __field1,
                            iterator: __field2,
                            body: __field3,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["element", "index", "iterator", "body"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "ForStmt",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<ForStmt>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ForStmt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                ForStmt {
                    element: ref __self_0_0,
                    index: ref __self_0_1,
                    iterator: ref __self_0_2,
                    body: ref __self_0_3,
                } => {
                    let mut debug_trait_builder = f.debug_struct("ForStmt");
                    let _ = debug_trait_builder.field("element", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("index", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("iterator", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("body", &&(*__self_0_3));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for ForStmt {
        #[inline]
        fn clone(&self) -> ForStmt {
            match *self {
                ForStmt {
                    element: ref __self_0_0,
                    index: ref __self_0_1,
                    iterator: ref __self_0_2,
                    body: ref __self_0_3,
                } => ForStmt {
                    element: ::std::clone::Clone::clone(&(*__self_0_0)),
                    index: ::std::clone::Clone::clone(&(*__self_0_1)),
                    iterator: ::std::clone::Clone::clone(&(*__self_0_2)),
                    body: ::std::clone::Clone::clone(&(*__self_0_3)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for ForStmt {
        #[inline]
        fn eq(&self, other: &ForStmt) -> bool {
            match *other {
                ForStmt {
                    element: ref __self_1_0,
                    index: ref __self_1_1,
                    iterator: ref __self_1_2,
                    body: ref __self_1_3,
                } => match *self {
                    ForStmt {
                        element: ref __self_0_0,
                        index: ref __self_0_1,
                        iterator: ref __self_0_2,
                        body: ref __self_0_3,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                            && (*__self_0_3) == (*__self_1_3)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &ForStmt) -> bool {
            match *other {
                ForStmt {
                    element: ref __self_1_0,
                    index: ref __self_1_1,
                    iterator: ref __self_1_2,
                    body: ref __self_1_3,
                } => match *self {
                    ForStmt {
                        element: ref __self_0_0,
                        index: ref __self_0_1,
                        iterator: ref __self_0_2,
                        body: ref __self_0_3,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                            || (*__self_0_3) != (*__self_1_3)
                    }
                },
            }
        }
    }
    impl ForStmt {
        pub fn new(
            element: Token,
            index: Option<Token>,
            iterator: Expr,
            body: Box<Stmt>,
        ) -> ForStmt {
            ForStmt {
                element,
                index,
                iterator,
                body,
            }
        }
    }
    pub struct ReturnStmt {
        pub expression: Option<Expr>,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_ReturnStmt: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ReturnStmt {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "ReturnStmt",
                    false as usize + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "expression",
                    &self.expression,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_ReturnStmt: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ReturnStmt {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 1",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "expression" => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"expression" => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<ReturnStmt>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ReturnStmt;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct ReturnStmt")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<Option<Expr>>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct ReturnStmt with 1 element",
                                ));
                            }
                        };
                        _serde::export::Ok(ReturnStmt {
                            expression: __field0,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<Option<Expr>> =
                            _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "expression",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<Expr>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("expression") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(ReturnStmt {
                            expression: __field0,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["expression"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "ReturnStmt",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<ReturnStmt>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ReturnStmt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                ReturnStmt {
                    expression: ref __self_0_0,
                } => {
                    let mut debug_trait_builder = f.debug_struct("ReturnStmt");
                    let _ = debug_trait_builder.field("expression", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for ReturnStmt {
        #[inline]
        fn clone(&self) -> ReturnStmt {
            match *self {
                ReturnStmt {
                    expression: ref __self_0_0,
                } => ReturnStmt {
                    expression: ::std::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for ReturnStmt {
        #[inline]
        fn eq(&self, other: &ReturnStmt) -> bool {
            match *other {
                ReturnStmt {
                    expression: ref __self_1_0,
                } => match *self {
                    ReturnStmt {
                        expression: ref __self_0_0,
                    } => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &ReturnStmt) -> bool {
            match *other {
                ReturnStmt {
                    expression: ref __self_1_0,
                } => match *self {
                    ReturnStmt {
                        expression: ref __self_0_0,
                    } => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl ReturnStmt {
        pub fn new(expression: Option<Expr>) -> ReturnStmt {
            ReturnStmt { expression }
        }
    }
    pub struct ContinueStmt {}
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_ContinueStmt: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ContinueStmt {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "ContinueStmt",
                    false as usize,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_ContinueStmt: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ContinueStmt {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 0",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<ContinueStmt>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ContinueStmt;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct ContinueStmt")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        _: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        _serde::export::Ok(ContinueStmt {})
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        _serde::export::Ok(ContinueStmt {})
                    }
                }
                const FIELDS: &'static [&'static str] = &[];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "ContinueStmt",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<ContinueStmt>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ContinueStmt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                ContinueStmt {} => {
                    let mut debug_trait_builder = f.debug_struct("ContinueStmt");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for ContinueStmt {
        #[inline]
        fn clone(&self) -> ContinueStmt {
            match *self {
                ContinueStmt {} => ContinueStmt {},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for ContinueStmt {
        #[inline]
        fn eq(&self, other: &ContinueStmt) -> bool {
            match *other {
                ContinueStmt {} => match *self {
                    ContinueStmt {} => true,
                },
            }
        }
    }
    impl ContinueStmt {
        pub fn new() -> ContinueStmt {
            ContinueStmt {}
        }
    }
    pub struct BreakStmt {}
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_BreakStmt: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for BreakStmt {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "BreakStmt",
                    false as usize,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_BreakStmt: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for BreakStmt {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 0",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<BreakStmt>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = BreakStmt;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct BreakStmt")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        _: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        _serde::export::Ok(BreakStmt {})
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        _serde::export::Ok(BreakStmt {})
                    }
                }
                const FIELDS: &'static [&'static str] = &[];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "BreakStmt",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<BreakStmt>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for BreakStmt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                BreakStmt {} => {
                    let mut debug_trait_builder = f.debug_struct("BreakStmt");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for BreakStmt {
        #[inline]
        fn clone(&self) -> BreakStmt {
            match *self {
                BreakStmt {} => BreakStmt {},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for BreakStmt {
        #[inline]
        fn eq(&self, other: &BreakStmt) -> bool {
            match *other {
                BreakStmt {} => match *self {
                    BreakStmt {} => true,
                },
            }
        }
    }
    impl BreakStmt {
        pub fn new() -> BreakStmt {
            BreakStmt {}
        }
    }
    pub struct AssignExpr {
        pub destination: Box<Expr>,
        pub value: Box<Expr>,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_AssignExpr: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for AssignExpr {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "AssignExpr",
                    false as usize + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "destination",
                    &self.destination,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "value",
                    &self.value,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_AssignExpr: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for AssignExpr {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 2",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "destination" => _serde::export::Ok(__Field::__field0),
                            "value" => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"destination" => _serde::export::Ok(__Field::__field0),
                            b"value" => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<AssignExpr>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = AssignExpr;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct AssignExpr")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<Box<Expr>>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct AssignExpr with 2 elements",
                                ));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<Box<Expr>>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct AssignExpr with 2 elements",
                                ));
                            }
                        };
                        _serde::export::Ok(AssignExpr {
                            destination: __field0,
                            value: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<Box<Expr>> = _serde::export::None;
                        let mut __field1: _serde::export::Option<Box<Expr>> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "destination",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Box<Expr>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "value",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Box<Expr>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("destination") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("value") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(AssignExpr {
                            destination: __field0,
                            value: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["destination", "value"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "AssignExpr",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<AssignExpr>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for AssignExpr {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                AssignExpr {
                    destination: ref __self_0_0,
                    value: ref __self_0_1,
                } => {
                    let mut debug_trait_builder = f.debug_struct("AssignExpr");
                    let _ = debug_trait_builder.field("destination", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("value", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for AssignExpr {
        #[inline]
        fn clone(&self) -> AssignExpr {
            match *self {
                AssignExpr {
                    destination: ref __self_0_0,
                    value: ref __self_0_1,
                } => AssignExpr {
                    destination: ::std::clone::Clone::clone(&(*__self_0_0)),
                    value: ::std::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for AssignExpr {
        #[inline]
        fn eq(&self, other: &AssignExpr) -> bool {
            match *other {
                AssignExpr {
                    destination: ref __self_1_0,
                    value: ref __self_1_1,
                } => match *self {
                    AssignExpr {
                        destination: ref __self_0_0,
                        value: ref __self_0_1,
                    } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &AssignExpr) -> bool {
            match *other {
                AssignExpr {
                    destination: ref __self_1_0,
                    value: ref __self_1_1,
                } => match *self {
                    AssignExpr {
                        destination: ref __self_0_0,
                        value: ref __self_0_1,
                    } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    impl AssignExpr {
        pub fn new(destination: Box<Expr>, value: Box<Expr>) -> AssignExpr {
            AssignExpr { destination, value }
        }
    }
    pub struct CallExpr {
        pub member: Box<Expr>,
        pub arguments: Box<Expr>,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_CallExpr: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for CallExpr {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "CallExpr",
                    false as usize + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "member",
                    &self.member,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "arguments",
                    &self.arguments,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_CallExpr: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for CallExpr {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 2",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "member" => _serde::export::Ok(__Field::__field0),
                            "arguments" => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"member" => _serde::export::Ok(__Field::__field0),
                            b"arguments" => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<CallExpr>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = CallExpr;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct CallExpr")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<Box<Expr>>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct CallExpr with 2 elements",
                                ));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<Box<Expr>>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct CallExpr with 2 elements",
                                ));
                            }
                        };
                        _serde::export::Ok(CallExpr {
                            member: __field0,
                            arguments: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<Box<Expr>> = _serde::export::None;
                        let mut __field1: _serde::export::Option<Box<Expr>> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "member",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Box<Expr>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "arguments",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Box<Expr>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("member") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("arguments") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(CallExpr {
                            member: __field0,
                            arguments: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["member", "arguments"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "CallExpr",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<CallExpr>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for CallExpr {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                CallExpr {
                    member: ref __self_0_0,
                    arguments: ref __self_0_1,
                } => {
                    let mut debug_trait_builder = f.debug_struct("CallExpr");
                    let _ = debug_trait_builder.field("member", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("arguments", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for CallExpr {
        #[inline]
        fn clone(&self) -> CallExpr {
            match *self {
                CallExpr {
                    member: ref __self_0_0,
                    arguments: ref __self_0_1,
                } => CallExpr {
                    member: ::std::clone::Clone::clone(&(*__self_0_0)),
                    arguments: ::std::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for CallExpr {
        #[inline]
        fn eq(&self, other: &CallExpr) -> bool {
            match *other {
                CallExpr {
                    member: ref __self_1_0,
                    arguments: ref __self_1_1,
                } => match *self {
                    CallExpr {
                        member: ref __self_0_0,
                        arguments: ref __self_0_1,
                    } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &CallExpr) -> bool {
            match *other {
                CallExpr {
                    member: ref __self_1_0,
                    arguments: ref __self_1_1,
                } => match *self {
                    CallExpr {
                        member: ref __self_0_0,
                        arguments: ref __self_0_1,
                    } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    impl CallExpr {
        pub fn new(member: Box<Expr>, arguments: Box<Expr>) -> CallExpr {
            CallExpr { member, arguments }
        }
    }
    pub struct LiteralExpr {
        pub value: Literal,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_LiteralExpr: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for LiteralExpr {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "LiteralExpr",
                    false as usize + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "value",
                    &self.value,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_LiteralExpr: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for LiteralExpr {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 1",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "value" => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"value" => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<LiteralExpr>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = LiteralExpr;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct LiteralExpr")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<Literal>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct LiteralExpr with 1 element",
                                ));
                            }
                        };
                        _serde::export::Ok(LiteralExpr { value: __field0 })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<Literal> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "value",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Literal>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("value") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(LiteralExpr { value: __field0 })
                    }
                }
                const FIELDS: &'static [&'static str] = &["value"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "LiteralExpr",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<LiteralExpr>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for LiteralExpr {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                LiteralExpr {
                    value: ref __self_0_0,
                } => {
                    let mut debug_trait_builder = f.debug_struct("LiteralExpr");
                    let _ = debug_trait_builder.field("value", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for LiteralExpr {
        #[inline]
        fn clone(&self) -> LiteralExpr {
            match *self {
                LiteralExpr {
                    value: ref __self_0_0,
                } => LiteralExpr {
                    value: ::std::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for LiteralExpr {
        #[inline]
        fn eq(&self, other: &LiteralExpr) -> bool {
            match *other {
                LiteralExpr {
                    value: ref __self_1_0,
                } => match *self {
                    LiteralExpr {
                        value: ref __self_0_0,
                    } => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &LiteralExpr) -> bool {
            match *other {
                LiteralExpr {
                    value: ref __self_1_0,
                } => match *self {
                    LiteralExpr {
                        value: ref __self_0_0,
                    } => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl LiteralExpr {
        pub fn new(value: Literal) -> LiteralExpr {
            LiteralExpr { value }
        }
    }
    pub struct BinaryExpr {
        pub left: Box<Expr>,
        pub right: Box<Expr>,
        pub operator: Token,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_BinaryExpr: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for BinaryExpr {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "BinaryExpr",
                    false as usize + 1 + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "left",
                    &self.left,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "right",
                    &self.right,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "operator",
                    &self.operator,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_BinaryExpr: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for BinaryExpr {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 3",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "left" => _serde::export::Ok(__Field::__field0),
                            "right" => _serde::export::Ok(__Field::__field1),
                            "operator" => _serde::export::Ok(__Field::__field2),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"left" => _serde::export::Ok(__Field::__field0),
                            b"right" => _serde::export::Ok(__Field::__field1),
                            b"operator" => _serde::export::Ok(__Field::__field2),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<BinaryExpr>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = BinaryExpr;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct BinaryExpr")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<Box<Expr>>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct BinaryExpr with 3 elements",
                                ));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<Box<Expr>>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct BinaryExpr with 3 elements",
                                ));
                            }
                        };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<Token>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct BinaryExpr with 3 elements",
                                    ));
                                }
                            };
                        _serde::export::Ok(BinaryExpr {
                            left: __field0,
                            right: __field1,
                            operator: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<Box<Expr>> = _serde::export::None;
                        let mut __field1: _serde::export::Option<Box<Expr>> = _serde::export::None;
                        let mut __field2: _serde::export::Option<Token> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "left",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Box<Expr>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "right",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Box<Expr>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "operator",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Token>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("left") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("right") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::export::Some(__field2) => __field2,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("operator") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(BinaryExpr {
                            left: __field0,
                            right: __field1,
                            operator: __field2,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["left", "right", "operator"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "BinaryExpr",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<BinaryExpr>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for BinaryExpr {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                BinaryExpr {
                    left: ref __self_0_0,
                    right: ref __self_0_1,
                    operator: ref __self_0_2,
                } => {
                    let mut debug_trait_builder = f.debug_struct("BinaryExpr");
                    let _ = debug_trait_builder.field("left", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("right", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("operator", &&(*__self_0_2));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for BinaryExpr {
        #[inline]
        fn clone(&self) -> BinaryExpr {
            match *self {
                BinaryExpr {
                    left: ref __self_0_0,
                    right: ref __self_0_1,
                    operator: ref __self_0_2,
                } => BinaryExpr {
                    left: ::std::clone::Clone::clone(&(*__self_0_0)),
                    right: ::std::clone::Clone::clone(&(*__self_0_1)),
                    operator: ::std::clone::Clone::clone(&(*__self_0_2)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for BinaryExpr {
        #[inline]
        fn eq(&self, other: &BinaryExpr) -> bool {
            match *other {
                BinaryExpr {
                    left: ref __self_1_0,
                    right: ref __self_1_1,
                    operator: ref __self_1_2,
                } => match *self {
                    BinaryExpr {
                        left: ref __self_0_0,
                        right: ref __self_0_1,
                        operator: ref __self_0_2,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &BinaryExpr) -> bool {
            match *other {
                BinaryExpr {
                    left: ref __self_1_0,
                    right: ref __self_1_1,
                    operator: ref __self_1_2,
                } => match *self {
                    BinaryExpr {
                        left: ref __self_0_0,
                        right: ref __self_0_1,
                        operator: ref __self_0_2,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                    }
                },
            }
        }
    }
    impl BinaryExpr {
        pub fn new(left: Box<Expr>, right: Box<Expr>, operator: Token) -> BinaryExpr {
            BinaryExpr {
                left,
                right,
                operator,
            }
        }
    }
    pub struct MemberExpr {
        pub object: Box<Expr>,
        pub property: Box<Expr>,
        pub computed: bool,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_MemberExpr: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for MemberExpr {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "MemberExpr",
                    false as usize + 1 + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "object",
                    &self.object,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "property",
                    &self.property,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "computed",
                    &self.computed,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_MemberExpr: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for MemberExpr {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 3",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "object" => _serde::export::Ok(__Field::__field0),
                            "property" => _serde::export::Ok(__Field::__field1),
                            "computed" => _serde::export::Ok(__Field::__field2),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"object" => _serde::export::Ok(__Field::__field0),
                            b"property" => _serde::export::Ok(__Field::__field1),
                            b"computed" => _serde::export::Ok(__Field::__field2),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<MemberExpr>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = MemberExpr;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct MemberExpr")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<Box<Expr>>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct MemberExpr with 3 elements",
                                ));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<Box<Expr>>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct MemberExpr with 3 elements",
                                ));
                            }
                        };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct MemberExpr with 3 elements",
                                    ));
                                }
                            };
                        _serde::export::Ok(MemberExpr {
                            object: __field0,
                            property: __field1,
                            computed: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<Box<Expr>> = _serde::export::None;
                        let mut __field1: _serde::export::Option<Box<Expr>> = _serde::export::None;
                        let mut __field2: _serde::export::Option<bool> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "object",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Box<Expr>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "property",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Box<Expr>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "computed",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<bool>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("object") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("property") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::export::Some(__field2) => __field2,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("computed") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(MemberExpr {
                            object: __field0,
                            property: __field1,
                            computed: __field2,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["object", "property", "computed"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "MemberExpr",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<MemberExpr>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for MemberExpr {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                MemberExpr {
                    object: ref __self_0_0,
                    property: ref __self_0_1,
                    computed: ref __self_0_2,
                } => {
                    let mut debug_trait_builder = f.debug_struct("MemberExpr");
                    let _ = debug_trait_builder.field("object", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("property", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("computed", &&(*__self_0_2));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for MemberExpr {
        #[inline]
        fn clone(&self) -> MemberExpr {
            match *self {
                MemberExpr {
                    object: ref __self_0_0,
                    property: ref __self_0_1,
                    computed: ref __self_0_2,
                } => MemberExpr {
                    object: ::std::clone::Clone::clone(&(*__self_0_0)),
                    property: ::std::clone::Clone::clone(&(*__self_0_1)),
                    computed: ::std::clone::Clone::clone(&(*__self_0_2)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for MemberExpr {
        #[inline]
        fn eq(&self, other: &MemberExpr) -> bool {
            match *other {
                MemberExpr {
                    object: ref __self_1_0,
                    property: ref __self_1_1,
                    computed: ref __self_1_2,
                } => match *self {
                    MemberExpr {
                        object: ref __self_0_0,
                        property: ref __self_0_1,
                        computed: ref __self_0_2,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &MemberExpr) -> bool {
            match *other {
                MemberExpr {
                    object: ref __self_1_0,
                    property: ref __self_1_1,
                    computed: ref __self_1_2,
                } => match *self {
                    MemberExpr {
                        object: ref __self_0_0,
                        property: ref __self_0_1,
                        computed: ref __self_0_2,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                    }
                },
            }
        }
    }
    impl MemberExpr {
        pub fn new(object: Box<Expr>, property: Box<Expr>, computed: bool) -> MemberExpr {
            MemberExpr {
                object,
                property,
                computed,
            }
        }
    }
    pub struct LookupExpr {
        pub token: Token,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_LookupExpr: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for LookupExpr {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "LookupExpr",
                    false as usize + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "token",
                    &self.token,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_LookupExpr: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for LookupExpr {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 1",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "token" => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"token" => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<LookupExpr>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = LookupExpr;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct LookupExpr")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<Token>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct LookupExpr with 1 element",
                                    ));
                                }
                            };
                        _serde::export::Ok(LookupExpr { token: __field0 })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<Token> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "token",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Token>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("token") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(LookupExpr { token: __field0 })
                    }
                }
                const FIELDS: &'static [&'static str] = &["token"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "LookupExpr",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<LookupExpr>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for LookupExpr {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                LookupExpr {
                    token: ref __self_0_0,
                } => {
                    let mut debug_trait_builder = f.debug_struct("LookupExpr");
                    let _ = debug_trait_builder.field("token", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for LookupExpr {
        #[inline]
        fn clone(&self) -> LookupExpr {
            match *self {
                LookupExpr {
                    token: ref __self_0_0,
                } => LookupExpr {
                    token: ::std::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for LookupExpr {
        #[inline]
        fn eq(&self, other: &LookupExpr) -> bool {
            match *other {
                LookupExpr {
                    token: ref __self_1_0,
                } => match *self {
                    LookupExpr {
                        token: ref __self_0_0,
                    } => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &LookupExpr) -> bool {
            match *other {
                LookupExpr {
                    token: ref __self_1_0,
                } => match *self {
                    LookupExpr {
                        token: ref __self_0_0,
                    } => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl LookupExpr {
        pub fn new(token: Token) -> LookupExpr {
            LookupExpr { token }
        }
    }
    pub struct ArgumentsExpr {
        pub expressions: Vec<Box<Expr>>,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_ArgumentsExpr: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ArgumentsExpr {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "ArgumentsExpr",
                    false as usize + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "expressions",
                    &self.expressions,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_ArgumentsExpr: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ArgumentsExpr {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 1",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "expressions" => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"expressions" => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<ArgumentsExpr>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ArgumentsExpr;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct ArgumentsExpr")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            Vec<Box<Expr>>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct ArgumentsExpr with 1 element",
                                ));
                            }
                        };
                        _serde::export::Ok(ArgumentsExpr {
                            expressions: __field0,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<Vec<Box<Expr>>> =
                            _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "expressions",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Vec<Box<Expr>>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("expressions") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(ArgumentsExpr {
                            expressions: __field0,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["expressions"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "ArgumentsExpr",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<ArgumentsExpr>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ArgumentsExpr {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                ArgumentsExpr {
                    expressions: ref __self_0_0,
                } => {
                    let mut debug_trait_builder = f.debug_struct("ArgumentsExpr");
                    let _ = debug_trait_builder.field("expressions", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for ArgumentsExpr {
        #[inline]
        fn clone(&self) -> ArgumentsExpr {
            match *self {
                ArgumentsExpr {
                    expressions: ref __self_0_0,
                } => ArgumentsExpr {
                    expressions: ::std::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for ArgumentsExpr {
        #[inline]
        fn eq(&self, other: &ArgumentsExpr) -> bool {
            match *other {
                ArgumentsExpr {
                    expressions: ref __self_1_0,
                } => match *self {
                    ArgumentsExpr {
                        expressions: ref __self_0_0,
                    } => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &ArgumentsExpr) -> bool {
            match *other {
                ArgumentsExpr {
                    expressions: ref __self_1_0,
                } => match *self {
                    ArgumentsExpr {
                        expressions: ref __self_0_0,
                    } => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl ArgumentsExpr {
        pub fn new(expressions: Vec<Box<Expr>>) -> ArgumentsExpr {
            ArgumentsExpr { expressions }
        }
    }
    pub struct LogicalExpr {
        pub left: Box<Expr>,
        pub right: Box<Expr>,
        pub operator: Token,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_LogicalExpr: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for LogicalExpr {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "LogicalExpr",
                    false as usize + 1 + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "left",
                    &self.left,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "right",
                    &self.right,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "operator",
                    &self.operator,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_LogicalExpr: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for LogicalExpr {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 3",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "left" => _serde::export::Ok(__Field::__field0),
                            "right" => _serde::export::Ok(__Field::__field1),
                            "operator" => _serde::export::Ok(__Field::__field2),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"left" => _serde::export::Ok(__Field::__field0),
                            b"right" => _serde::export::Ok(__Field::__field1),
                            b"operator" => _serde::export::Ok(__Field::__field2),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<LogicalExpr>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = LogicalExpr;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct LogicalExpr")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<Box<Expr>>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct LogicalExpr with 3 elements",
                                ));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<Box<Expr>>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct LogicalExpr with 3 elements",
                                ));
                            }
                        };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<Token>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct LogicalExpr with 3 elements",
                                    ));
                                }
                            };
                        _serde::export::Ok(LogicalExpr {
                            left: __field0,
                            right: __field1,
                            operator: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<Box<Expr>> = _serde::export::None;
                        let mut __field1: _serde::export::Option<Box<Expr>> = _serde::export::None;
                        let mut __field2: _serde::export::Option<Token> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "left",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Box<Expr>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "right",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Box<Expr>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "operator",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Token>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("left") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("right") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::export::Some(__field2) => __field2,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("operator") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(LogicalExpr {
                            left: __field0,
                            right: __field1,
                            operator: __field2,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["left", "right", "operator"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "LogicalExpr",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<LogicalExpr>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for LogicalExpr {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                LogicalExpr {
                    left: ref __self_0_0,
                    right: ref __self_0_1,
                    operator: ref __self_0_2,
                } => {
                    let mut debug_trait_builder = f.debug_struct("LogicalExpr");
                    let _ = debug_trait_builder.field("left", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("right", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("operator", &&(*__self_0_2));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for LogicalExpr {
        #[inline]
        fn clone(&self) -> LogicalExpr {
            match *self {
                LogicalExpr {
                    left: ref __self_0_0,
                    right: ref __self_0_1,
                    operator: ref __self_0_2,
                } => LogicalExpr {
                    left: ::std::clone::Clone::clone(&(*__self_0_0)),
                    right: ::std::clone::Clone::clone(&(*__self_0_1)),
                    operator: ::std::clone::Clone::clone(&(*__self_0_2)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for LogicalExpr {
        #[inline]
        fn eq(&self, other: &LogicalExpr) -> bool {
            match *other {
                LogicalExpr {
                    left: ref __self_1_0,
                    right: ref __self_1_1,
                    operator: ref __self_1_2,
                } => match *self {
                    LogicalExpr {
                        left: ref __self_0_0,
                        right: ref __self_0_1,
                        operator: ref __self_0_2,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &LogicalExpr) -> bool {
            match *other {
                LogicalExpr {
                    left: ref __self_1_0,
                    right: ref __self_1_1,
                    operator: ref __self_1_2,
                } => match *self {
                    LogicalExpr {
                        left: ref __self_0_0,
                        right: ref __self_0_1,
                        operator: ref __self_0_2,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                    }
                },
            }
        }
    }
    impl LogicalExpr {
        pub fn new(left: Box<Expr>, right: Box<Expr>, operator: Token) -> LogicalExpr {
            LogicalExpr {
                left,
                right,
                operator,
            }
        }
    }
}
pub mod parser {
    use self::RuleResult::{Failed, Matched};
    use super::ast::*;
    fn escape_default(s: &str) -> String {
        s.chars().flat_map(|c| c.escape_default()).collect()
    }
    fn char_range_at(s: &str, pos: usize) -> (char, usize) {
        let c = &s[pos..].chars().next().unwrap();
        let next_pos = pos + c.len_utf8();
        (*c, next_pos)
    }
    enum RuleResult<T> {
        Matched(usize, T),
        Failed,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::std::clone::Clone> ::std::clone::Clone for RuleResult<T> {
        #[inline]
        fn clone(&self) -> RuleResult<T> {
            match (&*self,) {
                (&RuleResult::Matched(ref __self_0, ref __self_1),) => RuleResult::Matched(
                    ::std::clone::Clone::clone(&(*__self_0)),
                    ::std::clone::Clone::clone(&(*__self_1)),
                ),
                (&RuleResult::Failed,) => RuleResult::Failed,
            }
        }
    }
    #[structural_match]
    pub struct ParseError {
        pub line: usize,
        pub column: usize,
        pub offset: usize,
        pub expected: ::std::collections::HashSet<&'static str>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for ParseError {
        #[inline]
        fn eq(&self, other: &ParseError) -> bool {
            match *other {
                ParseError {
                    line: ref __self_1_0,
                    column: ref __self_1_1,
                    offset: ref __self_1_2,
                    expected: ref __self_1_3,
                } => match *self {
                    ParseError {
                        line: ref __self_0_0,
                        column: ref __self_0_1,
                        offset: ref __self_0_2,
                        expected: ref __self_0_3,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                            && (*__self_0_3) == (*__self_1_3)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &ParseError) -> bool {
            match *other {
                ParseError {
                    line: ref __self_1_0,
                    column: ref __self_1_1,
                    offset: ref __self_1_2,
                    expected: ref __self_1_3,
                } => match *self {
                    ParseError {
                        line: ref __self_0_0,
                        column: ref __self_0_1,
                        offset: ref __self_0_2,
                        expected: ref __self_0_3,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                            || (*__self_0_3) != (*__self_1_3)
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for ParseError {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<usize>;
                let _: ::std::cmp::AssertParamIsEq<usize>;
                let _: ::std::cmp::AssertParamIsEq<usize>;
                let _: ::std::cmp::AssertParamIsEq<::std::collections::HashSet<&'static str>>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ParseError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                ParseError {
                    line: ref __self_0_0,
                    column: ref __self_0_1,
                    offset: ref __self_0_2,
                    expected: ref __self_0_3,
                } => {
                    let mut debug_trait_builder = f.debug_struct("ParseError");
                    let _ = debug_trait_builder.field("line", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("column", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("offset", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("expected", &&(*__self_0_3));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for ParseError {
        #[inline]
        fn clone(&self) -> ParseError {
            match *self {
                ParseError {
                    line: ref __self_0_0,
                    column: ref __self_0_1,
                    offset: ref __self_0_2,
                    expected: ref __self_0_3,
                } => ParseError {
                    line: ::std::clone::Clone::clone(&(*__self_0_0)),
                    column: ::std::clone::Clone::clone(&(*__self_0_1)),
                    offset: ::std::clone::Clone::clone(&(*__self_0_2)),
                    expected: ::std::clone::Clone::clone(&(*__self_0_3)),
                },
            }
        }
    }
    pub type ParseResult<T> = Result<T, ParseError>;
    impl ::std::fmt::Display for ParseError {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            fmt.write_fmt(::std::fmt::Arguments::new_v1(
                &["error at ", ":", ": expected "],
                &match (&self.line, &self.column) {
                    (arg0, arg1) => [
                        ::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt),
                        ::std::fmt::ArgumentV1::new(arg1, ::std::fmt::Display::fmt),
                    ],
                },
            ))?;
            if self.expected.len() == 0 {
                fmt.write_fmt(::std::fmt::Arguments::new_v1(
                    &["EOF"],
                    &match () {
                        () => [],
                    },
                ))?;
            } else if self.expected.len() == 1 {
                fmt.write_fmt(::std::fmt::Arguments::new_v1(
                    &["`", "`"],
                    &match (&escape_default(self.expected.iter().next().unwrap()),) {
                        (arg0,) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)],
                    },
                ))?;
            } else {
                let mut iter = self.expected.iter();
                fmt.write_fmt(::std::fmt::Arguments::new_v1(
                    &["one of `", "`"],
                    &match (&escape_default(iter.next().unwrap()),) {
                        (arg0,) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)],
                    },
                ))?;
                for elem in iter {
                    fmt.write_fmt(::std::fmt::Arguments::new_v1(
                        &[", `", "`"],
                        &match (&escape_default(elem),) {
                            (arg0,) => {
                                [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)]
                            }
                        },
                    ))?;
                }
            }
            Ok(())
        }
    }
    impl ::std::error::Error for ParseError {
        fn description(&self) -> &str {
            "parse error"
        }
    }
    fn slice_eq(
        input: &str,
        state: &mut ParseState,
        pos: usize,
        m: &'static str,
    ) -> RuleResult<()> {
        #![inline]
        #![allow(dead_code)]
        let l = m.len();
        if input.len() >= pos + l && &input.as_bytes()[pos..pos + l] == m.as_bytes() {
            Matched(pos + l, ())
        } else {
            state.mark_failure(pos, m)
        }
    }
    fn slice_eq_case_insensitive(
        input: &str,
        state: &mut ParseState,
        pos: usize,
        m: &'static str,
    ) -> RuleResult<()> {
        #![inline]
        #![allow(dead_code)]
        let mut used = 0usize;
        let mut input_iter = input[pos..].chars().flat_map(|x| x.to_uppercase());
        for m_char_upper in m.chars().flat_map(|x| x.to_uppercase()) {
            used += m_char_upper.len_utf8();
            let input_char_result = input_iter.next();
            if input_char_result.is_none() || input_char_result.unwrap() != m_char_upper {
                return state.mark_failure(pos, m);
            }
        }
        Matched(pos + used, ())
    }
    fn any_char(input: &str, state: &mut ParseState, pos: usize) -> RuleResult<()> {
        #![inline]
        #![allow(dead_code)]
        if input.len() > pos {
            let (_, next) = char_range_at(input, pos);
            Matched(next, ())
        } else {
            state.mark_failure(pos, "<character>")
        }
    }
    fn pos_to_line(input: &str, pos: usize) -> (usize, usize) {
        let before = &input[..pos];
        let line = before.as_bytes().iter().filter(|&&c| c == b'\n').count() + 1;
        let col = before.chars().rev().take_while(|&c| c != '\n').count() + 1;
        (line, col)
    }
    impl<'input> ParseState<'input> {
        fn mark_failure(&mut self, pos: usize, expected: &'static str) -> RuleResult<()> {
            if self.suppress_fail == 0 {
                if pos > self.max_err_pos {
                    self.max_err_pos = pos;
                    self.expected.clear();
                }
                if pos == self.max_err_pos {
                    self.expected.insert(expected);
                }
            }
            Failed
        }
    }
    struct ParseState<'input> {
        max_err_pos: usize,
        suppress_fail: usize,
        expected: ::std::collections::HashSet<&'static str>,
        _phantom: ::std::marker::PhantomData<&'input ()>,
    }
    impl<'input> ParseState<'input> {
        fn new() -> ParseState<'input> {
            ParseState {
                max_err_pos: 0,
                suppress_fail: 0,
                expected: ::std::collections::HashSet::new(),
                _phantom: ::std::marker::PhantomData,
            }
        }
    }
    fn __parse_program<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Vec<Stmt>> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = __parse_statements(__input, __state, __pos);
            match __seq_res {
                Matched(__pos, s) => Matched(__pos, { s }),
                Failed => Failed,
            }
        }
    }
    fn __parse_statements<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Vec<Stmt>> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = {
                let mut __repeat_pos = __pos;
                let mut __repeat_value = <[_]>::into_vec(box []);
                loop {
                    let __pos = __repeat_pos;
                    let __pos = if __repeat_value.len() > 0 {
                        let __sep_res = __parse___(__input, __state, __pos);
                        match __sep_res {
                            Matched(__newpos, _) => __newpos,
                            Failed => break,
                        }
                    } else {
                        __pos
                    };
                    let __step_res = __parse_statement(__input, __state, __pos);
                    match __step_res {
                        Matched(__newpos, __value) => {
                            __repeat_pos = __newpos;
                            __repeat_value.push(__value);
                        }
                        Failed => {
                            break;
                        }
                    }
                }
                Matched(__repeat_pos, __repeat_value)
            };
            match __seq_res {
                Matched(__pos, s) => Matched(__pos, { s }),
                Failed => Failed,
            }
        }
    }
    fn __parse_statement<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Stmt> {
        #![allow(non_snake_case, unused)]
        __parse_expression_statement(__input, __state, __pos)
    }
    fn __parse_expression_statement<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Stmt> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = __parse__(__input, __state, __pos);
            match __seq_res {
                Matched(__pos, _) => {
                    let __seq_res = __parse_expression(__input, __state, __pos);
                    match __seq_res {
                        Matched(__pos, e) => {
                            let __seq_res = __parse__s(__input, __state, __pos);
                            match __seq_res {
                                Matched(__pos, _) => {
                                    Matched(__pos, { Stmt::Expr(ExprStmt::new(e)) })
                                }
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_expression<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Expr> {
        #![allow(non_snake_case, unused)]
        __parse_literal(__input, __state, __pos)
    }
    fn __parse_literal<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Expr> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = {
                let __choice_res = __parse_literal_boolean(__input, __state, __pos);
                match __choice_res {
                    Matched(__pos, __value) => Matched(__pos, __value),
                    Failed => {
                        let __choice_res = __parse_literal_number(__input, __state, __pos);
                        match __choice_res {
                            Matched(__pos, __value) => Matched(__pos, __value),
                            Failed => __parse_literal_string(__input, __state, __pos),
                        }
                    }
                }
            };
            match __seq_res {
                Matched(__pos, lit) => Matched(__pos, { Expr::Literal(LiteralExpr::new(lit)) }),
                Failed => Failed,
            }
        }
    }
    fn __parse_literal_boolean<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Literal> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = {
                let str_start = __pos;
                match {
                    let __choice_res = slice_eq(__input, __state, __pos, "true");
                    match __choice_res {
                        Matched(__pos, __value) => Matched(__pos, __value),
                        Failed => slice_eq(__input, __state, __pos, "false"),
                    }
                } {
                    Matched(__newpos, _) => Matched(__newpos, &__input[str_start..__newpos]),
                    Failed => Failed,
                }
            };
            match __seq_res {
                Matched(__pos, b) => Matched(__pos, {
                    Literal::Boolean(if b == "true" { true } else { false })
                }),
                Failed => Failed,
            }
        }
    }
    fn __parse_literal_number<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Literal> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = {
                let __choice_res = __parse_double(__input, __state, __pos);
                match __choice_res {
                    Matched(__pos, __value) => Matched(__pos, __value),
                    Failed => __parse_int(__input, __state, __pos),
                }
            };
            match __seq_res {
                Matched(__pos, n) => Matched(__pos, { Literal::Number(n) }),
                Failed => Failed,
            }
        }
    }
    fn __parse_int<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Number> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = {
                let str_start = __pos;
                match {
                    let __choice_res = slice_eq(__input, __state, __pos, "0");
                    match __choice_res {
                        Matched(__pos, __value) => Matched(__pos, __value),
                        Failed => {
                            let __seq_res = if __input.len() > __pos {
                                let (__ch, __next) = char_range_at(__input, __pos);
                                match __ch {
                                    '1'...'9' => Matched(__next, ()),
                                    _ => __state.mark_failure(__pos, "[1-9]"),
                                }
                            } else {
                                __state.mark_failure(__pos, "[1-9]")
                            };
                            match __seq_res {
                                Matched(__pos, _) => {
                                    let mut __repeat_pos = __pos;
                                    loop {
                                        let __pos = __repeat_pos;
                                        let __step_res = if __input.len() > __pos {
                                            let (__ch, __next) = char_range_at(__input, __pos);
                                            match __ch {
                                                '0'...'9' => Matched(__next, ()),
                                                _ => __state.mark_failure(__pos, "[0-9]"),
                                            }
                                        } else {
                                            __state.mark_failure(__pos, "[0-9]")
                                        };
                                        match __step_res {
                                            Matched(__newpos, __value) => {
                                                __repeat_pos = __newpos;
                                            }
                                            Failed => {
                                                break;
                                            }
                                        }
                                    }
                                    Matched(__repeat_pos, ())
                                }
                                Failed => Failed,
                            }
                        }
                    }
                } {
                    Matched(__newpos, _) => Matched(__newpos, &__input[str_start..__newpos]),
                    Failed => Failed,
                }
            };
            match __seq_res {
                Matched(__pos, i) => Matched(__pos, { Number::Integer(i.parse().unwrap()) }),
                Failed => Failed,
            }
        }
    }
    fn __parse_double<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Number> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = {
                let str_start = __pos;
                match {
                    let __seq_res = {
                        let __choice_res = slice_eq(__input, __state, __pos, "0");
                        match __choice_res {
                            Matched(__pos, __value) => Matched(__pos, __value),
                            Failed => {
                                let __seq_res = if __input.len() > __pos {
                                    let (__ch, __next) = char_range_at(__input, __pos);
                                    match __ch {
                                        '1'...'9' => Matched(__next, ()),
                                        _ => __state.mark_failure(__pos, "[1-9]"),
                                    }
                                } else {
                                    __state.mark_failure(__pos, "[1-9]")
                                };
                                match __seq_res {
                                    Matched(__pos, _) => {
                                        let mut __repeat_pos = __pos;
                                        loop {
                                            let __pos = __repeat_pos;
                                            let __step_res = if __input.len() > __pos {
                                                let (__ch, __next) = char_range_at(__input, __pos);
                                                match __ch {
                                                    '0'...'9' => Matched(__next, ()),
                                                    _ => __state.mark_failure(__pos, "[0-9]"),
                                                }
                                            } else {
                                                __state.mark_failure(__pos, "[0-9]")
                                            };
                                            match __step_res {
                                                Matched(__newpos, __value) => {
                                                    __repeat_pos = __newpos;
                                                }
                                                Failed => {
                                                    break;
                                                }
                                            }
                                        }
                                        Matched(__repeat_pos, ())
                                    }
                                    Failed => Failed,
                                }
                            }
                        }
                    };
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = slice_eq(__input, __state, __pos, ".");
                            match __seq_res {
                                Matched(__pos, _) => {
                                    let mut __repeat_pos = __pos;
                                    let mut __repeat_value = <[_]>::into_vec(box []);
                                    loop {
                                        let __pos = __repeat_pos;
                                        let __step_res = if __input.len() > __pos {
                                            let (__ch, __next) = char_range_at(__input, __pos);
                                            match __ch {
                                                '0'...'9' => Matched(__next, ()),
                                                _ => __state.mark_failure(__pos, "[0-9]"),
                                            }
                                        } else {
                                            __state.mark_failure(__pos, "[0-9]")
                                        };
                                        match __step_res {
                                            Matched(__newpos, __value) => {
                                                __repeat_pos = __newpos;
                                                __repeat_value.push(__value);
                                            }
                                            Failed => {
                                                break;
                                            }
                                        }
                                    }
                                    if __repeat_value.len() >= 1 {
                                        Matched(__repeat_pos, ())
                                    } else {
                                        Failed
                                    }
                                }
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                } {
                    Matched(__newpos, _) => Matched(__newpos, &__input[str_start..__newpos]),
                    Failed => Failed,
                }
            };
            match __seq_res {
                Matched(__pos, i) => Matched(__pos, { Number::Double(i.parse().unwrap()) }),
                Failed => Failed,
            }
        }
    }
    fn __parse_literal_string<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Literal> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = slice_eq(__input, __state, __pos, "\"");
            match __seq_res {
                Matched(__pos, _) => {
                    let __seq_res = {
                        let str_start = __pos;
                        match {
                            let mut __repeat_pos = __pos;
                            loop {
                                let __pos = __repeat_pos;
                                let __step_res = {
                                    let __choice_res = __parse_raw_string(__input, __state, __pos);
                                    match __choice_res {
                                        Matched(__pos, __value) => Matched(__pos, __value),
                                        Failed => __parse_escape(__input, __state, __pos),
                                    }
                                };
                                match __step_res {
                                    Matched(__newpos, __value) => {
                                        __repeat_pos = __newpos;
                                    }
                                    Failed => {
                                        break;
                                    }
                                }
                            }
                            Matched(__repeat_pos, ())
                        } {
                            Matched(__newpos, _) => {
                                Matched(__newpos, &__input[str_start..__newpos])
                            }
                            Failed => Failed,
                        }
                    };
                    match __seq_res {
                        Matched(__pos, s) => {
                            let __seq_res = slice_eq(__input, __state, __pos, "\"");
                            match __seq_res {
                                Matched(__pos, _) => {
                                    Matched(__pos, { Literal::String(s.to_string()) })
                                }
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_raw_string<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        {
            let mut __repeat_pos = __pos;
            let mut __repeat_value = <[_]>::into_vec(box []);
            loop {
                let __pos = __repeat_pos;
                let __step_res = {
                    let __seq_res = {
                        __state.suppress_fail += 1;
                        let __assert_res = {
                            let __choice_res = slice_eq(__input, __state, __pos, "\\");
                            match __choice_res {
                                Matched(__pos, __value) => Matched(__pos, __value),
                                Failed => slice_eq(__input, __state, __pos, "\""),
                            }
                        };
                        __state.suppress_fail -= 1;
                        match __assert_res {
                            Failed => Matched(__pos, ()),
                            Matched(..) => Failed,
                        }
                    };
                    match __seq_res {
                        Matched(__pos, _) => any_char(__input, __state, __pos),
                        Failed => Failed,
                    }
                };
                match __step_res {
                    Matched(__newpos, __value) => {
                        __repeat_pos = __newpos;
                        __repeat_value.push(__value);
                    }
                    Failed => {
                        break;
                    }
                }
            }
            if __repeat_value.len() >= 1 {
                Matched(__repeat_pos, ())
            } else {
                Failed
            }
        }
    }
    fn __parse_hex<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        {
            let __choice_res = if __input.len() > __pos {
                let (__ch, __next) = char_range_at(__input, __pos);
                match __ch {
                    '0'...'9' => Matched(__next, ()),
                    _ => __state.mark_failure(__pos, "[0-9]"),
                }
            } else {
                __state.mark_failure(__pos, "[0-9]")
            };
            match __choice_res {
                Matched(__pos, __value) => Matched(__pos, __value),
                Failed => {
                    let __choice_res = if __input.len() > __pos {
                        let (__ch, __next) = char_range_at(__input, __pos);
                        match __ch {
                            'a'...'f' => Matched(__next, ()),
                            _ => __state.mark_failure(__pos, "[a-f]"),
                        }
                    } else {
                        __state.mark_failure(__pos, "[a-f]")
                    };
                    match __choice_res {
                        Matched(__pos, __value) => Matched(__pos, __value),
                        Failed => {
                            if __input.len() > __pos {
                                let (__ch, __next) = char_range_at(__input, __pos);
                                match __ch {
                                    'A'...'F' => Matched(__next, ()),
                                    _ => __state.mark_failure(__pos, "[A-F]"),
                                }
                            } else {
                                __state.mark_failure(__pos, "[A-F]")
                            }
                        }
                    }
                }
            }
        }
    }
    fn __parse_unicode_hex<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        {
            let mut __repeat_pos = __pos;
            let mut __repeat_value = <[_]>::into_vec(box []);
            loop {
                let __pos = __repeat_pos;
                if __repeat_value.len() >= 6 {
                    break;
                }
                let __step_res = __parse_hex(__input, __state, __pos);
                match __step_res {
                    Matched(__newpos, __value) => {
                        __repeat_pos = __newpos;
                        __repeat_value.push(__value);
                    }
                    Failed => {
                        break;
                    }
                }
            }
            if __repeat_value.len() >= 1 {
                Matched(__repeat_pos, ())
            } else {
                Failed
            }
        }
    }
    fn __parse_predefined<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        {
            let __choice_res = slice_eq(__input, __state, __pos, "n");
            match __choice_res {
                Matched(__pos, __value) => Matched(__pos, __value),
                Failed => {
                    let __choice_res = slice_eq(__input, __state, __pos, "r");
                    match __choice_res {
                        Matched(__pos, __value) => Matched(__pos, __value),
                        Failed => {
                            let __choice_res = slice_eq(__input, __state, __pos, "t");
                            match __choice_res {
                                Matched(__pos, __value) => Matched(__pos, __value),
                                Failed => {
                                    let __choice_res = slice_eq(__input, __state, __pos, "\\");
                                    match __choice_res {
                                        Matched(__pos, __value) => Matched(__pos, __value),
                                        Failed => {
                                            let __choice_res =
                                                slice_eq(__input, __state, __pos, "0");
                                            match __choice_res {
                                                Matched(__pos, __value) => Matched(__pos, __value),
                                                Failed => {
                                                    let __choice_res =
                                                        slice_eq(__input, __state, __pos, "\"");
                                                    match __choice_res {
                                                        Matched(__pos, __value) => {
                                                            Matched(__pos, __value)
                                                        }
                                                        Failed => {
                                                            slice_eq(__input, __state, __pos, "\'")
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    fn __parse_byte<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = slice_eq(__input, __state, __pos, "x");
            match __seq_res {
                Matched(__pos, _) => {
                    let mut __repeat_pos = __pos;
                    let mut __repeat_value = <[_]>::into_vec(box []);
                    loop {
                        let __pos = __repeat_pos;
                        if __repeat_value.len() >= 2 {
                            break;
                        }
                        let __step_res = __parse_hex(__input, __state, __pos);
                        match __step_res {
                            Matched(__newpos, __value) => {
                                __repeat_pos = __newpos;
                                __repeat_value.push(__value);
                            }
                            Failed => {
                                break;
                            }
                        }
                    }
                    if __repeat_value.len() >= 2 {
                        Matched(__repeat_pos, ())
                    } else {
                        Failed
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_unicode<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = slice_eq(__input, __state, __pos, "u");
            match __seq_res {
                Matched(__pos, _) => {
                    let __seq_res = slice_eq(__input, __state, __pos, "{");
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = __parse_unicode_hex(__input, __state, __pos);
                            match __seq_res {
                                Matched(__pos, _) => slice_eq(__input, __state, __pos, "}"),
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_escape<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = slice_eq(__input, __state, __pos, "\\");
            match __seq_res {
                Matched(__pos, _) => {
                    let __choice_res = __parse_predefined(__input, __state, __pos);
                    match __choice_res {
                        Matched(__pos, __value) => Matched(__pos, __value),
                        Failed => {
                            let __choice_res = __parse_byte(__input, __state, __pos);
                            match __choice_res {
                                Matched(__pos, __value) => Matched(__pos, __value),
                                Failed => __parse_unicode(__input, __state, __pos),
                            }
                        }
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_char_literal<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = slice_eq(__input, __state, __pos, "\'");
            match __seq_res {
                Matched(__pos, _) => {
                    let __seq_res = {
                        let __choice_res = __parse_escape(__input, __state, __pos);
                        match __choice_res {
                            Matched(__pos, __value) => Matched(__pos, __value),
                            Failed => any_char(__input, __state, __pos),
                        }
                    };
                    match __seq_res {
                        Matched(__pos, _) => slice_eq(__input, __state, __pos, "\'"),
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_whitespace<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        {
            let __choice_res = slice_eq(__input, __state, __pos, "\t");
            match __choice_res {
                Matched(__pos, __value) => Matched(__pos, __value),
                Failed => {
                    let __choice_res = slice_eq(__input, __state, __pos, "\\v");
                    match __choice_res {
                        Matched(__pos, __value) => Matched(__pos, __value),
                        Failed => {
                            let __choice_res = slice_eq(__input, __state, __pos, "\\f");
                            match __choice_res {
                                Matched(__pos, __value) => Matched(__pos, __value),
                                Failed => {
                                    let __choice_res = slice_eq(__input, __state, __pos, " ");
                                    match __choice_res {
                                        Matched(__pos, __value) => Matched(__pos, __value),
                                        Failed => {
                                            let __choice_res =
                                                slice_eq(__input, __state, __pos, "\u{a0}");
                                            match __choice_res {
                                                Matched(__pos, __value) => Matched(__pos, __value),
                                                Failed => {
                                                    slice_eq(__input, __state, __pos, "\u{feff}")
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    fn __parse_line_terminator<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        {
            let __choice_res = slice_eq(__input, __state, __pos, "\n");
            match __choice_res {
                Matched(__pos, __value) => Matched(__pos, __value),
                Failed => {
                    let __choice_res = slice_eq(__input, __state, __pos, "\r");
                    match __choice_res {
                        Matched(__pos, __value) => Matched(__pos, __value),
                        Failed => {
                            let __choice_res = slice_eq(__input, __state, __pos, "\u{2028}");
                            match __choice_res {
                                Matched(__pos, __value) => Matched(__pos, __value),
                                Failed => slice_eq(__input, __state, __pos, "\u{2029}"),
                            }
                        }
                    }
                }
            }
        }
    }
    fn __parse_line_terminator_sequence<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        {
            let __choice_res = slice_eq(__input, __state, __pos, "\n");
            match __choice_res {
                Matched(__pos, __value) => Matched(__pos, __value),
                Failed => {
                    let __choice_res = slice_eq(__input, __state, __pos, "\r\n");
                    match __choice_res {
                        Matched(__pos, __value) => Matched(__pos, __value),
                        Failed => {
                            let __choice_res = slice_eq(__input, __state, __pos, "\r");
                            match __choice_res {
                                Matched(__pos, __value) => Matched(__pos, __value),
                                Failed => {
                                    let __choice_res =
                                        slice_eq(__input, __state, __pos, "\u{2028}");
                                    match __choice_res {
                                        Matched(__pos, __value) => Matched(__pos, __value),
                                        Failed => slice_eq(__input, __state, __pos, "\u{2029}"),
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    fn __parse__<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        {
            let mut __repeat_pos = __pos;
            loop {
                let __pos = __repeat_pos;
                let __step_res = __parse_whitespace(__input, __state, __pos);
                match __step_res {
                    Matched(__newpos, __value) => {
                        __repeat_pos = __newpos;
                    }
                    Failed => {
                        break;
                    }
                }
            }
            Matched(__repeat_pos, ())
        }
    }
    fn __parse___<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        {
            let mut __repeat_pos = __pos;
            loop {
                let __pos = __repeat_pos;
                let __step_res = {
                    let __choice_res = __parse_whitespace(__input, __state, __pos);
                    match __choice_res {
                        Matched(__pos, __value) => Matched(__pos, __value),
                        Failed => __parse_line_terminator_sequence(__input, __state, __pos),
                    }
                };
                match __step_res {
                    Matched(__newpos, __value) => {
                        __repeat_pos = __newpos;
                    }
                    Failed => {
                        break;
                    }
                }
            }
            Matched(__repeat_pos, ())
        }
    }
    fn __parse__s<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = {
                let __choice_res = slice_eq(__input, __state, __pos, "\n");
                match __choice_res {
                    Matched(__pos, __value) => Matched(__pos, __value),
                    Failed => slice_eq(__input, __state, __pos, ";"),
                }
            };
            match __seq_res {
                Matched(__pos, _) => __parse__(__input, __state, __pos),
                Failed => Failed,
            }
        }
    }
    pub fn program<'input>(__input: &'input str) -> ParseResult<Vec<Stmt>> {
        #![allow(non_snake_case, unused)]
        let mut __state = ParseState::new();
        match __parse_program(__input, &mut __state, 0) {
            Matched(__pos, __value) => {
                if __pos == __input.len() {
                    return Ok(__value);
                }
            }
            _ => {}
        }
        let (__line, __col) = pos_to_line(__input, __state.max_err_pos);
        Err(ParseError {
            line: __line,
            column: __col,
            offset: __state.max_err_pos,
            expected: __state.expected,
        })
    }
}
