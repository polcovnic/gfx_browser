use std::any::Any;
use std::thread::sleep;
use std::time::Duration;
use gfx;
use gfx_text;
use gfx::Device;
use gfx::traits::FactoryExt;
use glutin::GlContext;
use gfx::Factory;
use gfx_text::Renderer;
use crate::css_parser::CssParser;

use crate::layout;
use crate::dom;
use crate::dom::NodeType;
use crate::html_parser::HtmlParser;
use crate::layout::{Color, Content, LayoutBox};

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;


const WIDTH: u32 = 600;
const HEIGHT: u32 = 600;
const CLEAR_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

gfx_defines! {
    #[derive(PartialEq)]
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
    let w = box_.actual_dimensions.width as f32 / (WIDTH / 2) as f32;
    let h = box_.actual_dimensions.height as f32 / (HEIGHT / 2) as f32;
    let x = box_.actual_dimensions.x as f32 / WIDTH as f32 * 2.0 - 1.0;
    let y = -(box_.actual_dimensions.y as f32 / HEIGHT as f32 * 2.0 - 1.0);

    (w, h, x, y)
}

fn render_content(box_: &LayoutBox) -> Vec<Vertex> {
    let (w, h, x, y) = transform_rectangle(box_);
    // println!("w: {}, h: {}, x: {}, y: {}", w, h, x, y);
    // println!("1: {}, {} \n2: {}, {} \n3: {}, {} \n4: {}, {}", x, y, x, y - h, x + w, y - h, x + w, y);
    vec![Vertex { pos: [x, y], color: box_.background_color.to_array() },
         Vertex { pos: [x, y - h], color: box_.background_color.to_array() },
         Vertex { pos: [x + w, y - h], color: box_.background_color.to_array() },
         Vertex { pos: [x + w, y], color: box_.background_color.to_array() }]
}

pub fn render_rec(box_: &LayoutBox) {
    if box_.children.len() != 0 {
        for child in &box_.children {
            render_rec(child);
        }
    }
}

fn layout_box_tree_to_vector_helper(boxes_tree: Vec<LayoutBox>) -> Vec<LayoutBox> {
    let mut boxes_out = Vec::new();
    for node in boxes_tree {
        boxes_out.push(node.clone());
        boxes_out.append(&mut layout_box_tree_to_vector_helper(node.children));
    }
    boxes_out
}

pub fn layout_box_tree_to_vector(boxes_tree: LayoutBox) -> Vec<LayoutBox> {
    let mut boxes = Vec::new();
    boxes.push(boxes_tree.clone());
    boxes.append(&mut layout_box_tree_to_vector_helper(boxes_tree.children));
    boxes
}

// fn get_vertices(boxes: &Vec<LayoutBox>) -> (Vec<Vertex>, Vec<u16>, Vec<(&'static str, [i32; 2], [f32; 4])>){
//     let boxes = layout_box_tree_to_vector(boxes[0].clone());
//     let mut vertices = Vec::new();
//     let mut index_data = Vec::new();
//     let mut text_vec = Vec::new();
//     for (rect_num, box_) in boxes.iter().enumerate() {
//         if let Some(text) = &box_.content.text{
//             println!("text: {}", text);
//             text_vec.push((
//                 text.as_str(),
//                 [10 + box_.actual_dimensions.y as i32, 10 + box_.actual_dimensions.x as i32],
//                 box_.color.to_array())
//             );
//         } else {
//             // println!("color: {}, {}, {}", box_.background_color.to_array()[0],
//             //          box_.background_color.to_array()[1], box_.background_color.to_array()[2]);
//             println!("name: {:?}", box_.name);
//             // println!("position: {}, {}", box_.actual_dimensions.x, box_.actual_dimensions.y);
//             let mut v = render_content(box_);
//             vertices.append(&mut v);
//             let index_base: u16 = rect_num as u16 * 4;
//             index_data.append(&mut vec![
//                 index_base,
//                 index_base + 1,
//                 index_base + 2,
//                 index_base + 2,
//                 index_base + 3,
//                 index_base,
//             ]);
//         }
//
//     }
//     (vertices, index_data, text_vec)
// }

pub fn render(boxes: Vec<LayoutBox>) {
    let boxes = layout_box_tree_to_vector(boxes[0].clone());
    let mut vertices = Vec::new();
    let mut index_data = Vec::new();
    let mut text_vec = Vec::new();
    let mut rect_num = 0;
    for box_ in boxes.iter() {
        if let Some(text) = &box_.content.text{
            println!("text: {}", text);
            text_vec.push((
                text.as_str(),
                [10 + box_.actual_dimensions.y as i32, 10 + box_.actual_dimensions.x as i32],
                box_.color.to_array())
            );

        } else {
            // println!("color: {}, {}, {}", box_.background_color.to_array()[0],
            //          box_.background_color.to_array()[1], box_.background_color.to_array()[2]);
            println!("name: {:?}", box_.name);
            // println!("position: {}, {}", box_.actual_dimensions.x, box_.actual_dimensions.y);
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
            rect_num += 1;
        }

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

    let mut text_renderer = gfx_text::new(factory).build().unwrap();
    let data = pipe::Data {
        vbuf: vertex_buffer,
        out: main_color,
    };


    let mut running = true;
    while running {
        for event in window.poll_events() {
            match event {
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
                glutin::Event::Closed => running = false,
                _ => {}
            }
        }

        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.draw(&slice, &pso, &data);

        for text in &text_vec {
            text_renderer.add(text.0, text.1, text.2);
        }

        sleep(Duration::from_millis(10));

        text_renderer.draw(&mut encoder, &data.out).unwrap();
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}


// #[test]
// fn test_render() {
//     let html1 = r#"
// <html>
//
// <head>
//     <link rel="stylesheet" type="text/css" href="style.css"></link>
//     <title>Example</title>
// </head>
//
// <body>
// <div id="blue">f</div>
// <div class="orange"></div>
// <div class="black"></div>
// <div class="green"></div>
//
// </body>
//
// </html>
//     "#;
//     let html2 = r#"
// <html>
//
// <head>
//     <link rel="stylesheet" type="text/css" href="style.css"></link>
//     <title>Example</title>
// </head>
//
// <body>
// <div id="blue"></div>
// <div class="orange"></div>
// <div class="black"></div>
// <div class="green"></div>
//
// </body>
//
// </html>
//     "#;
//     let css = r#"
//     .orange {
//     background-color: #ff6600;
//     padding: 20px;
//     margin: 50px;
// }
//
// #blue{
//     background-color: #0a1e77;
//     padding: 20px;
//     margin: 10px;
// }
//
// .black {
//     background-color: #000000;
//     padding: 20px;
//     margin: 30px;
// }
//
// .green {
//     background-color: #2ebe1a;
//     padding: 20px;
//     margin: 10px;
// }
//
//     "#;
//     let mut html_parser = HtmlParser::new(html1);
//     let nodes = html_parser.parse_nodes();
//     let mut body1 = nodes[0].children[1].clone();
//     let mut css_parser = CssParser::new(css);
//     let stylesheet = css_parser.parse_stylesheet();
//     body1.add_styles(&stylesheet);
//     let boxes1 = layout::LayoutBox::build_layout_tree(&body1);
//     let boxes1 = crate::render::layout_box_tree_to_vector(boxes1[0].clone());
//     let mut html_parser = HtmlParser::new(html2);
//     let nodes = html_parser.parse_nodes();
//     let mut body2 = nodes[0].children[1].clone();
//     let mut css_parser = CssParser::new(css);
//     let stylesheet = css_parser.parse_stylesheet();
//     body2.add_styles(&stylesheet);
//     let boxes2 = layout::LayoutBox::build_layout_tree(&body2);
//     let boxes2 = crate::render::layout_box_tree_to_vector(boxes2[0].clone());
//     let (vertices1, index_data1) = render(boxes1);
//     let (vertices2, index_data2) = render(boxes2);
//     assert_eq!(vertices1, vertices2);
//     assert_eq!(index_data1, index_data2);
// }