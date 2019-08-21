//! IMPLEMENTATION DETAILS USED BY MACROS

use core::cell::Cell;
use core::fmt::{self, Write};

use cortex_m::interrupt::{self, Mutex};

use crate::hio::{self, HostStream};

static HSTDOUT: Mutex<Cell<Option<HostStream>>> = Mutex::new(Cell::new(None));
static HSTDERR: Mutex<Cell<Option<HostStream>>> = Mutex::new(Cell::new(None));

pub fn hstdout_str(s: &str) -> Result<(), ()> {
    with_lazy_init(&HSTDOUT, hio::hstdout, |x| x.write_str(s))
}

pub fn hstdout_fmt(args: fmt::Arguments) -> Result<(), ()> {
    with_lazy_init(&HSTDOUT, hio::hstdout, |x| x.write_fmt(args))
}

pub fn hstderr_str(s: &str) -> Result<(), ()> {
    with_lazy_init(&HSTDERR, hio::hstderr, |x| x.write_str(s))
}

pub fn hstderr_fmt(args: fmt::Arguments) -> Result<(), ()> {
    with_lazy_init(&HSTDERR, hio::hstderr, |x| x.write_fmt(args))
}

fn with_lazy_init(
    mutex: &Mutex<Cell<Option<HostStream>>>,
    init: impl Fn() -> Result<HostStream, ()>,
    f: impl Fn(&mut HostStream) -> Result<(), fmt::Error>,
) -> Result<(), ()> {
    interrupt::free(|cs| {
        let cell = mutex.borrow(cs);
        let mut stream = match cell.get() {
            Some(s) => s,
            None => {
                let s = init()?;
                cell.set(Some(s));
                s
            }
        };
        f(&mut stream).map_err(drop)
    })
}
