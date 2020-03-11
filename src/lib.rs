//! [ISO 20022] Universal Financial Industry Messages
//!
//! [iso 20022]: https://www.iso20022.org/

/// BusinessApplicationHeaderV02 message header
pub mod head_001_001_02;
/// CustomerCreditTransferInitiationV10 message
pub mod pain_001_001_10;
/// CustomerPaymentStatusReportV11 message
pub mod pain_002_001_11;

pub use head_001_001_02::BusinessApplicationHeaderV02;
pub use pain_001_001_10::CustomerCreditTransferInitiationV10;
pub use pain_002_001_11::CustomerPaymentStatusReportV11;
