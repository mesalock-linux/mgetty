// Copyright (c) 2017, MesaLock Linux Authors.
// All rights reserved.
// 
// This work is licensed under the terms of the BSD 3-Clause License.
// For a copy, see the LICENSE file.

extern crate nix;
extern crate libc;

use nix::fcntl;
use nix::sys;
use nix::sys::termios;
use nix::unistd;
use std::ffi::CString;
use sys::stat;
use std::os::unix::io;

fn open_tty() {
    unistd::close(0).expect("close(0) failed");
    let _ = fcntl::open("/dev/tty1", fcntl::O_RDWR | fcntl::O_NONBLOCK, stat::Mode::empty()).expect("open failed");
    unsafe {
        libc::fchown(0, 0, 0);
        libc::fchmod(0, 0620);
    }

    if unistd::isatty(0).expect("isatty failed") == false {
        panic!("isatty failed");
    }
}

fn ndelay_off(fd: io::RawFd) {
    let original_flags = fcntl::OFlag::from_bits(fcntl::fcntl(fd, fcntl::F_GETFL).expect("fcntl failed")).expect("from_bits failed");
    fcntl::fcntl(fd, fcntl::F_SETFL(!fcntl::O_NONBLOCK & original_flags)).expect("fcntl failed");
}

fn main() {
    let pid = unistd::getpid();
    unistd::setsid().expect("setsid failed");
    open_tty();
    ndelay_off(0);

    if unistd::dup2(0, 1).expect("dup2 1 failed") != 1 {
        panic!("dup2 failed");
    }
    if unistd::dup2(0, 2).expect("dup2 2 failed") != 2 {
        panic!("dup2 failed");
    }

    unsafe { libc::ioctl(0, libc::TIOCSCTTY, 1); }

    unistd::tcsetpgrp(0, pid).expect("tcsetpgrp failed");

    unistd::execv(&(CString::new("/bin/ion").unwrap()), &[CString::new("ion").unwrap()]).expect("execv failed");
}
