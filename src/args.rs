use chrono::NaiveDate;

pub struct TotalInstitutionInvestorsArgs {
    pub date: NaiveDate,
    pub kind: TotalInstitutionInvestorsKinds,
}

pub enum TotalInstitutionInvestorsKinds {
    DAY,
    WEEK,
    MONTH,
}
