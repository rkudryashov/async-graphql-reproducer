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
            test_field_2: Successor1 {
                some_field: 3
            }.into(),
            enum_field: TestEnum::Value1,
        }
    }

    #[entity]
    async fn find_entity_by_id(&self, ctx: &Context<'_>, id: ID) -> SomeType {
        SomeType {
            id: "1".into(),
            test_field: CustomDecimal(dec!(5.0)),
            test_field_2: Successor1 {
                some_field: 3
            }.into(),
            enum_field: TestEnum::Value1,
        }
    }
}

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

#[Interface(
field(name = "some_field", type = "i32", context),
)]
#[derive(Clone)]
enum Interface {
    Successor1(Successor1),
}

#[SimpleObject]
#[derive(Clone)]
struct Successor1 {
    some_field: i32,
}

#[Enum]
enum TestEnum {
    Value1,
}
