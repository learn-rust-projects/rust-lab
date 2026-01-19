#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;

use enum_dispatch::enum_dispatch;
pub struct RespNull;
pub struct RespSet;
impl MyTrait for RespNull {
    async fn do_something(&self) {
        {
            ::std::io::_print(format_args!("RespNull called\n"));
        };
    }
}
impl MyTrait for RespSet {
    async fn do_something(&self) {
        {
            ::std::io::_print(format_args!("RespSet called\n"));
        };
    }
}
trait MyTrait {
    async fn do_something(&self);
}
enum RespEnum {
    RespNull(RespNull),
    RespSet(RespSet),
}
impl ::core::convert::From<RespNull> for RespEnum {
    fn from(v: RespNull) -> RespEnum {
        RespEnum::RespNull(v)
    }
}
impl ::core::convert::From<RespSet> for RespEnum {
    fn from(v: RespSet) -> RespEnum {
        RespEnum::RespSet(v)
    }
}
impl ::core::convert::TryInto<RespNull> for RespEnum {
    type Error = &'static str;
    fn try_into(
        self,
    ) -> ::core::result::Result<RespNull, <Self as ::core::convert::TryInto<RespNull>>::Error> {
        match self {
            RespEnum::RespNull(v) => Ok(v),
            RespEnum::RespSet(v) => Err("Tried to convert variant RespSet to RespNull"),
        }
    }
}
impl ::core::convert::TryInto<RespSet> for RespEnum {
    type Error = &'static str;
    fn try_into(
        self,
    ) -> ::core::result::Result<RespSet, <Self as ::core::convert::TryInto<RespSet>>::Error> {
        match self {
            RespEnum::RespSet(v) => Ok(v),
            RespEnum::RespNull(v) => Err("Tried to convert variant RespNull to RespSet"),
        }
    }
}
impl MyTrait for RespEnum {
    #[inline]
    async fn do_something(&self) {
        match self {
            RespEnum::RespNull(inner) => MyTrait::do_something(inner).await,
            RespEnum::RespSet(inner) => MyTrait::do_something(inner).await,
        }
    }
}
fn main() {
    let body = async {
        let r1: RespEnum = RespNull.into();
        let r2: RespEnum = RespSet.into();
        r1.do_something().await;
        r2.do_something().await;
        match &r1 {
            RespEnum::RespNull(_) => {
                ::std::io::_print(format_args!("Matched RespNull\n"));
            }
            RespEnum::RespSet(_) => {
                ::std::io::_print(format_args!("Matched RespSet\n"));
            }
        }
    };
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return,
        clippy::unwrap_in_result
    )]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
