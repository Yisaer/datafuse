// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

#[test]
fn test_data_block() -> anyhow::Result<()> {
    use std::sync::Arc;

    use common_datavalues::*;

    use crate::DataBlock;

    let schema = DataSchemaRefExt::create(vec![DataField::new("a", DataType::Int64, false)]);

    let block = DataBlock::create(schema.clone(), vec![Arc::new(Int64Array::from(vec![
        1, 2, 3,
    ]))]);
    assert_eq!(&schema, block.schema());

    assert_eq!(3, block.num_rows());
    assert_eq!(1, block.num_columns());
    assert_eq!(3, block.try_column_by_name("a")?.len());
    assert_eq!(3, block.column(0).len());

    assert_eq!(true, block.column_by_name("a").is_some());
    assert_eq!(None, block.column_by_name("a_not_found"));

    Ok(())
}
