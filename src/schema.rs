use alloy_sol_types::SolType;

pub struct SchemaField<T: SolType> {
    name: String,
    sol_type: T,
}
