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
    CONN_ERR = -2,
};

// ******************************************************
// Linux tun allocation
// ******************************************************

// Allocate a native linux tun interface
int alloc_linux_tun(unsigned char *ptr, int len) {

    // Get the file descriptor for the tun device
    int fd = open("/dev/net/tun", O_RDWR);
    if (fd < 0) {
        perror("Error opening tun device");
        return (int)OPEN_ERR;
    }

    // Determine the maximum length of the interface name
    if (len > IFNAMSIZ) {
        len = IFNAMSIZ;
    }

    // Initialize a struct to represent the interface
    struct ifreq ifr = {0};
    ifr.ifr_flags = IFF_TUN | IFF_NO_PI;
    strncpy(ifr.ifr_name, (char*)ptr, len);

    // Associate the file descriptor with the interface
    if (ioctl(fd, TUNSETIFF, &ifr) < 0) {
        perror("Error connecting the tun fd with interface");
        return (int)CONN_ERR;
    }

    // Copy the interface name back into the buffer
    strncpy((char*)ptr, ifr.ifr_name, len);

    // Return the file descriptor
    return fd;

}