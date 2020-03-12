# iso20022

[ISO 20022] Universal Financial Industry Messages in Rust.

[![drone]](https://cloud.drone.io/keithnoguchi/iso20022-rs)
[![crate]](https://lib.rs/iso20022)
[![docs]](https://docs.rs/iso20022)

[drone]: https://cloud.drone.io/api/badges/keithnoguchi/iso20022-rs/status.svg
[crate]: https://img.shields.io/crates/v/iso20022.svg
[docs]: https://docs.rs/iso20022/badge.svg

Here is the list of the currently supported messages:

- [Business Application Header]
  - [head.001.001.02] BusinessApplicationHeaderV02 message header
- [Payments Messages]
  - [pain.001.001.10] CustomerCreditTransferInitiationV10 message
  - [pain.002.001.11] CustomerPaymentStatusReportV11 message
  - [pain.013.001.08] CreditorPaymentActivationRequestV8 message
  - [pain.014.001.08] CreditorPaymentActivationRequestStatusReportV08 message

[iso 20022]: https://www.iso20022.org/
[business application header]: https://www.iso20022.org/bah.page
[payments messages]: https://www.iso20022.org/payments_messages.page
[head.001.001.02]: proto/head.001.001.02.proto
[pain.001.001.10]: proto/pain.001.001.10.proto
[pain.002.001.11]: proto/pain.002.001.11.proto
[pain.013.001.08]: proto/pain.013.001.08.proto
[pain.014.001.08]: proto/pain.014.001.08.proto
