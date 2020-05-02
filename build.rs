// Copyright (c) 2020 FaultyRAM
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

fn main() -> Result<(), cc::Error> {
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
}
