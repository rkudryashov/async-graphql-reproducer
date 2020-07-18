use async_graphql::*;
use bigdecimal::{BigDecimal, ToPrimitive};
use serde::Serialize;

pub type TestSchema = Schema<Query, EmptyMutation, Subscription>;

pub struct Query;

#[Object]
impl Query {
    async fn some_query(&self, ctx: &Context<'_>) -> SomeType {
        SomeType {
            id: "1".into(),
            test_field: CustomDecimal(BigDecimal::from(2439.3)),
            test_field_2: Successor1 { some_field: 3 }.into(),
            enum_field: TestEnum::Value1,
        }
    }

    #[entity]
    async fn find_entity_by_id(&self, ctx: &Context<'_>, id: ID) -> SomeType {
        SomeType {
            id: "1".into(),
            test_field: CustomDecimal(BigDecimal::from(5.0)),
            test_field_2: Successor1 {
                some_field: 3
            }.into(),
            enum_field: TestEnum::Value1,
        }
    }
}

pub struct Subscription;

#[Subscription]
impl Subscription {}

#[derive(Clone)]
pub struct SomeType {
    pub id: ID,
    test_field: CustomDecimal,
    test_field_2: Interface,
    enum_field: TestEnum,
}

#[Object]
impl SomeType {
    async fn id(&self) -> &ID {
        &self.id
    }

    async fn test_field(&self) -> &CustomDecimal {
        &self.test_field
    }

    async fn test_field_2(&self) -> &Interface {
        &self.test_field_2
    }

    async fn enum_field(&self) -> &TestEnum {
        &self.enum_field
    }
}

#[derive(Clone, Serialize)]
pub struct CustomDecimal(pub BigDecimal);

#[Scalar(name = "Decimal")]
impl ScalarType for CustomDecimal {
    fn parse(value: Value) -> InputValueResult<Self> {
        unimplemented!()
    }

    fn to_value(&self) -> Value {
        // Value::String(self.0.to_string())
        Value::Float(self.0.to_f64().unwrap())
    }
}

#[Interface(
field(name = "some_field", type = "i32"),
)]
#[derive(Clone)]
enum Interface {
    Successor1(Successor1),
}

#[SimpleObject]
#[derive(Clone)]
struct Successor1 {
    #[field(owned)]
    some_field: i32,
}

#[Enum]
enum TestEnum {
    Value1,
}
