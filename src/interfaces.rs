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

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "macos")]
pub mod macos;

// *****************************************************
// Error definitions
// *****************************************************

/// OS error enum
pub enum ErrorOS {
    None,
    LinuxErr(String),
    MacosErr(String),
    Unknown,
}

/// OS error enum implementations
impl ErrorOS {
    /// Print the error to the console
    pub fn print(&self) {
        match self {
            ErrorOS::None => println!("NO OS ERROR"),
            ErrorOS::LinuxErr(err) => println!("LINUX OS ERROR: {}", err),
            ErrorOS::MacosErr(err) => println!("MACOS OS ERROR: {}", err),
            ErrorOS::Unknown => println!("UNKNOWN OS ERROR"),
        }
    }
}

/// IO error enum
pub enum ErrorIO {
    None,
    LinuxErr(String),
    MacosErr(String),
    Unknown,
}

/// IO error enum implementations
impl ErrorIO {
    /// Print the error to the console
    pub fn print(&self) {
        match self {
            ErrorIO::None => println!("NO IO ERROR"),
            ErrorIO::LinuxErr(err) => println!("LINUX IO ERROR: {}", err),
            ErrorIO::MacosErr(err) => println!("MACOS IO ERROR: {}", err),
            ErrorIO::Unknown => println!("UNKNOWN IO ERROR"),
        }
    }
}

// *****************************************************
// Pusher/Puller trait definitions
// *****************************************************

/// Pusher trait definition
pub trait Pusher {
    /// Push/Write data to the interface
    fn push(&mut self, buf: &mut [u8]) -> usize;
}

/// Puller trait definition
pub trait Puller {
    /// Pull/Read data from the interface
    fn pull(&mut self, buf: &mut [u8]) -> usize;
}

// *****************************************************
// Interface trait definition
// *****************************************************

/// Interface trait definition
pub trait Interface {
    /// Pusher type for this interface
    type PUSHER: Pusher;
    /// Puller type for this interface
    type PULLER: Puller;
    /// Create a new interface
    fn open(name: &mut [u8]) -> Result<Self, ErrorOS> where Self: Sized;
    /// Get the interface pusher, only one pusher per interface is allowed to be in scope at a time
    fn pusher(&mut self) -> Self::PUSHER;
    /// Get the interface puller, only one puller per interface is allowed to be in scope at a time
    fn puller(&mut self) -> Self::PULLER;
}
