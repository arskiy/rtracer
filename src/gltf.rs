use gltf;
use gltf::buffer::Buffer;
use gltf::animation::util::{Reader, ReadOutputs};

use crate::vec3::*;
use crate::matrix4::Matrix4;

pub struct GLTF {
    pub nodes: Vec<Node>,
    pub meshes: Vec<Mesh>,
}

pub struct Node {
    pub index: usize,
    pub parent_index: i32,
    pub child_indices: Vec<usize>,
    pub rotation_indices: Vec<usize>,
    pub translation_indices: Vec<usize>,
    pub scale_indices: Vec<usize>,
    pub transform: Matrix4,
    pub global_transform: Matrix4,
    pub mesh: Option<usize>,
}

pub struct Mesh {
    pub positions: Vec<Vec3>,
    pub indices: Vec<u32>,
    pub normals: Vec<f32>,
    pub uvs: Vec<f32>,
    pub mat_index: usize,
}

impl GLTF {
    pub fn new(fname: String) -> Result<Self, gltf::Error> {
        let (document, buffers, images) = gltf::import(fname)?;

        let (nodes, meshes) = process_nodes(&document, &buffers);        

        Ok(Self {
            nodes,
            meshes,
        })
    }
}

fn process_nodes<'a>(document: &'a gltf::Document, buffers: &Vec<gltf::buffer::Data>) -> (Vec<Node>, Vec<Mesh>) {
    let mut nodes = vec!();
    let mut mesh_count = 0;
    let mut meshes = vec!();

    for scene in document.scenes() {
        for node in scene.nodes() {
            let transform = Matrix4(node.transform().matrix());
            let mut stack: Vec<(gltf::Node, Matrix4)> = vec!((node, transform));

            while let Some((node, transform)) = stack.pop() {
                let mut mesh_index = None;
                if let Some(_) = node.mesh() {
                    meshes.extend(process_meshes(&node, buffers).unwrap());
                    mesh_index = Some(mesh_count);
                    mesh_count += 1;
                }

                let mut new_node = Node {
                    index: node.index(),
                    parent_index: -1,
                    child_indices: vec!(),
                    rotation_indices: vec!(),
                    translation_indices: vec!(),
                    scale_indices: vec!(),
                    transform,
                    global_transform: transform,
                    mesh: mesh_index,
                };

                for child in node.children() {
                    new_node.child_indices.push(child.index());
                    let local_transform = Matrix4(child.transform().matrix());
                    stack.push((child, transform * local_transform));
                }

                nodes.push(new_node);
            }
        }
    }

    process_node_parents(&mut nodes);
    process_global_transforms(&mut nodes);

    (nodes, meshes)
}

fn process_meshes(node: &gltf::Node, buffers: &[gltf::buffer::Data]) -> Option<Vec<Mesh>> {
    node.mesh().map(|mesh| {
        mesh.primitives().map(|primitive| {
            let mut positions: Vec<Vec3> = vec!();
            let mut indices: Vec<u32> = vec!();
            let mut normals: Vec<f32> = vec!();
            let mut uvs: Vec<f32> = vec!();
            let mut mat_index = 0;

            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

            if let Some(pos) = reader.read_positions() {
                for p in pos {
                    let mut vec = Vec3::new_empty();
                    for i in 0..=2 {
                        vec[i] = p[i]
                    }
                    positions.push(vec);
                }
            }

            if let Some(normal) = reader.read_normals() {
                for n in normal {
                    for i in 0..=2 {
                        normals.push(n[i]);
                    }
                }
            }

            if let Some(tex) = reader.read_tex_coords(0) {
                for uv in tex.into_f32() {
                    for i in 0..=1 {
                        uvs.push(uv[i]);
                    }
                }
            }

            if let Some(index) = reader.read_indices() {
                indices.extend(index.into_u32());
            }

            if let Some(mat) = primitive.material().index() {
                mat_index = mat;
            }

            Mesh {
                positions,
                indices,
                normals,
                uvs,
                mat_index,
            }
        }).collect()
    })
}

fn process_node_parents(nodes: &mut Vec<Node>) -> usize {
    for i in 0..nodes.len() {
        let mut new_child_indices = vec![];
        for j in 0..nodes.len() {
            if nodes[i].child_indices.contains(&nodes[j].index) {
                new_child_indices.push(j);
                nodes[j].parent_index = i as i32;
            }
        }
        nodes[i].child_indices = new_child_indices;
    }

    for i in 0..nodes.len() {
        if nodes[i].parent_index == -1 {
            return i;
        }
    }

    panic!("Error: imported scene is either empty or cyclic!");
}

fn process_global_transforms(nodes: &mut Vec<Node>) {
    let mut stack: Vec<(usize, Matrix4)> = vec![];
    for i in 0..nodes.len() {
        if nodes[i].parent_index == -1 {
            stack.push((i, nodes[i].global_transform));
        }
    }

    while let Some((index, transform)) = stack.pop() {
        nodes[index].global_transform = transform;
        for child in nodes[index].child_indices.clone() {
            stack.push((child, nodes[child].transform * transform));
        }
    }
}
