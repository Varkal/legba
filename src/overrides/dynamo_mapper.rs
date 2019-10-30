use dynamo_mapper::*;
use rusoto_dynamodb::*;
use uuid::Uuid;

pub trait DynamoAttributeOverride {
    fn to_dynamo(&self) -> Option<AttributeValue>;
    fn from_dynamo(item: &DynamoItem, key: String) -> Option<Self>
    where
        Self: std::marker::Sized;
}

impl DynamoAttributeOverride for Uuid {
    fn to_dynamo(&self) -> Option<AttributeValue> {
        self.to_string().to_dynamo()
    }

    fn from_dynamo(item: &DynamoItem, key: String) -> Option<Uuid> {
        if let Some(string_value) = String::from_dynamo(item, key) {
            if let Ok(uuid) = Uuid::parse_str(string_value.as_str()) {
                return Some(uuid);
            }
        }

        return None;
    }
}
