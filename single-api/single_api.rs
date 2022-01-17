#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use capabilities::service;
use capabilities::capability;
use capabilities::SqliteDb;
use chrono::{Utc, serde::ts_seconds, NaiveDateTime, TimeZone};
use capabilities::capabilities_derive::capabilities;
use actix_web::middleware::Logger;
use serde::Serializer;
use serde::{Deserialize, Serialize};
use actix_web::{get, post, App, HttpServer, HttpResponse, Responder};
use actix_web::web::{self};
#[allow(unused_imports)]
use actix_web_httpauth::middleware::HttpAuthentication;
#[allow(unused_imports)]
use capabilities::{Create, Delete, Update, Read, ReadAll, DeleteAll, UpdateAll};
pub struct BowlsDTO {
    name: String,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for BowlsDTO {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
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
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "name" => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"name" => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<BowlsDTO>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = BowlsDTO;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct BowlsDTO")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 =
                        match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct BowlsDTO with 1 element",
                                ));
                            }
                        };
                    _serde::__private::Ok(BowlsDTO { name: __field0 })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                    while let _serde::__private::Some(__key) =
                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        }
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<String>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => {
                                let _ = match _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)
                                {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("name") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    _serde::__private::Ok(BowlsDTO { name: __field0 })
                }
            }
            const FIELDS: &'static [&'static str] = &["name"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "BowlsDTO",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<BowlsDTO>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
pub struct WaterlevelsDTO {
    #[allow(dead_code)]
    id: i64,
    #[allow(dead_code)]
    waterlevel: i64,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for WaterlevelsDTO {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
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
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "id" => _serde::__private::Ok(__Field::__field0),
                        "waterlevel" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"id" => _serde::__private::Ok(__Field::__field0),
                        b"waterlevel" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<WaterlevelsDTO>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = WaterlevelsDTO;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct WaterlevelsDTO")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 =
                        match match _serde::de::SeqAccess::next_element::<i64>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct WaterlevelsDTO with 2 elements",
                                ));
                            }
                        };
                    let __field1 =
                        match match _serde::de::SeqAccess::next_element::<i64>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct WaterlevelsDTO with 2 elements",
                                ));
                            }
                        };
                    _serde::__private::Ok(WaterlevelsDTO {
                        id: __field0,
                        waterlevel: __field1,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<i64> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<i64> = _serde::__private::None;
                    while let _serde::__private::Some(__key) =
                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        }
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<i64>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "waterlevel",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<i64>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => {
                                let _ = match _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)
                                {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => match _serde::__private::de::missing_field("id")
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("waterlevel") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    _serde::__private::Ok(WaterlevelsDTO {
                        id: __field0,
                        waterlevel: __field1,
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &["id", "waterlevel"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "WaterlevelsDTO",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<WaterlevelsDTO>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
pub struct Bowls {
    id: i64,
    name: String,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Bowls {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "Bowls",
                false as usize + 1 + 1,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "id", &self.id)
            {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "name",
                &self.name,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for Bowls {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Bowls {
                id: ref __self_0_0,
                name: ref __self_0_1,
            } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "Bowls");
                let _ = ::core::fmt::DebugStruct::field(debug_trait_builder, "id", &&(*__self_0_0));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "name", &&(*__self_0_1));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
pub struct BowlsId {
    id: i64,
}
pub trait CapCreateBowls: Capability<Create<Bowls>, Data = Bowls, Error = CapServiceError> {}
impl CapCreateBowls for CapService {}
pub trait CapDeleteBowlsId:
    Capability<Delete<BowlsId>, Data = Bowls, Error = CapServiceError>
{
}
impl CapDeleteBowlsId for CapService {}
pub trait CapDeleteBowls: Capability<Delete<Bowls>, Data = Bowls, Error = CapServiceError> {}
impl CapDeleteBowls for CapService {}
pub struct Waterlevels {
    #[warn(dead_code)]
    id: i64,
    #[serde(serialize_with = "serialize_dt")]
    date: Option<NaiveDateTime>,
    waterlevel: i64,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for Waterlevels {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Waterlevels {
                id: ref __self_0_0,
                date: ref __self_0_1,
                waterlevel: ref __self_0_2,
            } => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_struct(f, "Waterlevels");
                let _ = ::core::fmt::DebugStruct::field(debug_trait_builder, "id", &&(*__self_0_0));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "date", &&(*__self_0_1));
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "waterlevel",
                    &&(*__self_0_2),
                );
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for Waterlevels {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
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
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "id" => _serde::__private::Ok(__Field::__field0),
                        "date" => _serde::__private::Ok(__Field::__field1),
                        "waterlevel" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"id" => _serde::__private::Ok(__Field::__field0),
                        b"date" => _serde::__private::Ok(__Field::__field1),
                        b"waterlevel" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<Waterlevels>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Waterlevels;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct Waterlevels")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 =
                        match match _serde::de::SeqAccess::next_element::<i64>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct Waterlevels with 3 elements",
                                ));
                            }
                        };
                    let __field1 = match match _serde::de::SeqAccess::next_element::<
                        Option<NaiveDateTime>,
                    >(&mut __seq)
                    {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                1usize,
                                &"struct Waterlevels with 3 elements",
                            ));
                        }
                    };
                    let __field2 =
                        match match _serde::de::SeqAccess::next_element::<i64>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct Waterlevels with 3 elements",
                                ));
                            }
                        };
                    _serde::__private::Ok(Waterlevels {
                        id: __field0,
                        date: __field1,
                        waterlevel: __field2,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<i64> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<Option<NaiveDateTime>> =
                        _serde::__private::None;
                    let mut __field2: _serde::__private::Option<i64> = _serde::__private::None;
                    while let _serde::__private::Some(__key) =
                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        }
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<i64>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("date"),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Option<NaiveDateTime>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "waterlevel",
                                        ),
                                    );
                                }
                                __field2 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<i64>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => {
                                let _ = match _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)
                                {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => match _serde::__private::de::missing_field("id")
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("date") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("waterlevel") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    _serde::__private::Ok(Waterlevels {
                        id: __field0,
                        date: __field1,
                        waterlevel: __field2,
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &["id", "date", "waterlevel"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "Waterlevels",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<Waterlevels>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Waterlevels {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "Waterlevels",
                false as usize + 1 + 1 + 1,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "id", &self.id)
            {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "date", {
                struct __SerializeWith<'__a> {
                    values: (&'__a Option<NaiveDateTime>,),
                    phantom: _serde::__private::PhantomData<Waterlevels>,
                }
                impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                    fn serialize<__S>(
                        &self,
                        __s: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        serialize_dt(self.values.0, __s)
                    }
                }
                &__SerializeWith {
                    values: (&self.date,),
                    phantom: _serde::__private::PhantomData::<Waterlevels>,
                }
            }) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "waterlevel",
                &self.waterlevel,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
pub struct WaterlevelsId {
    id: i64,
}
pub trait CapCreateWaterlevels:
    Capability<Create<Waterlevels>, Data = Waterlevels, Error = CapServiceError>
{
}
impl CapCreateWaterlevels for CapService {}
pub trait CapReadWaterlevelsId:
    Capability<Read<WaterlevelsId>, Data = Waterlevels, Error = CapServiceError>
{
}
impl CapReadWaterlevelsId for CapService {}
pub trait CapReadWaterlevels:
    Capability<Read<Waterlevels>, Data = Waterlevels, Error = CapServiceError>
{
}
impl CapReadWaterlevels for CapService {}
pub trait CapDeleteWaterlevelsId:
    Capability<Delete<WaterlevelsId>, Data = Waterlevels, Error = CapServiceError>
{
}
impl CapDeleteWaterlevelsId for CapService {}
pub trait CapDeleteWaterlevels:
    Capability<Delete<Waterlevels>, Data = Waterlevels, Error = CapServiceError>
{
}
impl CapDeleteWaterlevels for CapService {}
use capabilities::EmptyInput;
pub trait CapReadAllWaterlevels:
    Capability<ReadAll<Waterlevels>, Data = Vec<Waterlevels>, Error = CapServiceError>
{
}
impl CapReadAllWaterlevels for CapService {}
pub fn serialize_dt<S>(nt: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let dt = &Utc.from_local_datetime(&nt.unwrap()).unwrap();
    ts_seconds::serialize(dt, serializer)
}
use sqlx::Pool;
use async_trait::async_trait;
pub struct CapService {
    db: SqliteDb,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for CapService {
    #[inline]
    fn clone(&self) -> CapService {
        match *self {
            CapService { db: ref __self_0_0 } => CapService {
                db: ::core::clone::Clone::clone(&(*__self_0_0)),
            },
        }
    }
}
pub struct CapServiceError;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for CapServiceError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            CapServiceError => ::core::fmt::Formatter::write_str(f, "CapServiceError"),
        }
    }
}
impl CapService {
    pub async fn build(conf: String) -> Result<Self, crate::CapServiceError> {
        let con = Pool::connect(&conf)
            .await
            .expect("Failed to connect database");
        Ok(Self { db: con })
    }
}
pub trait Capability<Operation> {
    type Data;
    type Error;
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn perform<'life0, 'async_trait>(
        &'life0 self,
        __arg1: Operation,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<Self::Data, Self::Error>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait;
}
fn main() -> Result<(), std::io::Error> {
    let body = async {
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(
                &["Hello, world!\n"],
                &match () {
                    _args => [],
                },
            ));
        };
        let root = "/api";
        let binding = "0.0.0.0:8080";
        env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
        let con_str = "sqlite:test.db".to_string();
        let service = CapService::build(con_str)
            .await
            .expect("Failed to connect to database");
        { :: sqlx :: migrate :: Migrator { migrations : :: std :: borrow :: Cow :: Borrowed (& [:: sqlx :: migrate :: Migration { version : 20211221095031i64 , description : :: std :: borrow :: Cow :: Borrowed ("create database") , migration_type : :: sqlx :: migrate :: MigrationType :: Simple , sql : :: std :: borrow :: Cow :: Borrowed ("-- Add migration script here\nCREATE TABLE bowls (\n    id integer primary key AUTOINCREMENT,\n    name TEXT not null\n);\n\nCREATE TABLE waterlevels (\n    date DATETIME PRIMARY KEY,\n    id integer not null,\n    waterlevel integer not null\n);") , checksum : :: std :: borrow :: Cow :: Borrowed (& [16u8 , 8u8 , 62u8 , 197u8 , 2u8 , 196u8 , 66u8 , 220u8 , 181u8 , 255u8 , 226u8 , 125u8 , 52u8 , 219u8 , 251u8 , 33u8 , 218u8 , 66u8 , 172u8 , 91u8 , 239u8 , 101u8 , 9u8 , 191u8 , 147u8 , 127u8 , 149u8 , 60u8 , 74u8 , 228u8 , 205u8 , 18u8 , 9u8 , 208u8 , 196u8 , 251u8 , 152u8 , 43u8 , 221u8 , 48u8 , 143u8 , 123u8 , 13u8 , 244u8 , 21u8 , 23u8 , 66u8 , 162u8]) , }]) , ignore_missing : false , } } . run (& service . db) . await . expect ("Failed to run sql mig on database") ;
        HttpServer::new(move || {
            App::new()
                .wrap(Logger::default())
                .service(
                    web::scope(&root)
                        .service(create_new_bowl)
                        .service(get_bowl)
                        .service(get_bowl_waterlevel)
                        .service(add_bowl_waterlevel)
                        .service(get_all_waterlevels),
                )
                .app_data(web::Data::new(service.clone()))
        })
        .bind(binding)?
        .run()
        .await?;
        Ok(())
    };
    #[allow(clippy::expect_used)]
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed building the Runtime")
        .block_on(body)
}
#[allow(non_camel_case_types, missing_docs)]
pub struct create_new_bowl;
impl actix_web::dev::HttpServiceFactory for create_new_bowl {
    fn register(self, __config: &mut actix_web::dev::AppService) {
        pub async fn create_new_bowl(
            json: web::Json<BowlsDTO>,
            svc: web::Data<CapService>,
        ) -> impl Responder {
            let svc = svc.get_ref();
            let newbowl: Bowls = Bowls {
                id: 0,
                name: json.name.to_owned(),
            };
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
                    &["", "\n"],
                    &match (&newbowl,) {
                        _args => [::core::fmt::ArgumentV1::new(
                            _args.0,
                            ::core::fmt::Debug::fmt,
                        )],
                    },
                    &[::core::fmt::rt::v1::Argument {
                        position: 0usize,
                        format: ::core::fmt::rt::v1::FormatSpec {
                            fill: ' ',
                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                            flags: 4u32,
                            precision: ::core::fmt::rt::v1::Count::Implied,
                            width: ::core::fmt::rt::v1::Count::Implied,
                        },
                    }],
                    unsafe { ::core::fmt::UnsafeArg::new() },
                ));
            };
            let res_bowl = create_db_bowl(svc, newbowl).await;
            match res_bowl {
                Ok(bowl) => HttpResponse::Ok().json(bowl),
                _ => HttpResponse::BadRequest().json("{ \"request\": \"bad request\" "),
            }
        }
        let __resource = actix_web::Resource::new("/bowls/")
            .name("create_new_bowl")
            .guard(actix_web::guard::Post())
            .to(create_new_bowl);
        actix_web::dev::HttpServiceFactory::register(__resource, __config)
    }
}
#[allow(non_camel_case_types, missing_docs)]
pub struct get_bowl;
impl actix_web::dev::HttpServiceFactory for get_bowl {
    fn register(self, __config: &mut actix_web::dev::AppService) {
        pub async fn get_bowl(
            _bowl_id: web::Path<String>,
            svc: web::Data<CapService>,
        ) -> impl Responder {
            let _svc = svc.get_ref();
            HttpResponse::Ok().json("Not implemented")
        }
        let __resource = actix_web::Resource::new("/bowls/{id}")
            .name("get_bowl")
            .guard(actix_web::guard::Get())
            .to(get_bowl);
        actix_web::dev::HttpServiceFactory::register(__resource, __config)
    }
}
#[allow(non_camel_case_types, missing_docs)]
pub struct get_bowl_waterlevel;
impl actix_web::dev::HttpServiceFactory for get_bowl_waterlevel {
    fn register(self, __config: &mut actix_web::dev::AppService) {
        pub async fn get_bowl_waterlevel(
            _bowl_id: web::Path<String>,
            _pool: web::Data<CapService>,
        ) -> impl Responder {
            HttpResponse::Ok().body("Not Implemented")
        }
        let __resource = actix_web::Resource::new("/bowls/{id}/waterlevels/")
            .name("get_bowl_waterlevel")
            .guard(actix_web::guard::Get())
            .to(get_bowl_waterlevel);
        actix_web::dev::HttpServiceFactory::register(__resource, __config)
    }
}
#[allow(non_camel_case_types, missing_docs)]
pub struct add_bowl_waterlevel;
impl actix_web::dev::HttpServiceFactory for add_bowl_waterlevel {
    fn register(self, __config: &mut actix_web::dev::AppService) {
        pub async fn add_bowl_waterlevel(
            _bowl_id: web::Path<String>,
            _json: web::Form<WaterlevelsDTO>,
            _pool: web::Data<CapService>,
        ) -> impl Responder {
            HttpResponse::Ok().body("Not Implemented")
        }
        let __resource = actix_web::Resource::new("/bowls/{id}/waterlevels/")
            .name("add_bowl_waterlevel")
            .guard(actix_web::guard::Post())
            .to(add_bowl_waterlevel);
        actix_web::dev::HttpServiceFactory::register(__resource, __config)
    }
}
#[allow(non_camel_case_types, missing_docs)]
pub struct get_all_waterlevels;
impl actix_web::dev::HttpServiceFactory for get_all_waterlevels {
    fn register(self, __config: &mut actix_web::dev::AppService) {
        pub async fn get_all_waterlevels(_pool: web::Data<CapService>) -> impl Responder {
            HttpResponse::Ok().body("Not Implemented")
        }
        let __resource = actix_web::Resource::new("/bowls/waterlevels/")
            .name("get_all_waterlevels")
            .guard(actix_web::guard::Get())
            .to(get_all_waterlevels);
        actix_web::dev::HttpServiceFactory::register(__resource, __config)
    }
}
pub async fn create_db_bowl<Service>(
    service: &Service,
    param: Bowls,
) -> Result<Bowls, CapServiceError>
where
    Service: CapCreateBowls,
{
    service
        .perform(::capabilities::Create { data: param })
        .await
}
impl Capability<Create<Bowls>> for CapService {
    type Data = Bowls;
    type Error = CapServiceError;
    #[allow(
        clippy::let_unit_value,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn perform<'life0, 'async_trait>(
        &'life0 self,
        action: Create<Bowls>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<Self::Data, Self::Error>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) =
                ::core::option::Option::None::<Result<Self::Data, Self::Error>>
            {
                return __ret;
            }
            let __self = self;
            let action = action;
            let __ret: Result<Self::Data, Self::Error> = {
                let bowl: Bowls = action.data;
                {
                    let _res = { { # [allow (clippy :: all)] { use :: sqlx :: Arguments as _ ; let arg0 = & (bowl . name) ; let mut query_args = < sqlx :: sqlite :: Sqlite as :: sqlx :: database :: HasArguments > :: Arguments :: default () ; query_args . reserve (1usize , 0 + :: sqlx :: encode :: Encode :: < sqlx :: sqlite :: Sqlite > :: size_hint (arg0)) ; query_args . add (arg0) ; :: sqlx :: query_with :: < sqlx :: sqlite :: Sqlite , _ > ("INSERT INTO bowls (name) VALUES ($1)" , query_args) } } } . execute (& __self . db) . await . expect ("unable to create bowl") ;
                    let b = { { # [allow (clippy :: all)] { use :: sqlx :: Arguments as _ ; let arg0 = & (bowl . name) ; let mut query_args = < sqlx :: sqlite :: Sqlite as :: sqlx :: database :: HasArguments > :: Arguments :: default () ; query_args . reserve (1usize , 0 + :: sqlx :: encode :: Encode :: < sqlx :: sqlite :: Sqlite > :: size_hint (arg0)) ; query_args . add (arg0) ; :: sqlx :: query_with :: < sqlx :: sqlite :: Sqlite , _ > ("SELECT * FROM bowls WHERE name = $1" , query_args) . try_map (| row : sqlx :: sqlite :: SqliteRow | { use :: sqlx :: Row as _ ; let sqlx_query_as_id = row . try_get_unchecked :: < i64 , _ > (0usize) ? ; let sqlx_query_as_name = row . try_get_unchecked :: < String , _ > (1usize) ? ; Ok (Bowls { id : sqlx_query_as_id , name : sqlx_query_as_name , }) }) } } } . fetch_one (& __self . db) . await . expect ("Didn't fint any bowls") ;
                    Ok(b)
                }
            };
            #[allow(unreachable_code)]
            __ret
        })
    }
}
pub async fn delete_db_bowl<Service>(
    service: &Service,
    param: Bowls,
) -> Result<Bowls, CapServiceError>
where
    Service: CapDeleteBowls,
{
    service
        .perform(::capabilities::Delete { data: param })
        .await
}
impl Capability<Delete<Bowls>> for CapService {
    type Data = Bowls;
    type Error = CapServiceError;
    #[allow(
        clippy::let_unit_value,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn perform<'life0, 'async_trait>(
        &'life0 self,
        action: Delete<Bowls>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<Self::Data, Self::Error>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) =
                ::core::option::Option::None::<Result<Self::Data, Self::Error>>
            {
                return __ret;
            }
            let __self = self;
            let action = action;
            let __ret: Result<Self::Data, Self::Error> = {
                let bowl: Bowls = action.data;
                {
                    let _res = { { # [allow (clippy :: all)] { use :: sqlx :: Arguments as _ ; let arg0 = & (bowl . name) ; let mut query_args = < sqlx :: sqlite :: Sqlite as :: sqlx :: database :: HasArguments > :: Arguments :: default () ; query_args . reserve (1usize , 0 + :: sqlx :: encode :: Encode :: < sqlx :: sqlite :: Sqlite > :: size_hint (arg0)) ; query_args . add (arg0) ; :: sqlx :: query_with :: < sqlx :: sqlite :: Sqlite , _ > ("DELETE FROM bowls WHERE name = $1" , query_args) } } } . execute (& __self . db) . await . expect ("unable to delete bowl") ;
                    Ok(Bowls {
                        id: 0,
                        name: bowl.name,
                    })
                }
            };
            #[allow(unreachable_code)]
            __ret
        })
    }
}
pub async fn delete_db_bowl_by_id<Service>(
    service: &Service,
    param: BowlsId,
) -> Result<Bowls, CapServiceError>
where
    Service: CapDeleteBowlsId,
{
    service
        .perform(::capabilities::Delete { data: param })
        .await
}
impl Capability<Delete<BowlsId>> for CapService {
    type Data = Bowls;
    type Error = CapServiceError;
    #[allow(
        clippy::let_unit_value,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn perform<'life0, 'async_trait>(
        &'life0 self,
        action: Delete<BowlsId>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<Self::Data, Self::Error>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) =
                ::core::option::Option::None::<Result<Self::Data, Self::Error>>
            {
                return __ret;
            }
            let __self = self;
            let action = action;
            let __ret: Result<Self::Data, Self::Error> = {
                let bowl_id: BowlsId = action.data;
                {
                    let _res = { { # [allow (clippy :: all)] { use :: sqlx :: Arguments as _ ; let arg0 = & (bowl_id . id) ; let mut query_args = < sqlx :: sqlite :: Sqlite as :: sqlx :: database :: HasArguments > :: Arguments :: default () ; query_args . reserve (1usize , 0 + :: sqlx :: encode :: Encode :: < sqlx :: sqlite :: Sqlite > :: size_hint (arg0)) ; query_args . add (arg0) ; :: sqlx :: query_with :: < sqlx :: sqlite :: Sqlite , _ > ("DELETE FROM bowls WHERE id = $1" , query_args) } } } . execute (& __self . db) . await . expect ("unable to delete bowl") ;
                    Ok(Bowls {
                        id: bowl_id.id,
                        name: "DELETED".to_string(),
                    })
                }
            };
            #[allow(unreachable_code)]
            __ret
        })
    }
}
pub async fn create_db_waterlevels<Service>(
    service: &Service,
    param: Waterlevels,
) -> Result<Waterlevels, CapServiceError>
where
    Service: CapCreateWaterlevels,
{
    service
        .perform(::capabilities::Create { data: param })
        .await
}
impl Capability<Create<Waterlevels>> for CapService {
    type Data = Waterlevels;
    type Error = CapServiceError;
    #[allow(
        clippy::let_unit_value,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn perform<'life0, 'async_trait>(
        &'life0 self,
        action: Create<Waterlevels>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<Self::Data, Self::Error>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) =
                ::core::option::Option::None::<Result<Self::Data, Self::Error>>
            {
                return __ret;
            }
            let __self = self;
            let action = action;
            let __ret: Result<Self::Data, Self::Error> = {
                let waterlevel: Waterlevels = action.data;
                {
                    { { # [allow (clippy :: all)] { use :: sqlx :: Arguments as _ ; let arg0 = & (waterlevel . id) ; let arg1 = & (waterlevel . date) ; let arg2 = & (waterlevel . waterlevel) ; let mut query_args = < sqlx :: sqlite :: Sqlite as :: sqlx :: database :: HasArguments > :: Arguments :: default () ; query_args . reserve (3usize , 0 + :: sqlx :: encode :: Encode :: < sqlx :: sqlite :: Sqlite > :: size_hint (arg0) + :: sqlx :: encode :: Encode :: < sqlx :: sqlite :: Sqlite > :: size_hint (arg1) + :: sqlx :: encode :: Encode :: < sqlx :: sqlite :: Sqlite > :: size_hint (arg2)) ; query_args . add (arg0) ; query_args . add (arg1) ; query_args . add (arg2) ; :: sqlx :: query_with :: < sqlx :: sqlite :: Sqlite , _ > ("INSERT INTO waterlevels (id, date, waterlevel) VALUES ($1, $2, $3)" , query_args) } } } . execute (& __self . db) . await . expect ("Failed to insert to database") ;
                    Ok(waterlevel)
                }
            };
            #[allow(unreachable_code)]
            __ret
        })
    }
}
pub async fn get_db_waterlevel_by_id<Service>(
    service: &Service,
    param: WaterlevelsId,
) -> Result<Waterlevels, CapServiceError>
where
    Service: CapReadWaterlevelsId,
{
    service.perform(::capabilities::Read { data: param }).await
}
impl Capability<Read<WaterlevelsId>> for CapService {
    type Data = Waterlevels;
    type Error = CapServiceError;
    #[allow(
        clippy::let_unit_value,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn perform<'life0, 'async_trait>(
        &'life0 self,
        action: Read<WaterlevelsId>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<Self::Data, Self::Error>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) =
                ::core::option::Option::None::<Result<Self::Data, Self::Error>>
            {
                return __ret;
            }
            let __self = self;
            let action = action;
            let __ret: Result<Self::Data, Self::Error> = {
                let waterlevel_id: WaterlevelsId = action.data;
                {
                    let waterlevel = { { # [allow (clippy :: all)] { use :: sqlx :: Arguments as _ ; let arg0 = & (waterlevel_id . id) ; let mut query_args = < sqlx :: sqlite :: Sqlite as :: sqlx :: database :: HasArguments > :: Arguments :: default () ; query_args . reserve (1usize , 0 + :: sqlx :: encode :: Encode :: < sqlx :: sqlite :: Sqlite > :: size_hint (arg0)) ; query_args . add (arg0) ; :: sqlx :: query_with :: < sqlx :: sqlite :: Sqlite , _ > ("SELECT * FROM waterlevels WHERE id = $1" , query_args) . try_map (| row : sqlx :: sqlite :: SqliteRow | { use :: sqlx :: Row as _ ; let sqlx_query_as_date = row . try_get_unchecked :: < :: std :: option :: Option < sqlx :: types :: chrono :: NaiveDateTime > , _ > (0usize) ? ; let sqlx_query_as_id = row . try_get_unchecked :: < i64 , _ > (1usize) ? ; let sqlx_query_as_waterlevel = row . try_get_unchecked :: < i64 , _ > (2usize) ? ; Ok (Waterlevels { date : sqlx_query_as_date , id : sqlx_query_as_id , waterlevel : sqlx_query_as_waterlevel , }) }) } } } . fetch_one (& __self . db) . await . expect ({ let res = :: alloc :: fmt :: format (:: core :: fmt :: Arguments :: new_v1 (& ["Failed to fetch bowl with id: "] , & match (& waterlevel_id . id ,) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Display :: fmt)] , })) ; res } . as_str ()) ;
                    Ok(waterlevel)
                }
            };
            #[allow(unreachable_code)]
            __ret
        })
    }
}
pub async fn get_db_waterlevel<Service>(
    service: &Service,
    param: Waterlevels,
) -> Result<Waterlevels, CapServiceError>
where
    Service: CapReadWaterlevels,
{
    service.perform(::capabilities::Read { data: param }).await
}
impl Capability<Read<Waterlevels>> for CapService {
    type Data = Waterlevels;
    type Error = CapServiceError;
    #[allow(
        clippy::let_unit_value,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn perform<'life0, 'async_trait>(
        &'life0 self,
        action: Read<Waterlevels>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<Self::Data, Self::Error>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) =
                ::core::option::Option::None::<Result<Self::Data, Self::Error>>
            {
                return __ret;
            }
            let __self = self;
            let action = action;
            let __ret: Result<Self::Data, Self::Error> = {
                let waterlevel: Waterlevels = action.data;
                {
                    let bowl = { { # [allow (clippy :: all)] { use :: sqlx :: Arguments as _ ; let arg0 = & (waterlevel . date) ; let mut query_args = < sqlx :: sqlite :: Sqlite as :: sqlx :: database :: HasArguments > :: Arguments :: default () ; query_args . reserve (1usize , 0 + :: sqlx :: encode :: Encode :: < sqlx :: sqlite :: Sqlite > :: size_hint (arg0)) ; query_args . add (arg0) ; :: sqlx :: query_with :: < sqlx :: sqlite :: Sqlite , _ > ("SELECT * FROM waterlevels WHERE date = $1" , query_args) . try_map (| row : sqlx :: sqlite :: SqliteRow | { use :: sqlx :: Row as _ ; let sqlx_query_as_date = row . try_get_unchecked :: < :: std :: option :: Option < sqlx :: types :: chrono :: NaiveDateTime > , _ > (0usize) ? ; let sqlx_query_as_id = row . try_get_unchecked :: < i64 , _ > (1usize) ? ; let sqlx_query_as_waterlevel = row . try_get_unchecked :: < i64 , _ > (2usize) ? ; Ok (Waterlevels { date : sqlx_query_as_date , id : sqlx_query_as_id , waterlevel : sqlx_query_as_waterlevel , }) }) } } } . fetch_one (& __self . db) . await . expect ({ let res = :: alloc :: fmt :: format (:: core :: fmt :: Arguments :: new_v1 (& ["Failed to fetch bowl with id: "] , & match (& waterlevel . id ,) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Display :: fmt)] , })) ; res } . as_str ()) ;
                    Ok(bowl)
                }
            };
            #[allow(unreachable_code)]
            __ret
        })
    }
}
pub async fn get_db_all_waterlevels<Service>(
    service: &Service,
    param: Waterlevels,
) -> Result<Vec<Waterlevels>, CapServiceError>
where
    Service: CapReadAllWaterlevels,
{
    service
        .perform(::capabilities::ReadAll { data: param })
        .await
}
impl Capability<ReadAll<Waterlevels>> for CapService {
    type Data = Vec<Waterlevels>;
    type Error = CapServiceError;
    #[allow(
        clippy::let_unit_value,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn perform<'life0, 'async_trait>(
        &'life0 self,
        action: ReadAll<Waterlevels>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<Self::Data, Self::Error>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) =
                ::core::option::Option::None::<Result<Self::Data, Self::Error>>
            {
                return __ret;
            }
            let __self = self;
            let action = action;
            let __ret: Result<Self::Data, Self::Error> = {
                {
                    let waterlevels : Vec < Waterlevels > = { { # [allow (clippy :: all)] { use :: sqlx :: Arguments as _ ; let query_args = < sqlx :: sqlite :: Sqlite as :: sqlx :: database :: HasArguments > :: Arguments :: default () ; :: sqlx :: query_with :: < sqlx :: sqlite :: Sqlite , _ > ("SELECT * FROM waterlevels" , query_args) . try_map (| row : sqlx :: sqlite :: SqliteRow | { use :: sqlx :: Row as _ ; let sqlx_query_as_date = row . try_get_unchecked :: < :: std :: option :: Option < sqlx :: types :: chrono :: NaiveDateTime > , _ > (0usize) ? ; let sqlx_query_as_id = row . try_get_unchecked :: < i64 , _ > (1usize) ? ; let sqlx_query_as_waterlevel = row . try_get_unchecked :: < i64 , _ > (2usize) ? ; Ok (Waterlevels { date : sqlx_query_as_date , id : sqlx_query_as_id , waterlevel : sqlx_query_as_waterlevel , }) }) } } } . fetch_all (& __self . db) . await . expect ("Failed to query database for all bowls") ;
                    Ok(waterlevels)
                }
            };
            #[allow(unreachable_code)]
            __ret
        })
    }
}
pub async fn delete_db_waterlevel<Service>(
    service: &Service,
    param: Waterlevels,
) -> Result<Waterlevels, CapServiceError>
where
    Service: CapDeleteWaterlevels,
{
    service
        .perform(::capabilities::Delete { data: param })
        .await
}
impl Capability<Delete<Waterlevels>> for CapService {
    type Data = Waterlevels;
    type Error = CapServiceError;
    #[allow(
        clippy::let_unit_value,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn perform<'life0, 'async_trait>(
        &'life0 self,
        action: Delete<Waterlevels>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<Self::Data, Self::Error>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) =
                ::core::option::Option::None::<Result<Self::Data, Self::Error>>
            {
                return __ret;
            }
            let __self = self;
            let action = action;
            let __ret: Result<Self::Data, Self::Error> = {
                let waterlevel: Waterlevels = action.data;
                {
                    let _res = { { # [allow (clippy :: all)] { use :: sqlx :: Arguments as _ ; let arg0 = & (waterlevel . date) ; let mut query_args = < sqlx :: sqlite :: Sqlite as :: sqlx :: database :: HasArguments > :: Arguments :: default () ; query_args . reserve (1usize , 0 + :: sqlx :: encode :: Encode :: < sqlx :: sqlite :: Sqlite > :: size_hint (arg0)) ; query_args . add (arg0) ; :: sqlx :: query_with :: < sqlx :: sqlite :: Sqlite , _ > ("DELETE FROM waterlevels WHERE date = $1" , query_args) } } } . execute (& __self . db) . await . expect ("Failed to delete") ;
                    Ok(Waterlevels {
                        id: waterlevel.id,
                        date: waterlevel.date,
                        waterlevel: waterlevel.waterlevel,
                    })
                }
            };
            #[allow(unreachable_code)]
            __ret
        })
    }
}
pub async fn delete_db_waterlevel_by_id<Service>(
    service: &Service,
    param: WaterlevelsId,
) -> Result<Waterlevels, CapServiceError>
where
    Service: CapDeleteWaterlevelsId,
{
    service
        .perform(::capabilities::Delete { data: param })
        .await
}
impl Capability<Delete<WaterlevelsId>> for CapService {
    type Data = Waterlevels;
    type Error = CapServiceError;
    #[allow(
        clippy::let_unit_value,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn perform<'life0, 'async_trait>(
        &'life0 self,
        action: Delete<WaterlevelsId>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<Self::Data, Self::Error>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) =
                ::core::option::Option::None::<Result<Self::Data, Self::Error>>
            {
                return __ret;
            }
            let __self = self;
            let action = action;
            let __ret: Result<Self::Data, Self::Error> = {
                let waterlevel: WaterlevelsId = action.data;
                {
                    let _res = { { # [allow (clippy :: all)] { use :: sqlx :: Arguments as _ ; let arg0 = & (waterlevel . id) ; let mut query_args = < sqlx :: sqlite :: Sqlite as :: sqlx :: database :: HasArguments > :: Arguments :: default () ; query_args . reserve (1usize , 0 + :: sqlx :: encode :: Encode :: < sqlx :: sqlite :: Sqlite > :: size_hint (arg0)) ; query_args . add (arg0) ; :: sqlx :: query_with :: < sqlx :: sqlite :: Sqlite , _ > ("DELETE FROM waterlevels WHERE id = $1" , query_args) } } } . execute (& __self . db) . await . expect ("Failed to delete") ;
                    let time = Utc::now().to_string();
                    let nt = NaiveDateTime::parse_from_str(&time, "%m-%d-%Y %H:%M:%S")
                        .expect("parsed not ok");
                    Ok(Waterlevels {
                        id: waterlevel.id,
                        date: Some(nt),
                        waterlevel: 0,
                    })
                }
            };
            #[allow(unreachable_code)]
            __ret
        })
    }
}
