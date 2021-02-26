use crate::args::{
    DateType, IndustryType, SingleStockInstitutionalInvestorsArgs, TotalInstitutionInvestorsArgs,
};
use crate::errors::TwseError;
use crate::schema::{
    Data, Response, SingleStockInstitutionalInvestors, TotalInstitutionalInvestors,
};

const SINGLE_INVESTORS_DAY_LINK: &str = "https://www.twse.com.tw/fund/T86";
const SINGLE_INVESTORS_WEEK_LINK: &str = "https://www.twse.com.tw/fund/TWT54U";
const SINGLE_INVESTORS_MONTH_LINK: &str = "https://www.twse.com.tw/fund/TWT47U";
const INVESTORS_LINK: &str = "https://www.twse.com.tw/fund/BFI82U";
const TWSE_STATE_OK: &str = "OK";

fn get_single_stock_institutional_investors(
    args: SingleStockInstitutionalInvestorsArgs,
) -> Result<Vec<SingleStockInstitutionalInvestors>, TwseError> {
    let date = format!("{}", args.date.format("%Y%m%d"));
    let mut request = ureq::get(SINGLE_INVESTORS_DAY_LINK)
        .query("response", "json")
        .query("date", &date);

    match args.date_type {
        DateType::Day => request = request.query("selectType", args.industry_type.value()),
        DateType::Week => {
            request = ureq::get(SINGLE_INVESTORS_WEEK_LINK)
                .query("response", "json")
                .query("date", &date)
                .query("selectType", args.industry_type.value())
        }

        DateType::Month => {
            request = ureq::get(SINGLE_INVESTORS_MONTH_LINK)
                .query("response", "json")
                .query("date", &date)
                .query("selectType", args.industry_type.value())
        }
    }

    let response: Response = request.call()?.into_json()?;
    if response.stat != TWSE_STATE_OK {
        return Err(TwseError::TWSEError(response.stat));
    }

    let mut ret: Vec<SingleStockInstitutionalInvestors> = vec![];
    for data in response.data {
        if let Data::SingleStockInstitutionalInvestors(d) = data {
            ret.push(d);
        } else {
            return Err(TwseError::EnumCastingError);
        }
    }

    Ok(ret)
}

fn get_total_institution_investors(
    args: TotalInstitutionInvestorsArgs,
) -> Result<Vec<TotalInstitutionalInvestors>, TwseError> {
    let mut request = ureq::get(INVESTORS_LINK).query("response", "json");
    let date = format!("{}", args.date.format("%Y%m%d"));

    match args.date_type {
        DateType::Day => request = request.query("dayDate", &date).query("type", "day"),
        DateType::Week => request = request.query("weekDate", &date).query("type", "week"),
        DateType::Month => request = request.query("monthDate", &date).query("type", "month"),
    }

    let response: Response = request.call()?.into_json()?;
    if response.stat != TWSE_STATE_OK {
        return Err(TwseError::TWSEError(response.stat));
    }

    let mut ret: Vec<TotalInstitutionalInvestors> = Vec::with_capacity(response.data.len());
    for data in response.data {
        if let Data::TotalInstitutionalInvestors(d) = data {
            ret.push(d);
        } else {
            return Err(TwseError::EnumCastingError);
        }
    }

    Ok(ret)
}

#[cfg(test)]
use chrono::NaiveDate;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_single_stock_institutional_investors() {
        let date = NaiveDate::parse_from_str("2021-02-01", "%Y-%m-%d").unwrap();
        let wrong_date = NaiveDate::parse_from_str("6021-02-01", "%Y-%m-%d").unwrap();

        let test_cases = vec![
            (
                SingleStockInstitutionalInvestorsArgs {
                    date: date,
                    date_type: DateType::Week,
                    industry_type: IndustryType::Cement,
                },
                "week date_type should be ok",
                true,
            ),
            (
                SingleStockInstitutionalInvestorsArgs {
                    date: date,
                    date_type: DateType::Day,
                    industry_type: IndustryType::Cement,
                },
                "day date_type should be ok",
                true,
            ),
            (
                SingleStockInstitutionalInvestorsArgs {
                    date: date,
                    date_type: DateType::Month,
                    industry_type: IndustryType::Cement,
                },
                "month date_type should be ok",
                true,
            ),
            (
                SingleStockInstitutionalInvestorsArgs {
                    date: wrong_date,
                    date_type: DateType::Week,
                    industry_type: IndustryType::Cement,
                },
                "week date_type should not be ok",
                false,
            ),
            (
                SingleStockInstitutionalInvestorsArgs {
                    date: wrong_date,
                    date_type: DateType::Day,
                    industry_type: IndustryType::Cement,
                },
                "day date_type should not be ok",
                false,
            ),
            (
                SingleStockInstitutionalInvestorsArgs {
                    date: wrong_date,
                    date_type: DateType::Month,
                    industry_type: IndustryType::Cement,
                },
                "month date_type should not be ok",
                false,
            ),
        ];

        for (input, description, want) in test_cases {
            let got = get_single_stock_institutional_investors(input);
            assert_eq!(want, got.is_ok(), "[{}]: input:{:?}", description, input);
        }
    }

    #[test]
    fn test_get_total_institution_investors() {
        let date = NaiveDate::parse_from_str("2021-02-01", "%Y-%m-%d").unwrap();
        let wrong_date = NaiveDate::parse_from_str("6021-02-01", "%Y-%m-%d").unwrap();

        let test_cases = vec![
            (
                TotalInstitutionInvestorsArgs {
                    date: date,
                    date_type: DateType::Week,
                },
                "week date_type should be ok",
                true,
            ),
            (
                TotalInstitutionInvestorsArgs {
                    date: date,
                    date_type: DateType::Day,
                },
                "day date_type should be ok",
                true,
            ),
            (
                TotalInstitutionInvestorsArgs {
                    date: date,
                    date_type: DateType::Month,
                },
                "month date_type should be ok",
                true,
            ),
            (
                TotalInstitutionInvestorsArgs {
                    date: wrong_date,
                    date_type: DateType::Week,
                },
                "week date_type should not be ok",
                false,
            ),
            (
                TotalInstitutionInvestorsArgs {
                    date: wrong_date,
                    date_type: DateType::Day,
                },
                "day date_type should not be ok",
                false,
            ),
            (
                TotalInstitutionInvestorsArgs {
                    date: wrong_date,
                    date_type: DateType::Month,
                },
                "month date_type should not be ok",
                false,
            ),
        ];

        for (input, description, want) in test_cases {
            let got = get_total_institution_investors(input);
            assert_eq!(want, got.is_ok(), "[{}]: input:{:?}", description, input);
        }
    }
}
