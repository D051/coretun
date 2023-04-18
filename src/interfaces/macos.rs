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

use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::os::fd::FromRawFd;

use super::ErrorOS;
use super::Interface;
use super::Puller;
use super::Pusher;

// ******************************************************
// Macos native dependencies
// ******************************************************

extern "C" {
    /// Allocate a macos interface file descriptor
    fn alloc_macos_tun(ptr: *mut u8, len: i32) -> i32;
}

// *****************************************************
// Macos pusher/puller defintion and implementations
// *****************************************************

/// Macos pusher type definition -> write data to the macos interface
pub struct MacosPusher {
    file: File,
}

/// Macos puller type definition -> read data from the macos interface
pub struct MacosPuller {
    file: File,
}

/// Macos pusher type pusher trait implementation
impl Pusher for MacosPusher {
    /// Push/Write data to the macos interface
    fn push(&mut self, buf: &mut [u8]) {
        self.file.write(buf).unwrap();
        println!("-> macos pusher pushed");
    }
}

/// Macos puller type puller trait implementation
impl Puller for MacosPuller {
    /// Pull/Read data from the macos interface
    fn pull(&mut self, buf: &mut [u8]) {
        self.file.read(buf).unwrap();
        println!("<- macos puller pulled");
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
    /// Create a new macos interface
    fn open(name: &mut [u8]) -> Result<Self, ErrorOS> {
        // allocate the macos interface
        let result: i32 = unsafe { alloc_macos_tun(name.as_mut_ptr(), name.len() as i32) };
        // parse the result
        let fd: i32 = match result {
            // return the error
            -1 => return Err(ErrorOS::MacosErr("SockErr".into())),
            -2 => return Err(ErrorOS::MacosErr("InfoErr".into())),
            -3 => return Err(ErrorOS::MacosErr("AddrErr".into())),
            -4 => return Err(ErrorOS::MacosErr("NameErr".into())),
            // get the interface
            _ => result,
        };
        // return macos interface
        Ok(MacosInterface { fd: fd })
    }
    /// Get the macos interface pusher, only one pusher per interface is allowed to be in scope at a time
    fn pusher(&mut self) -> Self::PUSHER {
        let file: File = unsafe { File::from_raw_fd(self.fd) };
        MacosPusher { file }
    }
    /// Get the macos interface puller, only one puller per interface is allowed to be in scope at a time
    fn puller(&mut self) -> Self::PULLER {
        let file: File = unsafe { File::from_raw_fd(self.fd) };
        MacosPuller { file }
    }
}
