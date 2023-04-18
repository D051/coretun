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

// Allocate a native linux tun interface
int alloc_linux_tun(unsigned char *ptr, int len) {
    // allocate process variables
    int result;
    // get the tun file descriptor
    int fd = open("/dev/net/tun", O_RDWR);
    if (fd < 0) {
        show_error(OPEN_ERR);
        return (int)OPEN_ERR;
    }
    // get the max length of the interface name
    size_t max_len = len;
    if (IFNAMSIZ < len) {
        max_len = IFNAMSIZ;
    }
    // configure the tun interface
	struct ifreq ifr;
	memset(&ifr, 0, sizeof ifr);
    ifr.ifr_flags = IFF_TUN | IFF_NO_PI;
    strncpy(ifr.ifr_name, (char*)ptr, max_len);
    // apply configuration to interface
	result = ioctl(fd, TUNSETIFF, &ifr);
	if (result < 0) {
        show_error(CTRL_ERR);
        return (int)CTRL_ERR;
	}
    // get the name of the tun interface
    strncpy((char*)ptr, ifr.ifr_name, max_len);
    // return the file descriptor
    return fd;
}


