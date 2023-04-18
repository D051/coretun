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
use super::Puller;
use super::Pusher;
use super::ErrorOS;

// ******************************************************
// Macos native dependencies
// ******************************************************

extern "C" {
    fn alloc_macos_tun(ptr: *mut u8, len: i32) -> i32;
}

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
        buf[0] = self.fd as u8;
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
    /// Pull/Read data from the macos interface
    fn pull(&self, buf: &mut [u8]) {
        buf[0] = self.fd as u8;
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
    fn open(name: &mut [u8]) -> Result<Self, ErrorOS> {
        // Create the pointer and length of the name
        let ptr: *mut u8 = name.as_mut_ptr();
        let len: i32 = name.len() as i32;
        // Allocate the macos interface
        let res: i32 = unsafe { alloc_macos_tun(ptr, len) };
        // Check the result
        let fd: i32 = match res {
            // Return the error
            -1 => return Err(ErrorOS::MacosErr("SockErr".into())),
            -2 => return Err(ErrorOS::MacosErr("InfoErr".into())),
            -3 => return Err(ErrorOS::MacosErr("AddrErr".into())),
            -4 => return Err(ErrorOS::MacosErr("NameErr".into())),
            // Get the interface
            _ => res,
        };
        // build the interface
        let interface: MacosInterface = MacosInterface { fd: fd };
        // return the interface
        Ok(interface)
    }
    /// get the macos interface pusher, only one pusher per interface can be in scope at a time
    fn pusher(&mut self) -> Self::PUSHER {
        MacosPusher { fd: self.fd }
    }
    /// get the macos interface puller, only one puller per interface can be in scope at a time
    fn puller(&mut self) -> Self::PULLER {
        MacosPuller { fd: self.fd }
    }
}