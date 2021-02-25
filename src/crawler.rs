use crate::args::{TotalInstitutionInvestorsArgs, TotalInstitutionInvestorsKinds};
use crate::errors::TwseError;
use crate::schema::{Data, Response, TotalInstitutionalInvestors};

const INVESTORS_LINK: &str = "https://www.twse.com.tw/fund/BFI82U";
const TWSE_STATE_OK: &str = "OK";

fn get_total_institution_investors(
    args: TotalInstitutionInvestorsArgs,
) -> Result<Vec<TotalInstitutionalInvestors>, TwseError> {
    let mut request = ureq::get(INVESTORS_LINK).query("response", "json");
    let date = format!("{}", args.date.format("%Y%m%d"));

    match args.kind {
        TotalInstitutionInvestorsKinds::DAY => {
            request = request.query("dayDate", &date).query("type", "day");
        }
        TotalInstitutionInvestorsKinds::WEEK => {
            request = request.query("weekDate", &date).query("type", "week");
        }
        TotalInstitutionInvestorsKinds::MONTH => {
            request = request.query("monthDate", &date).query("type", "month");
        }
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
    fn test_get_total_institution_investors_week_ok() {
        let got = get_total_institution_investors(TotalInstitutionInvestorsArgs {
            date: NaiveDate::parse_from_str("2021-02-01", "%Y-%m-%d").unwrap(),
            kind: TotalInstitutionInvestorsKinds::WEEK,
        });
        assert!(got.is_ok())
    }

    #[test]
    fn test_get_total_institution_investors_month_ok() {
        let got = get_total_institution_investors(TotalInstitutionInvestorsArgs {
            date: NaiveDate::parse_from_str("2021-02-01", "%Y-%m-%d").unwrap(),
            kind: TotalInstitutionInvestorsKinds::MONTH,
        });
        assert!(got.is_ok())
    }

    #[test]
    fn test_get_total_institution_investors_day_ok() {
        let got = get_total_institution_investors(TotalInstitutionInvestorsArgs {
            date: NaiveDate::parse_from_str("2021-02-01", "%Y-%m-%d").unwrap(),
            kind: TotalInstitutionInvestorsKinds::DAY,
        });
        assert!(got.is_ok())
    }

    #[test]
    fn test_get_total_institution_investors_week_not_ok() {
        let got = get_total_institution_investors(TotalInstitutionInvestorsArgs {
            date: NaiveDate::parse_from_str("6021-02-01", "%Y-%m-%d").unwrap(),
            kind: TotalInstitutionInvestorsKinds::WEEK,
        });
        assert!(got.is_err())
    }

    #[test]
    fn test_get_total_institution_investors_month_not_ok() {
        let got = get_total_institution_investors(TotalInstitutionInvestorsArgs {
            date: NaiveDate::parse_from_str("6021-02-01", "%Y-%m-%d").unwrap(),
            kind: TotalInstitutionInvestorsKinds::MONTH,
        });
        assert!(got.is_err())
    }

    #[test]
    fn test_get_total_institution_investors_day_not_ok() {
        let got = get_total_institution_investors(TotalInstitutionInvestorsArgs {
            date: NaiveDate::parse_from_str("6021-02-01", "%Y-%m-%d").unwrap(),
            kind: TotalInstitutionInvestorsKinds::DAY,
        });
        assert!(got.is_err())
    }
}
