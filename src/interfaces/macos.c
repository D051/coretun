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
    ADDR_ERR = -3,
    NAME_ERR = -4,
};

// Print the error in plaintext to the console
void show_error(enum error err) {
    switch(err){
        // error cases
        case OPEN_ERR : perror("PLATTFORM ERROR: open error"); break;
        case INFO_ERR : perror("PLATTFORM ERROR: info error"); break;
        case ADDR_ERR : perror("PLATTFORM ERROR: addr error"); break;
        case NAME_ERR : perror("PLATTFORM ERROR: name error"); break;
        // default case
        default: perror("PLATTFORM ERROR: unknown"); break;
    }
}

// ******************************************************
// Macos tun allocation
// ******************************************************

// Allocate a native macos tun interface
int alloc_macos_tun(unsigned char *ptr, int len) {
    // allocate process variables
    int result;
    // get the tun file descriptor
    int fd = socket(PF_SYSTEM, SOCK_DGRAM, SYSPROTO_CONTROL);
    if (fd < 0) {
        show_error(OPEN_ERR);
        return (int)OPEN_ERR;
    }
    // construct the control message
    struct ctl_info info;
    strncpy(info.ctl_name, UTUN_CONTROL_NAME, sizeof(info.ctl_name));
    // execute the control message
    result = ioctl(fd, CTLIOCGINFO, &info);
    if (result < 0) {
        close(fd);
        show_error(INFO_ERR);
        return (int)INFO_ERR;
    }
    // create and configure the socket address control
    struct sockaddr_ctl addr;
	addr.sc_id = info.ctl_id;
	addr.sc_len = sizeof(addr);
	addr.sc_family = AF_SYSTEM;
	addr.ss_sysaddr = AF_SYS_CONTROL;
	addr.sc_unit = 0;
    // create the tun device
    result = connect(fd, (struct sockaddr *)&addr, sizeof(addr));
    if (result < 0) {
        close(fd);
        show_error(ADDR_ERR);
        return (int)ADDR_ERR;
    }
    // get the name of the tun interface
    result = getsockopt(fd, SYSPROTO_CONTROL, UTUN_OPT_IFNAME, ptr, (socklen_t*)&len);
    if (result < 0) {
        close(fd);
        show_error(NAME_ERR);
        return (int)NAME_ERR;
    }
    // return the file descriptor
    return fd;
}