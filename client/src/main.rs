extern crate ffmpeg_next as ffmpeg;

use std::env;
use std::error::Error;
use std::time::Duration;
use std::thread;

use tokio::net::TcpStream;
use tokio::io::{self, Interest, AsyncWriteExt};

use sdl2::event::Event;
use sdl2::keyboard::Keycode; 
use sdl2::pixels::PixelFormatEnum;

use ffmpeg::media;
use ffmpeg::format::{Pixel, input};
use ffmpeg::util::frame::video::Video;
use ffmpeg::codec::Context as CodecContext;
use ffmpeg::software::scaling::{context::Context, flag::Flags};


const BEGIN_STREAM:u8 = 123;

fn main()-> Result<(), Box<dyn Error>>{
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 960, 540)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build()?;
    let mut event_pump = sdl_context.event_pump()?;
    let texture_creator = canvas.texture_creator();

    ffmpeg::init()?;

    let mut ictx = input(&env::args().nth(1).expect("Cannot open file."))?;
    let input = ictx
        .streams()
        .best(media::Type::Video)
        .ok_or(ffmpeg::Error::StreamNotFound)?;

    let video_stream_index = input.index();

    let context_decoder = CodecContext::from_parameters(input.parameters())?;
    let mut decoder = context_decoder.decoder().video()?;
    let mut scaler = Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        Pixel::RGB24,
        960,
        540,
        Flags::BILINEAR,
    )?;


    println!("format {:?}", decoder.format());

    while let Some((stream, packet)) = ictx.packets().next(){
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    println!("process stopped");
                    return Ok(());             
                },
                _ => {}
            }
        }

        if stream.index() == video_stream_index{
            decoder.send_packet(&packet)?;
            let mut decoded = Video::empty();
            while decoder.receive_frame(&mut decoded).is_ok() {
                let mut rgb_frame = Video::empty();
                scaler.run(&decoded, &mut rgb_frame)?;

                let mut texture = texture_creator.create_texture_streaming(
                    PixelFormatEnum::RGB24, 
                    960, 
                    540,
                )?;
                texture.update(
                    None, 
                    rgb_frame.plane::<[u8;3]>(0).concat().as_slice(), 
                    3*rgb_frame.width() as usize
                )?;
                //println!("width {}", rgb_frame.width());

                canvas.clear();
                canvas.copy(&texture, None, None)?;
                canvas.present();
                //println!("frame");
                thread::sleep(Duration::from_millis(16));
            }   
        }
    }
    return Ok(());
}




#[allow(dead_code)]
async fn not_main()-> Result<(), Box<dyn Error>>{
    let mut connection = TcpStream::connect("127.0.0.1:6969").await?;
    loop{
        let connection_ready = connection.ready(Interest::WRITABLE).await?;
        if connection_ready.is_writable(){
            connection.write_all(&[BEGIN_STREAM]).await?;
            break;
        }
    }
    let mut incomming_message_buffer = [0; 256];
    loop{
        let ready = connection.ready(Interest::READABLE).await?;
        if ready.is_readable() {
            match connection.try_read(&mut incomming_message_buffer){
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e)=>{panic!("paniced with {}", e)}
                Ok(c)=>{println!("read {} bytes", c)}
            }
            let mess = String::from_utf8(incomming_message_buffer.to_vec())?;
            println!("{}", mess);
        }
    }
}
