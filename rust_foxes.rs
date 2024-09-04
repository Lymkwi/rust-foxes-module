// SPDX-License-Identifier: GPL-2.0

//! Foxes, for linux
use core::sync::atomic::{AtomicUsize, Ordering};
use kernel::alloc::flags::GFP_KERNEL;
use kernel::file::{File, Operations};
use kernel::io_buffer::IoBufferWriter;
use kernel::miscdev;
use kernel::prelude::*;
use kernel::sync::{Arc, ArcBorrow};

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
    fn init(_module: &'static ThisModule) -> Result<Self> {
        // Print a banner to make sure the module is loaded and working
        pr_info!("+-------------------------------------+\n");
        pr_info!("| Foxes are supported on this kernel! |\n");
        pr_info!("+-------------------------------------+\n");
        //let reg = miscdev::Registration::new_pinned(fmt!("foxes"), ())?;
        let max_count = Arc::new(AtomicUsize::new(1_000_000), GFP_KERNEL)?;
        let reg = miscdev::Options::new()
            .mode(0o444)
            .register_new(kernel::fmt!("foxes"), max_count)?;
        Ok(Self { _dev: reg })
    }
}

#[vtable]
impl Operations for FoxDev {
    type Data = Arc<AtomicUsize>;
    type OpenData = Arc<AtomicUsize>;

    fn open(open_data: &Self::OpenData, _file: &File) -> Result<Self::Data> {
        Ok(open_data.clone())
    }

    fn read(
        data: ArcBorrow<'_, AtomicUsize>,
        _file: &File,
        writer: &mut impl IoBufferWriter,
        offset: u64,
    ) -> Result<usize> {
        let very = b"\xF0\x9F\xA6\x8A";
        let offset: usize = offset.try_into()?;
        let mut write_total = writer.len();
        let mut written = 0;
        // Considering the offset, we might have one part of a fox to write
        // If we started writing a fox, we will always have to finish it.
        if offset % 4 != 0 {
            // Write what remains of the fox, if enough room
            let wlen = core::cmp::min(write_total, 4_usize - (offset % 4_usize));
            writer.write_slice(&very[(offset % 4)..][..wlen])?;
            write_total -= wlen;
            if wlen < 4_usize - (offset % 4_usize) {
                // Couldn't complete a whole fox
                return Ok(wlen);
            }
            written += wlen;
            // If a fox was partly written, and we completed it, it has not been accounted for yet
            // Yes! A chardev driver keeps the same data for a given context between the open() and
            // close() of its file!
            data.fetch_sub(1, Ordering::Acquire);
        }
        let remain_foxes = data.load(Ordering::Acquire);
        let many_foxes = write_total.div_euclid(4).min(remain_foxes);
        for _ in 0..many_foxes {
            writer.write_slice(very)?;
        }
        // Subtract whatever was left
        data.fetch_sub(many_foxes, Ordering::Acquire);
        written += many_foxes * 4;
        // Write however many bytes of foxes we have left, if at least one fox can be written,
        // because the reader may not be 4-bytes aligned Without the following lines, running `dd
        // if=/dev/foxes bs=3 count=12` outputs nothing
        if data.load(Ordering::Acquire) > 0 {
            let remain = write_total - 4 * many_foxes;
            writer.write_slice(&very[..remain])?;
            written += remain;
        }
        // If we always return n, n>0, the reader never stops
        Ok(written)
    }
}
