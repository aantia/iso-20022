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
    static ref MAX_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref BIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:tsmt.010.001.03".to_string()
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
pub struct MessageIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[validate]
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: IsoDateTime,
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
pub struct Max3NumericText {
    #[validate(regex = "MAX_3_NUMERIC_TEXT_REGEX")]
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
pub struct PendingActivity2 {
    #[serde(rename = "Tp")]
    pub tp: Action2Code,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max140Text>,
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
pub struct TransactionStatus4 {
    #[serde(rename = "Sts")]
    pub sts: BaselineStatus3Code,
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
pub struct BicIdentifier {
    #[validate(regex = "BIC_IDENTIFIER_REGEX")]
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
pub struct PostalAddress5 {
    #[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
    pub strt_nm: Option<Max70Text>,
    #[serde(rename = "PstCdId", skip_serializing_if = "Option::is_none")]
    pub pst_cd_id: Option<Max16Text>,
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
pub struct Max140Text {
    #[validate(length(min = 1, max = 140,))]
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
pub struct Number {
    #[serde(rename = "$text")]
    pub value: f64,
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
pub struct SimpleIdentificationInformation {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
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
pub struct ValidationResult5 {
    #[validate]
    #[serde(rename = "SeqNb")]
    pub seq_nb: Number,
    #[validate]
    #[serde(rename = "RuleId")]
    pub rule_id: Max35Text,
    #[validate]
    #[serde(rename = "RuleDesc")]
    pub rule_desc: Max350Text,
    #[validate(length(min = 0,))]
    #[serde(rename = "MisMtchdElmt", default)]
    pub mis_mtchd_elmt: Vec<ElementIdentification1>,
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
pub struct DocumentIdentification4 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[validate]
    #[serde(rename = "Vrsn")]
    pub vrsn: Number,
    #[validate]
    #[serde(rename = "Submitr")]
    pub submitr: BicIdentification1,
    #[validate]
    #[serde(rename = "DocIndx")]
    pub doc_indx: Max3NumericText,
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
pub struct Limit1 {
    #[validate]
    #[serde(rename = "Cur")]
    pub cur: Max3NumericText,
    #[validate]
    #[serde(rename = "Lmt")]
    pub lmt: Max3NumericText,
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
pub struct ElementIdentification1 {
    #[validate]
    #[serde(rename = "DocIndx")]
    pub doc_indx: Max3NumericText,
    #[validate]
    #[serde(rename = "ElmtPth")]
    pub elmt_pth: Max350Text,
    #[validate]
    #[serde(rename = "ElmtNm")]
    pub elmt_nm: Max35Text,
    #[serde(rename = "ElmtVal", skip_serializing_if = "Option::is_none")]
    pub elmt_val: Option<Max140Text>,
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
pub struct PartyIdentification26 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max70Text,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification4>,
    #[validate]
    #[serde(rename = "PstlAdr")]
    pub pstl_adr: PostalAddress5,
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
pub struct DocumentIdentification5 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[validate]
    #[serde(rename = "IdIssr")]
    pub id_issr: BicIdentification1,
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
pub struct DocumentIdentification3 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[validate]
    #[serde(rename = "Vrsn")]
    pub vrsn: Number,
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
pub struct BicIdentification1 {
    #[validate]
    #[serde(rename = "BIC")]
    pub bic: BicIdentifier,
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
pub struct BaselineMatchReportV03 {
    #[validate]
    #[serde(rename = "RptId")]
    pub rpt_id: MessageIdentification1,
    #[validate]
    #[serde(rename = "TxId")]
    pub tx_id: SimpleIdentificationInformation,
    #[serde(rename = "EstblishdBaselnId", skip_serializing_if = "Option::is_none")]
    pub estblishd_baseln_id: Option<DocumentIdentification3>,
    #[validate]
    #[serde(rename = "TxSts")]
    pub tx_sts: TransactionStatus4,
    #[validate(length(min = 0, max = 2,))]
    #[serde(rename = "UsrTxRef", default)]
    pub usr_tx_ref: Vec<DocumentIdentification5>,
    #[validate]
    #[serde(rename = "Buyr")]
    pub buyr: PartyIdentification26,
    #[validate]
    #[serde(rename = "Sellr")]
    pub sellr: PartyIdentification26,
    #[validate]
    #[serde(rename = "BuyrBk")]
    pub buyr_bk: BicIdentification1,
    #[validate]
    #[serde(rename = "SellrBk")]
    pub sellr_bk: BicIdentification1,
    #[validate]
    #[serde(rename = "BaselnEstblishmtTrils")]
    pub baseln_estblishmt_trils: Limit1,
    #[validate(length(min = 2, max = 2,))]
    #[serde(rename = "CmpardDocRef", default)]
    pub cmpard_doc_ref: Vec<DocumentIdentification4>,
    #[validate]
    #[serde(rename = "Rpt")]
    pub rpt: MisMatchReport3,
    #[serde(rename = "ReqForActn", skip_serializing_if = "Option::is_none")]
    pub req_for_actn: Option<PendingActivity2>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Action2Code {
    #[serde(rename = "SBTW")]
    Sbtw,
    #[serde(rename = "RSTW")]
    Rstw,
    #[serde(rename = "RSBS")]
    Rsbs,
    #[serde(rename = "ARDM")]
    Ardm,
    #[serde(rename = "ARCS")]
    Arcs,
    #[serde(rename = "ARES")]
    Ares,
    #[serde(rename = "WAIT")]
    Wait,
    #[serde(rename = "UPDT")]
    Updt,
    #[serde(rename = "SBDS")]
    Sbds,
    #[serde(rename = "ARBA")]
    Arba,
    #[serde(rename = "ARRO")]
    Arro,
    #[serde(rename = "CINR")]
    Cinr,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BaselineStatus3Code {
    #[serde(rename = "PROP")]
    Prop,
    #[serde(rename = "CLSD")]
    Clsd,
    #[serde(rename = "PMTC")]
    Pmtc,
    #[serde(rename = "ESTD")]
    Estd,
    #[serde(rename = "ACTV")]
    Actv,
    #[serde(rename = "COMP")]
    Comp,
    #[serde(rename = "AMRQ")]
    Amrq,
    #[serde(rename = "RARQ")]
    Rarq,
    #[serde(rename = "CLRQ")]
    Clrq,
    #[serde(rename = "SCRQ")]
    Scrq,
    #[serde(rename = "SERQ")]
    Serq,
    #[serde(rename = "DARQ")]
    Darq,
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
pub struct MisMatchReport3 {
    #[validate]
    #[serde(rename = "NbOfMisMtchs")]
    pub nb_of_mis_mtchs: Number,
    #[validate(length(min = 0,))]
    #[serde(rename = "MisMtchInf", default)]
    pub mis_mtch_inf: Vec<ValidationResult5>,
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
pub struct GenericIdentification4 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[validate]
    #[serde(rename = "IdTp")]
    pub id_tp: Max35Text,
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
pub struct Document {
    #[validate]
    #[serde(rename = "BaselnMtchRpt")]
    pub baseln_mtch_rpt: BaselineMatchReportV03,
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
pub struct Max70Text {
    #[validate(length(min = 1, max = 70,))]
    #[serde(rename = "$text")]
    pub value: String,
}
