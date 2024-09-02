use lazy_static::lazy_static;
use nbdkit::*;
use std::sync::Mutex;

// The disk.
lazy_static! {
    static ref SIZE: usize = 1024 * 1024; // 1Mo
    static ref DISK: Mutex<Vec<u8>> = Mutex::new(vec![0; 0]);
}

#[derive(Default)]
struct MemoryDisk {
    _dummy: u8, // it is needed otherwise Box::new won't allocate anything...
}

impl Server for MemoryDisk {
    fn name() -> &'static str {
        "memorydisk"
    }

    fn open(_readonly: bool) -> Result<Box<dyn Server>> {
        // Write the header "Hello, Sailor!" at the beginning of the disk.
        let header = b"Hello, Sailor!";
        DISK.lock().unwrap()[..header.len()].copy_from_slice(header);

        Ok(Box::<MemoryDisk>::default())
    }

    fn get_size(&self) -> Result<i64> {
        Ok(DISK.lock().unwrap().len() as i64)
    }

    fn read_at(&self, buf: &mut [u8], offset: u64) -> Result<()> {
        let disk = DISK.lock().unwrap();
        let begin = offset as usize;
        let end = begin + buf.len();
        buf.copy_from_slice(&disk[begin..end]);
        Ok(())
    }

    fn thread_model() -> Result<ThreadModel> {
        Ok(ThreadModel::SerializeConnections)
    }

    fn get_ready() -> Result<()> {
        *DISK.lock().unwrap() = vec![0; *SIZE];
        Ok(())
    }
}

plugin!(MemoryDisk {
    thread_model,
    get_ready
});
