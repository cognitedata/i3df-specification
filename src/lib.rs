pub fn as_string() -> String {
    let spec = include_str!("../specification/i3df-specification.yaml");
    spec.to_string()
}

