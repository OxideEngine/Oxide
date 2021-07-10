use crate::obj_fbx_loader::{mesh, util};
use std::fs::File;
use std::io::Read;
use std::str;
use std::vec::Vec;

pub struct ObjLoader;

impl ObjLoader {
    pub fn load(&self, file: &mut File) -> mesh::Mesh {
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to open file");

        let mut mesh: mesh::Mesh = mesh::Mesh {
            ..Default::default()
        };

        for line in contents.lines() {
            let lines: Vec<&str> = line.split_whitespace().collect();

            for values in &lines {
                let mut position: Vec<f32> = Vec::new();
                let mut texcoord: Vec<f32> = Vec::new();
                let mut normal: Vec<f32> = Vec::new();
                let mut position_group: Vec<u32> = Vec::new();
                let mut texture_coord_group: Vec<u32> = Vec::new();
                let mut vertex_normal_group: Vec<u32> = Vec::new();

                match *values {
                    "mtllib" => {
                        // mtl file : not needed yet
                        mesh.mtl_library = util::join(&lines).split("mtllib").collect();
                    }
                    "v" => {
                        // vertex
                        position.push(lines[1].parse().unwrap());
                        position.push(lines[2].parse().unwrap());
                        position.push(lines[3].parse().unwrap());
                        mesh.positions.push(position);
                    }
                    "vt" => {
                        // texture coord
                        texcoord.push(lines[1].parse().unwrap());
                        texcoord.push(lines[2].parse().unwrap());
                        mesh.texture_coords.push(texcoord);
                    }
                    "vn" => {
                        // vector normal
                        normal.push(lines[1].parse().unwrap());
                        normal.push(lines[2].parse().unwrap());
                        normal.push(lines[3].parse().unwrap());
                        mesh.vertex_normals.push(normal);
                    }
                    "g" => {
                        // group name
                        mesh.group_name = util::join(&lines).split('g').collect();
                    }
                    "usemtl" => {
                        // mtl file : not needed yet
                        mesh.mtl_name = util::join(&lines).split("usemtl").collect();
                    }
                    "s" => {
                        // smoothing group
                        mesh.smoothing_groups.push(lines[1].parse().unwrap());
                    }
                    "f" => {
                        for (idx, first_letter) in lines.iter().enumerate() {
                            if *first_letter != "f" {
                                let vertex: Vec<&str> = lines[idx].split('/').collect();
                                position_group.push(vertex[0].parse().unwrap());
                                texture_coord_group.push(vertex[1].parse().unwrap());
                                vertex_normal_group.push(vertex[2].parse().unwrap());
                            }
                        }
                        mesh.position_groups.push(position_group);
                        mesh.texture_coord_groups.push(texture_coord_group);
                        mesh.vertex_normal_groups.push(vertex_normal_group);
                    }
                    _ => (),
                }
            }
        }

        mesh
    }
}
