use std::collections::HashMap;

use async_graphql::*;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal_macros::dec;
use serde::Serialize;

pub struct Query;

pub type TestSchema = Schema<Query, EmptyMutation, EmptySubscription>;

#[Object]
impl Query {
    async fn some_query(&self, ctx: &Context<'_>) -> SomeType {
        SomeType {
            id: "1".into(),
            test_field: CustomDecimal(dec!(5.0)),
        }
    }

    #[entity]
    async fn find_entity_by_id(&self, ctx: &Context<'_>, id: ID) -> SomeType {
        SomeType {
            id: "1".into(),
            test_field: CustomDecimal(dec!(5.0)),
        }
    }
}

#[derive(Clone)]
pub struct SomeType {
    pub id: ID,
    test_field: CustomDecimal,
}

#[Object]
impl SomeType {
    async fn id(&self) -> &ID {
        &self.id
    }

    async fn test_field(&self) -> &CustomDecimal {
        &self.test_field
    }
}

#[derive(Clone, Serialize)]
pub struct CustomDecimal(pub Decimal);

#[Scalar]
impl ScalarType for CustomDecimal {
    fn type_name() -> &'static str {
        "Decimal"
    }

    fn parse(value: Value) -> InputValueResult<Self> {
        unimplemented!()
    }

    fn to_json(&self) -> Result<serde_json::Value> {
        Ok(serde_json::to_value(&self.0).expect("Can't get json from Decimal"))
    }
}
