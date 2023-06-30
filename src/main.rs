use anyhow::Ok;
use cpal::{
    traits::{DeviceTrait, HostTrait},
    SampleFormat, StreamError,
};

fn main() -> anyhow::Result<()> {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("Failed to get default input device");

    let config = dbg!(device
        .default_input_config()
        .expect("Failed to get default input config"));

    let err_fn = move |err: StreamError| {
        eprintln!("an error occurred on stream: {}", err);
    };

    let stream = match config.sample_format() {
        SampleFormat::I8 => todo!(),
        SampleFormat::I16 => todo!(),
        SampleFormat::I32 => todo!(),
        SampleFormat::I64 => todo!(),
        SampleFormat::F32 => todo!(),
        SampleFormat::F64 => todo!(),
        _ => return Err(anyhow::Error::msg("Unsupported sample format")),
    };

    Ok(())
}
