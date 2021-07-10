#[cfg(test)]
mod test {
    use crate::obj_fbx_loader::mesh;
    use crate::obj_fbx_loader::obj_loader;
    use std::fs::File;

    #[test]
    fn loader() {
        let obj_loader = obj_loader::ObjLoader {};
        let expected_result = mesh::Mesh {
            mtl_library: String::from("test.mtl"),
            positions: vec![
                vec![-26.2355, 0.0, 30.9582],
                vec![-26.2355, 0.0, -14.8033],
                vec![13.5614, 0.0, -14.8033],
                vec![13.5614, 0.0, 30.9582],
                vec![-26.2355, 33.2881, 30.9582],
                vec![13.5614, 33.2881, 30.9582],
                vec![13.5614, 33.2881, -14.8033],
                vec![-26.2355, 33.2881, -14.8033],
            ],
            texture_coords: vec![
                vec![1.0, 0.0],
                vec![1.0, 1.0],
                vec![0.0, 1.0],
                vec![0.0, 0.0],
            ],
            vertex_normals: vec![
                vec![0.0, -1.0, -0.0],
                vec![0.0, 1.0, -0.0],
                vec![0.0, 0.0, 1.0],
                vec![1.0, 0.0, -0.0],
                vec![0.0, 0.0, -1.0],
                vec![-1.0, 0.0, -0.0],
            ],
            group_name: String::from("Box001"),
            mtl_name: String::from("wire_166229229"),
            smoothing_groups: vec![
                String::from("2"),
                String::from("4"),
                String::from("8"),
                String::from("16"),
                String::from("32"),
                String::from("64"),
            ],
            position_groups: vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![1, 4, 6, 5],
                vec![4, 3, 7, 6],
                vec![3, 2, 8, 7],
                vec![2, 1, 5, 8],
            ],
            texture_coord_groups: vec![
                vec![1, 2, 3, 4],
                vec![4, 1, 2, 3],
                vec![4, 1, 2, 3],
                vec![4, 1, 2, 3],
                vec![4, 1, 2, 3],
                vec![4, 1, 2, 3],
            ],
            vertex_normal_groups: vec![
                vec![1, 1, 1, 1],
                vec![2, 2, 2, 2],
                vec![3, 3, 3, 3],
                vec![4, 4, 4, 4],
                vec![5, 5, 5, 5],
                vec![6, 6, 6, 6],
            ],
        };
        let mut file =
            File::open("obj_fbx_loader/test/resources/test.obj").expect("File not found");
        let parsed_result = obj_loader.load(&mut file);
        assert_eq!(expected_result, parsed_result);
    }
}
