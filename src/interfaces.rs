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

// #[cfg(target_os = "macos")]
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

/// IO error enum
pub enum ErrorIO {
    None,
    PermissionErr,
    Unknown,
}

// *****************************************************
// Trait definitions
// *****************************************************

/// Pusher trait definition
pub trait Pusher {
    /// Push/Write data to the interface
    fn push(&mut self, buf: &mut [u8]);
}

/// Puller trait definition
pub trait Puller {
    /// Pull/Read data from the interface
    fn pull(&mut self, buf: &mut [u8]);
}

/// Interface trait definition
pub trait Interface {
    /// Pusher type for this interface
    type PUSHER: Pusher;
    /// Puller type for this interface
    type PULLER: Puller;
    /// create a new interface
    fn open(name: &mut [u8]) -> Result<Self, ErrorOS> where Self: Sized;
    /// get the interface pusher, only one pusher per interface can be in scope at a time
    fn pusher(&mut self) -> Self::PUSHER;
    /// get the interface puller, only one puller per interface can be in scope at a time
    fn puller(&mut self) -> Self::PULLER;
}
