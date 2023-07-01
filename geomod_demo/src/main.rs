mod renderer;

use crate::renderer::Renderer;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::window::Window;
use glutin::{Api, ContextBuilder, GlRequest};

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new().with_title("Learn OpenGL with Rust");

    let gl_context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .build_windowed(window, &event_loop)
        .expect("Cannot create windowed context");

    let gl_context = unsafe {
        gl_context
            .make_current()
            .expect("Failed to make context current")
    };

    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    let mut renderer = Renderer::new().expect("Cannot create renderer");

    let mut is_dragging: bool = false;

    struct Pos {
        x: f32,
        y: f32
    }

    let mut cursor_drag_origin: Option<Pos> = None;
    let mut grid_drag_origin: Pos = Pos {x: 0.0, y: 0.0};

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => (),
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CursorMoved { position, ..}   => {
                    println!("{}", format!("mouse moved to {} {}", position.x, position.y));
                    if (is_dragging) {
                        if (cursor_drag_origin.is_none()) {
                            cursor_drag_origin = Some(Pos { x: position.x as f32, y: position.y as f32} );
                            grid_drag_origin = Pos {x: renderer.gridoffset[0], y: renderer.gridoffset[1]};
                        }

                        renderer.gridoffset[0] = grid_drag_origin.x + -(position.x as f32 - cursor_drag_origin.as_ref().unwrap().x as f32);
                        renderer.gridoffset[1] = grid_drag_origin.y +(position.y as f32 - cursor_drag_origin.as_ref().unwrap().y as f32);


                    }
                    renderer.mousepos = ([-position.x as f32, position.y as f32]); // flip x for some reason
                    gl_context.window().request_redraw();
                },
                WindowEvent::MouseWheel { delta, ..}   => {
                    match delta { 
                        glutin::event::MouseScrollDelta::LineDelta(x, y) => {
                            println!("{}", format!("axis moved to {}", y));
                            renderer.gridscale += y;
                            gl_context.window().request_redraw();
                        }, 
                        _ => { }
                    }
                    
                    //renderer.mousepos = ([-position.x as f32, position.y as f32]); // flip x for some reason
                   
                },

                WindowEvent::MouseInput { button, state, ..}   => {
                    
                    if (button == glutin::event::MouseButton::Middle) {
                        if (state == glutin::event::ElementState::Pressed) {
                            is_dragging = true;
                            println!("middle down");
                        } else {
                            is_dragging = false;
                            println!("middle up");
                            cursor_drag_origin = None;
                        }
                    }
                   
                },
                WindowEvent::Resized(physical_size) => {
                    gl_context.resize(physical_size);
                    gl_context.window().set_title(&format!("GEOMOD demo - {},{}", physical_size.width, physical_size.height)[..]);
                },
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {
                println!("draw!");
                renderer.draw();
                gl_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}