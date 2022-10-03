// SPDX-License-Identifier: GPL-2.0

//! Foxes, for linux
use kernel::prelude::*;
use kernel::file::{File, Operations};
use kernel::miscdev;
use kernel::io_buffer::IoBufferWriter;

module! {
    type: FoxDev,
    name: "foxes",
    author: "Lux Amelia Phifollen",
    description: "Virtual device that outputs foxes",
    license: "GPL",
}

struct FoxDev {
    _dev: Pin<Box<miscdev::Registration<FoxDev>>>,
}

impl kernel::Module for FoxDev {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        // Print a banner to make sure the module is loaded and working
        pr_info!("+-------------------------------------+\n");
        pr_info!("| Foxes are supported on this kernel! |\n");
        pr_info!("+-------------------------------------+\n");
        //let reg = miscdev::Registration::new_pinned(fmt!("foxes"), ())?;
        let reg = miscdev::Options::new()
            .mode(0o444)
            .register_new(fmt!("foxes"), ())?;
        Ok(Self { _dev: reg })
    }
}

#[vtable]
impl Operations for FoxDev {
    fn open(_: &(), _file: &File) -> Result<()> {
        Ok(())
    }

    fn read(
        _data: (),
        _file: &File,
        writer: &mut impl IoBufferWriter,
        offset: u64,
    ) -> Result<usize> {
        let very = b"\xF0\x9F\xA6\x8A";
        let offset: usize = offset.try_into()?;
        let mut write_total = writer.len();
        let mut written = 0;
        // Considering the offset, we might have one part of a fox to write
        if offset % 4 != 0 {
            // Write what remains of the fox, if enough room
            let wlen = core::cmp::min(write_total, 4_usize - (offset%4_usize));
            writer.write_slice(&very[(offset%4)..][..wlen])?;
            write_total -= wlen;
            if write_total == 0 {
                return Ok(wlen);
            }
            written += wlen;
        }
        let many_foxes = write_total.div_euclid(4);
        for _ in 0..many_foxes {
            writer.write_slice(very)?;
        }
        written += many_foxes * 4;
        Ok(written) // If we always return n, n>0, the reader never stops
    }
}
