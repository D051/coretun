// *****************************************************
// License
// *****************************************************

// Copyright (C) 2023 Dominik Schweigler - All Rights Reserved

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// ******************************************************
// Imports / Exports
// ******************************************************

#include <fcntl.h>
#include <stdio.h>
#include <string.h>
#include <linux/if.h>
#include <linux/if_tun.h>
#include <sys/ioctl.h>

// ******************************************************
// Constant defintions
// ******************************************************

#define IFF_TUN 0x0001
#define IFF_TAP 0x0002
#define IFF_NO_PI 0x1000


// ******************************************************
// Error definition
// ******************************************************

// Linux error definitions
enum error {
    OPEN_ERR = -1,
    CTRL_ERR = -2,
};


// Print the error in plaintext to the console
void show_error(enum error err) {
    switch(err){
        // error cases
        case OPEN_ERR : perror("PLATTFORM ERROR: open error"); break;
        case CTRL_ERR : perror("PLATTFORM ERROR: ctrl error"); break;
        // default case
        default: perror("PLATTFORM ERROR: unknown"); break;
    }
}

// ******************************************************
// Linux tun allocation
// ******************************************************

/**
 * Allocates a Linux TUN interface.
 *
 * @param ptr The buffer where the name of the interface will be stored.
 * @param len The length of the buffer.
 * @return The file descriptor of the TUN interface on success, or an error code on failure.
 */

int alloc_linux_tun(unsigned char *ptr, int len) {
    int result;
    int fd = open("/dev/net/tun", O_RDWR); // Open the TUN device file
    if (fd < 0) {
        show_error(OPEN_ERR); // Show an error message if the open() call fails
        return (int)OPEN_ERR;
    }
    size_t max_len = len;
    if (IFNAMSIZ < len) { // If the buffer is too long, truncate it to the maximum length allowed
        max_len = IFNAMSIZ;
    }
	struct ifreq ifr;
	memset(&ifr, 0, sizeof ifr); // Initialize the ifreq structure to zero
    ifr.ifr_flags = IFF_TUN | IFF_NO_PI; // Set the interface type and no packet info flag
    strncpy(ifr.ifr_name, (char*)ptr, max_len); // Copy the interface name into the ifreq structure
	result = ioctl(fd, TUNSETIFF, &ifr); // Set the interface name and flags using the ioctl() system call
	if (result < 0) {
        show_error(CTRL_ERR); // Show an error message if the ioctl() call fails
        return (int)CTRL_ERR;
	}
    strncpy((char*)ptr, ifr.ifr_name, max_len); // Copy the interface name back into the buffer
    return fd; // Return the file descriptor of the TUN interface
}