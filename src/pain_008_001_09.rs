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
pub struct Authorisation1Choice {
    #[prost(
        enumeration = "authorisation1_choice::Authorisation1Code",
        optional,
        tag = "1"
    )]
    pub cd: ::std::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub prtry: ::std::option::Option<std::string::String>,
}
pub mod authorisation1_choice {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Authorisation1Code {
        Auth = 1,
        Fdet = 2,
        Fsum = 3,
        Ilev = 4,
    }
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
pub struct GenericIdentification30 {
    #[prost(string, required, tag = "1")]
    pub id: std::string::String,
    #[prost(string, required, tag = "2")]
    pub issr: std::string::String,
    #[prost(string, optional, tag = "3")]
    pub schme_nm: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalInstrument2Choice {
    #[prost(string, optional, tag = "1")]
    pub cd: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "2")]
    pub prtry: ::std::option::Option<std::string::String>,
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
pub struct DocumentLineType1 {
    #[prost(message, required, tag = "1")]
    pub cd_or_prtry: DocumentLineType1Choice,
    #[prost(string, optional, tag = "2")]
    pub issr: ::std::option::Option<std::string::String>,
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
pub struct OrganisationIdentification29 {
    #[prost(string, optional, tag = "1")]
    pub any_bic: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "2")]
    pub lei: ::std::option::Option<std::string::String>,
    #[prost(message, repeated, tag = "3")]
    pub othr: ::std::vec::Vec<GenericOrganisationIdentification1>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentTypeInformation29 {
    #[prost(message, optional, tag = "1")]
    pub ctgy_purp: ::std::option::Option<CategoryPurpose1Choice>,
    #[prost(
        enumeration = "payment_type_information29::Priority2Code",
        optional,
        tag = "2"
    )]
    pub instr_prty: ::std::option::Option<i32>,
    #[prost(message, optional, tag = "3")]
    pub lcl_instrm: ::std::option::Option<LocalInstrument2Choice>,
    #[prost(
        enumeration = "payment_type_information29::SequenceType3Code",
        optional,
        tag = "4"
    )]
    pub seq_tp: ::std::option::Option<i32>,
    #[prost(message, repeated, tag = "5")]
    pub svc_lvl: ::std::vec::Vec<ServiceLevel8Choice>,
}
pub mod payment_type_information29 {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Priority2Code {
        High = 1,
        Norm = 2,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SequenceType3Code {
        Fnal = 1,
        Frst = 2,
        Ooff = 3,
        Rcur = 4,
        Rpre = 5,
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
pub struct GroupHeader83 {
    #[prost(message, repeated, tag = "1")]
    pub authstn: ::std::vec::Vec<Authorisation1Choice>,
    #[prost(int64, required, tag = "2")]
    pub cre_dt_tm: i64,
    #[prost(double, optional, tag = "3")]
    pub ctrl_sum: ::std::option::Option<f64>,
    #[prost(message, optional, tag = "4")]
    pub fwdg_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    #[prost(message, required, tag = "5")]
    pub initg_pty: PartyIdentification135,
    #[prost(string, required, tag = "6")]
    pub msg_id: std::string::String,
    #[prost(string, required, tag = "7")]
    pub nb_of_txs: std::string::String,
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
pub struct AmendmentInformationDetails13 {
    #[prost(message, optional, tag = "1")]
    pub orgnl_cdtr_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    #[prost(message, optional, tag = "2")]
    pub orgnl_cdtr_agt_acct: ::std::option::Option<CashAccount38>,
    #[prost(message, optional, tag = "3")]
    pub orgnl_cdtr_schme_id: ::std::option::Option<PartyIdentification135>,
    #[prost(message, optional, tag = "4")]
    pub orgnl_dbtr: ::std::option::Option<PartyIdentification135>,
    #[prost(message, optional, tag = "5")]
    pub orgnl_dbtr_acct: ::std::option::Option<CashAccount38>,
    #[prost(message, optional, tag = "6")]
    pub orgnl_dbtr_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    #[prost(message, optional, tag = "7")]
    pub orgnl_dbtr_agt_acct: ::std::option::Option<CashAccount38>,
    #[prost(int32, optional, tag = "8")]
    pub orgnl_fnl_colltn_dt: ::std::option::Option<i32>,
    #[prost(message, optional, tag = "9")]
    pub orgnl_frqcy: ::std::option::Option<Frequency36Choice>,
    #[prost(string, optional, tag = "10")]
    pub orgnl_mndt_id: ::std::option::Option<std::string::String>,
    #[prost(message, optional, tag = "11")]
    pub orgnl_rsn: ::std::option::Option<MandateSetupReason1Choice>,
    #[prost(string, optional, tag = "12")]
    pub orgnl_trckg_days: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MandateRelatedInformation14 {
    #[prost(bool, optional, tag = "1")]
    pub amdmnt_ind: ::std::option::Option<bool>,
    #[prost(message, optional, tag = "2")]
    pub amdmnt_inf_dtls: ::std::option::Option<AmendmentInformationDetails13>,
    #[prost(int32, optional, tag = "3")]
    pub dt_of_sgntr: ::std::option::Option<i32>,
    #[prost(string, optional, tag = "4")]
    pub elctrnc_sgntr: ::std::option::Option<std::string::String>,
    #[prost(int32, optional, tag = "5")]
    pub fnl_colltn_dt: ::std::option::Option<i32>,
    #[prost(message, optional, tag = "6")]
    pub frqcy: ::std::option::Option<Frequency36Choice>,
    #[prost(int32, optional, tag = "7")]
    pub frst_colltn_dt: ::std::option::Option<i32>,
    #[prost(string, optional, tag = "8")]
    pub mndt_id: ::std::option::Option<std::string::String>,
    #[prost(message, optional, tag = "9")]
    pub rsn: ::std::option::Option<MandateSetupReason1Choice>,
    #[prost(string, optional, tag = "10")]
    pub trckg_days: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemittanceInformation16 {
    #[prost(message, repeated, tag = "1")]
    pub strd: ::std::vec::Vec<StructuredRemittanceInformation16>,
    #[prost(string, repeated, tag = "2")]
    pub ustrd: ::std::vec::Vec<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectDebitTransaction10 {
    #[prost(message, optional, tag = "1")]
    pub cdtr_schme_id: ::std::option::Option<PartyIdentification135>,
    #[prost(message, optional, tag = "2")]
    pub mndt_rltd_inf: ::std::option::Option<MandateRelatedInformation14>,
    #[prost(int32, optional, tag = "3")]
    pub pre_ntfctn_dt: ::std::option::Option<i32>,
    #[prost(string, optional, tag = "4")]
    pub pre_ntfctn_id: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectDebitTransactionInformation23 {
    #[prost(
        enumeration = "direct_debit_transaction_information23::ChargeBearerType1Code",
        optional,
        tag = "1"
    )]
    pub chrg_br: ::std::option::Option<i32>,
    #[prost(message, required, tag = "2")]
    pub dbtr: PartyIdentification135,
    #[prost(message, required, tag = "3")]
    pub dbtr_acct: CashAccount38,
    #[prost(message, required, tag = "4")]
    pub dbtr_agt: BranchAndFinancialInstitutionIdentification6,
    #[prost(message, optional, tag = "5")]
    pub dbtr_agt_acct: ::std::option::Option<CashAccount38>,
    #[prost(message, optional, tag = "6")]
    pub drct_dbt_tx: ::std::option::Option<DirectDebitTransaction10>,
    #[prost(message, required, tag = "7")]
    pub instd_amt: ActiveOrHistoricCurrencyAndAmount,
    #[prost(string, optional, tag = "8")]
    pub instr_for_cdtr_agt: ::std::option::Option<std::string::String>,
    #[prost(message, required, tag = "9")]
    pub pmt_id: PaymentIdentification6,
    #[prost(message, optional, tag = "10")]
    pub pmt_tp_inf: ::std::option::Option<PaymentTypeInformation29>,
    #[prost(message, optional, tag = "11")]
    pub purp: ::std::option::Option<Purpose2Choice>,
    #[prost(message, repeated, tag = "12")]
    pub rgltry_rptg: ::std::vec::Vec<RegulatoryReporting3>,
    #[prost(message, repeated, tag = "13")]
    pub rltd_rmt_inf: ::std::vec::Vec<RemittanceLocation7>,
    #[prost(message, optional, tag = "14")]
    pub rmt_inf: ::std::option::Option<RemittanceInformation16>,
    #[prost(message, repeated, tag = "15")]
    pub splmtry_data: ::std::vec::Vec<SupplementaryData1>,
    #[prost(message, optional, tag = "16")]
    pub tax: ::std::option::Option<TaxInformation8>,
    #[prost(message, optional, tag = "17")]
    pub ultmt_cdtr: ::std::option::Option<PartyIdentification135>,
    #[prost(message, optional, tag = "18")]
    pub ultmt_dbtr: ::std::option::Option<PartyIdentification135>,
}
pub mod direct_debit_transaction_information23 {
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
pub struct PaymentInstruction37 {
    #[prost(bool, optional, tag = "1")]
    pub btch_bookg: ::std::option::Option<bool>,
    #[prost(message, required, tag = "2")]
    pub cdtr: PartyIdentification135,
    #[prost(message, required, tag = "3")]
    pub cdtr_acct: CashAccount38,
    #[prost(message, required, tag = "4")]
    pub cdtr_agt: BranchAndFinancialInstitutionIdentification6,
    #[prost(message, optional, tag = "5")]
    pub cdtr_agt_acct: ::std::option::Option<CashAccount38>,
    #[prost(message, optional, tag = "6")]
    pub cdtr_schme_id: ::std::option::Option<PartyIdentification135>,
    #[prost(
        enumeration = "payment_instruction37::ChargeBearerType1Code",
        optional,
        tag = "7"
    )]
    pub chrg_br: ::std::option::Option<i32>,
    #[prost(message, optional, tag = "8")]
    pub chrgs_acct: ::std::option::Option<CashAccount38>,
    #[prost(message, optional, tag = "9")]
    pub chrgs_acct_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    #[prost(double, optional, tag = "10")]
    pub ctrl_sum: ::std::option::Option<f64>,
    #[prost(message, repeated, tag = "11")]
    pub drct_dbt_tx_inf: ::std::vec::Vec<DirectDebitTransactionInformation23>,
    #[prost(string, optional, tag = "12")]
    pub nb_of_txs: ::std::option::Option<std::string::String>,
    #[prost(string, required, tag = "13")]
    pub pmt_inf_id: std::string::String,
    #[prost(
        enumeration = "payment_instruction37::PaymentMethod2Code",
        required,
        tag = "14"
    )]
    pub pmt_mtd: i32,
    #[prost(message, optional, tag = "15")]
    pub pmt_tp_inf: ::std::option::Option<PaymentTypeInformation29>,
    #[prost(message, optional, tag = "16")]
    pub reqd_advc_tp: ::std::option::Option<AdviceType1>,
    #[prost(int32, required, tag = "17")]
    pub reqd_colltn_dt: i32,
    #[prost(message, optional, tag = "18")]
    pub ultmt_cdtr: ::std::option::Option<PartyIdentification135>,
}
pub mod payment_instruction37 {
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
    pub enum PaymentMethod2Code {
        Dd = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerDirectDebitInitiationV09 {
    #[prost(message, required, tag = "1")]
    pub grp_hdr: GroupHeader83,
    #[prost(message, repeated, tag = "2")]
    pub pmt_inf: ::std::vec::Vec<PaymentInstruction37>,
    #[prost(message, repeated, tag = "3")]
    pub splmtry_data: ::std::vec::Vec<SupplementaryData1>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Document {
    #[prost(message, required, tag = "1")]
    pub cstmr_drct_dbt_initn: CustomerDirectDebitInitiationV09,
}
