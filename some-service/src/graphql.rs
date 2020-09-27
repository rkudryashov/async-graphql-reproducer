use std::str::FromStr;

use async_graphql::*;
use bigdecimal::{BigDecimal, ToPrimitive};
use num_bigint::{BigInt, ToBigInt};
use serde::Serialize;

pub type TestSchema = Schema<Query, EmptyMutation, Subscription>;

pub struct Query;

#[Object]
impl Query {
    async fn some_query(&self, ctx: &Context<'_>) -> SomeType {
        SomeType {
            id: "1".into(),
            test_field: CustomBigDecimal(BigDecimal::from(2439.3)),
            test_field_2: Successor1 { some_field: 3 }.into(),
            enum_field: TestEnum::Value1,
            test_int: CustomBigInt(BigInt::from_str("4870000000000000000000000").expect("")),
        }
    }

    async fn query_that_should_be_excluded(&self) -> &str {
        ""
    }

    #[entity]
    async fn find_entity_by_id(&self, ctx: &Context<'_>, id: ID) -> SomeType {
        SomeType {
            id: "1".into(),
            test_field: CustomBigDecimal(BigDecimal::from(5.0)),
            test_field_2: Successor1 {
                some_field: 3
            }.into(),
            enum_field: TestEnum::Value1,
            test_int: CustomBigInt(BigInt::from_str("4870000000000000000000000").expect("")),
        }
    }
}

pub struct Subscription;

#[Subscription]
impl Subscription {}

#[derive(Clone)]
pub struct SomeType {
    pub id: ID,
    test_field: CustomBigDecimal,
    test_field_2: Interface,
    enum_field: TestEnum,
    test_int: CustomBigInt,
}

#[Object]
impl SomeType {
    async fn id(&self) -> &ID {
        &self.id
    }

    async fn test_field(&self) -> &CustomBigDecimal {
        &self.test_field
    }

    async fn test_field_2(&self) -> &Interface {
        &self.test_field_2
    }

    async fn enum_field(&self) -> &TestEnum {
        &self.enum_field
    }

    async fn test_int(&self) -> &CustomBigInt {
        &self.test_int
    }
}

#[derive(Clone)]
struct CustomBigInt(BigInt);

#[Scalar(name = "BigInt")]
impl ScalarType for CustomBigInt {
    fn parse(_value: Value) -> InputValueResult<Self> {
        unimplemented!()
    }

    fn to_value(&self) -> Value {
        // Value::String(self.0.to_string())

        // convert to float to represent a value as number with mantissa and exponent
        self.0.to_f64()
            .and_then(|value| async_graphql::Number::from_f64(value))
            .map(|value| Value::Number(value))
            .expect("Can't convert BigInt")
    }
}

#[derive(Clone)]
struct CustomBigDecimal(BigDecimal);

#[Scalar(name = "BigDecimal")]
impl ScalarType for CustomBigDecimal {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(s) => {
                let parsed_value = BigDecimal::from_str(s.as_str())?;
                Ok(CustomBigDecimal(parsed_value))
            }
            _ => Err(InputValueError::ExpectedType(value)),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_string())
    }
}

#[derive(Interface, Clone)]
#[graphql(
    field(name = "some_field", type = "i32"),
)]
enum Interface {
    Successor1(Successor1),
}

#[derive(SimpleObject,Clone)]
struct Successor1 {
    #[field(owned)]
    some_field: i32,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
enum TestEnum {
    Value1,
}
