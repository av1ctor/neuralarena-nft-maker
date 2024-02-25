use std::collections::HashMap;
use serde_json::Value;
use crate::model::Model;

pub struct ModelExporter<'a> {
    pub read_gltf_file: &'a dyn Fn(&String) -> Result<String, String>,
    pub write_gltf_file: &'a dyn Fn(&String, &String) -> Result<(), String>,
}

impl<'a> ModelExporter<'_> {
    pub fn export(
        &self,
        models: &Vec<Model>
    ) -> Result<(), String> {
        let mut gltf_cache = HashMap::<String, serde_json::Value>::new();

        for model in models {
            let model_path = &format!("{}{}", model.cfg.path, model.cfg.gltf);
            let mut gltf = match gltf_cache.get(model_path) {
                Some(res) => res.clone(),
                None => {
                    let text = (*self.read_gltf_file)(&model_path)?;
                    let gltf: Value = match serde_json::from_str(&text) {
                        Ok(res) => res,
                        Err(msg) => return Err(msg.to_string()),
                    };

                    gltf_cache.insert(model_path.clone(), gltf.clone());

                    gltf
                },
            };

            for tr4it in &model.traits {
                let trait_path = &format!("{}traits/{}", model.cfg.path, tr4it.cfg.path);
                let trait_full_path = &format!("{}{}", trait_path, tr4it.cfg.gltf);
                let mut trait_gltf = match gltf_cache.get(trait_full_path) {
                    Some(res) => res.clone(),
                    None => {
                        let text = (*self.read_gltf_file)(trait_full_path)?;
                        let gltf: Value = match serde_json::from_str(&text) {
                            Ok(res) => res,
                            Err(msg) => return Err(msg.to_string()),
                        };
    
                        gltf_cache.insert(trait_full_path.clone(), gltf.clone());
    
                        gltf
                    },
                };

                let buffers_len = {
                    let buffers = gltf.get_mut("buffers").expect("buffers[] not found").as_array_mut().unwrap();
                    let len = buffers.len();
                    let trait_buffers = trait_gltf.get("buffers").expect("buffers[] not found").as_array().unwrap();
                    for buff in trait_buffers {
                        buffers.push(buff.clone());
                    }

                    for (i, buff) in buffers.iter_mut().enumerate() {
                        buff["uri"] = format!(
                            "buffers/{}{}", 
                            if i < len {&model.cfg.path} else {trait_path},
                            buff["uri"].as_str().unwrap()
                        ).into();
                    }

                    len
                };
    
                let buffer_views_len = {
                    let buffer_views = gltf.get_mut("bufferViews").expect("bufferViews[] not found").as_array_mut().unwrap();
                    let len = buffer_views.len();
                    
                    let trait_buffer_views = trait_gltf.get_mut("bufferViews").expect("bufferViews[] not found").as_array_mut().unwrap();
                    for view in trait_buffer_views {
                        view["buffer"] = (view["buffer"].as_u64().unwrap() as usize + buffers_len).into();
                        buffer_views.push(view.clone());
                    }

                    len
                };

                let accessors_len = {
                    let accessors = gltf.get_mut("accessors").expect("accessors[] not found").as_array_mut().unwrap();
                    let len = accessors.len();
                    
                    let trait_accessors = trait_gltf.get_mut("accessors").expect("accessors[] not found").as_array_mut().unwrap();
                    for acc in trait_accessors {
                        acc["bufferView"] = (acc["bufferView"].as_u64().unwrap() as usize + buffer_views_len).into();
                        accessors.push(acc.clone());
                    }

                    len
                };
    
                let meshes_len = {
                    let meshes = gltf.get_mut("meshes").expect("meshes[] not found").as_array_mut().unwrap();
                    let len = meshes.len();
                    
                    let trait_meshes = trait_gltf.get_mut("meshes").expect("meshes[] not found").as_array_mut().unwrap();
                    for mesh in trait_meshes {
                        let mesh_primitives = mesh.get_mut("primitives").expect("primitives[] not found").as_array_mut().unwrap();
                        for prim in mesh_primitives {
                            prim["indices"] = (prim["indices"].as_u64().unwrap() as usize + accessors_len).into();
                            let prim_attributes = prim["attributes"].as_object_mut().unwrap();
                            for attr in &mut prim_attributes.values_mut() {
                                *attr = (attr.as_u64().unwrap() as usize + accessors_len).into();
                            }
                        }
                        meshes.push(mesh.clone());
                    }

                    len
                };

                let _nodes_len = {
                    let nodes = gltf.get_mut("nodes").expect("nodes[] not found").as_array_mut().unwrap();
                    let len = nodes.len();
                    
                    let mut nodes_to_link = vec![];
                    let trait_nodes = trait_gltf.get_mut("nodes").expect("nodes[] not found").as_array_mut().unwrap();
                    for node in trait_nodes {
                        node["mesh"] = (node["mesh"].as_u64().unwrap() as usize + meshes_len).into();
                        nodes_to_link.push(nodes.len());
                        nodes.push(node.clone());
                    }
                    
                    let bone_node = nodes.iter_mut().find(|n| n["name"] == tr4it.cfg.bone).unwrap();
                    for n in nodes_to_link {
                        if let Some(children) = bone_node.get_mut("children") {
                            children.as_array_mut().unwrap().push(n.into())
                        }
                        else {
                            bone_node["children"] = (vec![n]).into();
                        }
                    }
                    
                    len
                };

                let images_len = {
                    let images = gltf.get_mut("images").expect("images[] not found").as_array_mut().unwrap();
                    let len = images.len();
                    
                    if let Some(trait_images) = trait_gltf.get("images") {
                        for image in trait_images.as_array().unwrap() {
                            images.push(image.clone());
                        }
                    }

                    for (i, image) in images.iter_mut().enumerate() {
                        image["uri"] = format!(
                            "textures/{}{}", 
                            if i < len {&model.cfg.path} else {trait_path},
                            image["uri"].as_str().unwrap()
                        ).into();
                    }

                    len
                };

                let textures_len = {
                    let textures = gltf.get_mut("textures").expect("textures[] not found").as_array_mut().unwrap();
                    let len = textures.len();
                    
                    if let Some(trait_textures) = trait_gltf.get_mut("textures") {
                        for tex in trait_textures.as_array_mut().unwrap() {
                            tex["source"] = (tex["source"].as_u64().unwrap() as usize + images_len).into();
                            textures.push(tex.clone());
                        }
                    }

                    len
                };

                let _materials_len = {
                    let materials = gltf.get_mut("materials").expect("materials[] not found").as_array_mut().unwrap();
                    let len = materials.len();
                    
                    if let Some(trait_materials) = trait_gltf.get_mut("materials") {
                        for material in trait_materials.as_array_mut().unwrap() {
                            if let Some(normal_tex) = material.get_mut("normalTexture") {
                                normal_tex["index"] = (normal_tex["index"].as_u64().unwrap() as usize + textures_len).into();
                            }

                            if let Some(pbr_metallic_roughness) = material.get_mut("pbrMetallicRoughness") {
                                if let Some(base_color_tex) = pbr_metallic_roughness.get_mut("baseColorTexture") {
                                    base_color_tex["index"] = (base_color_tex["index"].as_u64().unwrap() as usize + textures_len).into();
                                }
                            }
                            
                            materials.push(material.clone());
                        }
                    }

                    len
                };
    
            }

            let path = &format!("{:0>5}.gltf", model.num);
            (*self.write_gltf_file)(path, &gltf.to_string())?;
        }
        
        Ok(())
    }
}