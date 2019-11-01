use dynamo_mapper::*;
use rusoto_core::{Region, RusotoResult};
use rusoto_dynamodb::*;
use std::marker::PhantomData;

pub struct EntityManager<T: DynamoMapper> {
    table_name: String,
    client: DynamoDbClient,
    entity_type: PhantomData<T>,
}

impl<T: DynamoMapper> EntityManager<T> {
    pub fn new(table_name: String) -> EntityManager<T> {
        EntityManager::<T> {
            table_name,
            client: DynamoDbClient::new(Region::EuWest1),
            entity_type: PhantomData,
        }
    }

    pub fn get_all(&self) -> Option<Vec<T>> {
        return self.scan(None);
    }

    pub fn put_item(&self, item: T) -> RusotoResult<PutItemOutput, PutItemError> {
        let item = item.to_dynamo();

        let response = self.client.put_item(PutItemInput {
            table_name: (&self.table_name).to_string(),
            item,
            ..Default::default()
        });

        return response.sync();
    }

    fn scan(&self, scroll_id: Option<DynamoItem>) -> Option<Vec<T>> {
        let limit: Option<i64> = Some(20);

        let input = ScanInput {
            table_name: (&self.table_name).to_string(),
            exclusive_start_key: scroll_id,
            limit,
            ..Default::default()
        };

        let mut result: Vec<T>;

        match self.client.scan(input).sync() {
            Ok(output) => {
                if let Some(items) = output.items {
                    result = items.iter().map(|item| T::from_dynamo(&item)).collect();

                    if let Some(scroll_id) = output.last_evaluated_key {
                        result.append(&mut self.scan(Some(scroll_id)).unwrap_or_default());
                    }

                    return Some(result);
                }
            }
            Err(error) => {
                println!("Error: {:?}", error);
            }
        }

        return None;
    }
}

