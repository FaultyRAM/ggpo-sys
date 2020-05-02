// Copyright (c) 2020 FaultyRAM
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

#![allow(bad_style)]

include!(concat!(env!("OUT_DIR"), "/ggponet.rs"));

pub const fn GGPO_SUCCEEDED(result: GGPOErrorCode) -> bool {
    result == GGPOErrorCode_GGPO_ERRORCODE_SUCCESS
}
