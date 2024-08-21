use apache_avro::{schema::*, Schema};


pub fn all_schemas_to_generate(root_schemas: Vec<&Schema>) -> Vec<&Schema> 
{
    let sub_schemas: Vec<&Schema> = root_schemas
        .iter()
        .map(|s| schemas_to_generate(s))
        .flat_map(|s|s)
        .collect();

    root_schemas
    .into_iter()
    .chain(sub_schemas)
    .collect()
}

pub fn schemas_to_generate(schema: &Schema) -> Vec<&Schema> 
{
    match schema {
        Schema::Array(subtype) => schemas_to_generate(subtype),
        Schema::Map(subtype) => schemas_to_generate(subtype),
        Schema::Union(subtype) => schemas_to_generate_union(subtype),// TODO:  VERIFY
        Schema::Record(r) => {
            let mut list= schemas_to_generate_record(r);
            list.push(schema);
            list
        },
        Schema::Enum(_) => Vec::from([schema]),// TODO:  VERIFY
        _ => Vec::new(),
    }
}

pub fn schemas_to_generate_union(schema: &UnionSchema) -> Vec<&Schema> {

    schema
    .variants()
    .iter()
    .map(|f| schemas_to_generate(&f))
    .flat_map(|c| c)
    .collect()
}


pub fn schemas_to_generate_record(schema: &RecordSchema) -> Vec<&Schema> {

    schema.fields
    .iter()
    .map(|f| schemas_to_generate(&f.schema))
    .flat_map(|c| c)
    .collect()
}
