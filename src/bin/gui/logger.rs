use std::{
    io::{Cursor, Write},
    sync::{Arc, Mutex},
};

use druid::Data;

pub struct LogBuffer {
    inner: Arc<Mutex<Cursor<Vec<u8>>>>,
}

impl LogBuffer {
    pub fn new() -> Self {
        LogBuffer {
            inner: Arc::new(Mutex::new(Cursor::new(Vec::new()))),
        }
    }

    pub fn clone_arc(&self) -> Self {
        LogBuffer {
            inner: Arc::clone(&self.inner),
        }
    }

    pub fn get_contents(&self) -> String {
        let guard = self.inner.lock().unwrap();
        let log_data = guard.get_ref();
        String::from_utf8_lossy(log_data).into_owned()
    }

    pub fn mutate_contents(
        &self,
        f: impl FnOnce(&mut Vec<u8>),
    ) {
        let mut guard = self.inner.lock().unwrap();
        f(&mut guard.get_mut());
    }
}

impl Clone for LogBuffer {
    fn clone(&self) -> Self {
        LogBuffer {
            inner: Arc::new(Mutex::new(Cursor::new(
                self.get_contents().as_bytes().into(),
            ))),
        }
    }
}

impl Write for LogBuffer {
    fn write(
        &mut self,
        buf: &[u8],
    ) -> std::io::Result<usize> {
        let mut guard = self.inner.lock().unwrap();
        guard.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let mut guard = self.inner.lock().unwrap();
        guard.flush()
    }
}

impl Data for LogBuffer {
    fn same(
        &self,
        other: &Self,
    ) -> bool {
        self.get_contents() == other.get_contents()
    }
}
