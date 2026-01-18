#[cfg(test)]
mod schema_tests {
    use super::super::*;
    use sea_query::PostgresQueryBuilder;

    // FluidRegulationSchema Tests
    #[test]
    fn test_fluid_regulation_column_to_str() {
        assert_eq!(FluidRegulationSchema::Table.column_to_str(), "FluidRegulation");
        assert_eq!(FluidRegulationSchema::FrId.column_to_str(), "fr_id");
        assert_eq!(FluidRegulationSchema::GpioPin.column_to_str(), "gpio_pin");
        assert_eq!(FluidRegulationSchema::RegulatorType.column_to_str(), "regulator_type");
        assert_eq!(FluidRegulationSchema::PumpNum.column_to_str(), "pump_num");
    }

    #[test]
    fn test_fluid_regulation_from_str() {
        assert_eq!(
            FluidRegulationSchema::from_str("FluidRegulation"),
            Some(FluidRegulationSchema::Table)
        );
        assert_eq!(
            FluidRegulationSchema::from_str("fr_id"),
            Some(FluidRegulationSchema::FrId)
        );
        assert_eq!(
            FluidRegulationSchema::from_str("gpio_pin"),
            Some(FluidRegulationSchema::GpioPin)
        );
        assert_eq!(
            FluidRegulationSchema::from_str("regulator_type"),
            Some(FluidRegulationSchema::RegulatorType)
        );
        assert_eq!(
            FluidRegulationSchema::from_str("pump_num"),
            Some(FluidRegulationSchema::PumpNum)
        );
        assert_eq!(FluidRegulationSchema::from_str("invalid"), None);
    }

    #[test]
    fn test_fluid_regulation_try_from_string() {
        assert_eq!(
            FluidRegulationSchema::try_from("FluidRegulation".to_string()).unwrap(),
            FluidRegulationSchema::Table
        );
        assert_eq!(
            FluidRegulationSchema::try_from("fr_id".to_string()).unwrap(),
            FluidRegulationSchema::FrId
        );
        assert!(FluidRegulationSchema::try_from("invalid".to_string()).is_err());
    }

    #[test]
    fn test_fluid_regulation_create_table() {
        let sql = FluidRegulationSchema::create_table(PostgresQueryBuilder);
        assert!(sql.contains("CREATE TABLE"));
        assert!(sql.contains("IF NOT EXISTS"));
        assert!(sql.contains("FluidRegulation"));
        assert!(sql.contains("fr_id"));
        assert!(sql.contains("gpio_pin"));
        assert!(sql.contains("regulator_type"));
        assert!(sql.contains("pump_num"));
        assert!(sql.contains("PRIMARY KEY"));
    }

    #[test]
    fn test_fluid_regulation_display() {
        let display_str = format!("{}", FluidRegulationSchema::Table);
        assert!(display_str.contains("Valid Fields are"));
        assert!(display_str.contains("fr_id"));
        assert!(display_str.contains("gpio_pin"));
    }

    // PumpLogSchema Tests
    #[test]
    fn test_pump_log_column_to_str() {
        assert_eq!(PumpLogSchema::Table.column_to_str(), "Pumplog");
        assert_eq!(PumpLogSchema::ReqId.column_to_str(), "ReqId");
        assert_eq!(PumpLogSchema::ReqType.column_to_str(), "ReqType");
        assert_eq!(PumpLogSchema::FluidId.column_to_str(), "FluidId");
    }

    #[test]
    fn test_pump_log_from_str() {
        assert_eq!(PumpLogSchema::from_str("Pumplog"), Some(PumpLogSchema::Table));
        assert_eq!(PumpLogSchema::from_str("ReqId"), Some(PumpLogSchema::ReqId));
        assert_eq!(PumpLogSchema::from_str("ReqType"), Some(PumpLogSchema::ReqType));
        assert_eq!(PumpLogSchema::from_str("FluidId"), Some(PumpLogSchema::FluidId));
        assert_eq!(PumpLogSchema::from_str("invalid"), None);
    }

    #[test]
    fn test_pump_log_try_from_string() {
        assert_eq!(
            PumpLogSchema::try_from("Pumplog".to_string()).unwrap(),
            PumpLogSchema::Table
        );
        assert_eq!(
            PumpLogSchema::try_from("ReqId".to_string()).unwrap(),
            PumpLogSchema::ReqId
        );
        assert!(PumpLogSchema::try_from("invalid".to_string()).is_err());
    }

    #[test]
    fn test_pump_log_create_table() {
        let sql = PumpLogSchema::create_table(PostgresQueryBuilder);
        assert!(sql.contains("CREATE TABLE"), "SQL should contain CREATE TABLE");
        assert!(sql.contains("IF NOT EXISTS"), "SQL should contain IF NOT EXISTS");
        assert!(sql.contains("Pumplog"), "SQL should contain Pumplog table name");
        // sea-query generates snake_case column names
        assert!(sql.contains("req_id"), "SQL should contain req_id column");
        assert!(sql.contains("req_type"), "SQL should contain req_type column");
        assert!(sql.contains("fluid_id"), "SQL should contain fluid_id column");
        assert!(sql.contains("uuid"), "req_id should be uuid type");
        assert!(sql.contains("PRIMARY KEY"), "req_id should be primary key");
    }

    // IngredientSchema Tests
    #[test]
    fn test_ingredient_column_to_str() {
        assert_eq!(IngredientSchema::Table.column_to_str(), "Ingredient");
        assert_eq!(IngredientSchema::IngredientId.column_to_str(), "ingredient_id");
        assert_eq!(IngredientSchema::Name.column_to_str(), "name");
        assert_eq!(IngredientSchema::Alcoholic.column_to_str(), "alcoholic");
        assert_eq!(IngredientSchema::Description.column_to_str(), "description");
        assert_eq!(IngredientSchema::IsActive.column_to_str(), "is_active");
        assert_eq!(IngredientSchema::FrId.column_to_str(), "fr_id");
        assert_eq!(IngredientSchema::Amount.column_to_str(), "amount");
        assert_eq!(IngredientSchema::InstructionId.column_to_str(), "instruction_id");
    }

    #[test]
    fn test_ingredient_from_str() {
        assert_eq!(
            IngredientSchema::from_str("Ingredient"),
            Some(IngredientSchema::Table)
        );
        assert_eq!(
            IngredientSchema::from_str("ingredient_id"),
            Some(IngredientSchema::IngredientId)
        );
        assert_eq!(IngredientSchema::from_str("name"), Some(IngredientSchema::Name));
        assert_eq!(
            IngredientSchema::from_str("alcoholic"),
            Some(IngredientSchema::Alcoholic)
        );
        assert_eq!(IngredientSchema::from_str("invalid"), None);
    }

    #[test]
    fn test_ingredient_try_from_string() {
        assert_eq!(
            IngredientSchema::try_from("Ingredient".to_string()).unwrap(),
            IngredientSchema::Table
        );
        assert_eq!(
            IngredientSchema::try_from("name".to_string()).unwrap(),
            IngredientSchema::Name
        );
        assert!(IngredientSchema::try_from("invalid".to_string()).is_err());
    }

    #[test]
    fn test_ingredient_create_table() {
        let sql = IngredientSchema::create_table(PostgresQueryBuilder);
        assert!(sql.contains("CREATE TABLE"));
        assert!(sql.contains("IF NOT EXISTS"));
        assert!(sql.contains("Ingredient"));
        assert!(sql.contains("ingredient_id"));
        assert!(sql.contains("name"));
        assert!(sql.contains("alcoholic"));
        assert!(sql.contains("description"));
        assert!(sql.contains("FOREIGN KEY"));
    }

    // InstructionSchema Tests
    #[test]
    fn test_instruction_column_to_str() {
        assert_eq!(InstructionSchema::Table.column_to_str(), "Instruction");
        assert_eq!(InstructionSchema::InstructionId.column_to_str(), "instruction_id");
        assert_eq!(InstructionSchema::InstructionDetail.column_to_str(), "instruction_detail");
        assert_eq!(InstructionSchema::InstructionName.column_to_str(), "instruction_name");
    }

    #[test]
    fn test_instruction_from_str() {
        assert_eq!(
            InstructionSchema::from_str("instruction"),
            Some(InstructionSchema::Table)
        );
        assert_eq!(
            InstructionSchema::from_str("instruction_id"),
            Some(InstructionSchema::InstructionId)
        );
        assert_eq!(
            InstructionSchema::from_str("instruction_detail"),
            Some(InstructionSchema::InstructionDetail)
        );
        assert_eq!(
            InstructionSchema::from_str("instruction_name"),
            Some(InstructionSchema::InstructionName)
        );
        assert_eq!(InstructionSchema::from_str("invalid"), None);
    }

    #[test]
    fn test_instruction_from_str_case_insensitive() {
        assert_eq!(
            InstructionSchema::from_str("INSTRUCTION"),
            Some(InstructionSchema::Table)
        );
        assert_eq!(
            InstructionSchema::from_str("INSTRUCTION_ID"),
            Some(InstructionSchema::InstructionId)
        );
    }

    #[test]
    fn test_instruction_try_from_string() {
        assert_eq!(
            InstructionSchema::try_from("instruction".to_string()).unwrap(),
            InstructionSchema::Table
        );
        assert_eq!(
            InstructionSchema::try_from("instruction_id".to_string()).unwrap(),
            InstructionSchema::InstructionId
        );
        assert!(InstructionSchema::try_from("invalid".to_string()).is_err());
    }

    #[test]
    fn test_instruction_create_table() {
        let sql = InstructionSchema::create_table(PostgresQueryBuilder);
        assert!(sql.contains("CREATE TABLE"));
        assert!(sql.contains("IF NOT EXISTS"));
        assert!(sql.contains("Instruction"));
        assert!(sql.contains("instruction_id"));
        assert!(sql.contains("instruction_detail"));
        assert!(sql.contains("instruction_name"));
    }

    // InstructionToRecipeSchema Tests
    #[test]
    fn test_instruction_to_recipe_column_to_str() {
        assert_eq!(InstructionToRecipeSchema::Table.column_to_str(), "InstructionToRecipe");
        assert_eq!(InstructionToRecipeSchema::Id.column_to_str(), "id");
        assert_eq!(InstructionToRecipeSchema::RecipeId.column_to_str(), "recipe_id");
        assert_eq!(InstructionToRecipeSchema::InstructionId.column_to_str(), "instruction_id");
        assert_eq!(InstructionToRecipeSchema::InstructionOrder.column_to_str(), "instruction_order");
    }

    #[test]
    fn test_instruction_to_recipe_from_str() {
        assert_eq!(
            InstructionToRecipeSchema::from_str("InstructionToRecipe"),
            Some(InstructionToRecipeSchema::Table)
        );
        assert_eq!(
            InstructionToRecipeSchema::from_str("recipe_id"),
            Some(InstructionToRecipeSchema::RecipeId)
        );
        assert_eq!(
            InstructionToRecipeSchema::from_str("instruction_id"),
            Some(InstructionToRecipeSchema::InstructionId)
        );
        assert_eq!(InstructionToRecipeSchema::from_str("invalid"), None);
    }

    #[test]
    fn test_instruction_to_recipe_try_from_string() {
        assert_eq!(
            InstructionToRecipeSchema::try_from("InstructionToRecipe".to_string()).unwrap(),
            InstructionToRecipeSchema::Table
        );
        assert_eq!(
            InstructionToRecipeSchema::try_from("recipe_id".to_string()).unwrap(),
            InstructionToRecipeSchema::RecipeId
        );
        assert!(InstructionToRecipeSchema::try_from("invalid".to_string()).is_err());
    }

    #[test]
    fn test_instruction_to_recipe_create_table() {
        let sql = InstructionToRecipeSchema::create_table(PostgresQueryBuilder);
        assert!(sql.contains("CREATE TABLE"));
        assert!(sql.contains("IF NOT EXISTS"));
        assert!(sql.contains("InstructionToRecipe"));
        assert!(sql.contains("recipe_id"));
        assert!(sql.contains("instruction_id"));
        assert!(sql.contains("instruction_order"));
        assert!(sql.contains("FOREIGN KEY"));
    }

    // RecipeSchema Tests
    #[test]
    fn test_recipe_column_to_str() {
        assert_eq!(RecipeSchema::Table.column_to_str(), "Recipe");
        assert_eq!(RecipeSchema::RecipeId.column_to_str(), "recipe_id");
        assert_eq!(RecipeSchema::Name.column_to_str(), "name");
        assert_eq!(RecipeSchema::UserInput.column_to_str(), "user_input");
        assert_eq!(RecipeSchema::DrinkSize.column_to_str(), "drink_size");
        assert_eq!(RecipeSchema::Description.column_to_str(), "description");
    }

    #[test]
    fn test_recipe_from_str() {
        assert_eq!(RecipeSchema::from_str("Recipe"), Some(RecipeSchema::Table));
        assert_eq!(RecipeSchema::from_str("recipe_id"), Some(RecipeSchema::RecipeId));
        assert_eq!(RecipeSchema::from_str("name"), Some(RecipeSchema::Name));
        assert_eq!(RecipeSchema::from_str("user_input"), Some(RecipeSchema::UserInput));
        assert_eq!(RecipeSchema::from_str("drink_size"), Some(RecipeSchema::DrinkSize));
        assert_eq!(RecipeSchema::from_str("description"), Some(RecipeSchema::Description));
        assert_eq!(RecipeSchema::from_str("invalid"), None);
    }

    #[test]
    fn test_recipe_try_from_string() {
        assert_eq!(
            RecipeSchema::try_from("Recipe".to_string()).unwrap(),
            RecipeSchema::Table
        );
        assert_eq!(
            RecipeSchema::try_from("name".to_string()).unwrap(),
            RecipeSchema::Name
        );
        // Note: RecipeSchema has a default case that returns Table for invalid inputs
        assert_eq!(
            RecipeSchema::try_from("invalid".to_string()).unwrap(),
            RecipeSchema::Table
        );
    }

    #[test]
    fn test_recipe_create_table() {
        let sql = RecipeSchema::create_table(PostgresQueryBuilder);
        assert!(sql.contains("CREATE TABLE"));
        assert!(sql.contains("IF NOT EXISTS"));
        assert!(sql.contains("Recipe"));
        assert!(sql.contains("recipe_id"));
        assert!(sql.contains("name"));
        assert!(sql.contains("user_input"));
        assert!(sql.contains("drink_size"));
        assert!(sql.contains("description"));
        assert!(sql.contains("UNIQUE"));
    }

    // DbType Tests
    #[test]
    fn test_db_type_equality() {
        use crate::parsers::settings::{PostgresConfigurer, SqliteConfigurer};

        let pg_config1 = PostgresConfigurer::default();
        let pg_config2 = PostgresConfigurer::default();

        let db_type1 = DbType::Postgres(pg_config1.clone());
        let db_type2 = DbType::Postgres(pg_config2.clone());

        assert_eq!(db_type1, db_type2);

        let sqlite_config1 = SqliteConfigurer::default();
        let sqlite_config2 = SqliteConfigurer::default();

        let db_type3 = DbType::Sqlite(sqlite_config1.clone());
        let db_type4 = DbType::Sqlite(sqlite_config2.clone());

        assert_eq!(db_type3, db_type4);
    }

    // BinaryType Tests
    #[test]
    fn test_binary_type_equality() {
        assert_eq!(BinaryType::Bin, BinaryType::Bin);
        assert_eq!(BinaryType::Daemon, BinaryType::Daemon);
        assert_eq!(BinaryType::DrinkCtrl, BinaryType::DrinkCtrl);
        assert_ne!(BinaryType::Bin, BinaryType::Daemon);
        assert_ne!(BinaryType::Daemon, BinaryType::DrinkCtrl);
    }

    #[test]
    fn test_binary_type_clone() {
        let bin_type = BinaryType::Daemon;
        let cloned = bin_type.clone();
        assert_eq!(bin_type, cloned);
    }

    // Truncate table tests
    #[test]
    fn test_truncate_table_fluid_regulation() {
        let sql = FluidRegulationSchema::truncate_table(
            FluidRegulationSchema::Table,
            PostgresQueryBuilder,
        );
        assert!(sql.contains("TRUNCATE"));
        assert!(sql.contains("FluidRegulation"));
    }

    #[test]
    fn test_truncate_table_pump_log() {
        let sql = PumpLogSchema::truncate_table(PumpLogSchema::Table, PostgresQueryBuilder);
        assert!(sql.contains("TRUNCATE"));
        assert!(sql.contains("Pumplog"));
    }

    #[test]
    fn test_truncate_table_ingredient() {
        let sql = IngredientSchema::truncate_table(IngredientSchema::Table, PostgresQueryBuilder);
        assert!(sql.contains("TRUNCATE"));
        assert!(sql.contains("Ingredient"));
    }

    #[test]
    fn test_truncate_table_instruction() {
        let sql = InstructionSchema::truncate_table(InstructionSchema::Table, PostgresQueryBuilder);
        assert!(sql.contains("TRUNCATE"));
        assert!(sql.contains("Instruction"));
    }

    #[test]
    fn test_truncate_table_recipe() {
        let sql = RecipeSchema::truncate_table(RecipeSchema::Table, PostgresQueryBuilder);
        assert!(sql.contains("TRUNCATE"));
        assert!(sql.contains("Recipe"));
    }

    // Display trait tests
    #[test]
    fn test_pump_log_display() {
        let display_str = format!("{}", PumpLogSchema::Table);
        assert!(display_str.contains("Valid Fields are"));
        assert!(display_str.contains("req_id"));
        assert!(display_str.contains("req_type"));
        assert!(display_str.contains("fluid_id"));
    }

    #[test]
    fn test_ingredient_display() {
        let display_str = format!("{}", IngredientSchema::Table);
        assert!(display_str.contains("Valid Fields are"));
        assert!(display_str.contains("ingredient_id"));
        assert!(display_str.contains("name"));
        assert!(display_str.contains("alcoholic"));
    }

    #[test]
    fn test_instruction_display() {
        let display_str = format!("{}", InstructionSchema::Table);
        assert!(display_str.contains("Valid Fields are"));
        assert!(display_str.contains("instruction_id"));
        assert!(display_str.contains("instruction_detail"));
    }

    #[test]
    fn test_instruction_to_recipe_display() {
        let display_str = format!("{}", InstructionToRecipeSchema::Table);
        assert!(display_str.contains("Valid Fields are"));
        assert!(display_str.contains("recipe_id"));
        assert!(display_str.contains("instruction_id"));
        assert!(display_str.contains("instruction_order"));
    }

    #[test]
    fn test_recipe_display() {
        let display_str = format!("{}", RecipeSchema::Table);
        assert!(display_str.contains("Valid Fields are"));
        assert!(display_str.contains("recipe_id"));
        assert!(display_str.contains("name"));
        assert!(display_str.contains("user_input"));
    }
}
