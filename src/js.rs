use std::collections::HashMap;
use v8;
use std::fs;
use std::io::Read;
use std::thread;
use std::time::Duration;
use crate::css::{PropertyName, PropertyValue};
use crate::css::PropertyValue::Color;
use crate::dom::{Node, NodeType};

use crate::NODES;


fn log_callback(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let message = args
        .get(0)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);
    println!("{}", message);
}

fn get_by_id_callback(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let arg = args.get(0);
    if let Some(id) = arg.to_string(scope) {
        let id_str = id.to_rust_string_lossy(scope);
        // let nodes = NODES.lock().unwrap(); // Отримуємо доступ до спільного ресурсу
        let nodes;
        unsafe {
            nodes = nodes_tree_to_vector(&NODES[0]);
        }
        for node in nodes {
            if let NodeType::Element(element_data) = &node.node_type {
                if let Some(node_id) = element_data.attributes.get("id") {
                    if node_id == &id_str {
                        let element_obj = v8::Object::new(scope);
                        let tag_name_key = v8::String::new(scope, "tagName").unwrap().into();
                        let tag_name_value = v8::String::new(scope, element_data.tag_name.as_str()).unwrap();
                        element_obj.set(scope, tag_name_key, tag_name_value.into());
                        let style_key = v8::String::new(scope, "style").unwrap().into();
                        let style_obj = v8::Object::new(scope);
                        println!("{:?}", node);
                        for (key, value) in &node.styles {
                            let key_str = dbg!(format!("{:?}", key));
                            let key = v8::String::new(scope, key_str.as_str()).unwrap().into();
                            let value_str = dbg!(format!("{:?}", value));
                            let value = v8::String::new(scope, value_str.as_str()).unwrap();
                            style_obj.set(scope, key, value.into());
                        }
                        element_obj.set(scope, style_key, style_obj.into());
                        rv.set(element_obj.into());
                    }
                }
            }
        }

    }
}


use v8::{FunctionCallback, FunctionCallbackArguments, ReturnValue};


use std::cell::RefCell;
use std::rc::Rc;

fn add_document_old<'a>(scope: &'a mut v8::ContextScope<v8::HandleScope>, nodes: &'a Vec<&'a Node>) -> v8::Local<'a, v8::Object> {
    // let nodes_rc = Rc::new(RefCell::new(nodes));

    let document_key = v8::String::new(scope, "document").unwrap().into();
    let document_obj = v8::Object::new(scope);

    for node in nodes {
        if let NodeType::Element(elem) = &node.node_type {
            let tag_key = v8::String::new(scope, elem.tag_name.as_str()).unwrap().into();
            let tag_obj = v8::Object::new(scope);

            let style_key = v8::String::new(scope, "style").unwrap().into();
            let style_obj = v8::Object::new(scope);

            for (property_name, property_value) in &node.styles {
                println!("Node: {:?}, {:?} {:?}", elem.tag_name, property_name.to_str(), property_value);
                let property_name_key = v8::String::new(scope, property_name.to_str()).unwrap().into();
                let property_value_key = v8::String::new(scope, property_value.to_str().as_str()).unwrap();
                style_obj.set(scope, property_name_key, property_value_key.into());
            }

            tag_obj.set(scope, style_key, style_obj.into());

            document_obj.set(scope, tag_key, tag_obj.into());
        }
    }

    scope.get_current_context().global(scope).set(scope, document_key, document_obj.into());
    document_obj
}

fn add_document(scope: &mut v8::HandleScope) {
    let global = scope.get_current_context().global(scope);

    // Create a document object
    let document_obj = v8::Object::new(scope);

    // Create a function template for get_by_id_callback
    let get_by_id_fn_template = v8::FunctionTemplate::new(scope, get_by_id_callback);
    let get_by_id_fn = get_by_id_fn_template.get_function(scope).unwrap();

    // Add the get_by_id_callback function to the document object
    let get_by_id_key = v8::String::new(scope, "getElementById").unwrap().into();
    document_obj.set(scope, get_by_id_key, get_by_id_fn.into());

    // Add the document object to the global object
    let document_key = v8::String::new(scope, "document").unwrap().into();
    global.set(scope, document_key, document_obj.into());
}


fn nodes_tree_to_vector(node: &Node) -> Vec<&Node>{
    let mut nodes_out = vec![];
    if let NodeType::Element(_) = node.node_type {
        nodes_out.push(node);
    }
    for node in &node.children {
        nodes_out.append(&mut nodes_tree_to_vector(node));
    }
    nodes_out
}

pub fn init(js: &str, node: &Node) {
    // init
    let platform = v8::new_default_platform(0, false);
    v8::V8::initialize_platform(platform.into());
    v8::V8::initialize();

    // add isolete and context
    let isolate = &mut v8::Isolate::new(Default::default());
    let scope = &mut v8::HandleScope::new(isolate);
    let global = v8::ObjectTemplate::new(scope);
    let context = v8::Context::new_from_template(scope, global);
    let mut scope = v8::ContextScope::new(scope, context);

    let log_fn = v8::FunctionTemplate::new(&mut scope, log_callback);
    let log_key = v8::String::new(&mut scope, "log").unwrap().into();
    let log_value = log_fn.get_function(&mut scope).unwrap();

    // add console
    let console_key = v8::String::new(&mut scope, "console").unwrap().into();
    let console_obj = v8::Object::new(&mut scope);
    console_obj.set(&mut scope, log_key, log_value.into());
    scope.get_current_context().global(&mut scope).set(&mut scope, console_key, console_obj.into());


    let nodes = nodes_tree_to_vector(node);
    // add document

    let document_obj = add_document_old(&mut scope, &nodes);

    let code = v8::String::new(&mut scope, &js).unwrap();
    let script = v8::Script::compile(&mut scope, code, None).unwrap();

    // run script

    let result = script.run(&mut scope).unwrap();
    let result = result.to_string(&mut scope).unwrap();
    println!("result: {}", result.to_rust_string_lossy(& mut scope));

}


