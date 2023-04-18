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
// Linux native dependencies
// ******************************************************

extern "C" {
    /// Allocate a linux interface file descriptor
    fn alloc_linux_tun(ptr: *mut u8, len: i32) -> i32;
}

// *****************************************************
// Macos pusher/puller defintion and implementations
// *****************************************************

/// Linux pusher type definition -> write data to the linux interface
pub struct LinuxPusher {
    file: File,
}

/// Linux puller type definition -> read data from the linux interface
pub struct LinuxPuller {
    file: File,
}

/// Linux pusher type pusher trait implementation
impl Pusher for LinuxPusher {
    /// Push/Write data to the linux interface
    fn push(&mut self, buf: &mut [u8]) {
        self.file.write(buf).unwrap();
        println!("-> linux pusher pushed");
    }
}

/// Linux puller type puller trait implementation
impl Puller for LinuxPuller {
    /// Pull/Read data from the linux interface
    fn pull(&mut self, buf: &mut [u8]) {
        self.file.read(buf).unwrap();
        println!("<- linux puller pulled");
    }
}

// *****************************************************
// Linux interface type definition and implementations
// *****************************************************

/// Linux interface type definition
pub struct LinuxInterface {
    fd: i32,
}

impl Interface for LinuxInterface {
    /// Linux pusher type for this interface
    type PUSHER = LinuxPusher;
    /// Linux puller type for this interface
    type PULLER = LinuxPuller;
    /// Create a new linux interface
    fn open(name: &mut [u8]) -> Result<Self, ErrorOS> {
        // allocate the linux interface
        let result: i32 = unsafe { alloc_linux_tun(name.as_mut_ptr(), name.len() as i32) };
        // parse the result
        let fd: i32 = match result {
            // return the error
            -1 => return Err(ErrorOS::LinuxErr("OpenErr".into())),
            -2 => return Err(ErrorOS::LinuxErr("CtrlErr".into())),
            // get the interface
            _ => result,
        };
        // return linux interface
        Ok(LinuxInterface { fd: fd })
    }
    /// Get the linux interface pusher, only one pusher per interface is allowed to be in scope at a time
    fn pusher(&mut self) -> Self::PUSHER {
        let file: File = unsafe { File::from_raw_fd(self.fd) };
        LinuxPusher { file }
    }
    /// Get the macos interface puller, only one puller per interface is allowed to be in scope at a time
    fn puller(&mut self) -> Self::PULLER {
        let file: File = unsafe { File::from_raw_fd(self.fd) };
        LinuxPuller { file }
    }
}
