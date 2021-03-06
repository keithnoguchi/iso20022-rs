#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnspecifiedType {
    #[prost(string, required, tag = "1")]
    pub base_object_type: std::string::String,
    #[prost(bytes, required, tag = "2")]
    pub object: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountSchemeName1Choice {
    #[prost(string, optional, tag = "1")]
    pub cd: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "2")]
    pub prtry: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveCurrencyAndAmount {
    #[prost(string, required, tag = "1")]
    pub ccy: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveOrHistoricCurrencyAndAmount {
    #[prost(string, required, tag = "1")]
    pub ccy: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdviceType1Choice {
    #[prost(
        enumeration = "advice_type1_choice::AdviceType1Code",
        optional,
        tag = "1"
    )]
    pub cd: ::std::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub prtry: ::std::option::Option<std::string::String>,
}
pub mod advice_type1_choice {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AdviceType1Code {
        Adnd = 1,
        Adwd = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AmountOrRate1Choice {
    #[prost(message, optional, tag = "1")]
    pub amt: ::std::option::Option<ActiveCurrencyAndAmount>,
    #[prost(double, optional, tag = "2")]
    pub rate: ::std::option::Option<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CashAccountType2Choice {
    #[prost(string, optional, tag = "1")]
    pub cd: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "2")]
    pub prtry: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CategoryPurpose1Choice {
    #[prost(string, optional, tag = "1")]
    pub cd: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "2")]
    pub prtry: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChequeDeliveryMethod1Choice {
    #[prost(
        enumeration = "cheque_delivery_method1_choice::ChequeDelivery1Code",
        optional,
        tag = "1"
    )]
    pub cd: ::std::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub prtry: ::std::option::Option<std::string::String>,
}
pub mod cheque_delivery_method1_choice {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ChequeDelivery1Code {
        Crcd = 1,
        Crdb = 2,
        Crfa = 3,
        Mlcd = 4,
        Mldb = 5,
        Mlfa = 6,
        Pucd = 7,
        Pudb = 8,
        Pufa = 9,
        Rgcd = 10,
        Rgdb = 11,
        Rgfa = 12,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearingSystemIdentification2Choice {
    #[prost(string, optional, tag = "1")]
    pub cd: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "2")]
    pub prtry: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearingSystemMemberIdentification2 {
    #[prost(message, optional, tag = "1")]
    pub clr_sys_id: ::std::option::Option<ClearingSystemIdentification2Choice>,
    #[prost(string, required, tag = "2")]
    pub mmb_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreditorReferenceType1Choice {
    #[prost(
        enumeration = "creditor_reference_type1_choice::DocumentType3Code",
        optional,
        tag = "1"
    )]
    pub cd: ::std::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub prtry: ::std::option::Option<std::string::String>,
}
pub mod creditor_reference_type1_choice {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DocumentType3Code {
        Disp = 1,
        Fxdr = 2,
        Puor = 3,
        Radm = 4,
        Rpin = 5,
        Scor = 6,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreditorReferenceType2 {
    #[prost(message, required, tag = "1")]
    pub cd_or_prtry: CreditorReferenceType1Choice,
    #[prost(string, optional, tag = "2")]
    pub issr: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateAndDateTime2Choice {
    #[prost(int32, optional, tag = "1")]
    pub dt: ::std::option::Option<i32>,
    #[prost(int64, optional, tag = "2")]
    pub dt_tm: ::std::option::Option<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateAndPlaceOfBirth1 {
    #[prost(int32, required, tag = "1")]
    pub birth_dt: i32,
    #[prost(string, required, tag = "2")]
    pub city_of_birth: std::string::String,
    #[prost(string, required, tag = "3")]
    pub ctry_of_birth: std::string::String,
    #[prost(string, optional, tag = "4")]
    pub prvc_of_birth: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatePeriod2 {
    #[prost(int32, required, tag = "1")]
    pub fr_dt: i32,
    #[prost(int32, required, tag = "2")]
    pub to_dt: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiscountAmountType1Choice {
    #[prost(string, optional, tag = "1")]
    pub cd: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "2")]
    pub prtry: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentAdjustment1 {
    #[prost(string, optional, tag = "1")]
    pub addtl_inf: ::std::option::Option<std::string::String>,
    #[prost(message, required, tag = "2")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[prost(
        enumeration = "document_adjustment1::CreditDebitCode",
        optional,
        tag = "3"
    )]
    pub cdt_dbt_ind: ::std::option::Option<i32>,
    #[prost(string, optional, tag = "4")]
    pub rsn: ::std::option::Option<std::string::String>,
}
pub mod document_adjustment1 {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CreditDebitCode {
        Crdt = 1,
        Dbit = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentLineType1Choice {
    #[prost(string, optional, tag = "1")]
    pub cd: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "2")]
    pub prtry: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EquivalentAmount2 {
    #[prost(message, required, tag = "1")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[prost(string, required, tag = "2")]
    pub ccy_of_trf: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinancialIdentificationSchemeName1Choice {
    #[prost(string, optional, tag = "1")]
    pub cd: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "2")]
    pub prtry: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FrequencyAndMoment1 {
    #[prost(string, required, tag = "1")]
    pub pt_in_tm: std::string::String,
    #[prost(
        enumeration = "frequency_and_moment1::Frequency6Code",
        required,
        tag = "2"
    )]
    pub tp: i32,
}
pub mod frequency_and_moment1 {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Frequency6Code {
        Adho = 1,
        Dail = 2,
        Frtn = 3,
        Inda = 4,
        Mian = 5,
        Mnth = 6,
        Qurt = 7,
        Week = 8,
        Year = 9,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FrequencyPeriod1 {
    #[prost(double, required, tag = "1")]
    pub cnt_per_prd: f64,
    #[prost(enumeration = "frequency_period1::Frequency6Code", required, tag = "2")]
    pub tp: i32,
}
pub mod frequency_period1 {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Frequency6Code {
        Adho = 1,
        Dail = 2,
        Frtn = 3,
        Inda = 4,
        Mian = 5,
        Mnth = 6,
        Qurt = 7,
        Week = 8,
        Year = 9,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GarnishmentType1Choice {
    #[prost(string, optional, tag = "1")]
    pub cd: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "2")]
    pub prtry: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericAccountIdentification1 {
    #[prost(string, required, tag = "1")]
    pub id: std::string::String,
    #[prost(string, optional, tag = "2")]
    pub issr: ::std::option::Option<std::string::String>,
    #[prost(message, optional, tag = "3")]
    pub schme_nm: ::std::option::Option<AccountSchemeName1Choice>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericFinancialIdentification1 {
    #[prost(string, required, tag = "1")]
    pub id: std::string::String,
    #[prost(string, optional, tag = "2")]
    pub issr: ::std::option::Option<std::string::String>,
    #[prost(message, optional, tag = "3")]
    pub schme_nm: ::std::option::Option<FinancialIdentificationSchemeName1Choice>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericIdentification1 {
    #[prost(string, required, tag = "1")]
    pub id: std::string::String,
    #[prost(string, optional, tag = "2")]
    pub issr: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "3")]
    pub schme_nm: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericIdentification30 {
    #[prost(string, required, tag = "1")]
    pub id: std::string::String,
    #[prost(string, required, tag = "2")]
    pub issr: std::string::String,
    #[prost(string, optional, tag = "3")]
    pub schme_nm: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstructionForCreditorAgent3 {
    #[prost(string, optional, tag = "1")]
    pub cd: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "2")]
    pub instr_inf: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalInstrument2Choice {
    #[prost(string, optional, tag = "1")]
    pub cd: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "2")]
    pub prtry: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MandateClassification1Choice {
    #[prost(
        enumeration = "mandate_classification1_choice::MandateClassification1Code",
        optional,
        tag = "1"
    )]
    pub cd: ::std::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub prtry: ::std::option::Option<std::string::String>,
}
pub mod mandate_classification1_choice {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MandateClassification1Code {
        Fixe = 1,
        Usgb = 2,
        Vari = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MandateSetupReason1Choice {
    #[prost(string, optional, tag = "1")]
    pub cd: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "2")]
    pub prtry: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrganisationIdentificationSchemeName1Choice {
    #[prost(string, optional, tag = "1")]
    pub cd: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "2")]
    pub prtry: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OtherContact1 {
    #[prost(string, required, tag = "1")]
    pub chanl_tp: std::string::String,
    #[prost(string, optional, tag = "2")]
    pub id: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentCondition1 {
    #[prost(bool, required, tag = "1")]
    pub amt_mod_allwd: bool,
    #[prost(string, optional, tag = "2")]
    pub dely_pnlty: ::std::option::Option<std::string::String>,
    #[prost(bool, required, tag = "3")]
    pub early_pmt_allwd: bool,
    #[prost(bool, required, tag = "4")]
    pub grnted_pmt_reqd: bool,
    #[prost(message, optional, tag = "5")]
    pub imdt_pmt_rbt: ::std::option::Option<AmountOrRate1Choice>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentIdentification6 {
    #[prost(string, required, tag = "1")]
    pub end_to_end_id: std::string::String,
    #[prost(string, optional, tag = "2")]
    pub instr_id: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "3")]
    pub uetr: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersonIdentificationSchemeName1Choice {
    #[prost(string, optional, tag = "1")]
    pub cd: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "2")]
    pub prtry: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProxyAccountType1Choice {
    #[prost(string, optional, tag = "1")]
    pub cd: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "2")]
    pub prtry: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Purpose2Choice {
    #[prost(string, optional, tag = "1")]
    pub cd: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "2")]
    pub prtry: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReferredDocumentType3Choice {
    #[prost(
        enumeration = "referred_document_type3_choice::DocumentType6Code",
        optional,
        tag = "1"
    )]
    pub cd: ::std::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub prtry: ::std::option::Option<std::string::String>,
}
pub mod referred_document_type3_choice {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DocumentType6Code {
        Aroi = 1,
        Bold = 2,
        Cinv = 3,
        Cmcn = 4,
        Cnfa = 5,
        Cren = 6,
        Debn = 7,
        Disp = 8,
        Dnfa = 9,
        Hiri = 10,
        Msin = 11,
        Puor = 12,
        Sbin = 13,
        Soac = 14,
        Tsut = 15,
        Vchr = 16,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReferredDocumentType4 {
    #[prost(message, required, tag = "1")]
    pub cd_or_prtry: ReferredDocumentType3Choice,
    #[prost(string, optional, tag = "2")]
    pub issr: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegulatoryAuthority2 {
    #[prost(string, optional, tag = "1")]
    pub ctry: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "2")]
    pub nm: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceLevel8Choice {
    #[prost(string, optional, tag = "1")]
    pub cd: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "2")]
    pub prtry: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SkipPayload {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StructuredRegulatoryReporting3 {
    #[prost(message, optional, tag = "1")]
    pub amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    #[prost(string, optional, tag = "2")]
    pub cd: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "3")]
    pub ctry: ::std::option::Option<std::string::String>,
    #[prost(int32, optional, tag = "4")]
    pub dt: ::std::option::Option<i32>,
    #[prost(string, repeated, tag = "5")]
    pub inf: ::std::vec::Vec<std::string::String>,
    #[prost(string, optional, tag = "6")]
    pub tp: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SupplementaryDataEnvelope1 {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaxAmountType1Choice {
    #[prost(string, optional, tag = "1")]
    pub cd: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "2")]
    pub prtry: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaxAuthorisation1 {
    #[prost(string, optional, tag = "1")]
    pub nm: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "2")]
    pub titl: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaxParty1 {
    #[prost(string, optional, tag = "1")]
    pub regn_id: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "2")]
    pub tax_id: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "3")]
    pub tax_tp: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaxParty2 {
    #[prost(message, optional, tag = "1")]
    pub authstn: ::std::option::Option<TaxAuthorisation1>,
    #[prost(string, optional, tag = "2")]
    pub regn_id: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "3")]
    pub tax_id: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "4")]
    pub tax_tp: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaxPeriod2 {
    #[prost(message, optional, tag = "1")]
    pub fr_to_dt: ::std::option::Option<DatePeriod2>,
    #[prost(enumeration = "tax_period2::TaxRecordPeriod1Code", optional, tag = "2")]
    pub tp: ::std::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub yr: ::std::option::Option<i32>,
}
pub mod tax_period2 {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TaxRecordPeriod1Code {
        Hlf1 = 1,
        Hlf2 = 2,
        Mm01 = 3,
        Mm02 = 4,
        Mm03 = 5,
        Mm04 = 6,
        Mm05 = 7,
        Mm06 = 8,
        Mm07 = 9,
        Mm08 = 10,
        Mm09 = 11,
        Mm10 = 12,
        Mm11 = 13,
        Mm12 = 14,
        Qtr1 = 15,
        Qtr2 = 16,
        Qtr3 = 17,
        Qtr4 = 18,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaxRecordDetails2 {
    #[prost(message, required, tag = "1")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[prost(message, optional, tag = "2")]
    pub prd: ::std::option::Option<TaxPeriod2>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountIdentification4Choice {
    #[prost(string, optional, tag = "1")]
    pub iban: ::std::option::Option<std::string::String>,
    #[prost(message, optional, tag = "2")]
    pub othr: ::std::option::Option<GenericAccountIdentification1>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressType3Choice {
    #[prost(
        enumeration = "address_type3_choice::AddressType2Code",
        optional,
        tag = "1"
    )]
    pub cd: ::std::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub prtry: ::std::option::Option<GenericIdentification30>,
}
pub mod address_type3_choice {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AddressType2Code {
        Addr = 1,
        Bizz = 2,
        Dlvy = 3,
        Home = 4,
        Mlto = 5,
        Pbox = 6,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdviceType1 {
    #[prost(message, optional, tag = "1")]
    pub cdt_advc: ::std::option::Option<AdviceType1Choice>,
    #[prost(message, optional, tag = "2")]
    pub dbt_advc: ::std::option::Option<AdviceType1Choice>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AmountType4Choice {
    #[prost(message, optional, tag = "1")]
    pub eqvt_amt: ::std::option::Option<EquivalentAmount2>,
    #[prost(message, optional, tag = "2")]
    pub instd_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contact4 {
    #[prost(string, optional, tag = "1")]
    pub dept: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "2")]
    pub email_adr: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "3")]
    pub email_purp: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "4")]
    pub fax_nb: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "5")]
    pub job_titl: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "6")]
    pub mob_nb: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "7")]
    pub nm: ::std::option::Option<std::string::String>,
    #[prost(enumeration = "contact4::NamePrefix2Code", optional, tag = "8")]
    pub nm_prfx: ::std::option::Option<i32>,
    #[prost(message, repeated, tag = "9")]
    pub othr: ::std::vec::Vec<OtherContact1>,
    #[prost(string, optional, tag = "10")]
    pub phne_nb: ::std::option::Option<std::string::String>,
    #[prost(
        enumeration = "contact4::PreferredContactMethod1Code",
        optional,
        tag = "11"
    )]
    pub prefrd_mtd: ::std::option::Option<i32>,
    #[prost(string, optional, tag = "12")]
    pub rspnsblty: ::std::option::Option<std::string::String>,
}
pub mod contact4 {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum NamePrefix2Code {
        Doct = 1,
        Madm = 2,
        Miks = 3,
        Miss = 4,
        Mist = 5,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PreferredContactMethod1Code {
        Cell = 1,
        Faxx = 2,
        Lett = 3,
        Mail = 4,
        Phon = 5,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreditorReferenceInformation2 {
    #[prost(string, optional, tag = "1")]
    pub r#ref: ::std::option::Option<std::string::String>,
    #[prost(message, optional, tag = "2")]
    pub tp: ::std::option::Option<CreditorReferenceType2>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiscountAmountAndType1 {
    #[prost(message, required, tag = "1")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[prost(message, optional, tag = "2")]
    pub tp: ::std::option::Option<DiscountAmountType1Choice>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentFormat1Choice {
    #[prost(string, optional, tag = "1")]
    pub cd: ::std::option::Option<std::string::String>,
    #[prost(message, optional, tag = "2")]
    pub prtry: ::std::option::Option<GenericIdentification1>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentLineType1 {
    #[prost(message, required, tag = "1")]
    pub cd_or_prtry: DocumentLineType1Choice,
    #[prost(string, optional, tag = "2")]
    pub issr: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentType1Choice {
    #[prost(string, optional, tag = "1")]
    pub cd: ::std::option::Option<std::string::String>,
    #[prost(message, optional, tag = "2")]
    pub prtry: ::std::option::Option<GenericIdentification1>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Frequency36Choice {
    #[prost(message, optional, tag = "1")]
    pub prd: ::std::option::Option<FrequencyPeriod1>,
    #[prost(message, optional, tag = "2")]
    pub pt_in_tm: ::std::option::Option<FrequencyAndMoment1>,
    #[prost(
        enumeration = "frequency36_choice::Frequency6Code",
        optional,
        tag = "3"
    )]
    pub tp: ::std::option::Option<i32>,
}
pub mod frequency36_choice {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Frequency6Code {
        Adho = 1,
        Dail = 2,
        Frtn = 3,
        Inda = 4,
        Mian = 5,
        Mnth = 6,
        Qurt = 7,
        Week = 8,
        Year = 9,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GarnishmentType1 {
    #[prost(message, required, tag = "1")]
    pub cd_or_prtry: GarnishmentType1Choice,
    #[prost(string, optional, tag = "2")]
    pub issr: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericOrganisationIdentification1 {
    #[prost(string, required, tag = "1")]
    pub id: std::string::String,
    #[prost(string, optional, tag = "2")]
    pub issr: ::std::option::Option<std::string::String>,
    #[prost(message, optional, tag = "3")]
    pub schme_nm: ::std::option::Option<OrganisationIdentificationSchemeName1Choice>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericPersonIdentification1 {
    #[prost(string, required, tag = "1")]
    pub id: std::string::String,
    #[prost(string, optional, tag = "2")]
    pub issr: ::std::option::Option<std::string::String>,
    #[prost(message, optional, tag = "3")]
    pub schme_nm: ::std::option::Option<PersonIdentificationSchemeName1Choice>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MandateTypeInformation2 {
    #[prost(message, optional, tag = "1")]
    pub clssfctn: ::std::option::Option<MandateClassification1Choice>,
    #[prost(message, optional, tag = "2")]
    pub ctgy_purp: ::std::option::Option<CategoryPurpose1Choice>,
    #[prost(message, optional, tag = "3")]
    pub lcl_instrm: ::std::option::Option<LocalInstrument2Choice>,
    #[prost(message, optional, tag = "4")]
    pub svc_lvl: ::std::option::Option<ServiceLevel8Choice>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrganisationIdentification29 {
    #[prost(string, optional, tag = "1")]
    pub any_bic: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "2")]
    pub lei: ::std::option::Option<std::string::String>,
    #[prost(message, repeated, tag = "3")]
    pub othr: ::std::vec::Vec<GenericOrganisationIdentification1>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentTypeInformation26 {
    #[prost(message, optional, tag = "1")]
    pub ctgy_purp: ::std::option::Option<CategoryPurpose1Choice>,
    #[prost(
        enumeration = "payment_type_information26::Priority2Code",
        optional,
        tag = "2"
    )]
    pub instr_prty: ::std::option::Option<i32>,
    #[prost(message, optional, tag = "3")]
    pub lcl_instrm: ::std::option::Option<LocalInstrument2Choice>,
    #[prost(message, repeated, tag = "4")]
    pub svc_lvl: ::std::vec::Vec<ServiceLevel8Choice>,
}
pub mod payment_type_information26 {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Priority2Code {
        High = 1,
        Norm = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersonIdentification13 {
    #[prost(message, optional, tag = "1")]
    pub dt_and_plc_of_birth: ::std::option::Option<DateAndPlaceOfBirth1>,
    #[prost(message, repeated, tag = "2")]
    pub othr: ::std::vec::Vec<GenericPersonIdentification1>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostalAddress24 {
    #[prost(string, repeated, tag = "1")]
    pub adr_line: ::std::vec::Vec<std::string::String>,
    #[prost(message, optional, tag = "2")]
    pub adr_tp: ::std::option::Option<AddressType3Choice>,
    #[prost(string, optional, tag = "3")]
    pub bldg_nb: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "4")]
    pub bldg_nm: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "5")]
    pub ctry: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "6")]
    pub ctry_sub_dvsn: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "7")]
    pub dept: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "8")]
    pub dstrct_nm: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "9")]
    pub flr: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "10")]
    pub pst_bx: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "11")]
    pub pst_cd: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "12")]
    pub room: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "13")]
    pub strt_nm: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "14")]
    pub sub_dept: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "15")]
    pub twn_lctn_nm: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "16")]
    pub twn_nm: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProxyAccountIdentification1 {
    #[prost(string, required, tag = "1")]
    pub id: std::string::String,
    #[prost(message, optional, tag = "2")]
    pub tp: ::std::option::Option<ProxyAccountType1Choice>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegulatoryReporting3 {
    #[prost(message, optional, tag = "1")]
    pub authrty: ::std::option::Option<RegulatoryAuthority2>,
    #[prost(
        enumeration = "regulatory_reporting3::RegulatoryReportingType1Code",
        optional,
        tag = "2"
    )]
    pub dbt_cdt_rptg_ind: ::std::option::Option<i32>,
    #[prost(message, repeated, tag = "3")]
    pub dtls: ::std::vec::Vec<StructuredRegulatoryReporting3>,
}
pub mod regulatory_reporting3 {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RegulatoryReportingType1Code {
        Both = 1,
        Cred = 2,
        Debt = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SupplementaryData1 {
    #[prost(message, required, tag = "1")]
    pub envlp: SupplementaryDataEnvelope1,
    #[prost(string, optional, tag = "2")]
    pub plc_and_nm: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaxAmount2 {
    #[prost(message, repeated, tag = "1")]
    pub dtls: ::std::vec::Vec<TaxRecordDetails2>,
    #[prost(double, optional, tag = "2")]
    pub rate: ::std::option::Option<f64>,
    #[prost(message, optional, tag = "3")]
    pub taxbl_base_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    #[prost(message, optional, tag = "4")]
    pub ttl_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaxAmountAndType1 {
    #[prost(message, required, tag = "1")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[prost(message, optional, tag = "2")]
    pub tp: ::std::option::Option<TaxAmountType1Choice>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaxRecord2 {
    #[prost(string, optional, tag = "1")]
    pub addtl_inf: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "2")]
    pub cert_id: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "3")]
    pub ctgy: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "4")]
    pub ctgy_dtls: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "5")]
    pub dbtr_sts: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "6")]
    pub frms_cd: ::std::option::Option<std::string::String>,
    #[prost(message, optional, tag = "7")]
    pub prd: ::std::option::Option<TaxPeriod2>,
    #[prost(message, optional, tag = "8")]
    pub tax_amt: ::std::option::Option<TaxAmount2>,
    #[prost(string, optional, tag = "9")]
    pub tp: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BranchData3 {
    #[prost(string, optional, tag = "1")]
    pub id: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "2")]
    pub lei: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "3")]
    pub nm: ::std::option::Option<std::string::String>,
    #[prost(message, optional, tag = "4")]
    pub pstl_adr: ::std::option::Option<PostalAddress24>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CashAccount38 {
    #[prost(string, optional, tag = "1")]
    pub ccy: ::std::option::Option<std::string::String>,
    #[prost(message, required, tag = "2")]
    pub id: AccountIdentification4Choice,
    #[prost(string, optional, tag = "3")]
    pub nm: ::std::option::Option<std::string::String>,
    #[prost(message, optional, tag = "4")]
    pub prxy: ::std::option::Option<ProxyAccountIdentification1>,
    #[prost(message, optional, tag = "5")]
    pub tp: ::std::option::Option<CashAccountType2Choice>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreditTransferMandateData1 {
    #[prost(int32, optional, tag = "1")]
    pub dt_of_sgntr: ::std::option::Option<i32>,
    #[prost(int64, optional, tag = "2")]
    pub dt_of_vrfctn: ::std::option::Option<i64>,
    #[prost(bytes, optional, tag = "3")]
    pub elctrnc_sgntr: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(int32, optional, tag = "4")]
    pub fnl_pmt_dt: ::std::option::Option<i32>,
    #[prost(message, optional, tag = "5")]
    pub frqcy: ::std::option::Option<Frequency36Choice>,
    #[prost(int32, optional, tag = "6")]
    pub frst_pmt_dt: ::std::option::Option<i32>,
    #[prost(string, optional, tag = "7")]
    pub mndt_id: ::std::option::Option<std::string::String>,
    #[prost(message, optional, tag = "8")]
    pub rsn: ::std::option::Option<MandateSetupReason1Choice>,
    #[prost(message, optional, tag = "9")]
    pub tp: ::std::option::Option<MandateTypeInformation2>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentLineIdentification1 {
    #[prost(string, optional, tag = "1")]
    pub nb: ::std::option::Option<std::string::String>,
    #[prost(int32, optional, tag = "2")]
    pub rltd_dt: ::std::option::Option<i32>,
    #[prost(message, optional, tag = "3")]
    pub tp: ::std::option::Option<DocumentLineType1>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinancialInstitutionIdentification18 {
    #[prost(string, optional, tag = "1")]
    pub bicfi: ::std::option::Option<std::string::String>,
    #[prost(message, optional, tag = "2")]
    pub clr_sys_mmb_id: ::std::option::Option<ClearingSystemMemberIdentification2>,
    #[prost(string, optional, tag = "3")]
    pub lei: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "4")]
    pub nm: ::std::option::Option<std::string::String>,
    #[prost(message, optional, tag = "5")]
    pub othr: ::std::option::Option<GenericFinancialIdentification1>,
    #[prost(message, optional, tag = "6")]
    pub pstl_adr: ::std::option::Option<PostalAddress24>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NameAndAddress16 {
    #[prost(message, required, tag = "1")]
    pub adr: PostalAddress24,
    #[prost(string, required, tag = "2")]
    pub nm: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Party38Choice {
    #[prost(message, optional, tag = "1")]
    pub org_id: ::std::option::Option<OrganisationIdentification29>,
    #[prost(message, optional, tag = "2")]
    pub prvt_id: ::std::option::Option<PersonIdentification13>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartyIdentification135 {
    #[prost(message, optional, tag = "1")]
    pub ctct_dtls: ::std::option::Option<Contact4>,
    #[prost(string, optional, tag = "2")]
    pub ctry_of_res: ::std::option::Option<std::string::String>,
    #[prost(message, optional, tag = "3")]
    pub id: ::std::option::Option<Party38Choice>,
    #[prost(string, optional, tag = "4")]
    pub nm: ::std::option::Option<std::string::String>,
    #[prost(message, optional, tag = "5")]
    pub pstl_adr: ::std::option::Option<PostalAddress24>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemittanceAmount2 {
    #[prost(message, repeated, tag = "1")]
    pub adjstmnt_amt_and_rsn: ::std::vec::Vec<DocumentAdjustment1>,
    #[prost(message, optional, tag = "2")]
    pub cdt_note_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    #[prost(message, repeated, tag = "3")]
    pub dscnt_apld_amt: ::std::vec::Vec<DiscountAmountAndType1>,
    #[prost(message, optional, tag = "4")]
    pub due_pybl_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    #[prost(message, optional, tag = "5")]
    pub rmtd_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    #[prost(message, repeated, tag = "6")]
    pub tax_amt: ::std::vec::Vec<TaxAmountAndType1>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemittanceAmount3 {
    #[prost(message, repeated, tag = "1")]
    pub adjstmnt_amt_and_rsn: ::std::vec::Vec<DocumentAdjustment1>,
    #[prost(message, optional, tag = "2")]
    pub cdt_note_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    #[prost(message, repeated, tag = "3")]
    pub dscnt_apld_amt: ::std::vec::Vec<DiscountAmountAndType1>,
    #[prost(message, optional, tag = "4")]
    pub due_pybl_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    #[prost(message, optional, tag = "5")]
    pub rmtd_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    #[prost(message, repeated, tag = "6")]
    pub tax_amt: ::std::vec::Vec<TaxAmountAndType1>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemittanceLocationData1 {
    #[prost(string, optional, tag = "1")]
    pub elctrnc_adr: ::std::option::Option<std::string::String>,
    #[prost(
        enumeration = "remittance_location_data1::RemittanceLocationMethod2Code",
        required,
        tag = "2"
    )]
    pub mtd: i32,
    #[prost(message, optional, tag = "3")]
    pub pstl_adr: ::std::option::Option<NameAndAddress16>,
}
pub mod remittance_location_data1 {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RemittanceLocationMethod2Code {
        Edic = 1,
        Emal = 2,
        Faxi = 3,
        Post = 4,
        Smsm = 5,
        Urid = 6,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaxInformation7 {
    #[prost(string, optional, tag = "1")]
    pub admstn_zone: ::std::option::Option<std::string::String>,
    #[prost(message, optional, tag = "2")]
    pub cdtr: ::std::option::Option<TaxParty1>,
    #[prost(message, optional, tag = "3")]
    pub dbtr: ::std::option::Option<TaxParty2>,
    #[prost(int32, optional, tag = "4")]
    pub dt: ::std::option::Option<i32>,
    #[prost(string, optional, tag = "5")]
    pub mtd: ::std::option::Option<std::string::String>,
    #[prost(message, repeated, tag = "6")]
    pub rcrd: ::std::vec::Vec<TaxRecord2>,
    #[prost(string, optional, tag = "7")]
    pub ref_nb: ::std::option::Option<std::string::String>,
    #[prost(double, optional, tag = "8")]
    pub seq_nb: ::std::option::Option<f64>,
    #[prost(message, optional, tag = "9")]
    pub ttl_tax_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    #[prost(message, optional, tag = "10")]
    pub ttl_taxbl_base_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    #[prost(message, optional, tag = "11")]
    pub ultmt_dbtr: ::std::option::Option<TaxParty2>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaxInformation8 {
    #[prost(string, optional, tag = "1")]
    pub admstn_zone: ::std::option::Option<std::string::String>,
    #[prost(message, optional, tag = "2")]
    pub cdtr: ::std::option::Option<TaxParty1>,
    #[prost(message, optional, tag = "3")]
    pub dbtr: ::std::option::Option<TaxParty2>,
    #[prost(int32, optional, tag = "4")]
    pub dt: ::std::option::Option<i32>,
    #[prost(string, optional, tag = "5")]
    pub mtd: ::std::option::Option<std::string::String>,
    #[prost(message, repeated, tag = "6")]
    pub rcrd: ::std::vec::Vec<TaxRecord2>,
    #[prost(string, optional, tag = "7")]
    pub ref_nb: ::std::option::Option<std::string::String>,
    #[prost(double, optional, tag = "8")]
    pub seq_nb: ::std::option::Option<f64>,
    #[prost(message, optional, tag = "9")]
    pub ttl_tax_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    #[prost(message, optional, tag = "10")]
    pub ttl_taxbl_base_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BranchAndFinancialInstitutionIdentification6 {
    #[prost(message, optional, tag = "1")]
    pub brnch_id: ::std::option::Option<BranchData3>,
    #[prost(message, required, tag = "2")]
    pub fin_instn_id: FinancialInstitutionIdentification18,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cheque11 {
    #[prost(message, optional, tag = "1")]
    pub chq_fr: ::std::option::Option<NameAndAddress16>,
    #[prost(int32, optional, tag = "2")]
    pub chq_mtrty_dt: ::std::option::Option<i32>,
    #[prost(string, optional, tag = "3")]
    pub chq_nb: ::std::option::Option<std::string::String>,
    #[prost(enumeration = "cheque11::ChequeType2Code", optional, tag = "4")]
    pub chq_tp: ::std::option::Option<i32>,
    #[prost(message, optional, tag = "5")]
    pub dlvr_to: ::std::option::Option<NameAndAddress16>,
    #[prost(message, optional, tag = "6")]
    pub dlvry_mtd: ::std::option::Option<ChequeDeliveryMethod1Choice>,
    #[prost(string, optional, tag = "7")]
    pub frms_cd: ::std::option::Option<std::string::String>,
    #[prost(enumeration = "cheque11::Priority2Code", optional, tag = "8")]
    pub instr_prty: ::std::option::Option<i32>,
    #[prost(string, repeated, tag = "9")]
    pub memo_fld: ::std::vec::Vec<std::string::String>,
    #[prost(string, optional, tag = "10")]
    pub prt_lctn: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "11")]
    pub rgnl_clr_zone: ::std::option::Option<std::string::String>,
    #[prost(string, repeated, tag = "12")]
    pub sgntr: ::std::vec::Vec<std::string::String>,
}
pub mod cheque11 {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ChequeType2Code {
        Bchq = 1,
        Ccch = 2,
        Cchq = 3,
        Drft = 4,
        Eldr = 5,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Priority2Code {
        High = 1,
        Norm = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentLineInformation1 {
    #[prost(message, optional, tag = "1")]
    pub amt: ::std::option::Option<RemittanceAmount3>,
    #[prost(string, optional, tag = "2")]
    pub desc: ::std::option::Option<std::string::String>,
    #[prost(message, repeated, tag = "3")]
    pub id: ::std::vec::Vec<DocumentLineIdentification1>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Garnishment3 {
    #[prost(int32, optional, tag = "1")]
    pub dt: ::std::option::Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub fmly_mdcl_insrnc_ind: ::std::option::Option<bool>,
    #[prost(message, optional, tag = "3")]
    pub grnshee: ::std::option::Option<PartyIdentification135>,
    #[prost(message, optional, tag = "4")]
    pub grnshmt_admstr: ::std::option::Option<PartyIdentification135>,
    #[prost(bool, optional, tag = "5")]
    pub mplyee_termntn_ind: ::std::option::Option<bool>,
    #[prost(string, optional, tag = "6")]
    pub ref_nb: ::std::option::Option<std::string::String>,
    #[prost(message, optional, tag = "7")]
    pub rmtd_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    #[prost(message, required, tag = "8")]
    pub tp: GarnishmentType1,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupHeader78 {
    #[prost(int64, required, tag = "1")]
    pub cre_dt_tm: i64,
    #[prost(double, optional, tag = "2")]
    pub ctrl_sum: ::std::option::Option<f64>,
    #[prost(message, required, tag = "3")]
    pub initg_pty: PartyIdentification135,
    #[prost(string, required, tag = "4")]
    pub msg_id: std::string::String,
    #[prost(string, required, tag = "5")]
    pub nb_of_txs: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartyAndSignature3 {
    #[prost(message, required, tag = "1")]
    pub pty: PartyIdentification135,
    #[prost(message, required, tag = "2")]
    pub sgntr: SkipPayload,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReferredDocumentInformation7 {
    #[prost(message, repeated, tag = "1")]
    pub line_dtls: ::std::vec::Vec<DocumentLineInformation1>,
    #[prost(string, optional, tag = "2")]
    pub nb: ::std::option::Option<std::string::String>,
    #[prost(int32, optional, tag = "3")]
    pub rltd_dt: ::std::option::Option<i32>,
    #[prost(message, optional, tag = "4")]
    pub tp: ::std::option::Option<ReferredDocumentType4>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemittanceLocation7 {
    #[prost(string, optional, tag = "1")]
    pub rmt_id: ::std::option::Option<std::string::String>,
    #[prost(message, repeated, tag = "2")]
    pub rmt_lctn_dtls: ::std::vec::Vec<RemittanceLocationData1>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StructuredRemittanceInformation16 {
    #[prost(string, repeated, tag = "1")]
    pub addtl_rmt_inf: ::std::vec::Vec<std::string::String>,
    #[prost(message, optional, tag = "2")]
    pub cdtr_ref_inf: ::std::option::Option<CreditorReferenceInformation2>,
    #[prost(message, optional, tag = "3")]
    pub grnshmt_rmt: ::std::option::Option<Garnishment3>,
    #[prost(message, optional, tag = "4")]
    pub invcee: ::std::option::Option<PartyIdentification135>,
    #[prost(message, optional, tag = "5")]
    pub invcr: ::std::option::Option<PartyIdentification135>,
    #[prost(message, optional, tag = "6")]
    pub rfrd_doc_amt: ::std::option::Option<RemittanceAmount2>,
    #[prost(message, repeated, tag = "7")]
    pub rfrd_doc_inf: ::std::vec::Vec<ReferredDocumentInformation7>,
    #[prost(message, optional, tag = "8")]
    pub tax_rmt: ::std::option::Option<TaxInformation7>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Document12 {
    #[prost(message, optional, tag = "1")]
    pub dgtl_sgntr: ::std::option::Option<PartyAndSignature3>,
    #[prost(string, optional, tag = "2")]
    pub file_nm: ::std::option::Option<std::string::String>,
    #[prost(message, required, tag = "3")]
    pub frmt: DocumentFormat1Choice,
    #[prost(string, required, tag = "4")]
    pub id: std::string::String,
    #[prost(message, required, tag = "5")]
    pub isse_dt: DateAndDateTime2Choice,
    #[prost(string, optional, tag = "6")]
    pub lang_cd: ::std::option::Option<std::string::String>,
    #[prost(bytes, required, tag = "7")]
    pub nclsr: std::vec::Vec<u8>,
    #[prost(string, optional, tag = "8")]
    pub nm: ::std::option::Option<std::string::String>,
    #[prost(message, required, tag = "9")]
    pub tp: DocumentType1Choice,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemittanceInformation16 {
    #[prost(message, repeated, tag = "1")]
    pub strd: ::std::vec::Vec<StructuredRemittanceInformation16>,
    #[prost(string, repeated, tag = "2")]
    pub ustrd: ::std::vec::Vec<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreditTransferTransaction42 {
    #[prost(message, required, tag = "1")]
    pub amt: AmountType4Choice,
    #[prost(message, required, tag = "2")]
    pub cdtr: PartyIdentification135,
    #[prost(message, optional, tag = "3")]
    pub cdtr_acct: ::std::option::Option<CashAccount38>,
    #[prost(message, required, tag = "4")]
    pub cdtr_agt: BranchAndFinancialInstitutionIdentification6,
    #[prost(message, optional, tag = "5")]
    pub chq_instr: ::std::option::Option<Cheque11>,
    #[prost(
        enumeration = "credit_transfer_transaction42::ChargeBearerType1Code",
        required,
        tag = "6"
    )]
    pub chrg_br: i32,
    #[prost(message, repeated, tag = "7")]
    pub instr_for_cdtr_agt: ::std::vec::Vec<InstructionForCreditorAgent3>,
    #[prost(message, optional, tag = "8")]
    pub intrmy_agt1: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    #[prost(message, optional, tag = "9")]
    pub intrmy_agt2: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    #[prost(message, optional, tag = "10")]
    pub intrmy_agt3: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    #[prost(message, optional, tag = "11")]
    pub mndt_rltd_inf: ::std::option::Option<CreditTransferMandateData1>,
    #[prost(message, repeated, tag = "12")]
    pub nclsd_file: ::std::vec::Vec<Document12>,
    #[prost(message, optional, tag = "13")]
    pub pmt_cond: ::std::option::Option<PaymentCondition1>,
    #[prost(message, required, tag = "14")]
    pub pmt_id: PaymentIdentification6,
    #[prost(message, optional, tag = "15")]
    pub pmt_tp_inf: ::std::option::Option<PaymentTypeInformation26>,
    #[prost(message, optional, tag = "16")]
    pub purp: ::std::option::Option<Purpose2Choice>,
    #[prost(message, repeated, tag = "17")]
    pub rgltry_rptg: ::std::vec::Vec<RegulatoryReporting3>,
    #[prost(message, repeated, tag = "18")]
    pub rltd_rmt_inf: ::std::vec::Vec<RemittanceLocation7>,
    #[prost(message, optional, tag = "19")]
    pub rmt_inf: ::std::option::Option<RemittanceInformation16>,
    #[prost(message, repeated, tag = "20")]
    pub splmtry_data: ::std::vec::Vec<SupplementaryData1>,
    #[prost(message, optional, tag = "21")]
    pub tax: ::std::option::Option<TaxInformation8>,
    #[prost(message, optional, tag = "22")]
    pub ultmt_cdtr: ::std::option::Option<PartyIdentification135>,
    #[prost(message, optional, tag = "23")]
    pub ultmt_dbtr: ::std::option::Option<PartyIdentification135>,
}
pub mod credit_transfer_transaction42 {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ChargeBearerType1Code {
        Cred = 1,
        Debt = 2,
        Shar = 3,
        Slev = 4,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentInstruction35 {
    #[prost(message, repeated, tag = "1")]
    pub cdt_trf_tx: ::std::vec::Vec<CreditTransferTransaction42>,
    #[prost(
        enumeration = "payment_instruction35::ChargeBearerType1Code",
        optional,
        tag = "2"
    )]
    pub chrg_br: ::std::option::Option<i32>,
    #[prost(message, required, tag = "3")]
    pub dbtr: PartyIdentification135,
    #[prost(message, optional, tag = "4")]
    pub dbtr_acct: ::std::option::Option<CashAccount38>,
    #[prost(message, required, tag = "5")]
    pub dbtr_agt: BranchAndFinancialInstitutionIdentification6,
    #[prost(message, optional, tag = "6")]
    pub pmt_cond: ::std::option::Option<PaymentCondition1>,
    #[prost(string, optional, tag = "7")]
    pub pmt_inf_id: ::std::option::Option<std::string::String>,
    #[prost(
        enumeration = "payment_instruction35::PaymentMethod7Code",
        required,
        tag = "8"
    )]
    pub pmt_mtd: i32,
    #[prost(message, optional, tag = "9")]
    pub pmt_tp_inf: ::std::option::Option<PaymentTypeInformation26>,
    #[prost(message, optional, tag = "10")]
    pub reqd_advc_tp: ::std::option::Option<AdviceType1>,
    #[prost(message, required, tag = "11")]
    pub reqd_exctn_dt: DateAndDateTime2Choice,
    #[prost(message, optional, tag = "12")]
    pub ultmt_dbtr: ::std::option::Option<PartyIdentification135>,
    #[prost(message, optional, tag = "13")]
    pub xpry_dt: ::std::option::Option<DateAndDateTime2Choice>,
}
pub mod payment_instruction35 {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ChargeBearerType1Code {
        Cred = 1,
        Debt = 2,
        Shar = 3,
        Slev = 4,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PaymentMethod7Code {
        Chk = 1,
        Trf = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreditorPaymentActivationRequestV08 {
    #[prost(message, required, tag = "1")]
    pub grp_hdr: GroupHeader78,
    #[prost(message, repeated, tag = "2")]
    pub pmt_inf: ::std::vec::Vec<PaymentInstruction35>,
    #[prost(message, repeated, tag = "3")]
    pub splmtry_data: ::std::vec::Vec<SupplementaryData1>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Document {
    #[prost(message, required, tag = "1")]
    pub cdtr_pmt_actvtn_req: CreditorPaymentActivationRequestV08,
}
