#[macro_use]
extern crate glium;
//extern crate sdl2;
extern crate gif;
pub mod ico_sphere;

use std::time::{SystemTime, UNIX_EPOCH, Instant, Duration};
use glium::texture::RawImage2d;
use std::fs::File;

// Return the second time of time eg SS.MMM of HH:MM:SS.MMM
fn seconds() -> f32 {
    let now = SystemTime::now();
    let since_epoch = now
        .duration_since(UNIX_EPOCH)
        .expect("Error with time since unix epoch calculation");
    
    (since_epoch.as_secs() % 60) as f32 + ( since_epoch.subsec_millis() as f32 / 1000.0 )
}

fn get_gif_frame(display: &glium::Display) -> gif::Frame {
    let nested: Vec<Vec<(u8, u8, u8, u8)>> = display.read_front_buffer().unwrap();

    let mut res: Vec<u8> = nested
        .iter()
        .flatten()
        .map(|tup| -> Vec<u8> {
            vec![tup.0, tup.1, tup.2, tup.3]
        })
        .flatten()
        .collect();
    
    gif::Frame::from_rgba(nested.len() as u16, nested[0].len() as u16, &mut res)
}

fn render_save(filename: &String, n_frames: u32) -> () {
    ()
}

fn main() {
    use glium::glutin;
    use glium::glutin::dpi::{PhysicalSize, LogicalSize, Pixel};
    use glium::Surface;
    
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(LogicalSize { height: 700, width: 700 })
        .with_title("ball");
    
    let cb = glutin::ContextBuilder::new();
    
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let sphere: ico_sphere::IcoSphere = ico_sphere::IcoSphere::new(0.7, 3);
    //println!("{:?}", sphere.vertices());

    let vertex_buffer = glium::VertexBuffer::new(&display, &sphere.vertices).unwrap();

    let program = glium::Program::from_source(&display, include_str!("shader.vert"), include_str!("shader.frag"), None).unwrap();

    //println!("Vertices: {:?}", &sphere.vertices);
    //println!("Indices: {:?}", &sphere.indices);
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &sphere.indices).unwrap();
    let point_indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);

    let params = glium::DrawParameters {
        point_size: Some(0.0),
        polygon_mode: glium::draw_parameters::PolygonMode::Line,
        ..Default::default()
    };

    let time_s: f32 = seconds();
    //println!("{}", time_s);
    let uniforms = glium::uniform! { time_s: time_s };

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 0.0, 1.0);
    target.draw(
        &vertex_buffer, 
        &indices, 
        &program, 
        &uniforms,
        &params
    ).unwrap();

    target.finish().unwrap();

    display.gl_window().window().request_redraw();

    //println!("{:?}", snapshot);
    /*
    target.draw(
        &vertex_buffer, 
        &point_indices, 
        &program, 
        &glium::uniforms::EmptyUniforms,
        &params
    ).unwrap();
    */
    let init_frame = get_gif_frame(&display);
    //println!("{:?}", init_frame);

    let n_frames: u32 = 60;
    let mut count: u32 = 0;
    let mut image_file = File::create("test.gif").unwrap();
    let mut encoder = gif::Encoder::new(
        image_file,
        init_frame.width,
        init_frame.height,
        &[]
    ).unwrap();

    // for i in 0..20 {
    //     std::thread::sleep(Duration::from_millis(100));
    // }
    //let mut stored_frames: Vec<gif::Frame> = Vec::new();
    
    let mut callback = |ev, _, control_floow| {
        println!("{:?}", ev);
        let time_s: f32 = seconds();
        //println!("{}", time_s);
        let uniforms = glium::uniform! { time_s: time_s };

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(
            &vertex_buffer, 
            &indices, 
            &program, 
            &uniforms,
            &params
        ).unwrap();
        
        target.finish().unwrap();
    };

    for i in 0..n_frames {
        let mut control_flow = ControlFlow::default();
        event_loop.drain_events(&mut callback, &mut control_flow);
        //let frame = get_gif_frame(&display);
        //encoder.write_frame(&frame).unwrap();
    }

    /*
    event_loop.run(move |ev, _, control_flow| {
        println!("{:?}", ev);
        let time_s: f32 = seconds();
        //println!("{}", time_s);
        let uniforms = glium::uniform! { time_s: time_s };

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(
            &vertex_buffer, 
            &indices, 
            &program, 
            &uniforms,
            &params
        ).unwrap();

        target.finish().unwrap();

        //let frame = get_gif_frame(&display);
        //stored_frames.push(frame);
        //encoder.write_frame(&frame).unwrap();

        let next_frame_time = Instant::now() +
            Duration::from_nanos(2 * 16_666_667);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => (),
        }

        // if count >= n_frames {
        //     println!("{}", count);
        //     *control_flow = glutin::event_loop::ControlFlow::Exit;
        //     return;
        //  }
        count += 1;
    });
    */
    println!("Hi");

}