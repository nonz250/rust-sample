use ulid::Ulid;

pub struct PostIdentifier {
    pub identifier: Ulid
}

impl PostIdentifier {
    pub fn to_string(&self) -> String {
        return self.identifier.to_string()
    }
}
