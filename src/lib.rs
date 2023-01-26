#![no_std]
#![cfg_attr(feature = "alloc", feature(allocator_api))]
#![cfg_attr(docsrs, feature(doc_cfg))]

macro_rules! flat_mod {
    ($($i:ident),+) => {
        $(
            mod $i;
            pub use $i::*;
        )+
    }
}

pub(crate) extern crate alloc;

#[docfg::docfg(feature = "serde")]
flat_mod! { serde_impl }

pub mod vec;
pub mod r#box;