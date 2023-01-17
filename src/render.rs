
use gfx;
use gfx::Device;
use gfx::traits::FactoryExt;
use glutin::GlContext;

use crate::layout;
use crate::dom;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;





const WIDTH: i16 = 1024;
const HEIGHT: i16 = 768;
const CLEAR_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 4] = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

const COLOR1: [f32; 4] = [0.8, 0.2, 0.2, 0.2];
const COLOR2: [f32; 4] = [0.2, 0.2, 0.8, 0.2];
const TRIANGLE: [Vertex; 4] = [
    Vertex { pos: [-0.1, -0.1], color: COLOR1 },
    Vertex { pos: [-0.5, 0.5], color: COLOR1 },
    Vertex { pos: [0.2, 0.2], color: COLOR1 },
    Vertex { pos: [0.2, -0.2], color: COLOR1 }
];

const TRIANGLE1: [Vertex; 4] = [
    Vertex { pos: [-0.2, -0.2], color: COLOR2 },
    Vertex { pos: [-0.2, 0.2], color: COLOR2 },
    Vertex { pos: [0.2, 0.2], color: COLOR2 },
    Vertex { pos: [0.2, -0.2], color: COLOR2 }
];

fn transform_rectangle(rect: &layout::Rect) -> (f32, f32, f32, f32) {
    let w = rect.width as f32 / WIDTH as f32 * 2.0;
    let h = rect.height as f32 / HEIGHT as f32 * 2.0;
    let x = rect.x as f32 / WIDTH as f32 * 2.0 - 1.0;
    let y = -(rect.y as f32 / HEIGHT as f32 * 2.0 - 1.0 + h);

    (x, y, h, w)
}


fn render_rect(color: &layout::Color, rect: &layout::Box) -> Vec<Vertex> {
    let (w, h, x, y) = transform_rectangle(&rect.rect);
    vec![Vertex{pos: [x, y], color: color.to_array()},
         Vertex{pos: [x, y + h], color: color.to_array()},
         Vertex{pos: [x + w, y + h], color: color.to_array()},
         Vertex{pos: [x + w, y], color: color.to_array()}]
}

pub fn render(nodes: Vec<dom::Node>) {
    let mut vertices = Vec::new();
    let mut index_data = Vec::new();
    let mut rect_num: u16 = 0;
    let mut v = TRIANGLE.to_vec();
    // for node in nodes {
    //     match node.node_type {
    //         dom::NodeType::Element(element) => {
    //             let mut v = render_rect(&element.color, &element.rect);
    //             vertices.append(&mut v);
    //             let index_base: u16 = rect_num * 4;
    //             index_data.append(&mut vec![
    //                 index_base,
    //                 index_base + 1,
    //                 index_base + 2,
    //                 index_base + 2,
    //                 index_base + 3,
    //                 index_base,
    //             ]);
    //             rect_num += 1;
    //         }
    //         dom::NodeType::Text(text) => {
    //             // todo
    //         }
    //     }
    // }
    vertices.append(&mut v);

    let index_base: u16 = rect_num * 4;
    index_data.append(&mut vec![
        index_base,
        index_base + 1,
        index_base + 2,
        index_base + 2,
        index_base + 3,
        index_base,
    ]);
    rect_num += 1;
    // vertices.append(&mut v1);
    let index_base: u16 = rect_num * 4;
    index_data.append(&mut vec![
        index_base,
        index_base + 1,
        index_base + 2,
        index_base + 2,
        index_base + 3,
        index_base,
    ]);
    let builder = glutin::WindowBuilder::new()
        .with_title("My First Triangle".to_string())
        .with_dimensions(640, 480);

    let gl_builder = glutin::ContextBuilder::new().with_vsync(true);
    let mut events_loop = glutin::EventsLoop::new();
    let (window, mut device, mut factory, main_color, _main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, gl_builder, &events_loop);

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let pso = factory
        .create_pipeline_simple(
            include_bytes!("box.glslv"),
            include_bytes!("box.glslf"),
            pipe::new(),
        )
        .unwrap();

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&vertices, &index_data[..]);

    let data = pipe::Data {
        vbuf: vertex_buffer,
        out: main_color,
    };
    // let mut text = gfx_text::new(factory).build().unwrap();

    // text.add(
    //     "The quick brown fox jumps over the lazy dog",
    //     [10, 10],
    //     [0.65, 0.16, 0.16, 1.0],
    // );


    let mut running = true;
    while running {
        // fetch events
        events_loop.poll_events(|event| {
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    glutin::WindowEvent::Closed => running = false,
                    glutin::WindowEvent::KeyboardInput {
                        input:
                        glutin::KeyboardInput {
                            virtual_keycode: Some(glutin::VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => return,
                    _ => (),
                }
            }
        });

        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.draw(&slice, &pso, &data);
        // text.draw(&mut encoder, &data.out);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}