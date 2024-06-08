use std::cmp::max;
use std::thread::available_parallelism;
use tokio::runtime;
use tokio::runtime::Runtime;
use v4l::{Device, FourCC};
use v4l::video::Capture;

fn main() {
    let core_count = if let Ok(parallelism_result) = available_parallelism() {
        parallelism_result.get()
    } else {
        panic!("Unable to retrieve core count");
    };

    let num_worker_threads = max(core_count - 1, 1);

    let runtime = create_runtime(num_worker_threads);

    let result = runtime.block_on(async {
        let mut device = Device::new(0)
            .map_err(Err("Unable to open device 0"))?;
        let mut format = device.format()
            .map_err(Err("Unable to read device format"))?;

        format.width = 800;
        format.height = 600;
        format.fourcc = FourCC::new(b"YUYV");
        let format = device.set_format(&format)
            .map_err(Err("Unable to write device format"))?;

        Ok(format)
    });

    match result {
        Ok(format) => println!("Format was: {format}"),
        Err(error) => println!("Error encountered: {error}")
    }
}

fn create_runtime(worker_threads: usize) -> Runtime {
    runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(worker_threads)
        .build()
        .expect("Unable to create tokio runtime")
}

