#[derive(Debug)]
pub struct Path{
    pub data: String
}

impl Path {
    pub fn new(data: String) -> Path {
        Path { data }
    }
}