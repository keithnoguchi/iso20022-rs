#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnspecifiedType {
    #[prost(string, required, tag = "1")]
    pub base_object_type: std::string::String,
    #[prost(bytes, required, tag = "2")]
    pub object: std::vec::Vec<u8>,
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
pub struct FinancialIdentificationSchemeName1Choice {
    #[prost(string, optional, tag = "1")]
    pub cd: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "2")]
    pub prtry: ::std::option::Option<std::string::String>,
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
pub struct ImplementationSpecification1 {
    #[prost(string, required, tag = "1")]
    pub id: std::string::String,
    #[prost(string, required, tag = "2")]
    pub regy: std::string::String,
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
pub struct PersonIdentificationSchemeName1Choice {
    #[prost(string, optional, tag = "1")]
    pub cd: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "2")]
    pub prtry: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignatureEnvelope {}
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
pub struct BranchAndFinancialInstitutionIdentification6 {
    #[prost(message, optional, tag = "1")]
    pub brnch_id: ::std::option::Option<BranchData3>,
    #[prost(message, required, tag = "2")]
    pub fin_instn_id: FinancialInstitutionIdentification18,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Party44Choice {
    #[prost(message, optional, tag = "1")]
    pub fi_id: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    #[prost(message, optional, tag = "2")]
    pub org_id: ::std::option::Option<PartyIdentification135>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BusinessApplicationHeader5 {
    #[prost(string, required, tag = "1")]
    pub biz_msg_idr: std::string::String,
    #[prost(string, optional, tag = "2")]
    pub biz_svc: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "3")]
    pub char_set: ::std::option::Option<std::string::String>,
    #[prost(
        enumeration = "business_application_header5::CopyDuplicate1Code",
        optional,
        tag = "4"
    )]
    pub cpy_dplct: ::std::option::Option<i32>,
    #[prost(int64, required, tag = "5")]
    pub cre_dt: i64,
    #[prost(message, required, tag = "6")]
    pub fr: Party44Choice,
    #[prost(string, required, tag = "7")]
    pub msg_def_idr: std::string::String,
    #[prost(string, optional, tag = "8")]
    pub prty: ::std::option::Option<std::string::String>,
    #[prost(bool, optional, tag = "9")]
    pub pssbl_dplct: ::std::option::Option<bool>,
    #[prost(message, optional, tag = "10")]
    pub sgntr: ::std::option::Option<SignatureEnvelope>,
    #[prost(message, required, tag = "11")]
    pub to: Party44Choice,
}
pub mod business_application_header5 {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CopyDuplicate1Code {
        Codu = 1,
        Copy = 2,
        Dupl = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BusinessApplicationHeaderV02 {
    #[prost(string, required, tag = "1")]
    pub biz_msg_idr: std::string::String,
    #[prost(int64, optional, tag = "2")]
    pub biz_prcg_dt: ::std::option::Option<i64>,
    #[prost(string, optional, tag = "3")]
    pub biz_svc: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "4")]
    pub char_set: ::std::option::Option<std::string::String>,
    #[prost(
        enumeration = "business_application_header_v02::CopyDuplicate1Code",
        optional,
        tag = "5"
    )]
    pub cpy_dplct: ::std::option::Option<i32>,
    #[prost(int64, required, tag = "6")]
    pub cre_dt: i64,
    #[prost(message, required, tag = "7")]
    pub fr: Party44Choice,
    #[prost(message, optional, tag = "8")]
    pub mkt_prctc: ::std::option::Option<ImplementationSpecification1>,
    #[prost(string, required, tag = "9")]
    pub msg_def_idr: std::string::String,
    #[prost(string, optional, tag = "10")]
    pub prty: ::std::option::Option<std::string::String>,
    #[prost(bool, optional, tag = "11")]
    pub pssbl_dplct: ::std::option::Option<bool>,
    #[prost(message, repeated, tag = "12")]
    pub rltd: ::std::vec::Vec<BusinessApplicationHeader5>,
    #[prost(message, optional, tag = "13")]
    pub sgntr: ::std::option::Option<SignatureEnvelope>,
    #[prost(message, required, tag = "14")]
    pub to: Party44Choice,
}
pub mod business_application_header_v02 {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CopyDuplicate1Code {
        Codu = 1,
        Copy = 2,
        Dupl = 3,
    }
}
