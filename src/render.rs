use std::thread::sleep;
use std::time::Duration;
use gfx;
use gfx_text;
use gfx::Device;
use gfx::traits::FactoryExt;
use glutin::GlContext;
use gfx::Factory;

use crate::layout;
use crate::dom;
use crate::dom::NodeType;
use crate::layout::{Color, Content, LayoutBox};

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;


const WIDTH: u32 = 600;
const HEIGHT: u32 = 600;
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


fn transform_rectangle(box_: &LayoutBox) -> (f32, f32, f32, f32) {
    let w = box_.content.width as f32 / WIDTH as f32;
    let h = box_.content.height as f32 / HEIGHT as f32;
    let x = (box_.x as f32 / WIDTH as f32 * 2.0 - 1.0);
    let y = -(box_.y as f32 / HEIGHT as f32 * 2.0 - 1.0);

    (w, h, x, y)
}


fn render_content(box_: &LayoutBox) -> Vec<Vertex> {
    let (w, h, x, y) = transform_rectangle(box_);
    println!("w: {}, h: {}, x: {}, y: {}", w, h, x, y);
    println!("1: {}, {} \n2: {}, {} \n3: {}, {} \n4: {}, {}", x, y, x, y - h, x + w, y - h, x + w, y);
    vec![Vertex { pos: [x, y], color: box_.color.to_array() },
         Vertex { pos: [x, y - h], color: box_.color.to_array() },
         Vertex { pos: [x + w, y - h], color: box_.color.to_array() },
         Vertex { pos: [x + w, y], color: box_.color.to_array() }]
}

pub fn render_rec(box_: &LayoutBox) {
    if box_.children.len() != 0 {
        for child in &box_.children {
            render_rec(child);
        }
    }
}

fn box_tree_to_vector(boxes_tree: Vec<LayoutBox>) -> Vec<LayoutBox> {
    let mut boxes_out = Vec::new();
    for node in boxes_tree {
        boxes_out.push(node.clone());
        boxes_out.append(&mut box_tree_to_vector(node.children));
    }
    boxes_out
}

fn layout_box_tree_to_vector(boxes_tree: LayoutBox) -> Vec<LayoutBox> {
    let mut boxes = Vec::new();
    boxes.push(boxes_tree.clone());
    boxes.append(&mut box_tree_to_vector(boxes_tree.children));
    boxes
}

pub fn render(boxes: Vec<LayoutBox>) {
    let boxes = layout_box_tree_to_vector(boxes[0].clone());
    let mut vertices = Vec::new();
    let mut index_data = Vec::new();
    for (rect_num, box_) in boxes.iter().enumerate() {
        let mut v = render_content(box_);
        vertices.append(&mut v);
        let index_base: u16 = rect_num as u16 * 4;
        index_data.append(&mut vec![
            index_base,
            index_base + 1,
            index_base + 2,
            index_base + 2,
            index_base + 3,
            index_base,
        ]);
    }

    let builder = glutin::WindowBuilder::new()
        .with_title(String::from("Browser"))
        .with_dimensions(WIDTH, HEIGHT)
        .with_vsync();

    let (window, mut device, mut factory, main_color, _main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);

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
    let mut text = gfx_text::new(factory).build().unwrap();



    let mut running = true;
    while running {
// fetch events
        for event in window.poll_events() {
            match event {
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
                glutin::Event::Closed => running = false,
                _ => {}
            }
        }

        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.draw(&slice, &pso, &data);

        text.add(
            "The quick brown fox jumps over the lazy dog",
            [100, 100],
            [0.65, 0.16, 0.16, 1.0],
        );
        text.draw(&mut encoder, &data.out).unwrap();
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}