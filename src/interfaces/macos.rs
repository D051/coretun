// *****************************************************
// License
// *****************************************************

// Copyright (C) 2023 Dominik Schweigler - All Rights Reserved

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// *****************************************************
// Imports / Exports
// *****************************************************

use super::Interface;
use super::Pusher;
use super::Puller;

// *****************************************************
// Macos pusher type definition and implementations
// *****************************************************

/// Macos pusher type definition
pub struct MacosPusher {
    fd: i32,
}

/// Macos pusher trait implementation
impl Pusher for MacosPusher {
    /// Push/Write data to the macos interface
    fn push(&self, buf: &mut [u8]) {
        println!("macos pusher push ->");
    }
}

// *****************************************************
// Macos puller type definition and implementations
// *****************************************************

/// Macos puller type definition
pub struct MacosPuller {
    fd: i32,
}

/// Macos puller trait implementation
impl Puller for MacosPuller {
    /// Push/Read data from the macos interface
    fn pull(&self, buf: &mut [u8]) {
        println!("macos puller pull <-");
    }
}

// *****************************************************
// Macos interface type definition and implementations
// *****************************************************

/// Macos interface type definition
pub struct MacosInterface {
    fd: i32,
}

impl Interface for MacosInterface {
    /// Macos pusher type for this interface
    type PUSHER = MacosPusher;
    /// Macos puller type for this interface
    type PULLER = MacosPuller;
    /// create a new macos interface
    fn new(ptr: *mut u8) -> Self {
        MacosInterface { fd: 0 }
    }
    /// get the macos interface pusher, only one pusher per interface can be in scope at a time
    fn pusher(&mut self) -> Self::PUSHER {
        MacosPusher { fd: 0 }
    }
    /// get the macos interface puller, only one puller per interface can be in scope at a time
    fn puller(&mut self) -> Self::PULLER {
        MacosPuller { fd: 0 }
    }
}
