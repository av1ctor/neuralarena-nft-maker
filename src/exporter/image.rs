use base64::prelude::*;
use serde::Deserialize;
use serde_json::Value;
use tao::{
    dpi::PhysicalSize, 
    event::{Event, WindowEvent}, 
    event_loop::{ControlFlow, EventLoopBuilder}, 
    window::WindowBuilder
};
use wry::{PageLoadEvent, WebViewBuilder};
use crate::model::Model;

const HTML: &str = include_str!("../../js/dist/index.html");

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
enum RpcCommand {
    SaveJpeg{
        num: usize,
        image: String
    },
}

pub struct ImageExporter {
}

impl ImageExporter {
    pub fn export(
        &self,
        models: &Vec<Model>
    ) -> Result<(), String> {

        let len_models = models.len();

        let event_loop = EventLoopBuilder::<String>::with_user_event().build();

        let window = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(512 + 20, 512 + 24))
            .with_title("Image exporter")
            .build(&event_loop)
            .unwrap();

        let proxy1 = event_loop.create_proxy();
        let page_load_handler = move |event, _msg| {
            match event {
                PageLoadEvent::Finished => {
                    let gltf = Self::load_gltf(0);
                    _ = proxy1.send_event(format!("window.app.begin('{}');", gltf).into());
                },
                _ => ()
            }
        };

        let proxy2 = event_loop.create_proxy();
        let ipc_handler = move |msg: String| {
            match serde_json::from_str::<RpcCommand>(&msg).unwrap() {
                RpcCommand::SaveJpeg { num, image } => {
                    Self::save_jpeg(num, &image);
                    if num+1 < len_models {
                        let gltf = Self::load_gltf(num+1);
                        _ = proxy2.send_event(format!("window.app.next('{}');", gltf).into());
                    }
                },
            }
        };
        
        let webview = WebViewBuilder::new(&window)
            .with_html(HTML)
            .with_ipc_handler(ipc_handler)
            .with_on_page_load_handler(page_load_handler)
            .build().map_err(|e| e.to_string())?;

        //webview.open_devtools();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::UserEvent(script) => {
                    //*control_flow = ControlFlow::Poll;
                    _ = webview.evaluate_script(&script);
                }
                Event::WindowEvent { event: WindowEvent::CloseRequested, ..} => {
                    *control_flow = ControlFlow::Exit
                },
                _ => (),
            }
        });
    }

    fn load_gltf(
        num: usize
    ) -> String {
        let text = std::fs::read_to_string(
            format!("output/models/{:0>5}.gltf", num)).unwrap();

        let mut gltf: Value = serde_json::from_str(&text)
            .expect("Could not parse the GLTF file");

        {
            let buffers = gltf.get_mut("buffers")
                .expect("buffers[] not found").as_array_mut().unwrap();

            for buff in buffers {
                let bin_name = buff["uri"].as_str().unwrap();
                let bin = std::fs::read(format!("output/models/{}", bin_name)).unwrap();
                buff["uri"] = format!("data:application/octet-stream;base64,{}", BASE64_STANDARD.encode(bin)).into();
            }
        }

        {
            let images = gltf.get_mut("images")
                .expect("images[] not found").as_array_mut().unwrap();

            for img in images {
                let mime_type = img["mimeType"].as_str().unwrap();
                let bin_name = img["uri"].as_str().unwrap();
                let bin = std::fs::read(format!("output/models/{}", bin_name)).unwrap();
                img["uri"] = format!("data:{};base64,{}", mime_type, BASE64_STANDARD.encode(bin)).into();
            }
        }

        let gltf_emb = serde_json::to_string(&gltf).unwrap();

        BASE64_STANDARD.encode(gltf_emb)
    }

    fn save_jpeg(
        num: usize,
        uri: &String
    ) {
        let image = BASE64_STANDARD.decode(
            uri.chars().skip(23).collect::<String>()).unwrap();

        println!("saving {:0>5}.jpg", num);
        
        std::fs::write(format!("output/images/{:0>5}.jpg", num), image)
            .expect("Could not write the jpeg file");
    }
}