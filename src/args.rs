use chrono::NaiveDate;

#[derive(Debug, Copy, Clone)]
pub struct TotalInstitutionInvestorsArgs {
    pub date: NaiveDate,
    pub date_type: DateType,
}

#[derive(Debug, Copy, Clone)]
pub struct SingleStockInstitutionalInvestorsArgs {
    pub date: NaiveDate,
    pub date_type: DateType,
    pub industry_type: IndustryType,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum DateType {
    Day,
    Week,
    Month,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum IndustryType {
    All,
    AllWithoutWarrantsAndCBBCs,
    ClosedEndFunds,
    ETF,
    ETN,
    BeneficialSecurities,
    SpecialStockWithOptions,
    CorporateBondsWithStockOptions,
    Warrants,
    Cement,
    Food,
    Plastic,
    TextileFiber,
    ElectricalMachinery,
    ElectricalCable,
    MedicalTechnology,
    Chemical,
    BiotechMedical,
    GlassCeramics,
    Paper,
    Metal,
    Rubber,
    Car,
    Electronics,
    Semiconductor,
    ComputerAndPeripheralEquipment,
    Optoelectronics,
    CommunicationNetwork,
    ElectronicComponents,
    ElectronicChannel,
    InformationService,
    OtherElectronics,
    BuildingMaterialsConstruction,
    Shipping,
    Tourism,
    FinancialInsurance,
    TradeDepartmentStore,
    OilAndElectricityAndGas,
    DepositoryReceipts,
    Comprehensive,
    Other,
    ConvertibleCorporateBonds,
}

#[allow(dead_code)]
impl IndustryType {
    pub fn value(&self) -> &str {
        match *self {
            IndustryType::All => "ALL",
            IndustryType::AllWithoutWarrantsAndCBBCs => "ALLBUT0999",
            IndustryType::ClosedEndFunds => "0049",
            IndustryType::ETF => "ETF",
            IndustryType::ETN => "ETN",
            IndustryType::BeneficialSecurities => "019919T",
            IndustryType::SpecialStockWithOptions => "0999GA",
            IndustryType::CorporateBondsWithStockOptions => "0999GD",
            IndustryType::Warrants => "0999G9",
            IndustryType::Cement => "01",
            IndustryType::Food => "02",
            IndustryType::Plastic => "03",
            IndustryType::TextileFiber => "04",
            IndustryType::ElectricalMachinery => "05",
            IndustryType::ElectricalCable => "06",
            IndustryType::MedicalTechnology => "07",
            IndustryType::Chemical => "21",
            IndustryType::BiotechMedical => "22",
            IndustryType::GlassCeramics => "08",
            IndustryType::Paper => "09",
            IndustryType::Metal => "10",
            IndustryType::Rubber => "11",
            IndustryType::Car => "12",
            IndustryType::Electronics => "13",
            IndustryType::Semiconductor => "24",
            IndustryType::ComputerAndPeripheralEquipment => "25",
            IndustryType::Optoelectronics => "26",
            IndustryType::CommunicationNetwork => "27",
            IndustryType::ElectronicComponents => "28",
            IndustryType::ElectronicChannel => "29",
            IndustryType::InformationService => "30",
            IndustryType::OtherElectronics => "31",
            IndustryType::BuildingMaterialsConstruction => "14",
            IndustryType::Shipping => "15",
            IndustryType::Tourism => "16",
            IndustryType::FinancialInsurance => "17",
            IndustryType::TradeDepartmentStore => "18",
            IndustryType::OilAndElectricityAndGas => "23",
            IndustryType::DepositoryReceipts => "9299",
            IndustryType::Comprehensive => "19",
            IndustryType::Other => "20",
            IndustryType::ConvertibleCorporateBonds => "CB",
        }
    }
}
