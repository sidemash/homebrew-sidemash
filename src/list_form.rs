use serde::{Deserialize, Serialize};
use structopt::StructOpt;

use crate::http_request::QueryString;

#[derive(Debug, Serialize, Deserialize, StructOpt)]
pub struct ListForm {
    /// Set the order of returned List. Default value is "createdTime:DESC"
    /// @see <https://developers.sidemash.com/qs-query/order-by>
    #[structopt(long)]
    pub (crate) order_by: Option<String>,

    /// Set the number of returned items. Default value is 50
    #[structopt(long)]
    pub (crate) limit : Option<u16>,

    /// Conditions to filter the returned list. Default value is None.  @see <https://developers.sidemash.com/qs-query/where>
    #[structopt(long)]
    pub (crate) _where : Option<String>
}

impl ListForm {

    pub fn to_query_string(&self) -> QueryString  {
        let mut qs = vec![];
        if self.order_by.is_some() { qs.push((String::from("orderBy"), self.order_by.as_ref().unwrap().to_string()) ) }
        if self.limit.is_some() { qs.push((String::from("limit"), self.limit.as_ref().unwrap().to_string()) ) }
        if self.order_by.is_some() { qs.push((String::from("where"), self._where.as_ref().unwrap().to_string()) ) }
        qs
    }
}