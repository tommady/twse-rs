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
    #[serde(deserialize_with = "clean_string")]
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
    #[serde(deserialize_with = "clean_string")]
    stock_id: String,
    #[serde(deserialize_with = "clean_string")]
    stock_name: String,
    #[serde(deserialize_with = "parse_u64")]
    foreign_investor_buy: u64,
    #[serde(deserialize_with = "parse_u64")]
    foreign_investor_sell: u64,
    #[serde(deserialize_with = "parse_i64")]
    foreign_investor_difference: i64,
    #[serde(deserialize_with = "parse_u64")]
    foreign_dealer_self_buy: u64,
    #[serde(deserialize_with = "parse_u64")]
    foreign_dealer_self_sell: u64,
    #[serde(deserialize_with = "parse_i64")]
    foreign_dealer_self_difference: i64,
    #[serde(deserialize_with = "parse_u64")]
    investment_trust_buy: u64,
    #[serde(deserialize_with = "parse_u64")]
    investment_trust_sell: u64,
    #[serde(deserialize_with = "parse_i64")]
    investment_trust_difference: i64,
    #[serde(deserialize_with = "parse_i64")]
    dealer_total_difference: i64,
    #[serde(deserialize_with = "parse_u64")]
    dealer_self_buy: u64,
    #[serde(deserialize_with = "parse_u64")]
    dealer_self_sell: u64,
    #[serde(deserialize_with = "parse_i64")]
    dealer_self_difference: i64,
    #[serde(deserialize_with = "parse_u64")]
    dealer_hedging_buy: u64,
    #[serde(deserialize_with = "parse_u64")]
    dealer_hedging_sell: u64,
    #[serde(deserialize_with = "parse_i64")]
    dealer_difference: i64,
    #[serde(deserialize_with = "parse_i64")]
    total_difference: i64,
}

const TWSE_DATETIME_FORMAT: &'static str = "%Y%m%d";

fn parse_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, TWSE_DATETIME_FORMAT).map_err(serde::de::Error::custom)
}

fn clean_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.replace(" ", ""))
    // NaiveDate::parse_from_str(&s, TWSE_DATETIME_FORMAT).map_err(serde::de::Error::custom)
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

    #[test]
    fn test_response_with_single_stock_institutional_investors() -> Result<()> {
        let response = r#"
        {
            "stat":"OK",
            "date":"20210205",
            "title":"110年02月05日 三大法人買賣超日報",
            "fields":[
                "證券代號",
                "證券名稱",
                "外陸資買進股數(不含外資自營商)",
                "外陸資賣出股數(不含外資自營商)",
                "外陸資買賣超股數(不含外資自營商)",
                "外資自營商買進股數",
                "外資自營商賣出股數",
                "外資自營商買賣超股數",
                "投信買進股數",
                "投信賣出股數",
                "投信買賣超股數",
                "自營商買賣超股數",
                "自營商買進股數(自行買賣)",
                "自營商賣出股數(自行買賣)",
                "自營商買賣超股數(自行買賣)",
                "自營商買進股數(避險)",
                "自營商賣出股數(避險)",
                "自營商買賣超股數(避險)",
                "三大法人買賣超股數"
            ],
            "data":[
                ["1104","環泥            ","121,000","86,000","35,000","0","0","0","0","0","0","3,000","3,000","0","3,000","0","0","0","38,000"],
                ["1108","幸福            ","46,000","30,000","16,000","0","0","0","0","0","0","2,000","2,000","0","2,000","0","0","0","18,000"],
                ["1109","信大            ","32,000","25,000","7,000","0","0","0","0","0","0","0","0","0","0","0","0","0","7,000"],
                ["1101B","台泥乙特        ","0","0","0","0","0","0","0","0","0","1,000","1,000","0","1,000","0","0","0","1,000"]
            ],
            "selectType":"01",
            "notes":[
                "自營商表示證券自營商專戶。",
                "投信表示本國投資信託基金。",
                "外資及陸資表示依「華僑及外國人投資證券管理辦法」及「大陸地區投資人來臺從事證券投資及期貨交易管理辦法」辦理登記等投資人。",
                "外資自營商買賣股數已計入自營商買賣股數，故不納入三大法人買賣股數之合計數計算。",
                "本統計資訊含一般、零股、盤後定價、鉅額，不含拍賣、標購。",
                "本資訊以當日原始成交情形統計，不以證券商申報錯帳、更正帳號等調整後資料統計。",
                "ETF證券代號第六碼為K、M、S、C者，表示該ETF以外幣交易。"
            ]
        }
        "#;
        let result: Response = serde_json::from_str(response)
            .with_context(|| format!("serde_json::from_str failed"))?;

        assert_eq!(result.stat, "OK");
        assert_eq!(result.data.len(), 4);

        let wants = [
            SingleStockInstitutionalInvestors {
                stock_id: "1104".to_string(),
                stock_name: "環泥".to_string(),
                foreign_investor_buy: 121000,
                foreign_investor_sell: 86000,
                foreign_investor_difference: 35000,
                foreign_dealer_self_buy: 0,
                foreign_dealer_self_sell: 0,
                foreign_dealer_self_difference: 0,
                investment_trust_buy: 0,
                investment_trust_sell: 0,
                investment_trust_difference: 0,
                dealer_total_difference: 3000,
                dealer_self_buy: 3000,
                dealer_self_sell: 0,
                dealer_self_difference: 3000,
                dealer_hedging_buy: 0,
                dealer_hedging_sell: 0,
                dealer_difference: 0,
                total_difference: 38000,
            },
            SingleStockInstitutionalInvestors {
                stock_id: "1108".to_string(),
                stock_name: "幸福".to_string(),
                foreign_investor_buy: 46000,
                foreign_investor_sell: 30000,
                foreign_investor_difference: 16000,
                foreign_dealer_self_buy: 0,
                foreign_dealer_self_sell: 0,
                foreign_dealer_self_difference: 0,
                investment_trust_buy: 0,
                investment_trust_sell: 0,
                investment_trust_difference: 0,
                dealer_total_difference: 2000,
                dealer_self_buy: 2000,
                dealer_self_sell: 0,
                dealer_self_difference: 2000,
                dealer_hedging_buy: 0,
                dealer_hedging_sell: 0,
                dealer_difference: 0,
                total_difference: 18000,
            },
            SingleStockInstitutionalInvestors {
                stock_id: "1109".to_string(),
                stock_name: "信大".to_string(),
                foreign_investor_buy: 32000,
                foreign_investor_sell: 25000,
                foreign_investor_difference: 7000,
                foreign_dealer_self_buy: 0,
                foreign_dealer_self_sell: 0,
                foreign_dealer_self_difference: 0,
                investment_trust_buy: 0,
                investment_trust_sell: 0,
                investment_trust_difference: 0,
                dealer_total_difference: 0,
                dealer_self_buy: 0,
                dealer_self_sell: 0,
                dealer_self_difference: 0,
                dealer_hedging_buy: 0,
                dealer_hedging_sell: 0,
                dealer_difference: 0,
                total_difference: 7000,
            },
            SingleStockInstitutionalInvestors {
                stock_id: "1101B".to_string(),
                stock_name: "台泥乙特".to_string(),
                foreign_investor_buy: 0,
                foreign_investor_sell: 0,
                foreign_investor_difference: 0,
                foreign_dealer_self_buy: 0,
                foreign_dealer_self_sell: 0,
                foreign_dealer_self_difference: 0,
                investment_trust_buy: 0,
                investment_trust_sell: 0,
                investment_trust_difference: 0,
                dealer_total_difference: 1000,
                dealer_self_buy: 1000,
                dealer_self_sell: 0,
                dealer_self_difference: 1000,
                dealer_hedging_buy: 0,
                dealer_hedging_sell: 0,
                dealer_difference: 0,
                total_difference: 1000,
            },
            SingleStockInstitutionalInvestors {
                stock_id: "1110".to_string(),
                stock_name: "東泥".to_string(),
                foreign_investor_buy: 28000,
                foreign_investor_sell: 31000,
                foreign_investor_difference: -3000,
                foreign_dealer_self_buy: 0,
                foreign_dealer_self_sell: 0,
                foreign_dealer_self_difference: 0,
                investment_trust_buy: 0,
                investment_trust_sell: 0,
                investment_trust_difference: 0,
                dealer_total_difference: 0,
                dealer_self_buy: 0,
                dealer_self_sell: 0,
                dealer_self_difference: 0,
                dealer_hedging_buy: 0,
                dealer_hedging_sell: 0,
                dealer_difference: 0,
                total_difference: -3000,
            },
        ];

        for n in 0..result.data.len() {
            if let Data::SingleStockInstitutionalInvestors(d) = &result.data[n] {
                assert_eq!(d.stock_id, wants[n].stock_id);
                assert_eq!(d.stock_name, wants[n].stock_name);
                assert_eq!(d.foreign_investor_buy, wants[n].foreign_investor_buy);
                assert_eq!(d.foreign_investor_sell, wants[n].foreign_investor_sell);
                assert_eq!(
                    d.foreign_investor_difference,
                    wants[n].foreign_investor_difference
                );
                assert_eq!(d.foreign_dealer_self_buy, wants[n].foreign_dealer_self_buy);
                assert_eq!(
                    d.foreign_dealer_self_sell,
                    wants[n].foreign_dealer_self_sell
                );
                assert_eq!(
                    d.foreign_dealer_self_difference,
                    wants[n].foreign_dealer_self_difference
                );
                assert_eq!(d.investment_trust_buy, wants[n].investment_trust_buy);
                assert_eq!(d.investment_trust_sell, wants[n].investment_trust_sell);
                assert_eq!(
                    d.investment_trust_difference,
                    wants[n].investment_trust_difference
                );
                assert_eq!(d.dealer_total_difference, wants[n].dealer_total_difference);
                assert_eq!(d.dealer_self_buy, wants[n].dealer_self_buy);
                assert_eq!(d.dealer_self_sell, wants[n].dealer_self_sell);
                assert_eq!(d.dealer_self_difference, wants[n].dealer_self_difference);
                assert_eq!(d.dealer_hedging_buy, wants[n].dealer_hedging_buy);
                assert_eq!(d.dealer_hedging_sell, wants[n].dealer_hedging_sell);
                assert_eq!(d.dealer_difference, wants[n].dealer_difference);
                assert_eq!(d.total_difference, wants[n].total_difference);
            } else {
                assert!(false, "data vector index {} cannot cast out", n);
            }
        }

        Ok(())
    }
}
