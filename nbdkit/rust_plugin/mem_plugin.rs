use nbdkit::*;
use std::{
    fs::OpenOptions,
    io::{Read, Seek, Write},
};

struct MemoryDisk {
    file: String,
}

impl Default for MemoryDisk {
    fn default() -> Self {
        MemoryDisk::new()
    }
}

impl MemoryDisk {
    pub fn new() -> Self {
        MemoryDisk {
            file: "./disk.raw".to_string(),
        }
    }
}

impl Server for MemoryDisk {
    fn name() -> &'static str {
        "memorydisk"
    }

    fn open(_readonly: bool) -> Result<Box<dyn Server>> {
        Ok(Box::<MemoryDisk>::default())
    }

    fn get_size(&self) -> Result<i64> {
        let size: i64 = 1024; // TODO:
        Ok(size)
    }

    fn read_at(&self, buf: &mut [u8], offset: u64) -> Result<()> {
        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(self.file.clone())
            .unwrap();
        f.seek(std::io::SeekFrom::Start(offset)).unwrap();
        let _ = f.read(buf);
        Ok(())
    }

    fn write_at(&self, buf: &[u8], offset: u64, _flags: Flags) -> Result<()> {
        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(self.file.clone())
            .unwrap();
        f.seek(std::io::SeekFrom::Start(offset)).unwrap();
        let _ = f.write_all(buf);
        Ok(())
    }

    fn thread_model() -> Result<ThreadModel> {
        Ok(ThreadModel::SerializeConnections)
    }
}

plugin!(MemoryDisk { thread_model });
