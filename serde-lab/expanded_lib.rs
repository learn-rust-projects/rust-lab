#![feature(prelude_import)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
pub mod snazzy {
    pub mod items {
        /// A snazzy new shirt!
        pub struct Shirt {
            /// The base color
            #[prost(string, tag = "1")]
            pub color: ::prost::alloc::string::String,
            /// The size as stated on the label
            #[prost(enumeration = "shirt::Size", tag = "2")]
            pub size: i32,
            #[prost(enumeration = "shirt::Size", tag = "3")]
            pub size1: i32,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Shirt {
            #[inline]
            fn clone(&self) -> Shirt {
                Shirt {
                    color: ::core::clone::Clone::clone(&self.color),
                    size: ::core::clone::Clone::clone(&self.size),
                    size1: ::core::clone::Clone::clone(&self.size1),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Shirt {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Shirt {
            #[inline]
            fn eq(&self, other: &Shirt) -> bool {
                self.size == other.size && self.size1 == other.size1
                    && self.color == other.color
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Shirt {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) {
                let _: ::core::cmp::AssertParamIsEq<::prost::alloc::string::String>;
                let _: ::core::cmp::AssertParamIsEq<i32>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Shirt {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
                ::core::hash::Hash::hash(&self.color, state);
                ::core::hash::Hash::hash(&self.size, state);
                ::core::hash::Hash::hash(&self.size1, state)
            }
        }
        impl ::prost::Message for Shirt {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                if self.color != "" {
                    ::prost::encoding::string::encode(1u32, &self.color, buf);
                }
                if self.size != shirt::Size::default() as i32 {
                    ::prost::encoding::int32::encode(2u32, &self.size, buf);
                }
                if self.size1 != shirt::Size::default() as i32 {
                    ::prost::encoding::int32::encode(3u32, &self.size1, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::wire_type::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                const STRUCT_NAME: &'static str = "Shirt";
                match tag {
                    1u32 => {
                        let mut value = &mut self.color;
                        ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "color");
                                error
                            })
                    }
                    2u32 => {
                        let mut value = &mut self.size;
                        ::prost::encoding::int32::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "size");
                                error
                            })
                    }
                    3u32 => {
                        let mut value = &mut self.size1;
                        ::prost::encoding::int32::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "size1");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + if self.color != "" {
                        ::prost::encoding::string::encoded_len(1u32, &self.color)
                    } else {
                        0
                    }
                    + if self.size != shirt::Size::default() as i32 {
                        ::prost::encoding::int32::encoded_len(2u32, &self.size)
                    } else {
                        0
                    }
                    + if self.size1 != shirt::Size::default() as i32 {
                        ::prost::encoding::int32::encoded_len(3u32, &self.size1)
                    } else {
                        0
                    }
            }
            fn clear(&mut self) {
                self.color.clear();
                self.size = shirt::Size::default() as i32;
                self.size1 = shirt::Size::default() as i32;
            }
        }
        impl ::core::default::Default for Shirt {
            fn default() -> Self {
                Shirt {
                    color: ::prost::alloc::string::String::new(),
                    size: shirt::Size::default() as i32,
                    size1: shirt::Size::default() as i32,
                }
            }
        }
        impl ::core::fmt::Debug for Shirt {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("Shirt");
                let builder = {
                    let wrapper = {
                        #[allow(non_snake_case)]
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.color)
                    };
                    builder.field("color", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        struct ScalarWrapper<'a>(&'a i32);
                        impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                            fn fmt(
                                &self,
                                f: &mut ::core::fmt::Formatter,
                            ) -> ::core::fmt::Result {
                                let res: ::core::result::Result<shirt::Size, _> = ::core::convert::TryFrom::try_from(
                                    *self.0,
                                );
                                match res {
                                    Err(_) => ::core::fmt::Debug::fmt(&self.0, f),
                                    Ok(en) => ::core::fmt::Debug::fmt(&en, f),
                                }
                            }
                        }
                        ScalarWrapper(&self.size)
                    };
                    builder.field("size", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        struct ScalarWrapper<'a>(&'a i32);
                        impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                            fn fmt(
                                &self,
                                f: &mut ::core::fmt::Formatter,
                            ) -> ::core::fmt::Result {
                                let res: ::core::result::Result<shirt::Size, _> = ::core::convert::TryFrom::try_from(
                                    *self.0,
                                );
                                match res {
                                    Err(_) => ::core::fmt::Debug::fmt(&self.0, f),
                                    Ok(en) => ::core::fmt::Debug::fmt(&en, f),
                                }
                            }
                        }
                        ScalarWrapper(&self.size1)
                    };
                    builder.field("size1", &wrapper)
                };
                builder.finish()
            }
        }
        #[allow(dead_code)]
        impl Shirt {
            ///Returns the enum value of `size`, or the default if the field is set to an invalid enum value.
            pub fn size(&self) -> shirt::Size {
                ::core::convert::TryFrom::try_from(self.size)
                    .unwrap_or(shirt::Size::default())
            }
            ///Sets `size` to the provided enum value.
            pub fn set_size(&mut self, value: shirt::Size) {
                self.size = value as i32;
            }
            ///Returns the enum value of `size1`, or the default if the field is set to an invalid enum value.
            pub fn size1(&self) -> shirt::Size {
                ::core::convert::TryFrom::try_from(self.size1)
                    .unwrap_or(shirt::Size::default())
            }
            ///Sets `size1` to the provided enum value.
            pub fn set_size1(&mut self, value: shirt::Size) {
                self.size1 = value as i32;
            }
        }
        /// Nested message and enum types in `Shirt`.
        pub mod shirt {
            /// Label sizes
            /// your will see enum method to lib.rs
            #[repr(i32)]
            pub enum Size {
                Small = 0,
                Medium = 1,
                Large = 2,
            }
            #[automatically_derived]
            #[doc(hidden)]
            unsafe impl ::core::clone::TrivialClone for Size {}
            #[automatically_derived]
            impl ::core::clone::Clone for Size {
                #[inline]
                fn clone(&self) -> Size {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Size {}
            #[automatically_derived]
            impl ::core::fmt::Debug for Size {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            Size::Small => "Small",
                            Size::Medium => "Medium",
                            Size::Large => "Large",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Size {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Size {
                #[inline]
                fn eq(&self, other: &Size) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for Size {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) {}
            }
            #[automatically_derived]
            impl ::core::hash::Hash for Size {
                #[inline]
                fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    ::core::hash::Hash::hash(&__self_discr, state)
                }
            }
            #[automatically_derived]
            impl ::core::cmp::PartialOrd for Size {
                #[inline]
                fn partial_cmp(
                    &self,
                    other: &Size,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Ord for Size {
                #[inline]
                fn cmp(&self, other: &Size) -> ::core::cmp::Ordering {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
                }
            }
            impl Size {
                ///Returns `true` if `value` is a variant of `Size`.
                pub fn is_valid(value: i32) -> bool {
                    match value {
                        0 => true,
                        1 => true,
                        2 => true,
                        _ => false,
                    }
                }
                #[deprecated = "Use the TryFrom<i32> implementation instead"]
                ///Converts an `i32` to a `Size`, or `None` if `value` is not a valid variant.
                pub fn from_i32(value: i32) -> ::core::option::Option<Size> {
                    match value {
                        0 => ::core::option::Option::Some(Size::Small),
                        1 => ::core::option::Option::Some(Size::Medium),
                        2 => ::core::option::Option::Some(Size::Large),
                        _ => ::core::option::Option::None,
                    }
                }
            }
            impl ::core::default::Default for Size {
                fn default() -> Size {
                    Size::Small
                }
            }
            impl ::core::convert::From<Size> for i32 {
                fn from(value: Size) -> i32 {
                    value as i32
                }
            }
            impl ::core::convert::TryFrom<i32> for Size {
                type Error = ::prost::UnknownEnumValue;
                fn try_from(
                    value: i32,
                ) -> ::core::result::Result<Size, ::prost::UnknownEnumValue> {
                    match value {
                        0 => ::core::result::Result::Ok(Size::Small),
                        1 => ::core::result::Result::Ok(Size::Medium),
                        2 => ::core::result::Result::Ok(Size::Large),
                        _ => {
                            ::core::result::Result::Err(::prost::UnknownEnumValue(value))
                        }
                    }
                }
            }
            impl Size {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        Self::Small => "SMALL",
                        Self::Medium => "MEDIUM",
                        Self::Large => "LARGE",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "SMALL" => Some(Self::Small),
                        "MEDIUM" => Some(Self::Medium),
                        "LARGE" => Some(Self::Large),
                        _ => None,
                    }
                }
            }
        }
        pub struct Person {
            #[prost(string, tag = "1")]
            pub name: ::prost::alloc::string::String,
            #[prost(string, repeated, tag = "3")]
            pub phones: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Person {
            #[inline]
            fn clone(&self) -> Person {
                Person {
                    name: ::core::clone::Clone::clone(&self.name),
                    phones: ::core::clone::Clone::clone(&self.phones),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Person {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Person {
            #[inline]
            fn eq(&self, other: &Person) -> bool {
                self.name == other.name && self.phones == other.phones
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Person {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) {
                let _: ::core::cmp::AssertParamIsEq<::prost::alloc::string::String>;
                let _: ::core::cmp::AssertParamIsEq<
                    ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
                >;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Person {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
                ::core::hash::Hash::hash(&self.name, state);
                ::core::hash::Hash::hash(&self.phones, state)
            }
        }
        impl ::prost::Message for Person {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                if self.name != "" {
                    ::prost::encoding::string::encode(1u32, &self.name, buf);
                }
                ::prost::encoding::string::encode_repeated(3u32, &self.phones, buf);
            }
            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::wire_type::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                const STRUCT_NAME: &'static str = "Person";
                match tag {
                    1u32 => {
                        let mut value = &mut self.name;
                        ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "name");
                                error
                            })
                    }
                    3u32 => {
                        let mut value = &mut self.phones;
                        ::prost::encoding::string::merge_repeated(
                                wire_type,
                                value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "phones");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + if self.name != "" {
                        ::prost::encoding::string::encoded_len(1u32, &self.name)
                    } else {
                        0
                    }
                    + ::prost::encoding::string::encoded_len_repeated(3u32, &self.phones)
            }
            fn clear(&mut self) {
                self.name.clear();
                self.phones.clear();
            }
        }
        impl ::core::default::Default for Person {
            fn default() -> Self {
                Person {
                    name: ::prost::alloc::string::String::new(),
                    phones: ::prost::alloc::vec::Vec::new(),
                }
            }
        }
        impl ::core::fmt::Debug for Person {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("Person");
                let builder = {
                    let wrapper = {
                        #[allow(non_snake_case)]
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.name)
                    };
                    builder.field("name", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        struct ScalarWrapper<'a>(
                            &'a ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
                        );
                        impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                            fn fmt(
                                &self,
                                f: &mut ::core::fmt::Formatter,
                            ) -> ::core::fmt::Result {
                                let mut vec_builder = f.debug_list();
                                for v in self.0 {
                                    #[allow(non_snake_case)]
                                    fn Inner<T>(v: T) -> T {
                                        v
                                    }
                                    vec_builder.entry(&Inner(v));
                                }
                                vec_builder.finish()
                            }
                        }
                        ScalarWrapper(&self.phones)
                    };
                    builder.field("phones", &wrapper)
                };
                builder.finish()
            }
        }
        pub struct Foo {
            #[prost(string, tag = "3")]
            pub color: ::prost::alloc::string::String,
            #[prost(oneof = "foo::Widget", tags = "1, 2")]
            pub widget: ::core::option::Option<foo::Widget>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Foo {
            #[inline]
            fn clone(&self) -> Foo {
                Foo {
                    color: ::core::clone::Clone::clone(&self.color),
                    widget: ::core::clone::Clone::clone(&self.widget),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Foo {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Foo {
            #[inline]
            fn eq(&self, other: &Foo) -> bool {
                self.color == other.color && self.widget == other.widget
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Foo {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) {
                let _: ::core::cmp::AssertParamIsEq<::prost::alloc::string::String>;
                let _: ::core::cmp::AssertParamIsEq<::core::option::Option<foo::Widget>>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Foo {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
                ::core::hash::Hash::hash(&self.color, state);
                ::core::hash::Hash::hash(&self.widget, state)
            }
        }
        impl ::prost::Message for Foo {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                if let Some(ref oneof) = self.widget {
                    oneof.encode(buf)
                }
                if self.color != "" {
                    ::prost::encoding::string::encode(3u32, &self.color, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::wire_type::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                const STRUCT_NAME: &'static str = "Foo";
                match tag {
                    1u32 | 2u32 => {
                        let mut value = &mut self.widget;
                        foo::Widget::merge(value, tag, wire_type, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "widget");
                                error
                            })
                    }
                    3u32 => {
                        let mut value = &mut self.color;
                        ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "color");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0 + self.widget.as_ref().map_or(0, foo::Widget::encoded_len)
                    + if self.color != "" {
                        ::prost::encoding::string::encoded_len(3u32, &self.color)
                    } else {
                        0
                    }
            }
            fn clear(&mut self) {
                self.widget = ::core::option::Option::None;
                self.color.clear();
            }
        }
        impl ::core::default::Default for Foo {
            fn default() -> Self {
                Foo {
                    widget: ::core::default::Default::default(),
                    color: ::prost::alloc::string::String::new(),
                }
            }
        }
        impl ::core::fmt::Debug for Foo {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("Foo");
                let builder = {
                    let wrapper = {
                        #[allow(non_snake_case)]
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.color)
                    };
                    builder.field("color", &wrapper)
                };
                let builder = {
                    let wrapper = &self.widget;
                    builder.field("widget", &wrapper)
                };
                builder.finish()
            }
        }
        /// Nested message and enum types in `Foo`.
        pub mod foo {
            pub enum Widget {
                #[prost(int32, tag = "1")]
                Quux(i32),
                #[prost(string, tag = "2")]
                Bar(::prost::alloc::string::String),
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Widget {
                #[inline]
                fn clone(&self) -> Widget {
                    match self {
                        Widget::Quux(__self_0) => {
                            Widget::Quux(::core::clone::Clone::clone(__self_0))
                        }
                        Widget::Bar(__self_0) => {
                            Widget::Bar(::core::clone::Clone::clone(__self_0))
                        }
                    }
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Widget {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Widget {
                #[inline]
                fn eq(&self, other: &Widget) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                        && match (self, other) {
                            (Widget::Quux(__self_0), Widget::Quux(__arg1_0)) => {
                                __self_0 == __arg1_0
                            }
                            (Widget::Bar(__self_0), Widget::Bar(__arg1_0)) => {
                                __self_0 == __arg1_0
                            }
                            _ => unsafe { ::core::intrinsics::unreachable() }
                        }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for Widget {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) {
                    let _: ::core::cmp::AssertParamIsEq<i32>;
                    let _: ::core::cmp::AssertParamIsEq<::prost::alloc::string::String>;
                }
            }
            #[automatically_derived]
            impl ::core::hash::Hash for Widget {
                #[inline]
                fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    ::core::hash::Hash::hash(&__self_discr, state);
                    match self {
                        Widget::Quux(__self_0) => {
                            ::core::hash::Hash::hash(__self_0, state)
                        }
                        Widget::Bar(__self_0) => {
                            ::core::hash::Hash::hash(__self_0, state)
                        }
                    }
                }
            }
            impl Widget {
                /// Encodes the message to a buffer.
                pub fn encode(&self, buf: &mut impl ::prost::bytes::BufMut) {
                    match *self {
                        Widget::Quux(ref value) => {
                            ::prost::encoding::int32::encode(1u32, &*value, buf);
                        }
                        Widget::Bar(ref value) => {
                            ::prost::encoding::string::encode(2u32, &*value, buf);
                        }
                    }
                }
                /// Decodes an instance of the message from a buffer, and merges it into self.
                pub fn merge(
                    field: &mut ::core::option::Option<Widget>,
                    tag: u32,
                    wire_type: ::prost::encoding::wire_type::WireType,
                    buf: &mut impl ::prost::bytes::Buf,
                    ctx: ::prost::encoding::DecodeContext,
                ) -> ::core::result::Result<(), ::prost::DecodeError> {
                    match tag {
                        1u32 => {
                            if let ::core::option::Option::Some(Widget::Quux(value)) = field {
                                ::prost::encoding::int32::merge(wire_type, value, buf, ctx)
                            } else {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::prost::encoding::int32::merge(wire_type, value, buf, ctx)
                                    .map(|_| {
                                        *field = ::core::option::Option::Some(
                                            Widget::Quux(owned_value),
                                        );
                                    })
                            }
                        }
                        2u32 => {
                            if let ::core::option::Option::Some(Widget::Bar(value)) = field {
                                ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                            } else {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                                    .map(|_| {
                                        *field = ::core::option::Option::Some(
                                            Widget::Bar(owned_value),
                                        );
                                    })
                            }
                        }
                        _ => {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "internal error: entered unreachable code: {0}",
                                    format_args!("invalid Widget tag: {0}", tag),
                                ),
                            );
                        }
                    }
                }
                /// Returns the encoded length of the message without a length delimiter.
                #[inline]
                pub fn encoded_len(&self) -> usize {
                    match *self {
                        Widget::Quux(ref value) => {
                            ::prost::encoding::int32::encoded_len(1u32, &*value)
                        }
                        Widget::Bar(ref value) => {
                            ::prost::encoding::string::encoded_len(2u32, &*value)
                        }
                    }
                }
            }
            impl ::core::fmt::Debug for Widget {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        Widget::Quux(ref value) => {
                            let wrapper = {
                                #[allow(non_snake_case)]
                                fn ScalarWrapper<T>(v: T) -> T {
                                    v
                                }
                                ScalarWrapper(&*value)
                            };
                            f.debug_tuple("Quux").field(&wrapper).finish()
                        }
                        Widget::Bar(ref value) => {
                            let wrapper = {
                                #[allow(non_snake_case)]
                                fn ScalarWrapper<T>(v: T) -> T {
                                    v
                                }
                                ScalarWrapper(&*value)
                            };
                            f.debug_tuple("Bar").field(&wrapper).finish()
                        }
                    }
                }
            }
        }
        /// do not have Eq, Hash becuse have repeated struct
        pub struct AddressBook {
            #[prost(message, repeated, tag = "1")]
            pub people: ::prost::alloc::vec::Vec<Person>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for AddressBook {
            #[inline]
            fn clone(&self) -> AddressBook {
                AddressBook {
                    people: ::core::clone::Clone::clone(&self.people),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for AddressBook {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for AddressBook {
            #[inline]
            fn eq(&self, other: &AddressBook) -> bool {
                self.people == other.people
            }
        }
        impl ::prost::Message for AddressBook {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                for msg in &self.people {
                    ::prost::encoding::message::encode(1u32, msg, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::wire_type::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                const STRUCT_NAME: &'static str = "AddressBook";
                match tag {
                    1u32 => {
                        let mut value = &mut self.people;
                        ::prost::encoding::message::merge_repeated(
                                wire_type,
                                value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "people");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0 + ::prost::encoding::message::encoded_len_repeated(1u32, &self.people)
            }
            fn clear(&mut self) {
                self.people.clear();
            }
        }
        impl ::core::default::Default for AddressBook {
            fn default() -> Self {
                AddressBook {
                    people: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::fmt::Debug for AddressBook {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("AddressBook");
                let builder = {
                    let wrapper = &self.people;
                    builder.field("people", &wrapper)
                };
                builder.finish()
            }
        }
        pub struct AddressBook2 {
            #[prost(message, optional, tag = "2")]
            pub foo: ::core::option::Option<Foo>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for AddressBook2 {
            #[inline]
            fn clone(&self) -> AddressBook2 {
                AddressBook2 {
                    foo: ::core::clone::Clone::clone(&self.foo),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for AddressBook2 {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for AddressBook2 {
            #[inline]
            fn eq(&self, other: &AddressBook2) -> bool {
                self.foo == other.foo
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for AddressBook2 {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) {
                let _: ::core::cmp::AssertParamIsEq<::core::option::Option<Foo>>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for AddressBook2 {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
                ::core::hash::Hash::hash(&self.foo, state)
            }
        }
        impl ::prost::Message for AddressBook2 {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                if let Some(ref msg) = self.foo {
                    ::prost::encoding::message::encode(2u32, msg, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::wire_type::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                const STRUCT_NAME: &'static str = "AddressBook2";
                match tag {
                    2u32 => {
                        let mut value = &mut self.foo;
                        ::prost::encoding::message::merge(
                                wire_type,
                                value.get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "foo");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + self
                        .foo
                        .as_ref()
                        .map_or(
                            0,
                            |msg| ::prost::encoding::message::encoded_len(2u32, msg),
                        )
            }
            fn clear(&mut self) {
                self.foo = ::core::option::Option::None;
            }
        }
        impl ::core::default::Default for AddressBook2 {
            fn default() -> Self {
                AddressBook2 {
                    foo: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::fmt::Debug for AddressBook2 {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("AddressBook2");
                let builder = {
                    let wrapper = &self.foo;
                    builder.field("foo", &wrapper)
                };
                builder.finish()
            }
        }
        pub struct OnlyCopy {
            #[prost(int32, tag = "1")]
            pub number: i32,
        }
        #[automatically_derived]
        #[doc(hidden)]
        unsafe impl ::core::clone::TrivialClone for OnlyCopy {}
        #[automatically_derived]
        impl ::core::clone::Clone for OnlyCopy {
            #[inline]
            fn clone(&self) -> OnlyCopy {
                let _: ::core::clone::AssertParamIsClone<i32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for OnlyCopy {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for OnlyCopy {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for OnlyCopy {
            #[inline]
            fn eq(&self, other: &OnlyCopy) -> bool {
                self.number == other.number
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for OnlyCopy {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) {
                let _: ::core::cmp::AssertParamIsEq<i32>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for OnlyCopy {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
                ::core::hash::Hash::hash(&self.number, state)
            }
        }
        impl ::prost::Message for OnlyCopy {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                if self.number != 0i32 {
                    ::prost::encoding::int32::encode(1u32, &self.number, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::wire_type::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                const STRUCT_NAME: &'static str = "OnlyCopy";
                match tag {
                    1u32 => {
                        let mut value = &mut self.number;
                        ::prost::encoding::int32::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "number");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + if self.number != 0i32 {
                        ::prost::encoding::int32::encoded_len(1u32, &self.number)
                    } else {
                        0
                    }
            }
            fn clear(&mut self) {
                self.number = 0i32;
            }
        }
        impl ::core::default::Default for OnlyCopy {
            fn default() -> Self {
                OnlyCopy { number: 0i32 }
            }
        }
        impl ::core::fmt::Debug for OnlyCopy {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("OnlyCopy");
                let builder = {
                    let wrapper = {
                        #[allow(non_snake_case)]
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.number)
                    };
                    builder.field("number", &wrapper)
                };
                builder.finish()
            }
        }
    }
}
use snazzy::items;
/// Returns a large shirt of the specified color
pub fn create_large_shirt(color: String) -> items::Shirt {
    let mut shirt: items::Shirt = items::Shirt {
        color,
        ..Default::default()
    };
    shirt.set_size(items::shirt::Size::Large);
    shirt
}
