// obj

#[derive(Debug)]
pub struct Mesh {
    pub mtl_library: String,
    pub positions: Vec<Vec<f32>>,
    pub texture_coords: Vec<Vec<f32>>,
    pub vertex_normals: Vec<Vec<f32>>,
    pub group_name: String,
    pub mtl_name: String,
    pub smoothing_groups: Vec<String>,
    pub position_groups: Vec<Vec<u32>>,
    pub texture_coord_groups: Vec<Vec<u32>>,
    pub vertex_normal_groups: Vec<Vec<u32>>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            mtl_library: String::new(),
            positions: Vec::new(),
            texture_coords: Vec::new(),
            vertex_normals:Vec::new(),
            group_name: String::new(),
            mtl_name: String::new(),
            smoothing_groups: Vec::new(),
            position_groups: Vec::new(),
            texture_coord_groups: Vec::new(),
            vertex_normal_groups: Vec::new(), 
        }
    }
}
