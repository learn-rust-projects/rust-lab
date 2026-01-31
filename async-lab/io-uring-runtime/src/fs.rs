use std::{fs::File, io, os::unix::io::AsRawFd, path::Path};

use io_uring::{opcode, types};

use crate::future::UringFuture;

pub struct AsyncFile {
    fd: File,
}

impl AsyncFile {
    pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
        let fd = File::open(path)?;
        Ok(Self { fd })
    }

    pub async fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> {
        let fd = types::Fd(self.fd.as_raw_fd());
        let read_op = opcode::Read::new(fd, buf.as_mut_ptr(), buf.len() as u32)
            .offset(offset)
            .build();

        let res = UringFuture::new(read_op).await?;
        if res < 0 {
            Err(io::Error::from_raw_os_error(-res))
        } else {
            Ok(res as usize)
        }
    }

    pub async fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.read_at(buf, 0).await
    }
}
