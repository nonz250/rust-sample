pub struct PostIdentifier {
    pub identifier: u32
}

impl PostIdentifier {
    pub fn to_int(&self) -> u32 {
        return self.identifier
    }

    pub fn to_string(&self) -> String {
        return self.identifier.to_string()
    }
}
