use tokio::io::{self, Interest};
use tokio::net::TcpStream;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

use std::error::Error;

//const BEGIN_STREAM: u8 = 123;



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("let the pain begin");
    let sdl_context = sdl2::init().unwrap();
    println!("created sdl context");
    let video_subsystem = sdl_context.video().unwrap();
    println!("created sdl video subsystem");
    let window = video_subsystem
        .window("rust-sdl2 demo", 640, 360)
        .position_centered()
        .build()
        .unwrap();

    println!("created sdl window");

    let mut canvas = window.into_canvas().build()?;
    let mut event_pump = sdl_context.event_pump()?;
    let texture_creator = canvas.texture_creator();

    let connection = TcpStream::connect("127.0.0.1:6969").await?;
    let mut pixel_buf:Vec<u8> = Vec::with_capacity(691200);
    
    loop {
        let ready = connection.ready(Interest::READABLE).await?;
        if ready.is_readable() {
            match connection.try_read(&mut pixel_buf) {
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    panic!("paniced with {}", e)
                }
                Ok(ref c) if *c==0=>{continue;}
                Ok(c) => {
                    println!("read {} bytes", c);
                    let mut texture = texture_creator.create_texture_streaming(
                        PixelFormatEnum::RGB24,
                        640,
                        360,
                    )?;
                    texture.update(None, pixel_buf.as_slice(), 3 * 640)?;

                    canvas.clear();
                    canvas.copy(&texture, None, None)?;
                    canvas.present();
                }
            }

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        println!("process stopped");
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }
    }
}



/*fn not_main()-> Result<(), Box<dyn Error>>{
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 640, 360)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build()?;
    let mut event_pump = sdl_context.event_pump()?;
    let texture_creator = canvas.texture_creator();

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
                    640,
                    360,
                )?;
                texture.update(
                    None,
                    rgb_frame.plane::<[u8;3]>(0).concat().as_slice(),
                    3*rgb_frame.width() as usize
                )?;
                println!("size: {}", 3*rgb_frame.width()*rgb_frame.width());
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
*/