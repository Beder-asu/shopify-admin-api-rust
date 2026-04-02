//! Apple Pay Certificate resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApplePayCertificate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoded_signed_certificate: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ApplePayCertificateWrapper { apple_pay_certificate: ApplePayCertificate }
#[derive(Debug, Deserialize)]
struct ApplePayCertificatesWrapper { apple_pay_certificates: Vec<ApplePayCertificate> }
#[derive(Debug, Serialize)]
struct ApplePayCertificateRequest { apple_pay_certificate: ApplePayCertificate }

impl ApplePayCertificate {
    pub async fn find(client: &Client, id: i64) -> Result<Option<Self>> {
        let path = format!("apple_pay_certificates/{}.json", id);
        let response = client.get::<ApplePayCertificateWrapper>(&path).await?;
        Ok(Some(response.data.apple_pay_certificate))
    }

    pub async fn all(client: &Client) -> Result<FindAllResponse<Self>> {
        let response = client.get::<ApplePayCertificatesWrapper>("apple_pay_certificates.json").await?;
        Ok(FindAllResponse {
            data: response.data.apple_pay_certificates,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn create(client: &Client, cert: &Self) -> Result<Self> {
        let request = ApplePayCertificateRequest { apple_pay_certificate: cert.clone() };
        let response = client.post::<ApplePayCertificateWrapper, _>("apple_pay_certificates.json", &request).await?;
        Ok(response.data.apple_pay_certificate)
    }

    pub async fn update(client: &Client, cert: &Self) -> Result<Self> {
        let id = cert.id.ok_or_else(|| crate::ShopifyError::ValidationError("Apple Pay Certificate ID required".to_string()))?;
        let path = format!("apple_pay_certificates/{}.json", id);
        let request = ApplePayCertificateRequest { apple_pay_certificate: cert.clone() };
        let response = client.put::<ApplePayCertificateWrapper, _>(&path, &request).await?;
        Ok(response.data.apple_pay_certificate)
    }

    pub async fn delete(client: &Client, id: i64) -> Result<()> {
        let path = format!("apple_pay_certificates/{}.json", id);
        client.delete(&path).await
    }

    pub async fn csr(client: &Client, id: i64) -> Result<String> {
        let path = format!("apple_pay_certificates/{}/csr.json", id);
        #[derive(Deserialize)]
        struct CsrWrapper { apple_pay_certificate_csr: String }
        let response = client.get::<CsrWrapper>(&path).await?;
        Ok(response.data.apple_pay_certificate_csr)
    }
}
