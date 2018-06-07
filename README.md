# libkvm

A Rust library interface to KVM

## Prerequisites

libkvm depends on Rust, which is [typically installed using the
`rustup` tool](https://www.rust-lang.org/en-US/install.html). Most
Linux distributions also have packages for Rust, but check that the
version of the Rust package is 1.24.0 or higher, because libkvm uses
`mem::size_of` as a constant function which was not allowed in earlier
versions of Rust.

If you installed with `rustup`, you may need to set your PATH
environment variable as in ~/.cargo/env.

Rust will download the source files for libkvm's library
dependencies automatically when you run `cargo` the first time.

### GCC

Rust uses GCC for linking. On Debian/Ubuntu you can install GCC with:

```
$ sudo apt-get install gcc
```

### KVM

It is likely that KVM is already installed if you're using a modern
Linux distro. If you're unsure, on Debian/Ubuntu you can install the
`cpu-checker` package, and then run:

```
$ kvm-ok
```

This should report:

```
INFO: /dev/kvm exists
KVM acceleration can be used
```

If you get permission errors when attempting to run libkvm for the
first time, you may need to set up the `kvm` group manually. (If you
installed QEMU or `kvmtool` they would do this for you, but libkvm
doesn't depend on these.)

```
$ sudo groupadd kvm
$ sudo chgrp -R kvm /dev/kvm
$ sudo chmod g+rw /dev/kvm
$ sudo adduser <username> kvm
```


## Copyright

Copyright (C) Allison Randal, 2018

This library is free software; you can redistribute it and/or modify
it under the terms of the GNU Library/Lesser General Public License as
published by the Free Software Foundation; either version 2 of the
License, or (at your option) any later version.
