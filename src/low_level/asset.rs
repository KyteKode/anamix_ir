pub struct LLCostume {
    pub name: String,
    pub data_format: String,

    pub bitmap_resolution: Option<f64>,
    pub rotation_center_x: Option<f64>,
    pub rotation_center_y: Option<f64>,
}

pub struct LLSound {
    pub name: String,
    pub data_format: String,
    
    pub rate: Option<f64>,
    pub sample_count: Option<f64>,
}
