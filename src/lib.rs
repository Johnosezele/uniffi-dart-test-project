#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MyUuid(String);

uniffi::custom_type!(MyUuid, String);

impl From<MyUuid> for String {
    fn from(val: MyUuid) -> Self {
        val.0
    }
}

impl From<String> for MyUuid {
    fn from(val: String) -> Self {
        MyUuid(val)
    }
}

impl MyUuid {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn process_uuid(uuid: MyUuid) -> MyUuid {
    println!("Rust received UUID: {}", uuid.as_str());
    MyUuid::new(format!("processed_{}", uuid.as_str()))
}

uniffi::include_scaffolding!("api");
