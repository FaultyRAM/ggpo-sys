// Copyright (c) 2020 FaultyRAM
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

use bindgen::{
    callbacks::{IntKind, ParseCallbacks},
    CargoCallbacks,
};
use std::{
    env,
    error::Error,
    fmt::{self, Display, Formatter},
    io,
    path::Path,
};

#[derive(Clone, Copy, Debug)]
struct BindgenError;

impl Display for BindgenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("bindgen failed to create `ggponet.h` bindings")
    }
}

impl Error for BindgenError {}

#[derive(Debug)]
enum BuildError {
    Compile(cc::Error),
    Bindgen(BindgenError),
    IO(io::Error),
}

impl Display for BuildError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("build script failed to finish")
    }
}

impl Error for BuildError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Compile(e) => Some(e),
            Self::Bindgen(e) => Some(e),
            Self::IO(e) => Some(e),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct CustomCallbacks;

impl ParseCallbacks for CustomCallbacks {
    fn include_file(&self, filename: &str) {
        CargoCallbacks.include_file(filename)
    }

    fn int_macro(&self, name: &str, _value: i64) -> Option<IntKind> {
        match name {
            "GGPO_MAX_PLAYERS"
            | "GGPO_MAX_PREDICTION_FRAMES"
            | "GGPO_MAX_SPECTATORS"
            | "GGPO_SPECTATOR_INPUT_INTERVAL" => Some(IntKind::Int),
            "GGPO_INVALID_HANDLE" => Some(IntKind::Custom {
                name: "GGPOPlayerHandle",
                is_signed: true,
            }),
            _ => None,
        }
    }
}

fn compile_libggpo() -> Result<(), BuildError> {
    let mut build = cc::Build::new();
    #[cfg(unix)]
    const GGPO_PLATFORM_CPP: &str = "libggpo/src/lib/ggpo/platform_linux.cpp";
    #[cfg(windows)]
    const GGPO_PLATFORM_CPP: &str = "libggpo/src/lib/ggpo/platform_windows.cpp";
    #[cfg(windows)] // GGPO queries the `_WINDOWS` macro, which is not always pre-defined.
    let _ = build.define("_WINDOWS", None);
    build
        .include("libggpo/src/include")
        .include("libggpo/src/lib/ggpo")
        .files(&[
            "libggpo/src/lib/ggpo/bitvector.cpp",
            "libggpo/src/lib/ggpo/game_input.cpp",
            "libggpo/src/lib/ggpo/input_queue.cpp",
            "libggpo/src/lib/ggpo/log.cpp",
            "libggpo/src/lib/ggpo/main.cpp",
            "libggpo/src/lib/ggpo/poll.cpp",
            "libggpo/src/lib/ggpo/sync.cpp",
            "libggpo/src/lib/ggpo/timesync.cpp",
            GGPO_PLATFORM_CPP,
            "libggpo/src/lib/ggpo/network/udp_proto.cpp",
            "libggpo/src/lib/ggpo/backends/p2p.cpp",
            "libggpo/src/lib/ggpo/backends/spectator.cpp",
            "libggpo/src/lib/ggpo/backends/synctest.cpp",
        ])
        .cpp(true)
        .try_compile("GGPO")
        .map_err(BuildError::Compile)
}

fn generate_bindings() -> Result<(), BuildError> {
    bindgen::Builder::default()
        .header("wrapper.hpp")
        .generate_comments(true)
        .opaque_type("GGPOSession")
        .whitelist_type("GGPOSession")
        .whitelist_type("GGPOPlayerHandle")
        .whitelist_type("GGPOPlayerType")
        .whitelist_type("GGPOPlayer")
        .whitelist_type("GGPOLocalEndpoint")
        .whitelist_type("GGPOErrorCode")
        .whitelist_type("GGPOEventCode")
        .whitelist_type("GGPOEvent")
        .whitelist_type("GGPOSessionCallbacks")
        .whitelist_type("GGPONetworkStats")
        .whitelist_function("ggpo_start_session")
        .whitelist_function("ggpo_add_player")
        .whitelist_function("ggpo_start_synctest")
        .whitelist_function("ggpo_start_spectating")
        .whitelist_function("ggpo_close_session")
        .whitelist_function("ggpo_set_frame_delay")
        .whitelist_function("ggpo_idle")
        .whitelist_function("ggpo_add_local_input")
        .whitelist_function("ggpo_synchronize_input")
        .whitelist_function("ggpo_disconnect_player")
        .whitelist_function("ggpo_advance_frame")
        .whitelist_function("ggpo_get_network_stats")
        .whitelist_function("ggpo_set_disconnect_timeout")
        .whitelist_function("ggpo_set_disconnect_notify_start")
        .whitelist_function("ggpo_log")
        .whitelist_var("GGPO_MAX_PLAYERS")
        .whitelist_var("GGPO_MAX_PREDICTION_FRAMES")
        .whitelist_var("GGPO_MAX_SPECTATORS")
        .whitelist_var("GGPO_SPECTATOR_INPUT_INTERVAL")
        .whitelist_var("GGPO_INVALID_HANDLE")
        .parse_callbacks(Box::new(CustomCallbacks))
        .generate()
        .map_err(|_| BuildError::Bindgen(BindgenError))
        .and_then(|bindings| {
            let out_path = Path::new(&env::var_os("OUT_DIR").unwrap()).join("ggponet.rs");
            bindings.write_to_file(out_path).map_err(BuildError::IO)
        })
}

fn main() -> Result<(), BuildError> {
    compile_libggpo().and_then(|_| generate_bindings())
}
