pub struct Project {
    stage: Stage,
    sprites: Vec<Sprite>,
    broadcasts: Vec<Broadcast>,
}

pub struct Costume {
    data: String,
    name: String,
}

pub struct Broadcast {
    name: String,
}

pub struct Sprite {
    pub name: String,
    pub costumes: Vec<Costume>,
}

pub struct Backdrop {
    data: String,
    name: String,
}

pub struct Stage {
    pub name: String,
    pub backdrops: Vec<Backdrop>,
}

// TODO: make constructor in Project for Costume, Backdrop, and Broadcast