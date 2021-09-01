//! Type definitions for dealing with Starling webhook payloads.

use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

/// Webhook payload dispatched in response to an event occurring
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WebhookDispatchFeedItem {
    /// Unique identifier representing an event occurring for an account holder
    pub webhook_event_uid: Uuid,
    /// Timestamp indicating when the webhook event occurred
    pub event_timestamp: DateTime<Utc>,
    /// An account holder for which an event has occurred
    pub account_holder_uid: Uuid,
    /// An item from the account holders' transaction feed
    pub content: FeedItem,
}

/// Webhook payload dispatched in response to an event occurring
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WebhookDispatchPaymentOrderWebhookEventPayload {
    /// Unique identifier representing an event occurring for an account holder
    pub webhook_event_uid: Uuid,
    /// Timestamp indicating when the webhook event occurred
    pub event_timestamp: DateTime<Utc>,
    /// An account holder for which an event has occurred
    pub account_holder_uid: Uuid,
    /// Webhook event payload indicating operation success or failure
    pub content: PaymentOrderWebhookEventPayload,
}

/// Webhook payload dispatched in response to an event occurring
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WebhookDispatchPaymentOrder {
    /// Unique identifier representing an event occurring for an account holder
    pub webhook_event_uid: Uuid,
    /// Timestamp indicating when the webhook event occurred
    pub event_timestamp: DateTime<Utc>,
    /// An account holder for which an event has occurred
    pub account_holder_uid: Uuid,
    /// A payment instruction to be carried out at a specific point in time
    pub content: PaymentOrder,
}

/// An item from the account holders' transaction feed
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FeedItem {
    /// Unique identifier for this item
    pub feed_item_uid: Uuid,
    /// The category on which the transaction happened
    pub category_uid: Uuid,
    /// The account for which the transaction happened
    pub account_uid: Uuid,
    /// Representation of money
    pub amount: CurrencyAndAmount,
    /// Representation of money
    pub source_amount: CurrencyAndAmount,
    /// Was this an inbound or outbound transaction
    pub direction: Direction,
    /// The time the transaction was last updated at
    pub updated_at: DateTime<Utc>,
    /// The time of the transaction
    pub transaction_time: DateTime<Utc>,
    /// The time the transaction settled
    pub settlement_time: DateTime<Utc>,
    /// The source of the transaction
    pub source: Source,
    /// The source subtype of the transaction
    pub source_sub_type: Option<SourceSubtype>,
    /// The status of the transaction
    pub status: Status,
    /// The application user that made the transaction
    pub transacting_application_user_uid: Option<Uuid>,
    /// The type of counter party for this transaction
    pub counter_party_type: CounterPartyType,
    /// The unique identifier for the counter party
    pub counter_party_uid: Option<Uuid>,
    /// The name of the counter party
    pub counter_party_name: String,
    /// An identifier for the counter party sub entity
    pub counter_party_sub_entity_uid: Option<Uuid>,
    /// A name for the counter party sub entity
    pub counter_party_sub_entity_name: String,
    /// An external identifier for the sub entity
    pub counter_party_sub_entity_identifier: String,
    /// An external sub identifier for the sub entity
    pub counter_party_sub_entity_sub_identifier: String,
    /// The exchange rate applied between different currencies
    pub exchange_rate: Option<f64>,
    /// Representation of money
    pub total_fee_amount: Option<CurrencyAndAmount>,
    /// The reference for the transaction
    pub reference: String,
    /// The country in which the transaction took place
    pub country: Country,
    /// The category of the transaction
    pub spending_category: SpendingCategory,
    /// The user-provided transaction note
    pub user_note: Option<String>,
    /// Round up details associated with a feed item
    pub round_up: Option<AssociatedFeedRoundUp>,
    /// Indicates whether an attachment exists for the feed item
    pub has_attachment: bool,
    /// Indicates if a copy of the receipt is present
    pub receipt_present: bool,
    /// Provides the failure reason for a failed transaction
    pub feed_item_failure_reason: Option<FeedItemFailureReason>,
    /// The MasterCard feed item details
    pub master_card_feed_details: Option<MasterCardFeedItemData>,
}

/// Webhook event payload indicating operation success or failure
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PaymentOrderWebhookEventPayload {
    /// A payment instruction to be carried out at a specific point in time
    pub payment_order: PaymentOrder,
    /// Indicates if the payment was successful
    pub success: bool,
    /// Provides the reported success or failure reason code
    pub reason: String,
    /// Provides the payment uuid of the successful payment
    pub payment_uid: Uuid,
}

/// A payment instruction to be carried out at a specific point in time
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PaymentOrder {
    /// Unique identifier of this payment order
    pub payment_order_uid: Uuid,
    /// Unique identifier of the category associated with this payment order
    pub category_uid: Uuid,
    /// Representation of money
    pub amount: CurrencyAndAmount,
    /// The reference set by the payer
    pub reference: String,
    /// The ID of the payee receiving the payments
    pub payee_uid: Uuid,
    /// The account ID of the payee account receiving the payments
    pub payee_account_uid: Uuid,
    /// Recurrence rules of a standing order
    pub payment_order_recurrance: Option<StandingOrderRecurrance>,
    /// Indicates if the payment order should process immediately or if this is a future dated
    /// payment
    pub processed_immediately: bool,
    /// Date on which the next standing order payment will be made
    pub next_date: NaiveDate,
    /// The time the payment order is cancelled at
    pub cancelled_at: DateTime<Utc>,
    /// The time the payment order is updated at
    pub updated_at: DateTime<Utc>,
    /// Optional spending category for the payment order
    pub spending_category: Option<SpendingCategory>,
    /// Recurrence rules of a standing order
    pub standing_order_recurrance: Option<StandingOrderRecurrance>,
}

/// Recurrence rules of a standing order
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StandingOrderRecurrance {
    /// Date that the first standing order payment should be made
    pub start_date: NaiveDate,
    /// Frequency of which payments should be made
    pub frequency: Option<Frequency>,
    /// Interval of specified frequency on which payments should be made
    pub interval: Option<i32>,
    /// Number of payments that should be made before standing order is stopped
    pub count: Option<i32>,
    /// Date on which to stop standing order
    pub until_date: NaiveDate,
}

/// Representation of money
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyAndAmount {
    /// ISO-4217 3 character currency code
    pub currency: Currency,
    /// Amount in the minor units of the given currency
    pub minor_units: i64,
}

/// Round up details associated with a feed item
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AssociatedFeedRoundUp {
    /// Unique identifier of associated category
    pub goal_category_uid: Uuid,
    /// Representation of money
    pub amount: CurrencyAndAmount,
}

/// The MasterCard feed item details
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MasterCardFeedItemData {
    /// The identifier for the merchant
    pub merchant_identifier: String,
    /// The category given by the merchant
    pub mcc: i32,
    /// The point of sale timestamp
    pub pos_timestamp: LocalTime,
    /// The authorisation code for the feed item
    pub authorisation_code: String,
    /// The last 4 digits on the card
    pub card_last_4: String,
}

/// The point of sale timestamp
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LocalTime {
    pub hour: i32,
    pub minute: i32,
    pub second: i32,
    pub nano: i32,
}

/// ISO-4217 3 character currency code
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum Currency {
    NDEFINED,
    AED,
    AFN,
    ALL,
    AMD,
    ANG,
    AOA,
    ARS,
    AUD,
    AWG,
    AZN,
    BAM,
    BBD,
    BDT,
    BGN,
    BHD,
    BIF,
    BMD,
    BND,
    BOB,
    BOV,
    BRL,
    BSD,
    BTN,
    BWP,
    BYN,
    BYR,
    BZD,
    CAD,
    CDF,
    CHE,
    CHF,
    CHW,
    CLF,
    CLP,
    CNY,
    COP,
    COU,
    CRC,
    CUC,
    CUP,
    CVE,
    CZK,
    DJF,
    DKK,
    DOP,
    DZD,
    EGP,
    ERN,
    ETB,
    EUR,
    FJD,
    FKP,
    GBP,
    GEL,
    GHS,
    GIP,
    GMD,
    GNF,
    GTQ,
    GYD,
    HKD,
    HNL,
    HRK,
    HTG,
    HUF,
    IDR,
    ILS,
    INR,
    IQD,
    IRR,
    ISK,
    JMD,
    JOD,
    JPY,
    KES,
    KGS,
    KHR,
    KMF,
    KPW,
    KRW,
    KWD,
    KYD,
    KZT,
    LAK,
    LBP,
    LKR,
    LRD,
    LSL,
    LTL,
    LYD,
    MAD,
    MDL,
    MGA,
    MKD,
    MMK,
    MNT,
    MOP,
    MRO,
    MRU,
    MUR,
    MVR,
    MWK,
    MXN,
    MXV,
    MYR,
    MZN,
    NAD,
    NGN,
    NIO,
    NOK,
    NPR,
    NZD,
    OMR,
    PAB,
    PEN,
    PGK,
    PHP,
    PKR,
    PLN,
    PYG,
    QAR,
    RON,
    RSD,
    RUB,
    RUR,
    RWF,
    SAR,
    SBD,
    SCR,
    SDG,
    SEK,
    SGD,
    SHP,
    SLL,
    SOS,
    SRD,
    SSP,
    STD,
    STN,
    SVC,
    SYP,
    SZL,
    THB,
    TJS,
    TMT,
    TND,
    TOP,
    TRY,
    TTD,
    TWD,
    TZS,
    UAH,
    UGX,
    USD,
    USN,
    USS,
    UYI,
    UYU,
    UZS,
    VEF,
    VES,
    VND,
    VUV,
    WST,
    XAF,
    XAG,
    XAU,
    XBA,
    XBB,
    XBC,
    XBD,
    XCD,
    XDR,
    XOF,
    XPD,
    XPF,
    XPT,
    XSU,
    XTS,
    XUA,
    XXX,
    YER,
    ZAR,
    ZMW,
    ZWL,
}

/// Was this an inbound or outbound transaction
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Direction {
    In,
    Out,
}

/// The source of the transaction
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Source {
    BritishBusinessBankFees,
    CashDeposit,
    CashDepositCharge,
    CashWithdrawal,
    CashWithdrawalCharge,
    Chaps,
    Cheque,
    CicsCheque,
    CurrencyCloud,
    DirectCredit,
    DirectDebit,
    DirectDebitDispute,
    InternalTransfer,
    MasterCard,
    MastercardMoneysend,
    MastercardChargeback,
    FasterPaymentsIn,
    FasterPaymentsOut,
    FasterPaymentsReversal,
    StripeFunding,
    InterestPayment,
    NostroDeposit,
    Overdraft,
    OverdraftInterestWaived,
    FasterPaymentsRefund,
    StarlingPayStripe,
    OnUsPayMe,
    LoanPrincipalPayment,
    LoanRepayment,
    LoanOverpayment,
    LoanLatePayment,
    LoanFeePayment,
    SepaCreditTransfer,
    SepaDirectDebit,
    Target2CustomerPayment,
    FxTransfer,
    IssPayment,
    StarlingPayment,
    SubscriptionCharge,
    OverdraftFee,
}

/// The source subtype of the transaction
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SourceSubtype {
    Contactless,
    MagneticStrip,
    ManualKeyEntry,
    ChipAndPin,
    Online,
    Atm,
    CreditAuth,
    ApplePay,
    AndroidPay,
    FitbitPay,
    GarminPay,
    SamsungPay,
    OtherWallet,
    NotApplicable,
    Unknown,
    Deposit,
    Overdraft,
    SettleUp,
    Nearby,
    TransferSameCurrency,
    SctPayment,
    SctRejection,
    SctReturn,
    SctRecall,
    SddPending,
    SddPayment,
    SddReturn,
    CcExchange,
    CcDomestic,
    FxTransferSameAccountHolder,
    FxTransferBetweenAccountHolders,
}

/// The status of the transaction
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    Upcoming,
    Pending,
    Retrying,
    Reversed,
    Settled,
    Declined,
    Refunded,
    AccountCheck,
}

/// The type of counter party for this transaction
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CounterPartyType {
    Category,
    Cheque,
    Customer,
    Payee,
    Merchant,
    Sender,
    Starling,
    Loan,
}

/// The country in which the transaction took place
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum Country {
    UNDEFINED,
    AC,
    AD,
    AE,
    AF,
    AG,
    AI,
    AL,
    AM,
    AN,
    AO,
    AQ,
    AR,
    AS,
    AT,
    AU,
    AW,
    AX,
    AZ,
    BA,
    BB,
    BD,
    BE,
    BF,
    BG,
    BH,
    BI,
    BJ,
    BL,
    BM,
    BN,
    BO,
    BQ,
    BR,
    BS,
    BT,
    BU,
    BV,
    BW,
    BY,
    BZ,
    CA,
    CC,
    CD,
    CF,
    CG,
    CH,
    CI,
    CK,
    CL,
    CM,
    CN,
    CO,
    CP,
    CR,
    CS,
    CU,
    CV,
    CW,
    CX,
    CY,
    CZ,
    DE,
    DG,
    DJ,
    DK,
    DM,
    DO,
    DZ,
    EA,
    EC,
    EE,
    EG,
    EH,
    ER,
    ES,
    ET,
    EU,
    EZ,
    FI,
    FJ,
    FK,
    FM,
    FO,
    FR,
    FX,
    GA,
    GB,
    GD,
    GE,
    GF,
    GG,
    GH,
    GI,
    GL,
    GM,
    GN,
    GP,
    GQ,
    GR,
    GS,
    GT,
    GU,
    GW,
    GY,
    HK,
    HM,
    HN,
    HR,
    HT,
    HU,
    IC,
    ID,
    IE,
    IL,
    IM,
    IN,
    IO,
    IQ,
    IR,
    IS,
    IT,
    JE,
    JM,
    JO,
    JP,
    KE,
    KG,
    KH,
    KI,
    KM,
    KN,
    KP,
    KR,
    KW,
    KY,
    KZ,
    LA,
    LB,
    LC,
    LI,
    LK,
    LR,
    LS,
    LT,
    LU,
    LV,
    LY,
    MA,
    MC,
    MD,
    ME,
    MF,
    MG,
    MH,
    MK,
    ML,
    MM,
    MN,
    MO,
    MP,
    MQ,
    MR,
    MS,
    MT,
    MU,
    MV,
    MW,
    MX,
    MY,
    MZ,
    NA,
    NC,
    NE,
    NF,
    NG,
    NI,
    NL,
    NO,
    NP,
    NR,
    NT,
    NU,
    NZ,
    OM,
    PA,
    PE,
    PF,
    PG,
    PH,
    PK,
    PL,
    PM,
    PN,
    PR,
    PS,
    PT,
    PW,
    PY,
    QA,
    RE,
    RO,
    RS,
    RU,
    RW,
    SA,
    SB,
    SC,
    SD,
    SE,
    SF,
    SG,
    SH,
    SI,
    SJ,
    SK,
    SL,
    SM,
    SN,
    SO,
    SR,
    SS,
    ST,
    SU,
    SV,
    SX,
    SY,
    SZ,
    TA,
    TC,
    TD,
    TF,
    TG,
    TH,
    TJ,
    TK,
    TL,
    TM,
    TN,
    TO,
    TP,
    TR,
    TT,
    TV,
    TW,
    TZ,
    UA,
    UG,
    UK,
    UM,
    US,
    UY,
    UZ,
    VA,
    VC,
    VE,
    VG,
    VI,
    VN,
    VU,
    WF,
    WS,
    XK,
    YE,
    YT,
    YU,
    ZA,
    ZM,
    ZR,
    ZW,
}

/// The category of the transaction
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SpendingCategory {
    BillsAndServices,
    Charity,
    EatingOut,
    Entertainment,
    Expenses,
    Family,
    Gambling,
    General,
    Gifts,
    Groceries,
    Holidays,
    Home,
    Income,
    Lifestyle,
    Payments,
    Pets,
    Saving,
    Shopping,
    Transport,
    None,
    Revenue,
    OtherIncome,
    ClientRefunds,
    Inventory,
    Staff,
    Travel,
    Workplace,
    RepairsAndMaintenance,
    Admin,
    Marketing,
    BusinessEntertainment,
    InterestPayments,
    BankCharges,
    Other,
    FoodAndDrink,
    Equipment,
    ProfessionalServices,
    PhoneAndInternet,
    Vehicles,
    DirectorsWages,
    Vat,
    CorporationTax,
    SelfAssessmentTax,
    InvestmentCapital,
    Transfers,
    LoanPrincipal,
    Personal,
    Dividends,
}

/// Provides the failure reason for a failed transaction
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FeedItemFailureReason {
    CardWalletLimit,
    CardApplePayLimit,
    CardPosDisabled,
    CardAtmDisabled,
    CardMobileWalletDisabled,
    CardOnlineDisabled,
    CardGamblingDisabled,
    CardDisabled,
    CardCancelled,
    CardNotActivated,
    CardMagneticStripDisabled,
    CardManualKeyEntryDisabled,
    CardPayAtPumpDeclined,
    CardInsufficientFunds,
    ChildCardMerchantDisabled,
    DestinationAccountInvalid,
    ReferenceInformationIncorrect,
    DestinationAccountCurrencyIncorrect,
    DestinationAccountNameMismatch,
    DestinationAccountUnavailable,
    IncorrectPin,
    IncorrectCvv2,
    InsufficientFunds,
    MandateCancelled,
    MandateNotFound,
    ChequeBeingRepresented,
    ChequeIssuerAccountClosed,
    ChequeNotSignedInAccordanceWithMandate,
    ChequeStopped,
    ChequeDeclinedReferToIssuer,
    LastBillItemCancelled,
    ScaRequired,
    PinTriesExceeded,
    CvcTriesExceeded,
    SuspiciousCardTransaction,
}

/// Frequency of which payments should be made
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum Frequency {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}
