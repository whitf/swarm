




pub const TABLE_COUNT: usize = 0;
pub const FUNCTION_COUNT: usize = 0;

















pub const SELECT_TABLE_COUNT: &str = "
	SELECT count(*) FROM sqlite_master WHERE type = 'table' AND name != 'sqlite_sequence';
";
