use chrono::NaiveDate;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Response {
    stat: String,
    #[serde(deserialize_with = "parse_date")]
    date: NaiveDate,
    title: String,
    fields: Vec<String>,
    data: Vec<Data>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", untagged)]
pub enum Data {
    TotalInstitutionalInvestors(TotalInstitutionalInvestors),
    SingleStockInstitutionalInvestors(SingleStockInstitutionalInvestors),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TotalInstitutionalInvestors {
    name: String,
    #[serde(deserialize_with = "parse_u64")]
    buy: u64,
    #[serde(deserialize_with = "parse_u64")]
    sell: u64,
    #[serde(deserialize_with = "parse_i64")]
    difference: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SingleStockInstitutionalInvestors {
    stock_id: String,
    stock_name: String,
    #[serde(deserialize_with = "parse_u64")]
    buy: u64,
    #[serde(deserialize_with = "parse_u64")]
    sell: u64,
    #[serde(deserialize_with = "parse_i64")]
    difference: i64,
}

const TWSE_DATETIME_FORMAT: &'static str = "%Y%m%d";

fn parse_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, TWSE_DATETIME_FORMAT).map_err(serde::de::Error::custom)
}

fn parse_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.replace(",", "")
        .parse::<u64>()
        .map_err(serde::de::Error::custom)
}

fn parse_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.replace(",", "")
        .parse::<i64>()
        .map_err(serde::de::Error::custom)
}

#[cfg(test)]
use anyhow::{Context, Result};
#[cfg(test)]
use serde_json;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_with_total_institutional_investors() -> Result<()> {
        let response = r#"
        {
            "stat":"OK",
            "title":"110年02月05日 三大法人買賣金額統計表",
            "fields":["單位名稱","買進金額","賣出金額","買賣差額"],
            "date":"20210205",
            "data":[
                ["自營商(自行買賣)","3,720,692,397","2,657,469,622","1,063,222,775"],
                ["自營商(避險)","7,930,149,338","6,161,180,878","1,768,968,460"],
                ["投信","2,305,132,480","1,693,938,980","611,193,500"],
                ["外資及陸資","73,576,875,154","79,202,203,729","-5,625,328,575"],
                ["合計","87,532,849,369","89,714,793,209","-2,181,943,840"]
            ],
            "params":{"controller":"fund","format":null,"action":"BFI82U","lang":"zh","monthDate":"20210205","weekDate":"20210201","dayDate":"20210205"},
            "notes":[
                "自營商表示證券自營商專戶。",
                "投信表示本國投資信託基金。",
                "外資及陸資表示依「華僑及外國人投資證券管理辦法」及「大陸地區投資人來臺從事證券投資及期貨交易管理辦法」辦理登記等投資人。",
                "本統計資訊含一般、零股、盤後定價、鉅額，不含拍賣、標購。",
                "本資訊以當日原始成交情形統計，不以證券商申報錯帳、更正帳號等調整後資料統計。"
            ]
        }
        "#;
        let result: Response = serde_json::from_str(response)
            .with_context(|| format!("serde_json::from_str failed"))?;

        assert_eq!(result.stat, "OK");
        assert_eq!(result.data.len(), 5);

        let wants = [
            TotalInstitutionalInvestors {
                name: "自營商(自行買賣)".to_string(),
                buy: 3720692397,
                sell: 2657469622,
                difference: 1063222775,
            },
            TotalInstitutionalInvestors {
                name: "自營商(避險)".to_string(),
                buy: 7930149338,
                sell: 6161180878,
                difference: 1768968460,
            },
            TotalInstitutionalInvestors {
                name: "投信".to_string(),
                buy: 2305132480,
                sell: 1693938980,
                difference: 611193500,
            },
            TotalInstitutionalInvestors {
                name: "外資及陸資".to_string(),
                buy: 73576875154,
                sell: 79202203729,
                difference: -5625328575,
            },
            TotalInstitutionalInvestors {
                name: "合計".to_string(),
                buy: 87532849369,
                sell: 89714793209,
                difference: -2181943840,
            },
        ];

        for n in 0..result.data.len() {
            if let Data::TotalInstitutionalInvestors(d) = &result.data[n] {
                assert_eq!(d.buy, wants[n].buy);
                assert_eq!(d.sell, wants[n].sell);
                assert_eq!(d.difference, wants[n].difference);
                assert_eq!(d.name, wants[n].name);
            } else {
                assert!(false, "data vector index {} cannot cast out", n);
            }
        }

        Ok(())
    }
}
