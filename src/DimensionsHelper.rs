use crate::structs::Dimensions;

pub struct DimensionsHelper {}

impl DimensionsHelper {
    pub fn getDimensionsVolume(dimensions: &Dimensions) -> i32 {
        return dimensions.length * dimensions.width * dimensions.height;
    }
}