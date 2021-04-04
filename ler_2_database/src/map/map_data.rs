use std::borrow::Borrow;

use dxf::entities::*;
use dxf::tables::*;
use dxf::Drawing;

pub fn crop_map() -> () {
    debug!("Testing dxf module");

    let mut drawing =
        Drawing::load_file("../test_data/Herlufmagle/IVand_K08_F2_T53_H1.dxf").unwrap();

    for entity in drawing.entities.iter_mut() {
        match entity.specific {
            EntityType::Line(ref mut line) => {
                // debug!("{:?}", line);
                line.thickness = 10.0;
                // debug!("{:?}", line);
            }
            // EntityType::Circle(ref mut circle) => {
            // circle.radius = 500.0;
            // }
            _ => (),
        }
    }
    drawing
        .save_file("../test_data/Herlufmagle/test.dxf")
        .expect("Saving did not succeed");
}
