// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

#[test]
fn test_having_plan() -> anyhow::Result<()> {
    use pretty_assertions::assert_eq;

    use crate::*;

    let source = Test::create().generate_source_plan_for_test(10000)?;
    let plan = PlanBuilder::from(&source)
        .having(col("number").eq(lit(1i64)))?
        .project(&[col("number")])?
        .build()?;

    let expect = "\
    Projection: number:UInt64\
    \n  Having: (number = 1)\
    \n    ReadDataSource: scan partitions: [8], scan schema: [number:UInt64], statistics: [read_rows: 10000, read_bytes: 80000]";
    let actual = format!("{:?}", plan);

    assert_eq!(expect, actual);
    Ok(())
}
