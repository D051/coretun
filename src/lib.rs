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

pub mod interfaces;

pub use interfaces::Interface;
pub use interfaces::Puller;
pub use interfaces::Pusher;
pub use interfaces::ErrorOS;
pub use interfaces::ErrorIO;

// *****************************************************
// Adapter type definition and implementations
// *****************************************************

/// Adapter type defintion
pub struct Adapter<I: Interface> {
    pub kernel_name: [u8; 24],
    pub custom_name: [u8; 24],
    pub interface: I,
}

/// Adapter type implementations
impl<I: Interface> Adapter<I> {
    /// Open a new coretun adapter

    pub fn open(name: &str) -> Result<Self, ErrorOS> {
        // build name buffers
        let mut kernel_name: [u8; 24] = [0u8; 24];
        let mut custom_name: [u8; 24] = [0u8; 24];
        kernel_name[0..name.len()].copy_from_slice(name.as_bytes());
        custom_name[0..name.len()].copy_from_slice(name.as_bytes());
        // build interface
        let interface: I =  match I::open(&mut kernel_name) {
            Ok(interface) => interface,
            Err(error) => return Err(error),
        };
        // build adapter
        let adapter: Adapter<I> = Adapter {
            kernel_name,
            custom_name,
            interface,
        };
        // return the adapter
        Ok(adapter)
    }
    /// Get the adapter interface pusher
    
    pub fn pusher(&mut self) -> I::PUSHER {
        self.interface.pusher()
    }
    
    /// Get the adapter interface puller
    pub fn puller(&mut self) -> I::PULLER {
        self.interface.puller()
    }
    
    /// Get the adapter kernel name
    pub fn kernel_name(&self) -> String {
        // find the last non-zero byte
        let mut last_non_zero: usize = 0;
        for i in 0..self.kernel_name.len() {
            if self.kernel_name[i] != 0 {
                last_non_zero = i;
            }
        }
        // decode the kernel name
        let string: &str = match std::str::from_utf8(&self.kernel_name[0..last_non_zero + 1]) {
            Ok(v) => v,
            Err(e) => panic!("kernel name - invalid UTF-8 sequence: {}", e),
        };
        // return the kernel name
        String::from(string)
    }
    
    /// Get the adapter custom name
    pub fn custom_name(&self) -> String {
        // find the last non-zero byte
        let mut last_non_zero: usize = 0;
        for i in 0..self.custom_name.len() {
            if self.custom_name[i] != 0 {
                last_non_zero = i;
            }
        }
        // decode the custom name
        let string: &str = match std::str::from_utf8(&self.custom_name[0..last_non_zero + 1]) {
            Ok(v) => v,
            Err(e) => panic!("custom name - invalid UTF-8 sequence: {}", e),
        };
        // return the custom name
        String::from(string)
    }
    
    /// Show the adapter in the console
    pub fn show(&self) {
        println!("----------------------------------------");
        println!("**              ADAPTER               **");
        println!("----------------------------------------");
        println!("-> kernel name : {}", self.kernel_name());
        println!("----------------------------------------");
        println!("-> custom name : {}", self.custom_name());
        println!("----------------------------------------");
    }
}

// *****************************************************
// Plattform specific shortcuts
// *****************************************************

#[cfg(target_os = "linux")]
pub fn open(name: &str) -> Result<Adapter<interfaces::linux::LinuxInterface>, ErrorOS> {
    Adapter::open(name)
}

#[cfg(target_os = "macos")]
pub fn open(name: &str) -> Result<Adapter<interfaces::macos::MacosInterface>, ErrorOS> {
    Adapter::open(name)
}
