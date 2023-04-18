// *****************************************************
// License
// *****************************************************

// Copyright (C) 2023 Dominik Schweigler - All Rights Reserved

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// *****************************************************
// Plattform instructions
// *****************************************************

use cc::Build;

// *****************************************************
// Build instructions
// *****************************************************

fn main() {
    // create builder
    let mut build: cc::Build = cc::Build::new();
    // add include path for macos
    #[cfg(target_os = "macos")]
    let builder: &mut Build = build.file("./src/interfaces/macos.c");
    // add include path for linux
    #[cfg(target_os = "linux")]
    let builder: &mut Build = build.file("./src/interfaces/linux.c");
    // compile and link
    builder.compile("libnative.a");
}