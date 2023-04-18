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

#include <sys/socket.h>
#include <sys/ioctl.h>
#include <net/if_utun.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <sys/sys_domain.h>
#include <sys/kern_control.h>

// ******************************************************
// Error definition
// ******************************************************

// Macos error definitions
enum error {
    OPEN_ERR = -1,
    INFO_ERR = -2,
    CONN_ERR = -3,
    NAME_ERR = -4,
};

// ******************************************************
// Allocation fucntion
// ******************************************************

// Allocate a native macOS tun interface
int alloc_macos_tun(unsigned char *ptr, int len) {

    // Get the file descriptor for the tun device
    int fd = socket(PF_SYSTEM, SOCK_DGRAM, SYSPROTO_CONTROL);
    if (fd < 0) {
        perror("Error opening tun device");
        return (int)OPEN_ERR;
    }

    // Get control information for the tun device
    struct ctl_info control_info;
    strncpy(control_info.ctl_name, UTUN_CONTROL_NAME, sizeof(control_info.ctl_name));
    if (ioctl(fd, CTLIOCGINFO, &control_info) < 0) {
        perror("Error getting control info for tun device");
        close(fd);
        return (int)INFO_ERR;
    }

    // Create a socket address control for the tun device
    struct sockaddr_ctl socket_address_control;
    socket_address_control.sc_id = control_info.ctl_id;
    socket_address_control.sc_len = sizeof(socket_address_control);
    socket_address_control.sc_family = AF_SYSTEM;
    socket_address_control.ss_sysaddr = AF_SYS_CONTROL;
    socket_address_control.sc_unit = 0;

    // Connect the socket to the tun device
    if (connect(fd, (struct sockaddr *)&socket_address_control, sizeof(socket_address_control)) < 0) {
        perror("Error connecting to tun device");
        close(fd);
        return (int)CONN_ERR;
    }

    // Get the interface name for the tun device
    socklen_t interface_name_length = len;
    if (getsockopt(fd, SYSPROTO_CONTROL, UTUN_OPT_IFNAME, ptr, &interface_name_length) < 0) {
        perror("Error getting tun interface name");
        close(fd);
        return (int)NAME_ERR;
    }

    // Return the file descriptor
    return fd;

}