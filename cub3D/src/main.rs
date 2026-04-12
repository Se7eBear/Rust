// use std::{
//     clone,
//     num::{NonZero, NonZeroU32},
//     sync::Arc,
// };

// use softbuffer::{Buffer, Context, Surface};
// use winit::{
//     application::ApplicationHandler,
//     dpi::LogicalSize,
//     event::WindowEvent,
//     event_loop::{self, EventLoop},
//     window::{self, Window},
// };

// #[derive(Debug, Clone)]
// struct Vec2 {
//     x: f32,
//     y: f32,
// }

// impl Vec2 {
//     fn new(x: f32, y: f32) -> Self {
//         Self { x, y }
//     }

//     fn to_px(&self, size: Vec2) -> Self {
//         Self {
//             x: self.x + (size.x / 2.0),
//             y: -self.y + (size.y / 2.0),
//         }
//     }
// }

// struct App {
//     window: Option<Arc<Window>>,
//     surface: Option<Surface<Arc<Window>, Arc<Window>>>,
//     angle: f32,
// }

// impl ApplicationHandler for App {
//     fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
//         let window = Arc::new(
//             event_loop
//                 .create_window(
//                     Window::default_attributes()
//                         .with_title("Alelo3D")
//                         .with_inner_size(LogicalSize::new(800, 800)),
//                 )
//                 .unwrap(),
//         );

//         let context = Context::new(window.clone()).unwrap();
//         let surface = Surface::new(&context, window.clone()).unwrap();

//         self.surface = Some(surface);
//         self.window = Some(window);
//     }

//     fn window_event(
//         &mut self,
//         event_loop: &winit::event_loop::ActiveEventLoop,
//         window_id: window::WindowId,
//         event: winit::event::WindowEvent,
//     ) {
//         match event {
//             WindowEvent::CloseRequested => event_loop.exit(),
//             WindowEvent::RedrawRequested => {
//                 let window = self.window.as_ref().unwrap();
//                 let surface = self.surface.as_mut().unwrap();
//                 let size = window.inner_size();
//                 let width = NonZeroU32::new(size.width).unwrap();
//                 let height = NonZeroU32::new(size.height).unwrap();
//                 surface.resize(width, height).unwrap();

//                 let mut buffer = surface.buffer_mut().unwrap();
//                 buffer.fill(0x00000000);

//                 let screen = Vec2::new(size.width as f32, size.height as f32);
//                 let radius = 50;
//                 let thickness = 2;
//                 for xd in -radius..radius {
//                     for yd in -radius..radius {
//                         if (xd * xd + yd * yd) >= (radius - thickness) * (radius - thickness)
//                             && (xd * xd + yd * yd) <= radius * radius
//                         {
//                             let pixel = Vec2::new(xd as f32, yd as f32).to_px(screen.clone());
//                             let x = pixel.x as usize;
//                             let y = pixel.y as usize;
//                             buffer[y * screen.x as usize + x] = 0x00FFFFFF
//                         }
//                     }
//                 }

//                 buffer.present().unwrap();
//                 window.request_redraw();
//             }
//             _ => {}
//         }
//     }
// }

// fn main() {
//     let event_loop = EventLoop::new().unwrap();
//     let mut app = App {
//         window: None,
//         surface: None,
//         angle: 0.0,
//     };
//     event_loop.run_app(&mut app).unwrap();
// }

use std::{num::NonZeroU32, sync::Arc};

use softbuffer::{Context, Surface};
use winit::{
    application::ApplicationHandler, dpi::LogicalSize, event::WindowEvent, event_loop::EventLoop,
    window::Window,
};

struct App {
    window: Option<Arc<Window>>,
    surface: Option<Surface<Arc<Window>, Arc<Window>>>,
    angle: f32,
}

#[derive(Debug, Clone)]
struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    fn transform_to_pixel(pos: Vec2, size: Vec2) -> Self {
        Self {
            x: pos.x + (size.x / 2.0),
            y: -pos.y + (size.y / 2.0),
        }
    }

    fn to_pixel(&self, size: &Vec2) -> Self {
        Self {
            x: self.x + (size.x / 2.0),
            y: -self.y + (size.y / 2.0),
        }
    }

    fn dist_sq(&self, other: &Vec2) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx) + (dy * dy)
    }
}

#[derive(Debug)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    fn project(&self, fov: f32) -> Vec2 {
        Vec2::new(self.x / self.z * fov, self.y / self.z * fov)
    }

    fn rotate_y(&self, angle: f32) -> Vec3 {
        Vec3::new(
            self.x * angle.cos() - self.z * angle.sin(),
            self.y,
            self.x * angle.sin() + self.z * angle.cos(),
        )
    }

    fn rotate_x(&self, angle: f32) -> Vec3 {
        Vec3::new(
            self.x,
            self.y * angle.cos() - self.z * angle.sin(),
            self.y * angle.sin() + self.z * angle.cos(),
        )
    }
}

fn cube_vertices() -> [Vec3; 8] {
    [
        Vec3::new(-1.0, -1.0, -1.0),
        Vec3::new(1.0, -1.0, -1.0),
        Vec3::new(1.0, 1.0, -1.0),
        Vec3::new(-1.0, 1.0, -1.0),
        Vec3::new(-1.0, -1.0, 1.0),
        Vec3::new(1.0, -1.0, 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(-1.0, 1.0, 1.0),
    ]
}

#[derive(Debug)]
struct Circle {
    size: f32,
    radius: f32,
}

impl Circle {
    fn new() {}
}

const EDGES: [(usize, usize); 12] = [
    (0, 1),
    (1, 2),
    (2, 3),
    (3, 0),
    (4, 5),
    (5, 6),
    (6, 7),
    (7, 4),
    (0, 4),
    (1, 5),
    (2, 6),
    (3, 7),
];

fn draw_line(buffer: &mut [u32], width: usize, height: usize, a: &Vec2, b: &Vec2, color: u32) {
    let mut x0 = a.x as i32;
    let mut y0 = a.y as i32;
    let x1 = b.x as i32;
    let y1 = b.y as i32;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    loop {
        if x0 >= 0 && y0 >= 0 && (x0 as usize) < width && (y0 as usize) < height {
            buffer[y0 as usize * width + x0 as usize] = color;
        }

        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }
        if e2 < dx {
        	err += dx;
         	y0 += sy;
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("Bouncing Ball")
                        .with_inner_size(LogicalSize::new(1920, 1080))
                        .with_maximized(true)
                )
                .unwrap(),
        );

        let context = Context::new(window.clone()).unwrap();
        let surface = Surface::new(&context, window.clone()).unwrap();

        self.surface = Some(surface);
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                let window = self.window.as_ref().unwrap();
                let surface = self.surface.as_mut().unwrap();
                let size = window.inner_size();
                let width = NonZeroU32::new(size.width).unwrap();
                let height = NonZeroU32::new(size.height).unwrap();
                surface.resize(width, height).unwrap();

                let mut buffer = surface.buffer_mut().unwrap();
                buffer.fill(0x00000000);

                let w = size.width as usize;
                let h = size.height as usize;
                let screen = Vec2::new(size.width as f32, size.height as f32);
                let fov = 400.0;
                let dist = 4.0;

                self.angle += 0.01;

                let projected: Vec<Vec2> = cube_vertices()
                    .iter()
                    .map(|v| {
                        let rotated = v.rotate_x(self.angle).rotate_y(self.angle);
                        let translated = Vec3::new(rotated.x, rotated.y, rotated.z + dist);
                        translated.project(fov).to_pixel(&screen)
                    })
                    .collect();

                for (a, b) in EDGES {
                    draw_line(&mut buffer, w, h, &projected[a], &projected[b], 0x00FFFFFF);
                }

                buffer.present().unwrap();

                window.request_redraw();
            }
            _ => {}
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let mut app = App {
        window: None,
        surface: None,
        angle: 0.0,
    };
    event_loop.run_app(&mut app).unwrap();
}
