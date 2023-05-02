// Copyright 2023 Emergent Financial, LLC - All Rights Reserved
//
// This software is dual-licensed under the MIT License OR the Apache License, Version 2.0.
//
// MIT License
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the “Software”),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
// WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// See ISO-20022 Intellectual Property Rights Policy at
// <https://www.iso20022.org/intellectual-property-rights>
// for more information.

use validator::Validate;

::lazy_static::lazy_static! {
    static ref NACE_DOMAIN_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-U]{1,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:auth.094.001.02".to_string()
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeRecurrentQuery5 {
    #[validate]
    #[serde(rename = "QryTp")]
    pub qry_tp: Max1000Text,
    #[validate]
    #[serde(rename = "Frqcy")]
    pub frqcy: TradeQueryExecutionFrequency3,
    #[validate]
    #[serde(rename = "VldUntil")]
    pub vld_until: IsoDate,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeReportQuery13ChoiceEnum {
    #[serde(rename = "AdHocQry", skip_serializing_if = "Option::is_none")]
    pub ad_hoc_qry: Option<TradeQueryCriteria10>,
    #[serde(rename = "RcrntQry", skip_serializing_if = "Option::is_none")]
    pub rcrnt_qry: Option<TradeRecurrentQuery5>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeReportQuery13Choice {
    #[serde(flatten)]
    pub value: TradeReportQuery13ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeTypeQueryCriteria2 {
    #[serde(rename = "Oprtr")]
    pub oprtr: Operation3Code,
    #[validate(length(min = 0,))]
    #[serde(rename = "SctiesFincgTxTp", default)]
    pub scties_fincg_tx_tp: Vec<ExposureType10Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CollCmpntTp", default)]
    pub coll_cmpnt_tp: Vec<CollateralType6Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
#[serde(rename = "Document")]
pub struct Document<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "SctiesFincgRptgTxQry")]
    pub scties_fincg_rptg_tx_qry: SecuritiesFinancingReportingTransactionQueryV02<A>,
    #[serde(rename = "@xmlns", default = "namespace")]
    pub xmlns: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NaceDomainIdentifier {
    #[validate(regex = "NACE_DOMAIN_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Operation3Code {
    #[serde(rename = "ANDD")]
    Andd,
    #[serde(rename = "ORRR")]
    Orrr,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max350Text {
    #[validate(length(min = 1, max = 350,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max50Text {
    #[validate(length(min = 1, max = 50,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateSectorCriteria5 {
    #[validate(length(min = 0,))]
    #[serde(rename = "FISctr", default)]
    pub fi_sctr: Vec<FinancialPartySectorType2Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NFISctr", default)]
    pub nfi_sctr: Vec<NaceDomainIdentifier>,
    #[serde(rename = "NotRptd", skip_serializing_if = "Option::is_none")]
    pub not_rptd: Option<NotReported1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateTimePeriod1 {
    #[validate]
    #[serde(rename = "FrDtTm")]
    pub fr_dt_tm: IsoDateTime,
    #[validate]
    #[serde(rename = "ToDtTm")]
    pub to_dt_tm: IsoDateTime,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max70Text {
    #[validate(length(min = 1, max = 70,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradePartyQueryCriteria5 {
    #[serde(rename = "Oprtr")]
    pub oprtr: Operation3Code,
    #[serde(rename = "RptgCtrPty", skip_serializing_if = "Option::is_none")]
    pub rptg_ctr_pty: Option<TradePartyIdentificationQuery8>,
    #[serde(rename = "RptgCtrPtyBrnch", skip_serializing_if = "Option::is_none")]
    pub rptg_ctr_pty_brnch: Option<TradePartyIdentificationQuery9>,
    #[serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none")]
    pub othr_ctr_pty: Option<TradePartyIdentificationQuery8>,
    #[serde(rename = "OthrCtrPtyBrnch", skip_serializing_if = "Option::is_none")]
    pub othr_ctr_pty_brnch: Option<TradePartyIdentificationQuery9>,
    #[serde(rename = "Bnfcry", skip_serializing_if = "Option::is_none")]
    pub bnfcry: Option<TradePartyIdentificationQuery8>,
    #[serde(rename = "SubmitgAgt", skip_serializing_if = "Option::is_none")]
    pub submitg_agt: Option<TradePartyIdentificationQuery8>,
    #[serde(rename = "Brkr", skip_serializing_if = "Option::is_none")]
    pub brkr: Option<TradePartyIdentificationQuery8>,
    #[serde(rename = "CCP", skip_serializing_if = "Option::is_none")]
    pub ccp: Option<TradePartyIdentificationQuery8>,
    #[serde(rename = "AgtLndr", skip_serializing_if = "Option::is_none")]
    pub agt_lndr: Option<TradePartyIdentificationQuery8>,
    #[serde(rename = "TrptyAgt", skip_serializing_if = "Option::is_none")]
    pub trpty_agt: Option<TradePartyIdentificationQuery8>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MicIdentifier {
    #[validate(regex = "MIC_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AnyBicDec2014Identifier {
    #[validate(regex = "ANY_BIC_DEC_2014_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DatePeriod1 {
    #[serde(rename = "FrDt", skip_serializing_if = "Option::is_none")]
    pub fr_dt: Option<IsoDate>,
    #[validate]
    #[serde(rename = "ToDt")]
    pub to_dt: IsoDate,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesFinancingReportingTransactionQueryV02<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "RqstngAuthrty")]
    pub rqstng_authrty: PartyIdentification121Choice,
    #[serde(rename = "TradQryData")]
    pub trad_qry_data: TradeReportQuery13Choice,
    #[validate(length(min = 0,))]
    #[serde(rename = "SplmtryData", default)]
    pub splmtry_data: Vec<SupplementaryData1<A>>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TrueFalseIndicator {
    #[serde(rename = "$text")]
    pub value: bool,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum WeekDay3Code {
    #[serde(rename = "ALLD")]
    Alld,
    #[serde(rename = "XBHL")]
    Xbhl,
    #[serde(rename = "IBHL")]
    Ibhl,
    #[serde(rename = "FRID")]
    Frid,
    #[serde(rename = "MOND")]
    Mond,
    #[serde(rename = "SATD")]
    Satd,
    #[serde(rename = "SUND")]
    Sund,
    #[serde(rename = "THUD")]
    Thud,
    #[serde(rename = "TUED")]
    Tued,
    #[serde(rename = "WEDD")]
    Wedd,
    #[serde(rename = "WDAY")]
    Wday,
    #[serde(rename = "WEND")]
    Wend,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradePartyIdentificationQuery9 {
    #[validate(length(min = 0,))]
    #[serde(rename = "LEI", default)]
    pub lei: Vec<LeiIdentifier>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CtryCd", default)]
    pub ctry_cd: Vec<CountryCode>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AnyBIC", default)]
    pub any_bic: Vec<AnyBicDec2014Identifier>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ClntId", default)]
    pub clnt_id: Vec<Max50Text>,
    #[serde(rename = "NotRptd", skip_serializing_if = "Option::is_none")]
    pub not_rptd: Option<NotReported1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max1000Text {
    #[validate(length(min = 1, max = 1000,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CollateralType6Code {
    #[serde(rename = "GBBK")]
    Gbbk,
    #[serde(rename = "BOND")]
    Bond,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "COMM")]
    Comm,
    #[serde(rename = "INSU")]
    Insu,
    #[serde(rename = "LCRE")]
    Lcre,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "PHYS")]
    Phys,
    #[serde(rename = "SECU")]
    Secu,
    #[serde(rename = "STCF")]
    Stcf,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<Max35Text>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeQueryExecutionFrequency3 {
    #[serde(rename = "FrqcyTp")]
    pub frqcy_tp: Frequency14Code,
    #[validate(length(min = 0,))]
    #[serde(rename = "DlvryDay", default)]
    pub dlvry_day: Vec<WeekDay3Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "DayOfMnth", default)]
    pub day_of_mnth: Vec<DayOfMonthNumber>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Frequency14Code {
    #[serde(rename = "DAIL")]
    Dail,
    #[serde(rename = "WEEK")]
    Week,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "ADHO")]
    Adho,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NameAndAddress5 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max350Text,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<PostalAddress1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeAdditionalQueryCriteria7 {
    #[validate(length(min = 0,))]
    #[serde(rename = "ActnTp", default)]
    pub actn_tp: Vec<TransactionOperationType6Code>,
    #[serde(rename = "ExctnVn", skip_serializing_if = "Option::is_none")]
    pub exctn_vn: Option<SecuritiesTradeVenueCriteria1Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NtrOfCtrPty", default)]
    pub ntr_of_ctr_pty: Vec<PartyNatureType1Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CorpSctr", default)]
    pub corp_sctr: Vec<CorporateSectorCriteria5>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeQueryCriteria10 {
    #[validate]
    #[serde(rename = "TradLifeCyclHstry")]
    pub trad_life_cycl_hstry: TrueFalseIndicator,
    #[validate]
    #[serde(rename = "OutsdngTradInd")]
    pub outsdng_trad_ind: TrueFalseIndicator,
    #[serde(rename = "TradPtyCrit", skip_serializing_if = "Option::is_none")]
    pub trad_pty_crit: Option<TradePartyQueryCriteria5>,
    #[serde(rename = "TradTpCrit", skip_serializing_if = "Option::is_none")]
    pub trad_tp_crit: Option<TradeTypeQueryCriteria2>,
    #[serde(rename = "TmCrit", skip_serializing_if = "Option::is_none")]
    pub tm_crit: Option<TradeDateTimeQueryCriteria2>,
    #[serde(rename = "OthrCrit", skip_serializing_if = "Option::is_none")]
    pub othr_crit: Option<TradeAdditionalQueryCriteria7>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AddressType2Code {
    #[serde(rename = "ADDR")]
    Addr,
    #[serde(rename = "PBOX")]
    Pbox,
    #[serde(rename = "HOME")]
    Home,
    #[serde(rename = "BIZZ")]
    Bizz,
    #[serde(rename = "MLTO")]
    Mlto,
    #[serde(rename = "DLVY")]
    Dlvy,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LeiIdentifier {
    #[validate(regex = "LEI_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification121ChoiceEnum {
    #[serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none")]
    pub lgl_ntty_idr: Option<LeiIdentifier>,
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress5>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification1>,
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification121Choice {
    #[serde(flatten)]
    pub value: PartyIdentification121ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeDateTimeQueryCriteria2 {
    #[serde(rename = "RptgDtTm", skip_serializing_if = "Option::is_none")]
    pub rptg_dt_tm: Option<DateTimePeriod1>,
    #[serde(rename = "ExctnDtTm", skip_serializing_if = "Option::is_none")]
    pub exctn_dt_tm: Option<DateTimePeriod1>,
    #[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
    pub mtrty_dt: Option<DateOrBlankQuery2Choice>,
    #[serde(rename = "TermntnDt", skip_serializing_if = "Option::is_none")]
    pub termntn_dt: Option<DateOrBlankQuery2Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesTradeVenueCriteria1ChoiceEnum {
    #[serde(rename = "MIC", skip_serializing_if = "Option::is_none")]
    pub mic: Option<MicIdentifier>,
    #[serde(rename = "AnyMIC", skip_serializing_if = "Option::is_none")]
    pub any_mic: Option<AnyMic1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesTradeVenueCriteria1Choice {
    #[serde(flatten)]
    pub value: SecuritiesTradeVenueCriteria1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TransactionOperationType6Code {
    #[serde(rename = "REUU")]
    Reuu,
    #[serde(rename = "COLU")]
    Colu,
    #[serde(rename = "CORR")]
    Corr,
    #[serde(rename = "ETRM")]
    Etrm,
    #[serde(rename = "VALU")]
    Valu,
    #[serde(rename = "POSC")]
    Posc,
    #[serde(rename = "NEWT")]
    Newt,
    #[serde(rename = "MODI")]
    Modi,
    #[serde(rename = "MARU")]
    Maru,
    #[serde(rename = "EROR")]
    Eror,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IsoDateTime {
    #[serde(rename = "$text")]
    pub value: ::chrono::DateTime<::chrono::Utc>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AnyMic1Code {
    #[serde(rename = "ANYM")]
    Anym,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PartyNatureType1Code {
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "NFIN")]
    Nfin,
    #[serde(rename = "FIIN")]
    Fiin,
    #[serde(rename = "CCPS")]
    Ccps,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PostalAddress1 {
    #[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
    pub adr_tp: Option<AddressType2Code>,
    #[validate(length(min = 0, max = 5,))]
    #[serde(rename = "AdrLine", default)]
    pub adr_line: Vec<Max70Text>,
    #[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
    pub strt_nm: Option<Max70Text>,
    #[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
    pub bldg_nb: Option<Max16Text>,
    #[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
    pub pst_cd: Option<Max16Text>,
    #[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
    pub twn_nm: Option<Max35Text>,
    #[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn: Option<Max35Text>,
    #[serde(rename = "Ctry")]
    pub ctry: CountryCode,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DayOfMonthNumber {
    #[validate(range(min = 1, max = 31,))]
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum NotReported1Code {
    #[serde(rename = "NORP")]
    Norp,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max16Text {
    #[validate(length(min = 1, max = 16,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max35Text {
    #[validate(length(min = 1, max = 35,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SupplementaryDataEnvelope1<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(flatten)]
    pub value: A,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradePartyIdentificationQuery8 {
    #[validate(length(min = 0,))]
    #[serde(rename = "LEI", default)]
    pub lei: Vec<LeiIdentifier>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AnyBIC", default)]
    pub any_bic: Vec<AnyBicDec2014Identifier>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ClntId", default)]
    pub clnt_id: Vec<Max50Text>,
    #[serde(rename = "NotRptd", skip_serializing_if = "Option::is_none")]
    pub not_rptd: Option<NotReported1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateOrBlankQuery2ChoiceEnum {
    #[serde(rename = "NotRptd", skip_serializing_if = "Option::is_none")]
    pub not_rptd: Option<NotReported1Code>,
    #[serde(rename = "Rg", skip_serializing_if = "Option::is_none")]
    pub rg: Option<DatePeriod1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateOrBlankQuery2Choice {
    #[serde(flatten)]
    pub value: DateOrBlankQuery2ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExposureType10Code {
    #[serde(rename = "SBSC")]
    Sbsc,
    #[serde(rename = "MGLD")]
    Mgld,
    #[serde(rename = "SLEB")]
    Sleb,
    #[serde(rename = "REPO")]
    Repo,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SupplementaryData1<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none")]
    pub plc_and_nm: Option<Max350Text>,
    #[validate]
    #[serde(rename = "Envlp")]
    pub envlp: SupplementaryDataEnvelope1<A>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IsoDate {
    #[serde(rename = "$text")]
    pub value: ::chrono::NaiveDate,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FinancialPartySectorType2Code {
    #[serde(rename = "AIFD")]
    Aifd,
    #[serde(rename = "CSDS")]
    Csds,
    #[serde(rename = "CCPS")]
    Ccps,
    #[serde(rename = "CDTI")]
    Cdti,
    #[serde(rename = "INUN")]
    Inun,
    #[serde(rename = "ORPI")]
    Orpi,
    #[serde(rename = "INVF")]
    Invf,
    #[serde(rename = "REIN")]
    Rein,
    #[serde(rename = "UCIT")]
    Ucit,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CountryCode {
    #[validate(regex = "COUNTRY_CODE_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
