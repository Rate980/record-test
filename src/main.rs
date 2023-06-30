use anyhow::Ok;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    SampleFormat, StreamError,
};
use tokio::sync::mpsc::Sender;

async fn send(channel_tx: Sender<i16>, data: &[i16]) {
    for sample in data {
        channel_tx.send(*sample).await.unwrap();
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

    let (tx, mut rx) = tokio::sync::mpsc::channel::<i16>(1 << 10);

    let stream = match config.sample_format() {
        SampleFormat::I8 => device.build_input_stream(
            &config.into(),
            move |data: &[u8], _: &_| {
                let data = data
                    .iter()
                    .map(|x| *x as i16)
                    .map(|x| x << 8)
                    .collect::<Vec<i16>>();
                futures::executor::block_on(send(tx.clone(), &data))
            },
            err_fn,
            None,
        )?,
        SampleFormat::I16 => todo!(),
        SampleFormat::I32 => todo!(),
        SampleFormat::I64 => todo!(),
        SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |data: &[f32], _: &_| {
                let data = data
                    .iter()
                    .map(|x| (i16::MAX as f32) * x)
                    .map(|x| x as i16)
                    .collect::<Vec<i16>>();
                futures::executor::block_on(send(tx.clone(), &data))
            },
            err_fn,
            None,
        )?,
        SampleFormat::F64 => todo!(),
        _ => return Err(anyhow::Error::msg("Unsupported sample format")),
    };

    stream.play()?;
    tokio::spawn(async move {
        while let Some(sample) = rx.recv().await {
            // let samples = Vec::new();

            tokio::task::spawn_blocking(move || {
                println!("{}", sample);
            })
            .await
            .unwrap();
        }
    })
    .await?;
    Ok(())
}
