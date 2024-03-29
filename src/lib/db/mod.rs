use log;
use tonic::async_trait;

use std::rc::Rc;

use crate::parsers::settings;
use crate::{error, UdmResult};
use sea_query::foreign_key::{ForeignKeyAction, ForeignKeyCreateStatement};
use sea_query::value::Value;
use sea_query::{ColumnDef, Iden, Table};
pub mod postgres;
pub mod sqlite;

// Build "loadable" different db types with their relevant information

// This represents the table operations itself.
// Connection and Manipulation will be handled into a different struct
pub trait SqlTransactionsFactory {
    fn column_to_str(&self) -> &'static str;
    fn from_str(value: &'static str) -> Option<Self>
    where
        Self: Sized;
}

// This generates schemas and manupulates tables outside of the data itself
// This ipml on each individual table you want to
pub trait SqlTableTransactionsFactory: SqlTransactionsFactory {
    fn create_table(builder: impl sea_query::backend::SchemaBuilder) -> String;
    fn alter_table(
        builder: impl sea_query::backend::SchemaBuilder,
        column_def: &mut ColumnDef,
    ) -> String;
}

// This generates schemas and manipulates the database outside of the data itself
// This ipml on each individual table you want to
#[async_trait]
pub trait DatabaseTransactionsFactory {
    async fn collect_all_current_tables(&mut self) -> UdmResult<Vec<String>>;
    async fn gen_schmea(&mut self) -> UdmResult<()>;
}

// This will generate all the queries
// This manipluates the data itself
#[async_trait]
pub trait SqlQueryExecutor {
    fn gen_query(&self) -> Box<dyn SqlTransactionsFactory>;
    async fn execute<T>(&self) -> Result<T, error::UdmError>;
}

// A loadable enum depending on the mechanism is chosen
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DbType {
    Postgres(settings::PostgresConfigurer),
    Sqlite(settings::SqliteConfigurer),
}
impl DbType {
    pub fn load_db(udm_configurer: Rc<settings::UdmConfigurer>) -> Self {
        if let Some(postgres_configurer) = udm_configurer.daemon.postgres.clone() {
            log::info!("Using postgres as the Database");
            Self::Postgres(postgres_configurer)
        } else if let Some(sqlite_configurer) = udm_configurer.daemon.sqlite.clone() {
            log::info!("Using sqlite as the database");
            Self::Sqlite(sqlite_configurer)
        } else {
            panic!("Could not determine database to use and load")
        }
    }
    pub async fn establish_connection(&self) -> Box<dyn DbConnection> {
        match self {
            DbType::Postgres(config) => {
                Box::new(postgres::conn::OpenPostgresConnection::new(config.to_owned()).await)
            }
            DbType::Sqlite(config) => {
                Box::new(sqlite::conn::OpenSqliteConnection::new(config.to_owned()).await)
            }
        }
    }
}

#[async_trait]
pub trait DbConnection: DatabaseTransactionsFactory {}

// Defines the Schema and how we interact with the DB.
// The structs generated in RPC Frameworks
// We will Transform different types
#[derive(Iden, Eq, PartialEq, Debug)]
#[iden = "FluidRegulation"]
pub enum FluidRegulationSchema {
    Table,
    FrId, // Primary Key
    GpioPin,
    RegulatorType,
}
impl SqlTransactionsFactory for FluidRegulationSchema {
    fn column_to_str(&self) -> &'static str {
        match self {
            Self::Table => "FluidRegulation",
            Self::FrId => "fr_id",
            Self::GpioPin => "gpio_pin",
            Self::RegulatorType => "regulator_type",
        }
    }
    fn from_str(value: &'static str) -> Option<Self> {
        match value {
            "FluidRegulation" => Some(FluidRegulationSchema::Table),
            "fr_id" => Some(FluidRegulationSchema::FrId),
            "gpio_pin" => Some(FluidRegulationSchema::GpioPin),
            "regulator_type" => Some(FluidRegulationSchema::RegulatorType),
            _ => None,
        }
    }
}

impl SqlTableTransactionsFactory for FluidRegulationSchema {
    fn create_table(builder: impl sea_query::backend::SchemaBuilder) -> String {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Self::FrId)
                    .integer()
                    .auto_increment()
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new(Self::RegulatorType).integer().not_null())
            .col(ColumnDef::new(Self::GpioPin).integer())
            .build(builder)
    }

    fn alter_table(
        builder: impl sea_query::backend::SchemaBuilder,
        column_def: &mut ColumnDef,
    ) -> String {
        Table::alter()
            .table(Self::Table)
            .add_column(column_def)
            .build(builder)
    }
}
#[derive(Iden, Eq, PartialEq, Debug)]
#[iden = "Ingredient"]
pub enum IngredientSchema {
    Table,
    IngredientId,
    Name,
    Alcoholic,
    Description,
    IsActive,
    FrId, // Foreign Key
    Amount,
    IngredientType,
    InstructionId, // Foriegn Key
}
impl SqlTransactionsFactory for IngredientSchema {
    fn column_to_str(&self) -> &'static str {
        match self {
            Self::Table => "Ingredient",
            Self::IngredientId => "ingredient_id",
            Self::Name => "name",
            Self::Alcoholic => "alcoholic",
            Self::Description => "description",
            Self::IsActive => "is_active",
            Self::FrId => "fr_id",
            Self::Amount => "amount",
            Self::IngredientType => "amount",
            Self::InstructionId => "instruction_id",
        }
    }
    fn from_str(value: &'static str) -> Option<Self> {
        match value {
            "Ingredient" => Some(Self::Table),
            "ingredient_id" => Some(Self::IngredientId),
            "name" => Some(Self::Name),
            "alcoholic" => Some(Self::Alcoholic),
            "description" => Some(Self::Description),
            "is_active" => Some(Self::IsActive),
            "amount" => Some(Self::Amount),
            "ingredient_type" => Some(Self::IngredientType),
            "fr_id" => Some(Self::FrId),
            "instruction_id" => Some(Self::InstructionId),
            _ => None,
        }
    }
}

impl SqlTableTransactionsFactory for IngredientSchema {
    fn create_table(builder: impl sea_query::backend::SchemaBuilder) -> String {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Self::IngredientId)
                    .integer()
                    .auto_increment()
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new(Self::Name).text().not_null())
            .col(
                ColumnDef::new(Self::Alcoholic)
                    .boolean()
                    .not_null()
                    .default(Value::Bool(Some(false))),
            )
            .col(ColumnDef::new(Self::Description).text())
            .col(
                ColumnDef::new(Self::IsActive)
                    .boolean()
                    .not_null()
                    .default(Value::Bool(Some(false))),
            )
            .col(ColumnDef::new(Self::Amount).float())
            .col(ColumnDef::new(Self::IngredientType).integer().not_null())
            .col(ColumnDef::new(Self::FrId).integer())
            .col(ColumnDef::new(Self::InstructionId).integer())
            .foreign_key(
                ForeignKeyCreateStatement::new()
                    .name("fk_fluidregulation")
                    .from(Self::Table, Self::FrId)
                    .to(FluidRegulationSchema::Table, FluidRegulationSchema::FrId)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKeyCreateStatement::new()
                    .name("fk_instruction")
                    .from(Self::Table, Self::InstructionId)
                    .to(InstructionSchema::Table, InstructionSchema::InstructionId)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .build(builder)
    }

    fn alter_table(
        builder: impl sea_query::backend::SchemaBuilder,
        column_def: &mut ColumnDef,
    ) -> String {
        Table::alter()
            .table(Self::Table)
            .add_column(column_def)
            .build(builder)
    }
}

#[derive(Iden, Eq, PartialEq, Debug)]
#[iden = "Instruction"]
pub enum InstructionSchema {
    Table,
    InstructionId,
    InstructionDetail,
    InstructionName,
}
impl SqlTransactionsFactory for InstructionSchema {
    fn column_to_str(&self) -> &'static str {
        match self {
            Self::Table => "Instruction",
            Self::InstructionId => "instruction_id",
            Self::InstructionDetail => "instruction_detail",
            Self::InstructionName => "instruction_name",
        }
    }

    fn from_str(value: &'static str) -> Option<Self>
    where
        Self: Sized,
    {
        match value.to_lowercase().as_str() {
            "instruction" => Some(Self::Table),
            "instruction_id" => Some(Self::InstructionId),
            "instruction_detail" => Some(Self::InstructionDetail),
            "instruction_name" => Some(Self::InstructionName),
            _ => None,
        }
    }
}
impl SqlTableTransactionsFactory for InstructionSchema {
    fn create_table(builder: impl sea_query::backend::SchemaBuilder) -> String {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Self::InstructionId)
                    .integer()
                    .auto_increment()
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new(Self::InstructionDetail).text())
            .col(ColumnDef::new(Self::InstructionName).text().not_null())
            .build(builder)
    }

    fn alter_table(
        builder: impl sea_query::backend::SchemaBuilder,
        column_def: &mut ColumnDef,
    ) -> String {
        Table::alter()
            .table(Self::Table)
            .add_column(column_def)
            .build(builder)
    }
}

#[derive(Iden, Eq, PartialEq, Debug)]
#[iden = "InstructionToRecipe"]
pub enum InstructionToRecipeSchema {
    Table,
    RecipeId,
    InstructionId,
    InstructionOrder,
}
impl SqlTransactionsFactory for InstructionToRecipeSchema {
    fn column_to_str(&self) -> &'static str {
        match self {
            Self::Table => "InstructionToRecipe",
            Self::RecipeId => "recipe_id",
            Self::InstructionId => "instruction_id",
            Self::InstructionOrder => "instruction_order",
        }
    }
    fn from_str(value: &'static str) -> Option<Self> {
        match value {
            "InstructionToRecipe" => Some(Self::Table),
            "recipe_id" => Some(Self::RecipeId),
            "instruction_id" => Some(Self::InstructionId),
            "instruction_order" => Some(Self::InstructionOrder),
            _ => None,
        }
    }
}
impl SqlTableTransactionsFactory for InstructionToRecipeSchema {
    fn create_table(builder: impl sea_query::backend::SchemaBuilder) -> String {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(ColumnDef::new(Self::RecipeId).integer())
            .col(ColumnDef::new(Self::InstructionId).integer())
            .foreign_key(
                ForeignKeyCreateStatement::new()
                    .name("fk_recipe")
                    .from(Self::Table, Self::RecipeId)
                    .to(RecipeSchema::Table, RecipeSchema::RecipeId)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKeyCreateStatement::new()
                    .name("fk_instruction")
                    .from(Self::Table, Self::InstructionId)
                    .to(InstructionSchema::Table, InstructionSchema::InstructionId)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .col(ColumnDef::new(Self::InstructionOrder).integer().not_null())
            .build(builder)
    }

    fn alter_table(
        builder: impl sea_query::backend::SchemaBuilder,
        column_def: &mut ColumnDef,
    ) -> String {
        Table::alter()
            .table(Self::Table)
            .add_column(column_def)
            .build(builder)
    }
}
#[derive(Iden, Eq, PartialEq, Debug)]
#[iden = "Recipe"]
pub enum RecipeSchema {
    Table,
    RecipeId,
    Name,
    UserInput,
    DrinkSize,
    Description,
}
impl SqlTransactionsFactory for RecipeSchema {
    fn column_to_str(&self) -> &'static str {
        match self {
            Self::Table => "Recipe",
            Self::RecipeId => "recipe_id",
            Self::Name => "name",
            Self::UserInput => "user_input",
            Self::DrinkSize => "drink_size",
            Self::Description => "description",
        }
    }
    fn from_str(value: &'static str) -> Option<Self> {
        match value {
            "Recipe" => Some(Self::Table),
            "recipe_id" => Some(Self::RecipeId),
            "name" => Some(Self::Name),
            "user_input" => Some(Self::UserInput),
            "drink_size" => Some(Self::DrinkSize),
            "description" => Some(Self::Description),
            _ => None,
        }
    }
}
impl SqlTableTransactionsFactory for RecipeSchema {
    fn create_table(builder: impl sea_query::backend::SchemaBuilder) -> String {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Self::RecipeId)
                    .integer()
                    .auto_increment()
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new(Self::Name).text().not_null().unique_key())
            .col(
                ColumnDef::new(Self::UserInput)
                    .boolean()
                    .not_null()
                    .default(Value::Bool(Some(false))),
            )
            .col(
                ColumnDef::new(Self::DrinkSize)
                    .integer()
                    .not_null()
                    .default(Value::Int(Some(0))),
            )
            .col(
                ColumnDef::new(Self::Description)
                    .text()
                    .not_null()
                    .unique_key(),
            )
            .build(builder)
    }

    fn alter_table(
        builder: impl sea_query::backend::SchemaBuilder,
        column_def: &mut ColumnDef,
    ) -> String {
        Table::alter()
            .table(Self::Table)
            .add_column(column_def)
            .build(builder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn column_to_str() {
        let fr = FluidRegulationSchema::GpioPin;
        assert_eq!(fr.column_to_str(), "gpio_pin")
    }

    #[test]
    fn str_to_column() {
        let fr_str = "gpio_pin";
        assert_eq!(
            FluidRegulationSchema::from_str(fr_str),
            Some(FluidRegulationSchema::GpioPin)
        )
    }
}
