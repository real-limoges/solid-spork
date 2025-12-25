/*
This is where data transformation logic lives. It should be a collection of pure functions
and an orchestrator, denoted `transform_orch`
 */
use polars::prelude::DataFrame;

pub fn transform_orch(data: DataFrame) -> DataFrame {
    data
}