use anyhow::{anyhow, Result};
use polars::prelude::*;
use sqlparser::parser::Parser;
use std::ops::{Deref, DerefMut};
use tracing::info;

mod convert;
mod dialect;
mod loader;
mod fetcher;
use convert::Sql;
use loader::detect_content;
use fetcher::retrieve_data;

pub use dialect::example_sql;
pub use dialect::TyrDialect;

#[derive(Debug)]
pub struct DataSet(DataFrame);

/// 让DataSet用起来和DataFrame一致
impl Deref for DataSet {
    type Target = DataFrame;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// 让DataSet用起来和DataFram一致
impl DerefMut for DataSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DataSet {
    // 从DataSet转换成csv
    pub fn to_csv(&self) -> Result<String> {
        let mut buf = Vec::new();
        let writer = CsvWriter::new(&mut buf);
        writer.finish(self)?;
        Ok(String::from_utf8(buf)?)
    }
}

/// 从from 中获取数据,从where中过滤，最后选取需要返回的列
pub async fn query<T: AsRef<str>>(sql: T) -> Result<DataSet> {
    let ast = Parser::parse_sql(&TyrDialect::default(), sql.as_ref())?;

    if ast.len() != 1 {
        return Err(anyhow!("Only support single sql at the moment"));
    }

    let sql = &ast[0];

    // 整个SQL AST转换成我们定义的Sql结构的细节都埋藏在try_into()中
    // 我们只需要关注数据结构的使用，怎么转换可以之后需要的时候才关注，这是
    // 关注点分离，是我们控制软件复杂度的法宝
    let Sql {
        source,
        condition,
        selection,
        offset,
        limit,
        order_by,
    } = sql.try_into()?;

    info!("retrieving data from source: {}", source);

    // 从source读入一个DataSet
    // detect_content， 怎么detect不重要，重要的是能根据内容返回DataSet
    let ds = detect_content(retrieve_data(source).await?).load()?;

    let mut filtered = match condition {
        Some(expr) => ds.0.lazy().filter(expr),
        None => ds.0.lazy(),
    };

    filtered = order_by
        .into_iter()
        .fold(filtered, |acc, (col, desc)| acc.sort(&col, desc));

    if offset.is_some() || limit.is_some() {
        filtered = filtered.slice(offset.unwrap_or(0), limit.unwrap_or(usize::MAX));
    }

    Ok(DataSet(filtered.select(selection).collect()?))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
