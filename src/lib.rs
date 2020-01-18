mod lexoffice {
    mod contacts {
        use uuid::Uuid;
        use chrono::{DateTime, Utc};
        #[derive(Debug, Clone, PartialEq)]
        enum SalutationEnum {
            Herr,
            Frau,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct ContactProperties {
            field_id: Uuid,
            field_organization_id: Uuid,
            field_version: i64,
            field_roles: RolesDetails,
            field_company: CompanyDetails,
            field_person: PersonDetails,
            field_addresses: AddressesDetails,
            field_email_addresses: EMailAddressesDetails,
            field_phone_numbers: PhoneNumbersDetails,
            field_note: String,
            field_archived: bool,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct RolesDetails {
            field_customer: CustomerDetails,
            field_vendor: VendorDetails,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct CustomerDetails {
            field_number: i64,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct VendorDetails {
            field_number: i64,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct CompanyDetails {
            field_allow_tax_free_invoices: bool,
            field_name: String,
            field_tax_number: String,
            field_vat_registration_id: String,
            contact_persons: Vec<CompanyContactPersonDetails>,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct CompanyContactPersonDetails {
            field_salutation: SalutationEnum,
            field_first_name: String,
            field_last_name: String,
            field_email_address: String,
            field_phone_number: String,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct PersonDetails {
            field_salutation: SalutationEnum,
            field_first_name: String,
            field_last_name: String,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct AddressesDetails {
            billing: Vec<AddressDetails>,
            shipping: Vec<AddressDetails>,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct AddressDetails {
            field_supplement: String,
            field_street: String,
            field_zip: String,
            field_city: String,
            field_country_code: String,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct EMailAddressesDetails {
            business: Vec<String>,
            office: Vec<String>,
            private: Vec<String>,
            other: Vec<String>,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct PhoneNumbersDetails {
            business: Vec<String>,
            office: Vec<String>,
            mobile: Vec<String>,
            private: Vec<String>,
            fax: Vec<String>,
            other: Vec<String>,
        }
    }
    mod credit_notes {
        use uuid::Uuid;
        use chrono::{DateTime, Utc};
        #[derive(Debug, Clone, PartialEq)]
        enum CountryCodeEnum { }
        #[derive(Debug, Clone, PartialEq)]
        enum CurrencyEnum {
            EUR,
        }
        #[derive(Debug, Clone, PartialEq)]
        enum TaxTypeEnum {
            Net,
            Gross,
            Vatfree,
            IntraCommunitySupply,
            ConstructionService13b,
            ExternalService13b,
            ThirdPartyCountryService,
            ThirdPartyCountryDelivery,
        }
        #[derive(Debug, Clone, PartialEq)]
        enum TypeEnum {
            Service,
            Material,
            Custom,
            Text,
        }
        #[derive(Debug, Clone, PartialEq)]
        enum VoucherStatusEnum {
            Draft,
            Open,
            Paidoff,
            Voided,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct CreditNotesProperties {
            field_id: Uuid,
            field_organization_id: Uuid,
            field_created_date: DateTime<Utc>,
            field_updated_date: DateTime<Utc>,
            field_version: i64,
            field_language: String,
            field_archived: bool,
            field_voucher_status: VoucherStatusEnum,
            field_voucher_number: String,
            field_voucher_date: DateTime<Utc>,
            field_address: AddressDetails,
            line_items: Vec<LineItemsDetails>,
            field_total_price: TotalPriceDetails,
            tax_amounts: Vec<TaxAmountsDetails>,
            field_tax_conditions: TaxConditionsDetails,
            field_title: String,
            field_introduction: String,
            field_remark: String,
            field_files: FilesDetails,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct AddressDetails {
            field_contact_id: Uuid,
            field_name: String,
            field_supplement: String,
            field_street: String,
            field_city: String,
            field_zip: String,
            field_country_code: CountryCodeEnum,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct LineItemsDetails {
            field_id: Uuid,
            field_type: TypeEnum,
            field_name: String,
            field_description: String,
            field_quantity: f64,
            field_unit_name: String,
            field_unit_price: UnitPriceDetails,
            field_line_item_amount: f64,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct UnitPriceDetails {
            field_currency: CurrencyEnum,
            field_net_amount: f64,
            field_gross_amount: f64,
            field_tax_rate_percentage: f64,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct TotalPriceDetails {
            field_currency: String,
            field_total_net_amount: f64,
            field_total_gross_amount: f64,
            field_total_tax_amount: f64,
            field_total_discount_absolute: f64,
            field_total_discount_percentage: f64,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct TaxAmountsDetails {
            field_tax_rate_percentage: f64,
            field_tax_amount: f64,
            field_net_amount: f64,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct TaxConditionsDetails {
            field_tax_type: TaxTypeEnum,
            field_tax_type_note: String,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct FilesDetails {
            field_document_file_id: Uuid,
        }
    }
    mod event_subscriptions {
        use uuid::Uuid;
        use chrono::{DateTime, Utc};
        #[derive(Debug, Clone, PartialEq)]
        struct EventSubscriptionsProperties {
            field_subscription_id: Uuid,
            field_organization_id: Uuid,
            field_created_date: DateTime<Utc>,
            field_event_type: String,
            field_callback_url: String,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct WebhookCallbackProperties {
            field_organization_id: Uuid,
            field_event_type: String,
            field_resource_id: Uuid,
            field_event_date: DateTime<Utc>,
        }
    }
    mod files {
        use uuid::Uuid;
        use chrono::{DateTime, Utc};
    }
    mod invoices {
        use uuid::Uuid;
        use chrono::{DateTime, Utc};
        #[derive(Debug, Clone, PartialEq)]
        enum CountryCodeEnum { }
        #[derive(Debug, Clone, PartialEq)]
        enum CurrencyEnum {
            EUR,
        }
        #[derive(Debug, Clone, PartialEq)]
        enum ShippingTypeEnum {
            Service,
            Serviceperiod,
            Delivery,
            Deliveryperiod,
            None,
        }
        #[derive(Debug, Clone, PartialEq)]
        enum TaxTypeEnum {
            Net,
            Gross,
            Vatfree,
            IntraCommunitySupply,
            ConstructionService13b,
            ExternalService13b,
            ThirdPartyCountryService,
            ThirdPartyCountryDelivery,
        }
        #[derive(Debug, Clone, PartialEq)]
        enum TypeEnum {
            Service,
            Material,
            Custom,
            Text,
        }
        #[derive(Debug, Clone, PartialEq)]
        enum VoucherStatusEnum {
            Draft,
            Open,
            Paid,
            Voided,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct InvoicesProperties {
            field_id: Uuid,
            field_organization_id: Uuid,
            field_created_date: DateTime<Utc>,
            field_updated_date: DateTime<Utc>,
            field_version: i64,
            field_language: String,
            field_archived: bool,
            field_voucher_status: VoucherStatusEnum,
            field_voucher_number: String,
            field_voucher_date: DateTime<Utc>,
            field_due_date: DateTime<Utc>,
            field_address: AddressDetails,
            line_items: Vec<LineItemsDetails>,
            field_total_price: TotalPriceDetails,
            tax_amounts: Vec<TaxAmountsDetails>,
            field_tax_conditions: TaxConditionsDetails,
            field_payment_conditions: PaymentConditionsDetails,
            field_shipping_conditions: ShippingConditionsDetails,
            field_title: String,
            field_introduction: String,
            field_remark: String,
            field_files: FilesDetails,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct AddressDetails {
            field_contact_id: Uuid,
            field_name: String,
            field_supplement: String,
            field_street: String,
            field_city: String,
            field_zip: String,
            field_country_code: CountryCodeEnum,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct LineItemsDetails {
            field_id: Uuid,
            field_type: TypeEnum,
            field_name: String,
            field_description: String,
            field_quantity: f64,
            field_unit_name: String,
            field_unit_price: UnitPriceDetails,
            field_discount_percentage: f64,
            field_line_item_amount: f64,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct UnitPriceDetails {
            field_currency: CurrencyEnum,
            field_net_amount: f64,
            field_gross_amount: f64,
            field_tax_rate_percentage: f64,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct TotalPriceDetails {
            field_currency: String,
            field_total_net_amount: f64,
            field_total_gross_amount: f64,
            field_total_tax_amount: f64,
            field_total_discount_absolute: f64,
            field_total_discount_percentage: f64,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct TaxAmountsDetails {
            field_tax_rate_percentage: f64,
            field_tax_amount: f64,
            field_net_amount: f64,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct TaxConditionsDetails {
            field_tax_type: TaxTypeEnum,
            field_tax_type_note: String,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct PaymentConditionsDetails {
            field_payment_term_label: String,
            field_payment_term_duration: i64,
            payment_discount_conditions: Vec<PaymentDiscountConditionsDetails>,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct PaymentDiscountConditionsDetails {
            field_discount_percentage: f64,
            field_discount_range: i64,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct ShippingConditionsDetails {
            field_shipping_date: DateTime<Utc>,
            field_shipping_end_date: DateTime<Utc>,
            field_shipping_type: ShippingTypeEnum,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct FilesDetails {
            field_document_file_id: Uuid,
        }
    }
    mod order_confirmations {
        use uuid::Uuid;
        use chrono::{DateTime, Utc};
        #[derive(Debug, Clone, PartialEq)]
        enum CountryCodeEnum { }
        #[derive(Debug, Clone, PartialEq)]
        enum CurrencyEnum {
            EUR,
        }
        #[derive(Debug, Clone, PartialEq)]
        enum ShippingTypeEnum {
            Service,
            Serviceperiod,
            Delivery,
            Deliveryperiod,
            None,
        }
        #[derive(Debug, Clone, PartialEq)]
        enum TaxTypeEnum {
            Net,
            Gross,
            Vatfree,
            IntraCommunitySupply,
            ConstructionService13b,
            ExternalService13b,
            ThirdPartyCountryService,
            ThirdPartyCountryDelivery,
        }
        #[derive(Debug, Clone, PartialEq)]
        enum TypeEnum {
            Service,
            Material,
            Custom,
            Text,
        }
        #[derive(Debug, Clone, PartialEq)]
        enum VoucherStatusEnum {
            Draft,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct OrderConfirmationsProperties {
            field_id: Uuid,
            field_organization_id: Uuid,
            field_created_date: DateTime<Utc>,
            field_updated_date: DateTime<Utc>,
            field_version: i64,
            field_language: String,
            field_archived: bool,
            field_voucher_status: VoucherStatusEnum,
            field_voucher_number: String,
            field_voucher_date: DateTime<Utc>,
            field_address: AddressDetails,
            line_items: Vec<LineItemsDetails>,
            field_total_price: TotalPriceDetails,
            tax_amounts: Vec<TaxAmountsDetails>,
            field_tax_conditions: TaxConditionsDetails,
            field_payment_conditions: PaymentConditionsDetails,
            field_shipping_conditions: ShippingConditionsDetails,
            field_title: String,
            field_introduction: String,
            field_remark: String,
            field_delivery_terms: String,
            field_files: FilesDetails,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct AddressDetails {
            field_contact_id: Uuid,
            field_name: String,
            field_supplement: String,
            field_street: String,
            field_city: String,
            field_zip: String,
            field_country_code: CountryCodeEnum,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct LineItemsDetails {
            field_id: Uuid,
            field_type: TypeEnum,
            field_name: String,
            field_description: String,
            field_quantity: f64,
            field_unit_name: String,
            field_unit_price: UnitPriceDetails,
            field_discount_percentage: f64,
            field_line_item_amount: f64,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct UnitPriceDetails {
            field_currency: CurrencyEnum,
            field_net_amount: f64,
            field_gross_amount: f64,
            field_tax_rate_percentage: f64,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct TotalPriceDetails {
            field_currency: String,
            field_total_net_amount: f64,
            field_total_gross_amount: f64,
            field_total_tax_amount: f64,
            field_total_discount_absolute: f64,
            field_total_discount_percentage: f64,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct TaxAmountsDetails {
            field_tax_rate_percentage: f64,
            field_tax_amount: f64,
            field_net_amount: f64,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct TaxConditionsDetails {
            field_tax_type: TaxTypeEnum,
            field_tax_type_note: String,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct PaymentConditionsDetails {
            field_payment_term_label: String,
            field_payment_term_duration: i64,
            payment_discount_conditions: Vec<PaymentDiscountConditionsDetails>,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct PaymentDiscountConditionsDetails {
            field_discount_percentage: f64,
            field_discount_range: i64,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct ShippingConditionsDetails {
            field_shipping_date: DateTime<Utc>,
            field_shipping_end_date: DateTime<Utc>,
            field_shipping_type: ShippingTypeEnum,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct FilesDetails {
            field_document_file_id: Uuid,
        }
    }
    mod profile {
        use uuid::Uuid;
        use chrono::{DateTime, Utc};
        #[derive(Debug, Clone, PartialEq)]
        enum TaxTypeEnum {
            Net,
            Gross,
            Vatfree,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct ProfileProperties {
            field_organization_id: Uuid,
            field_company_name: String,
            field_created: CreatedDetails,
            field_connection_id: Uuid,
            field_tax_type: TaxTypeEnum,
            field_small_business: bool,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct CreatedDetails {
            field_user_name: String,
            field_user_email: String,
            field_date: String,
        }
    }
    mod voucherlist {
        use uuid::Uuid;
        use chrono::{DateTime, Utc};
        #[derive(Debug, Clone, PartialEq)]
        enum CurrencyEnum {
            EUR,
        }
        #[derive(Debug, Clone, PartialEq)]
        enum VoucherStatusEnum {
            Draft,
            Open,
            Paid,
            Paidoff,
            Voided,
            Transferred,
            Overdue,
        }
        #[derive(Debug, Clone, PartialEq)]
        enum VoucherTypeEnum {
            Invoice,
            Creditnote,
            Orderconfirmation,
        }
        #[derive(Debug, Clone, PartialEq)]
        struct VoucherlistProperties {
            field_id: Uuid,
            field_voucher_type: VoucherTypeEnum,
            field_voucher_status: VoucherStatusEnum,
            field_voucher_number: String,
            field_voucher_date: DateTime<Utc>,
            field_updated_date: DateTime<Utc>,
            field_due_date: DateTime<Utc>,
            field_contact_name: String,
            field_total_amount: f64,
            field_currency: CurrencyEnum,
            field_archived: bool,
        }
    }
}
