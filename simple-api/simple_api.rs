#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use actix_web::middleware::Logger;
use actix_web::web::self;
use actix_web::{get, post, delete, App, HttpResponse, HttpServer, Responder};
#[allow(unused_imports)]
use actix_web_httpauth::middleware::HttpAuthentication;
use capabilities::capabilities_derive::capabilities;
use capabilities::service;
use capabilities::SqliteDb;
use capabilities::{capability, token_introspection};
#[allow(unused_imports)]
use capabilities::{Create, Delete, DeleteAll, Read, ReadAll, Update, UpdateAll};
use chrono::{serde::ts_seconds, NaiveDateTime, TimeZone, Utc};
use gnap_cli::GnapClient;
use serde::Serializer;
use serde::{Deserialize, Serialize};
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
                        "waterlevel" => _serde::__private::Ok(__Field::__field0),
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
                        b"waterlevel" => _serde::__private::Ok(__Field::__field0),
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
                                    &"struct WaterlevelsDTO with 1 element",
                                ));
                            }
                        };
                    _serde::__private::Ok(WaterlevelsDTO {
                        waterlevel: __field0,
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
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "waterlevel",
                                        ),
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
                            match _serde::__private::de::missing_field("waterlevel") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    _serde::__private::Ok(WaterlevelsDTO {
                        waterlevel: __field0,
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &["waterlevel"];
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
pub struct Bowl {
    id: i64,
    name: String,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Bowl {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "Bowl",
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
impl ::core::fmt::Debug for Bowl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Bowl {
                id: ref __self_0_0,
                name: ref __self_0_1,
            } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "Bowl");
                let _ = ::core::fmt::DebugStruct::field(debug_trait_builder, "id", &&(*__self_0_0));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "name", &&(*__self_0_1));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
pub struct BowlId {
    id: i64,
}
pub trait CapCreateBowl:
    CapabilityTrait<Create<Bowl>, Data = Bowl, Error = CapServiceError>
{
}
impl CapCreateBowl for CapService {}
impl CapToEnum for Create<Bowl> {
    fn into_enum(&self) -> Capability {
        Capability::Create
    }
}
pub trait CapReadBowlId:
    CapabilityTrait<Read<BowlId>, Data = Bowl, Error = CapServiceError>
{
}
impl CapReadBowlId for CapService {}
pub trait CapReadBowl: CapabilityTrait<Read<Bowl>, Data = Bowl, Error = CapServiceError> {}
impl CapReadBowl for CapService {}
impl CapToEnum for Read<Bowl> {
    fn into_enum(&self) -> ::capabilities::Capability {
        ::capabilities::Capability::Read
    }
}
impl CapToEnum for Read<BowlId> {
    fn into_enum(&self) -> ::capabilities::Capability {
        ::capabilities::Capability::Read
    }
}
pub trait CapDeleteBowlId:
    CapabilityTrait<Delete<BowlId>, Data = (), Error = CapServiceError>
{
}
impl CapDeleteBowlId for CapService {}
pub trait CapDeleteBowl: CapabilityTrait<Delete<Bowl>, Data = (), Error = CapServiceError> {}
impl CapDeleteBowl for CapService {}
impl CapToEnum for Delete<Bowl> {
    fn into_enum(&self) -> Capability {
        Capability::Delete
    }
}
impl CapToEnum for Delete<BowlId> {
    fn into_enum(&self) -> Capability {
        Capability::Delete
    }
}
pub struct Waterlevel {
    id: i64,
    bowl_id: i64,
    #[serde(serialize_with = "serialize_dt")]
    date: Option<NaiveDateTime>,
    waterlevel: i64,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for Waterlevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Waterlevel {
                id: ref __self_0_0,
                bowl_id: ref __self_0_1,
                date: ref __self_0_2,
                waterlevel: ref __self_0_3,
            } => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_struct(f, "Waterlevel");
                let _ = ::core::fmt::DebugStruct::field(debug_trait_builder, "id", &&(*__self_0_0));
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "bowl_id",
                    &&(*__self_0_1),
                );
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "date", &&(*__self_0_2));
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "waterlevel",
                    &&(*__self_0_3),
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
    impl<'de> _serde::Deserialize<'de> for Waterlevel {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
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
                        3u64 => _serde::__private::Ok(__Field::__field3),
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
                        "bowl_id" => _serde::__private::Ok(__Field::__field1),
                        "date" => _serde::__private::Ok(__Field::__field2),
                        "waterlevel" => _serde::__private::Ok(__Field::__field3),
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
                        b"bowl_id" => _serde::__private::Ok(__Field::__field1),
                        b"date" => _serde::__private::Ok(__Field::__field2),
                        b"waterlevel" => _serde::__private::Ok(__Field::__field3),
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
                marker: _serde::__private::PhantomData<Waterlevel>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Waterlevel;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct Waterlevel")
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
                                    &"struct Waterlevel with 4 elements",
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
                                    &"struct Waterlevel with 4 elements",
                                ));
                            }
                        };
                    let __field2 = match match _serde::de::SeqAccess::next_element::<
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
                                2usize,
                                &"struct Waterlevel with 4 elements",
                            ));
                        }
                    };
                    let __field3 =
                        match match _serde::de::SeqAccess::next_element::<i64>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    3usize,
                                    &"struct Waterlevel with 4 elements",
                                ));
                            }
                        };
                    _serde::__private::Ok(Waterlevel {
                        id: __field0,
                        bowl_id: __field1,
                        date: __field2,
                        waterlevel: __field3,
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
                    let mut __field2: _serde::__private::Option<Option<NaiveDateTime>> =
                        _serde::__private::None;
                    let mut __field3: _serde::__private::Option<i64> = _serde::__private::None;
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
                                            "bowl_id",
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
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("date"),
                                    );
                                }
                                __field2 = _serde::__private::Some(
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
                            __Field::__field3 => {
                                if _serde::__private::Option::is_some(&__field3) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "waterlevel",
                                        ),
                                    );
                                }
                                __field3 = _serde::__private::Some(
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
                            match _serde::__private::de::missing_field("bowl_id") {
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
                            match _serde::__private::de::missing_field("date") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field3 = match __field3 {
                        _serde::__private::Some(__field3) => __field3,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("waterlevel") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    _serde::__private::Ok(Waterlevel {
                        id: __field0,
                        bowl_id: __field1,
                        date: __field2,
                        waterlevel: __field3,
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &["id", "bowl_id", "date", "waterlevel"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "Waterlevel",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<Waterlevel>,
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
    impl _serde::Serialize for Waterlevel {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "Waterlevel",
                false as usize + 1 + 1 + 1 + 1,
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
                "bowl_id",
                &self.bowl_id,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "date", {
                struct __SerializeWith<'__a> {
                    values: (&'__a Option<NaiveDateTime>,),
                    phantom: _serde::__private::PhantomData<Waterlevel>,
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
                    phantom: _serde::__private::PhantomData::<Waterlevel>,
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
pub struct WaterlevelId {
    id: i64,
}
pub trait CapCreateWaterlevel:
    CapabilityTrait<Create<Waterlevel>, Data = Waterlevel, Error = CapServiceError>
{
}
impl CapCreateWaterlevel for CapService {}
impl CapToEnum for Create<Waterlevel> {
    fn into_enum(&self) -> Capability {
        Capability::Create
    }
}
pub trait CapReadWaterlevelId:
    CapabilityTrait<Read<WaterlevelId>, Data = Waterlevel, Error = CapServiceError>
{
}
impl CapReadWaterlevelId for CapService {}
pub trait CapReadWaterlevel:
    CapabilityTrait<Read<Waterlevel>, Data = Waterlevel, Error = CapServiceError>
{
}
impl CapReadWaterlevel for CapService {}
impl CapToEnum for Read<Waterlevel> {
    fn into_enum(&self) -> ::capabilities::Capability {
        ::capabilities::Capability::Read
    }
}
impl CapToEnum for Read<WaterlevelId> {
    fn into_enum(&self) -> ::capabilities::Capability {
        ::capabilities::Capability::Read
    }
}
pub trait CapDeleteWaterlevelId:
    CapabilityTrait<Delete<WaterlevelId>, Data = (), Error = CapServiceError>
{
}
impl CapDeleteWaterlevelId for CapService {}
pub trait CapDeleteWaterlevel:
    CapabilityTrait<Delete<Waterlevel>, Data = (), Error = CapServiceError>
{
}
impl CapDeleteWaterlevel for CapService {}
impl CapToEnum for Delete<Waterlevel> {
    fn into_enum(&self) -> Capability {
        Capability::Delete
    }
}
impl CapToEnum for Delete<WaterlevelId> {
    fn into_enum(&self) -> Capability {
        Capability::Delete
    }
}
use capabilities::EmptyInput;
pub trait CapReadAllWaterlevel:
    CapabilityTrait<ReadAll<Vec<Waterlevel>>, Data = Vec<Waterlevel>, Error = CapServiceError>
{
}
impl CapReadAllWaterlevel for CapService {}
impl CapToEnum for ReadAll<Vec<Waterlevel>> {
    fn into_enum(&self) -> Capability {
        Capability::ReadAll
    }
}
pub fn serialize_dt<S>(nt: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let dt = &Utc.from_local_datetime(&nt.unwrap()).unwrap();
    ts_seconds::serialize(dt, serializer)
}
use sqlx::Pool;
use async_trait::async_trait;
use ::capabilities::Capability;
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
pub trait CapabilityTrait<Operation> {
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
pub trait CapToEnum {
    fn into_enum(&self) -> Capability;
}
fn main() -> Result<(), std::io::Error> {
    < :: actix_web :: rt :: System > :: new () . block_on (async move { { :: std :: io :: _print (:: core :: fmt :: Arguments :: new_v1 (& ["Hello, world!\n"] , & [])) ; let binding = "0.0.0.0:8080" ; env_logger :: init_from_env (env_logger :: Env :: new () . default_filter_or ("debug")) ; let con_str = "sqlite:test.db" . to_string () ; let service = CapService :: build (con_str) . await . expect ("Failed to connect to database") ; { :: sqlx :: migrate :: Migrator { migrations : :: std :: borrow :: Cow :: Borrowed (& [:: sqlx :: migrate :: Migration { version : 20211221095031i64 , description : :: std :: borrow :: Cow :: Borrowed ("create database") , migration_type : :: sqlx :: migrate :: MigrationType :: Simple , sql : :: std :: borrow :: Cow :: Borrowed ("-- Add migration script here\nCREATE TABLE bowls (\n    id INTEGER PRIMARY KEY AUTOINCREMENT,\n    name TEXT NOT NULL\n);\n\nCREATE TABLE waterlevels (\n    id INTEGER PRIMARY KEY AUTOINCREMENT,\n    bowl_id INTEGER NOT NULL,\n    date DATETIME,\n    waterlevel INTEGER NOT NULL\n);") , checksum : :: std :: borrow :: Cow :: Borrowed (& [254u8 , 41u8 , 85u8 , 253u8 , 102u8 , 106u8 , 146u8 , 210u8 , 148u8 , 210u8 , 46u8 , 96u8 , 148u8 , 226u8 , 64u8 , 150u8 , 47u8 , 20u8 , 86u8 , 57u8 , 61u8 , 162u8 , 70u8 , 171u8 , 249u8 , 245u8 , 218u8 , 153u8 , 229u8 , 109u8 , 165u8 , 148u8 , 175u8 , 83u8 , 125u8 , 31u8 , 136u8 , 241u8 , 13u8 , 45u8 , 54u8 , 25u8 , 221u8 , 24u8 , 102u8 , 27u8 , 56u8 , 230u8]) , }]) , ignore_missing : false , } } . run (& service . db) . await . expect ("Failed to run sql mig on database") ; let rs_ref = "e8a2968a-f183-45a3-b63d-4bbbd1dad276" . to_string () ; let basepath = "http://localhost:8000/gnap" . to_string () ; let gnap_client = GnapClient :: build (basepath , rs_ref) ; let bearer_auth = HttpAuthentication :: bearer (token_introspection) ; HttpServer :: new (move | | { App :: new () . app_data (gnap_client . clone ()) . wrap (bearer_auth . clone ()) . wrap (Logger :: default ()) . service (create_new_bowl) . service (get_bowl_by_id) . service (add_bowl_waterlevel) . service (get_all_bowl_waterlevels) . service (delete_bowl_waterlevels_by_id) . app_data (web :: Data :: new (service . clone ())) }) . bind (binding) ? . run () . await ? ; Ok (()) } })
}
#[allow(non_camel_case_types, missing_docs)]
pub struct create_new_bowl;
impl ::actix_web::dev::HttpServiceFactory for create_new_bowl {
    fn register(self, __config: &mut actix_web::dev::AppService) {
        pub async fn create_new_bowl(
            json: web::Json<BowlsDTO>,
            svc: web::Data<CapService>,
            cap: Capability,
        ) -> impl Responder {
            let svc = svc.get_ref();
            let newbowl: Bowl = Bowl {
                id: 0,
                name: json.name.to_owned(),
            };
            ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
                &["", "\n"],
                &[::core::fmt::ArgumentV1::new_debug(&newbowl)],
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
            ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
                &["Cap: ", "\n"],
                &[::core::fmt::ArgumentV1::new_debug(&cap)],
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
            match create_db_bowl(svc, newbowl, cap).await {
                Ok(bowl) => HttpResponse::Ok().json(bowl),
                _ => HttpResponse::BadRequest().json("{ \"request\": \"bad request\" "),
            }
        }
        let __resource = ::actix_web::Resource::new("/bowls/")
            .name("create_new_bowl")
            .guard(::actix_web::guard::Post())
            .to(create_new_bowl);
        ::actix_web::dev::HttpServiceFactory::register(__resource, __config)
    }
}
#[allow(non_camel_case_types, missing_docs)]
pub struct get_bowl_by_id;
impl ::actix_web::dev::HttpServiceFactory for get_bowl_by_id {
    fn register(self, __config: &mut actix_web::dev::AppService) {
        pub async fn get_bowl_by_id(
            bowl_id: web::Path<String>,
            svc: web::Data<CapService>,
            cap: Capability,
        ) -> impl Responder {
            let svc = svc.get_ref();
            let id = bowl_id.into_inner();
            let bowl_id = BowlId {
                id: id.parse::<i64>().unwrap(),
            };
            ::std::io::_print(::core::fmt::Arguments::new_v1(
                &["Finding: ", "\n"],
                &[::core::fmt::ArgumentV1::new_debug(&id)],
            ));
            ::std::io::_print(::core::fmt::Arguments::new_v1(
                &["Cap: ", "\n"],
                &[::core::fmt::ArgumentV1::new_debug(&cap)],
            ));
            match read_db_bowl_by_id(svc, bowl_id, cap).await {
                Ok(bowl) => HttpResponse::Ok().json(bowl),
                _ => HttpResponse::NoContent().json("{ msg : no content } "),
            }
        }
        let __resource = ::actix_web::Resource::new("/bowls/{id}")
            .name("get_bowl_by_id")
            .guard(::actix_web::guard::Get())
            .to(get_bowl_by_id);
        ::actix_web::dev::HttpServiceFactory::register(__resource, __config)
    }
}
#[allow(non_camel_case_types, missing_docs)]
pub struct add_bowl_waterlevel;
impl ::actix_web::dev::HttpServiceFactory for add_bowl_waterlevel {
    fn register(self, __config: &mut actix_web::dev::AppService) {
        pub async fn add_bowl_waterlevel(
            bowl_id: web::Path<String>,
            json: web::Json<WaterlevelsDTO>,
            svc: web::Data<CapService>,
            cap: Capability,
        ) -> impl Responder {
            let svc = svc.get_ref();
            let bowl_id = bowl_id.parse::<i64>().unwrap();
            let json = json.into_inner();
            let date: NaiveDateTime = Utc::now().naive_utc();
            let waterlevel = Waterlevel {
                id: 0,
                bowl_id,
                waterlevel: json.waterlevel,
                date: Some(date),
            };
            ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
                &["", "\n"],
                &[::core::fmt::ArgumentV1::new_debug(&cap)],
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
            match create_db_waterlevels(svc, waterlevel, cap).await {
                Ok(d) => HttpResponse::Ok().json(d),
                _ => HttpResponse::BadRequest().json("{ msg : bad request } "),
            }
        }
        let __resource = ::actix_web::Resource::new("/waterlevels/{id}")
            .name("add_bowl_waterlevel")
            .guard(::actix_web::guard::Post())
            .to(add_bowl_waterlevel);
        ::actix_web::dev::HttpServiceFactory::register(__resource, __config)
    }
}
#[allow(non_camel_case_types, missing_docs)]
pub struct get_all_bowl_waterlevels;
impl ::actix_web::dev::HttpServiceFactory for get_all_bowl_waterlevels {
    fn register(self, __config: &mut actix_web::dev::AppService) {
        pub async fn get_all_bowl_waterlevels(
            bowl_id: web::Path<String>,
            svc: web::Data<CapService>,
            cap: Capability,
        ) -> impl Responder {
            let svc = svc.get_ref();
            let bowl_id = WaterlevelId {
                id: bowl_id.parse::<i64>().unwrap(),
            };
            match read_db_waterlevel_by_id(svc, bowl_id, cap).await {
                Ok(d) => HttpResponse::Ok().json(d),
                _ => HttpResponse::Forbidden().body("no access"),
            }
        }
        let __resource = ::actix_web::Resource::new("/waterlevels/{id}")
            .name("get_all_bowl_waterlevels")
            .guard(::actix_web::guard::Get())
            .to(get_all_bowl_waterlevels);
        ::actix_web::dev::HttpServiceFactory::register(__resource, __config)
    }
}
#[allow(non_camel_case_types, missing_docs)]
pub struct delete_bowl_waterlevels_by_id;
impl ::actix_web::dev::HttpServiceFactory for delete_bowl_waterlevels_by_id {
    fn register(self, __config: &mut actix_web::dev::AppService) {
        pub async fn delete_bowl_waterlevels_by_id(
            bowl_id: web::Path<String>,
            svc: web::Data<CapService>,
            cap: Capability,
        ) -> impl Responder {
            let svc = svc.get_ref();
            let bowl_id = WaterlevelId {
                id: bowl_id.parse::<i64>().unwrap(),
            };
            match delete_db_waterlevel_by_id(svc, bowl_id, cap).await {
                Ok(_) => HttpResponse::Ok().json("success"),
                _ => HttpResponse::Forbidden().json("no access"),
            }
        }
        let __resource = ::actix_web::Resource::new("/waterlevels/{id}")
            .name("delete_bowl_waterlevels_by_id")
            .guard(::actix_web::guard::Delete())
            .to(delete_bowl_waterlevels_by_id);
        ::actix_web::dev::HttpServiceFactory::register(__resource, __config)
    }
}
pub async fn create_db_bowl<Service>(
    service: &Service,
    param: Bowl,
    cap: ::capabilities::Capability,
) -> Result<Bowl, CapServiceError>
where
    Service: CapCreateBowl,
{
    let valid = ::capabilities::Create { data: param };
    if valid.into_enum().eq(&cap) {
        service.perform(valid).await
    } else {
        Err(CapServiceError)
    }
}
impl CapabilityTrait<Create<Bowl>> for CapService {
    type Data = Bowl;
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
        action: Create<Bowl>,
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
                let bowl: Bowl = action.data;
                {
                    let _res = { { # [allow (clippy :: all)] { use :: sqlx :: Arguments as _ ; let arg0 = & (bowl . name) ; let mut query_args = < sqlx :: sqlite :: Sqlite as :: sqlx :: database :: HasArguments > :: Arguments :: default () ; query_args . reserve (1usize , 0 + :: sqlx :: encode :: Encode :: < sqlx :: sqlite :: Sqlite > :: size_hint (arg0)) ; query_args . add (arg0) ; :: sqlx :: query_with :: < sqlx :: sqlite :: Sqlite , _ > ("INSERT INTO bowls (name) VALUES ($1)" , query_args) } } } . execute (& __self . db) . await . expect ("unable to create bowl") ;
                    let b = { { # [allow (clippy :: all)] { use :: sqlx :: Arguments as _ ; let arg0 = & (bowl . name) ; let mut query_args = < sqlx :: sqlite :: Sqlite as :: sqlx :: database :: HasArguments > :: Arguments :: default () ; query_args . reserve (1usize , 0 + :: sqlx :: encode :: Encode :: < sqlx :: sqlite :: Sqlite > :: size_hint (arg0)) ; query_args . add (arg0) ; :: sqlx :: query_with :: < sqlx :: sqlite :: Sqlite , _ > ("SELECT * FROM bowls WHERE name = $1" , query_args) . try_map (| row : sqlx :: sqlite :: SqliteRow | { use :: sqlx :: Row as _ ; let sqlx_query_as_id = row . try_get_unchecked :: < i64 , _ > (0usize) ? ; let sqlx_query_as_name = row . try_get_unchecked :: < String , _ > (1usize) ? ; Ok (Bowl { id : sqlx_query_as_id , name : sqlx_query_as_name }) }) } } } . fetch_one (& __self . db) . await . expect ("Didn't fint any bowls") ;
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
    param: Bowl,
    cap: ::capabilities::Capability,
) -> Result<(), CapServiceError>
where
    Service: CapDeleteBowl,
{
    let valid = ::capabilities::Delete { data: param };
    if valid.into_enum().eq(&cap) {
        service.perform(valid).await
    } else {
        Err(CapServiceError)
    }
}
impl CapabilityTrait<Delete<Bowl>> for CapService {
    type Data = ();
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
        action: Delete<Bowl>,
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
                let bowl = action.data;
                {
                    match { { # [allow (clippy :: all)] { use :: sqlx :: Arguments as _ ; let arg0 = & (bowl . name) ; let mut query_args = < sqlx :: sqlite :: Sqlite as :: sqlx :: database :: HasArguments > :: Arguments :: default () ; query_args . reserve (1usize , 0 + :: sqlx :: encode :: Encode :: < sqlx :: sqlite :: Sqlite > :: size_hint (arg0)) ; query_args . add (arg0) ; :: sqlx :: query_with :: < sqlx :: sqlite :: Sqlite , _ > ("DELETE FROM bowls WHERE name = $1" , query_args) } } } . execute (& __self . db) . await { Ok (_) => Ok (()) , Err (_) => Err (CapServiceError) , }
                }
            };
            #[allow(unreachable_code)]
            __ret
        })
    }
}
pub async fn delete_db_bowl_by_id<Service>(
    service: &Service,
    param: BowlId,
    cap: ::capabilities::Capability,
) -> Result<(), CapServiceError>
where
    Service: CapDeleteBowlId,
{
    let valid = ::capabilities::Delete { data: param };
    if valid.into_enum().eq(&cap) {
        service.perform(valid).await
    } else {
        Err(CapServiceError)
    }
}
impl CapabilityTrait<Delete<BowlId>> for CapService {
    type Data = ();
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
        action: Delete<BowlId>,
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
                let bowl_id = action.data;
                {
                    match { { # [allow (clippy :: all)] { use :: sqlx :: Arguments as _ ; let arg0 = & (bowl_id . id) ; let mut query_args = < sqlx :: sqlite :: Sqlite as :: sqlx :: database :: HasArguments > :: Arguments :: default () ; query_args . reserve (1usize , 0 + :: sqlx :: encode :: Encode :: < sqlx :: sqlite :: Sqlite > :: size_hint (arg0)) ; query_args . add (arg0) ; :: sqlx :: query_with :: < sqlx :: sqlite :: Sqlite , _ > ("DELETE FROM bowls WHERE id = $1" , query_args) } } } . execute (& __self . db) . await { Ok (_) => Ok (()) , Err (_) => Err (CapServiceError) , }
                }
            };
            #[allow(unreachable_code)]
            __ret
        })
    }
}
pub async fn create_db_waterlevels<Service>(
    service: &Service,
    param: Waterlevel,
    cap: ::capabilities::Capability,
) -> Result<Waterlevel, CapServiceError>
where
    Service: CapCreateWaterlevel,
{
    let valid = ::capabilities::Create { data: param };
    if valid.into_enum().eq(&cap) {
        service.perform(valid).await
    } else {
        Err(CapServiceError)
    }
}
impl CapabilityTrait<Create<Waterlevel>> for CapService {
    type Data = Waterlevel;
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
        action: Create<Waterlevel>,
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
                let waterlevel: Waterlevel = action.data;
                {
                    { { # [allow (clippy :: all)] { use :: sqlx :: Arguments as _ ; let arg0 = & (waterlevel . bowl_id) ; let arg1 = & (waterlevel . date) ; let arg2 = & (waterlevel . waterlevel) ; let mut query_args = < sqlx :: sqlite :: Sqlite as :: sqlx :: database :: HasArguments > :: Arguments :: default () ; query_args . reserve (3usize , 0 + :: sqlx :: encode :: Encode :: < sqlx :: sqlite :: Sqlite > :: size_hint (arg0) + :: sqlx :: encode :: Encode :: < sqlx :: sqlite :: Sqlite > :: size_hint (arg1) + :: sqlx :: encode :: Encode :: < sqlx :: sqlite :: Sqlite > :: size_hint (arg2)) ; query_args . add (arg0) ; query_args . add (arg1) ; query_args . add (arg2) ; :: sqlx :: query_with :: < sqlx :: sqlite :: Sqlite , _ > ("INSERT INTO waterlevels (bowl_id, date, waterlevel) VALUES ($1, $2, $3)" , query_args) } } } . execute (& __self . db) . await . expect ("Failed to insert to database") ;
                    Ok(waterlevel)
                }
            };
            #[allow(unreachable_code)]
            __ret
        })
    }
}
pub async fn read_db_waterlevel_by_id<Service>(
    service: &Service,
    param: WaterlevelId,
    cap: ::capabilities::Capability,
) -> Result<Waterlevel, CapServiceError>
where
    Service: CapReadWaterlevelId,
{
    let valid = ::capabilities::Read { data: param };
    if valid.into_enum().eq(&cap) {
        service.perform(valid).await
    } else {
        Err(CapServiceError)
    }
}
impl CapabilityTrait<Read<WaterlevelId>> for CapService {
    type Data = Waterlevel;
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
        action: Read<WaterlevelId>,
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
                let waterlevel_id: WaterlevelId = action.data;
                {
                    let waterlevel = { { # [allow (clippy :: all)] { use :: sqlx :: Arguments as _ ; let arg0 = & (waterlevel_id . id) ; let mut query_args = < sqlx :: sqlite :: Sqlite as :: sqlx :: database :: HasArguments > :: Arguments :: default () ; query_args . reserve (1usize , 0 + :: sqlx :: encode :: Encode :: < sqlx :: sqlite :: Sqlite > :: size_hint (arg0)) ; query_args . add (arg0) ; :: sqlx :: query_with :: < sqlx :: sqlite :: Sqlite , _ > ("SELECT * FROM waterlevels WHERE id = $1" , query_args) . try_map (| row : sqlx :: sqlite :: SqliteRow | { use :: sqlx :: Row as _ ; let sqlx_query_as_id = row . try_get_unchecked :: < i64 , _ > (0usize) ? ; let sqlx_query_as_bowl_id = row . try_get_unchecked :: < i64 , _ > (1usize) ? ; let sqlx_query_as_date = row . try_get_unchecked :: < :: std :: option :: Option < sqlx :: types :: chrono :: NaiveDateTime > , _ > (2usize) ? ; let sqlx_query_as_waterlevel = row . try_get_unchecked :: < i64 , _ > (3usize) ? ; Ok (Waterlevel { id : sqlx_query_as_id , bowl_id : sqlx_query_as_bowl_id , date : sqlx_query_as_date , waterlevel : sqlx_query_as_waterlevel , }) }) } } } . fetch_one (& __self . db) . await . unwrap_or_else (| _ | :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Failed to fetch bowl with id: "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& waterlevel_id . id)]))) ;
                    Ok(waterlevel)
                }
            };
            #[allow(unreachable_code)]
            __ret
        })
    }
}
pub async fn read_db_waterlevel<Service>(
    service: &Service,
    param: Waterlevel,
    cap: ::capabilities::Capability,
) -> Result<Waterlevel, CapServiceError>
where
    Service: CapReadWaterlevel,
{
    let valid = ::capabilities::Read { data: param };
    if valid.into_enum().eq(&cap) {
        service.perform(valid).await
    } else {
        Err(CapServiceError)
    }
}
impl CapabilityTrait<Read<Waterlevel>> for CapService {
    type Data = Waterlevel;
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
        action: Read<Waterlevel>,
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
                let waterlevel: Waterlevel = action.data;
                {
                    let bowl = { { # [allow (clippy :: all)] { use :: sqlx :: Arguments as _ ; let arg0 = & (waterlevel . id) ; let mut query_args = < sqlx :: sqlite :: Sqlite as :: sqlx :: database :: HasArguments > :: Arguments :: default () ; query_args . reserve (1usize , 0 + :: sqlx :: encode :: Encode :: < sqlx :: sqlite :: Sqlite > :: size_hint (arg0)) ; query_args . add (arg0) ; :: sqlx :: query_with :: < sqlx :: sqlite :: Sqlite , _ > ("SELECT * FROM waterlevels WHERE id = $1" , query_args) . try_map (| row : sqlx :: sqlite :: SqliteRow | { use :: sqlx :: Row as _ ; let sqlx_query_as_id = row . try_get_unchecked :: < i64 , _ > (0usize) ? ; let sqlx_query_as_bowl_id = row . try_get_unchecked :: < i64 , _ > (1usize) ? ; let sqlx_query_as_date = row . try_get_unchecked :: < :: std :: option :: Option < sqlx :: types :: chrono :: NaiveDateTime > , _ > (2usize) ? ; let sqlx_query_as_waterlevel = row . try_get_unchecked :: < i64 , _ > (3usize) ? ; Ok (Waterlevel { id : sqlx_query_as_id , bowl_id : sqlx_query_as_bowl_id , date : sqlx_query_as_date , waterlevel : sqlx_query_as_waterlevel , }) }) } } } . fetch_one (& __self . db) . await . unwrap_or_else (| _ | :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Failed to fetch bowl with id: "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& waterlevel . id)]))) ;
                    Ok(bowl)
                }
            };
            #[allow(unreachable_code)]
            __ret
        })
    }
}
pub async fn read_db_all_waterlevels<Service>(
    service: &Service,
    cap: ::capabilities::Capability,
) -> Result<Vec<Waterlevel>, CapServiceError>
where
    Service: CapReadAllWaterlevel,
{
    let param: Vec<Waterlevel> = Vec::<Waterlevel>::new();
    let valid = ::capabilities::ReadAll { data: param };
    if valid.into_enum().eq(&cap) {
        service.perform(valid).await
    } else {
        Err(CapServiceError)
    }
}
impl CapabilityTrait<ReadAll<Vec<Waterlevel>>> for CapService {
    type Data = Vec<Waterlevel>;
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
        action: ReadAll<Vec<Waterlevel>>,
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
                    let waterlevels : Vec < Waterlevel > = { { # [allow (clippy :: all)] { use :: sqlx :: Arguments as _ ; let query_args = < sqlx :: sqlite :: Sqlite as :: sqlx :: database :: HasArguments > :: Arguments :: default () ; :: sqlx :: query_with :: < sqlx :: sqlite :: Sqlite , _ > ("SELECT * FROM waterlevels" , query_args) . try_map (| row : sqlx :: sqlite :: SqliteRow | { use :: sqlx :: Row as _ ; let sqlx_query_as_id = row . try_get_unchecked :: < i64 , _ > (0usize) ? ; let sqlx_query_as_bowl_id = row . try_get_unchecked :: < i64 , _ > (1usize) ? ; let sqlx_query_as_date = row . try_get_unchecked :: < :: std :: option :: Option < sqlx :: types :: chrono :: NaiveDateTime > , _ > (2usize) ? ; let sqlx_query_as_waterlevel = row . try_get_unchecked :: < i64 , _ > (3usize) ? ; Ok (Waterlevel { id : sqlx_query_as_id , bowl_id : sqlx_query_as_bowl_id , date : sqlx_query_as_date , waterlevel : sqlx_query_as_waterlevel , }) }) } } } . fetch_all (& __self . db) . await . expect ("Failed to query database for all bowls") ;
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
    param: Waterlevel,
    cap: ::capabilities::Capability,
) -> Result<(), CapServiceError>
where
    Service: CapDeleteWaterlevel,
{
    let valid = ::capabilities::Delete { data: param };
    if valid.into_enum().eq(&cap) {
        service.perform(valid).await
    } else {
        Err(CapServiceError)
    }
}
impl CapabilityTrait<Delete<Waterlevel>> for CapService {
    type Data = ();
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
        action: Delete<Waterlevel>,
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
                let waterlevel = action.data;
                {
                    match { { # [allow (clippy :: all)] { use :: sqlx :: Arguments as _ ; let arg0 = & (waterlevel . id) ; let mut query_args = < sqlx :: sqlite :: Sqlite as :: sqlx :: database :: HasArguments > :: Arguments :: default () ; query_args . reserve (1usize , 0 + :: sqlx :: encode :: Encode :: < sqlx :: sqlite :: Sqlite > :: size_hint (arg0)) ; query_args . add (arg0) ; :: sqlx :: query_with :: < sqlx :: sqlite :: Sqlite , _ > ("DELETE FROM waterlevels WHERE id = $1" , query_args) } } } . execute (& __self . db) . await { Ok (_) => Ok (()) , Err (_) => Err (CapServiceError) , }
                }
            };
            #[allow(unreachable_code)]
            __ret
        })
    }
}
pub async fn delete_db_waterlevel_by_id<Service>(
    service: &Service,
    param: WaterlevelId,
    cap: ::capabilities::Capability,
) -> Result<(), CapServiceError>
where
    Service: CapDeleteWaterlevelId,
{
    let valid = ::capabilities::Delete { data: param };
    if valid.into_enum().eq(&cap) {
        service.perform(valid).await
    } else {
        Err(CapServiceError)
    }
}
impl CapabilityTrait<Delete<WaterlevelId>> for CapService {
    type Data = ();
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
        action: Delete<WaterlevelId>,
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
                let waterlevel = action.data;
                {
                    match { { # [allow (clippy :: all)] { use :: sqlx :: Arguments as _ ; let arg0 = & (waterlevel . id) ; let mut query_args = < sqlx :: sqlite :: Sqlite as :: sqlx :: database :: HasArguments > :: Arguments :: default () ; query_args . reserve (1usize , 0 + :: sqlx :: encode :: Encode :: < sqlx :: sqlite :: Sqlite > :: size_hint (arg0)) ; query_args . add (arg0) ; :: sqlx :: query_with :: < sqlx :: sqlite :: Sqlite , _ > ("DELETE FROM waterlevels WHERE id = $1" , query_args) } } } . execute (& __self . db) . await { Ok (_) => Ok (()) , Err (_) => Err (CapServiceError) , }
                }
            };
            #[allow(unreachable_code)]
            __ret
        })
    }
}
pub async fn read_db_bowl_by_id<Service>(
    service: &Service,
    param: BowlId,
    cap: ::capabilities::Capability,
) -> Result<Bowl, CapServiceError>
where
    Service: CapReadBowlId,
{
    let valid = ::capabilities::Read { data: param };
    if valid.into_enum().eq(&cap) {
        service.perform(valid).await
    } else {
        Err(CapServiceError)
    }
}
impl CapabilityTrait<Read<BowlId>> for CapService {
    type Data = Bowl;
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
        action: Read<BowlId>,
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
                let bowl_id: BowlId = action.data;
                {
                    let b = { { # [allow (clippy :: all)] { use :: sqlx :: Arguments as _ ; let arg0 = & (bowl_id . id) ; let mut query_args = < sqlx :: sqlite :: Sqlite as :: sqlx :: database :: HasArguments > :: Arguments :: default () ; query_args . reserve (1usize , 0 + :: sqlx :: encode :: Encode :: < sqlx :: sqlite :: Sqlite > :: size_hint (arg0)) ; query_args . add (arg0) ; :: sqlx :: query_with :: < sqlx :: sqlite :: Sqlite , _ > ("SELECT * FROM bowls WHERE id = $1" , query_args) . try_map (| row : sqlx :: sqlite :: SqliteRow | { use :: sqlx :: Row as _ ; let sqlx_query_as_id = row . try_get_unchecked :: < i64 , _ > (0usize) ? ; let sqlx_query_as_name = row . try_get_unchecked :: < String , _ > (1usize) ? ; Ok (Bowl { id : sqlx_query_as_id , name : sqlx_query_as_name }) }) } } } . fetch_one (& __self . db) . await . expect ("Failed to get a bowl") ;
                    Ok(b)
                }
            };
            #[allow(unreachable_code)]
            __ret
        })
    }
}
pub async fn read_db_bowl<Service>(
    service: &Service,
    param: Bowl,
    cap: ::capabilities::Capability,
) -> Result<Bowl, CapServiceError>
where
    Service: CapReadBowl,
{
    let valid = ::capabilities::Read { data: param };
    if valid.into_enum().eq(&cap) {
        service.perform(valid).await
    } else {
        Err(CapServiceError)
    }
}
impl CapabilityTrait<Read<Bowl>> for CapService {
    type Data = Bowl;
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
        action: Read<Bowl>,
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
                let bowl: Bowl = action.data;
                {
                    let b = { { # [allow (clippy :: all)] { use :: sqlx :: Arguments as _ ; let arg0 = & (bowl . name) ; let mut query_args = < sqlx :: sqlite :: Sqlite as :: sqlx :: database :: HasArguments > :: Arguments :: default () ; query_args . reserve (1usize , 0 + :: sqlx :: encode :: Encode :: < sqlx :: sqlite :: Sqlite > :: size_hint (arg0)) ; query_args . add (arg0) ; :: sqlx :: query_with :: < sqlx :: sqlite :: Sqlite , _ > ("SELECT * FROM bowls WHERE name = $1" , query_args) . try_map (| row : sqlx :: sqlite :: SqliteRow | { use :: sqlx :: Row as _ ; let sqlx_query_as_id = row . try_get_unchecked :: < i64 , _ > (0usize) ? ; let sqlx_query_as_name = row . try_get_unchecked :: < String , _ > (1usize) ? ; Ok (Bowl { id : sqlx_query_as_id , name : sqlx_query_as_name }) }) } } } . fetch_one (& __self . db) . await . expect ("Failed to get a bowl") ;
                    Ok(b)
                }
            };
            #[allow(unreachable_code)]
            __ret
        })
    }
}
