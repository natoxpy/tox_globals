
// SECTION: DataStream

pub struct DataStream {
    pub segments: Vec<DataSegment>,
}

impl DataStream {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }

    pub fn wait_for_stream(&mut self) {

    }
}

impl DataStream {
    pub fn add_segment(&mut self, segment: DataSegment) {
        self.segments.push(segment);
    }

    pub fn get_segment(&mut self, index: usize) -> Option<&mut DataSegment> {
        self.segments.get_mut(index)
    }
}

impl DataStream {
    pub async fn send(_data: Vec<u8>) {

    }
}

// SECTION: DataSegment

pub struct DataSegment {
    pub data: Vec<u8>,
    pub position: usize,
}

impl DataSegment {
    pub fn new(data: Vec<u8>, position: usize) -> Self {
        Self {
            data,
            position,
        }
    }
}
