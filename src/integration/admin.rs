use crate::types::api::ErrorWrapper;
use crate::types::auth::{AuthTokens, SSORes, UserClaims};
use crate::types::masternode::{AssignMasternodeRes, MasternodeInfo};
use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;
use reqwest::{Client, StatusCode};
use std::{fmt::Debug, sync::Arc};

#[automock]
#[async_trait]
pub trait AdminService: Debug + Send + Sync + 'static {
    async fn health_check(self: Arc<Self>) -> Result<()>;
    async fn verify_auth_token(self: Arc<Self>, auth_tokens: AuthTokens) -> Result<UserClaims>;
    async fn refresh_token(self: Arc<Self>, auth_tokens: AuthTokens) -> Result<AuthTokens>;
    async fn assign_masternode(self: Arc<Self>, auth_tokens: AuthTokens) -> Result<MasternodeInfo>;
    async fn register_masternode(
        self: Arc<Self>,
        x_api_key: String,
        info: MasternodeInfo,
    ) -> Result<()>;
    async fn deregister_masternode(self: Arc<Self>, x_api_key: String) -> Result<()>;
}

#[derive(Debug)]
pub struct AdminServiceImpl {
    base_url: String,

    // paths
    health_check: String,

    verify_auth_token: String,
    refresh_token_path: String,

    assign_masternode_path: String,
    register_masternode: String,
    deregister_masternode: String,
}

impl AdminServiceImpl {
    pub async fn new(base_url: String) -> Result<Self, Error> {
        let base_url: String = match base_url.strip_suffix('/') {
            Some(url) => url.to_string(),
            None => base_url.clone(),
        };

        Ok(Self {
            base_url: base_url.clone(),
            health_check: format!("{}/{}", base_url.clone(), "metrics/health"),
            verify_auth_token: format!("{}/{}", base_url.clone(), "auth/verify_auth_token"),
            refresh_token_path: format!("{}/{}", base_url.clone(), "auth/refresh_token"),
            assign_masternode_path: format!(
                "{}/{}",
                base_url.clone(),
                "connections/assign_masternode"
            ),
            register_masternode: format!(
                "{}/{}",
                base_url.clone(),
                "masternode/register_masternode"
            ),
            deregister_masternode: format!(
                "{}/{}",
                base_url.clone(),
                "masternode/deregister_masternode"
            ),
        })
    }

    fn get_authorized_client(self: Arc<Self>, access_token: String) -> Client {
        Client::builder()
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    reqwest::header::AUTHORIZATION,
                    reqwest::header::HeaderValue::from_str(&format!("Bearer {}", access_token))
                        .unwrap(),
                );
                headers
            })
            .build()
            .unwrap()
    }

    fn get_xapikey_client(self: Arc<Self>, x_api_key: String) -> Client {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            "x-api-key",
            reqwest::header::HeaderValue::from_str(&x_api_key).unwrap(),
        );

        reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap()
    }
}

#[async_trait]
impl AdminService for AdminServiceImpl {
    async fn health_check(self: Arc<Self>) -> Result<(), Error> {
        let client = reqwest::Client::new();
        let admin_health_response = client.clone().get(self.health_check.clone()).send().await;

        match admin_health_response {
            Ok(res) => match res.status() {
                StatusCode::OK => Ok(()),
                code => {
                    return Err(anyhow!(
                        "health check failed err=unknonw status code {}",
                        code
                    ))
                }
            },
            Err(err) => Err(anyhow!("health check failed err={}", err)),
        }
    }

    async fn verify_auth_token(
        self: Arc<Self>,
        auth_tokens: AuthTokens,
    ) -> Result<UserClaims, Error> {
        let client = reqwest::Client::new();
        match client
            .post(self.verify_auth_token.clone())
            .json(&auth_tokens)
            .send()
            .await
        {
            Ok(res) => match res.status() {
                StatusCode::OK => match res.json::<UserClaims>().await {
                    Ok(claims) => Ok(claims),
                    Err(e) => return Err(anyhow!("decode user claims failed err={}", e)),
                },
                StatusCode::UNAUTHORIZED => {
                    return Err(anyhow!("unauthorized token={:?}", auth_tokens))
                }
                unknown => {
                    return Err(anyhow!(
                        "verify auth token failed unknown status code={}",
                        unknown
                    ))
                }
            },
            Err(e) => {
                return Err(anyhow!(
                    "verify auth token failed http request failed err={}",
                    e
                ))
            }
        }
    }

    async fn refresh_token(self: Arc<Self>, auth_tokens: AuthTokens) -> Result<AuthTokens> {
        let client = reqwest::Client::new();
        match client
            .post(self.refresh_token_path.clone())
            .json(&auth_tokens)
            .send()
            .await
        {
            Ok(res) => match res.status() {
                StatusCode::OK => match res.json::<SSORes>().await {
                    Ok(sso_res) => match sso_res.code {
                        1 => {
                            let auth_tokens = AuthTokens {
                                access_token: sso_res.access_token.unwrap(),
                                refresh_token: sso_res.refresh_token.unwrap(),
                            };
                            Ok(auth_tokens.clone())
                        }
                        _ => return Err(anyhow!("failed to refresh token")),
                    },
                    Err(_) => return Err(anyhow!("decode json failed")),
                },
                _ => return Err(anyhow!("get assign masternode failed")),
            },
            Err(_) => return Err(anyhow!("get assign masternode failed")),
        }
    }

    async fn assign_masternode(self: Arc<Self>, auth_tokens: AuthTokens) -> Result<MasternodeInfo> {
        let _self = self.clone();
        let client = self.get_authorized_client(auth_tokens.access_token);
        match client
            .get(_self.assign_masternode_path.clone())
            .send()
            .await
        {
            Ok(res) => match res.status() {
                StatusCode::OK => match res.json::<AssignMasternodeRes>().await {
                    Ok(assign_masternode_rs) => match assign_masternode_rs.masternode {
                        Some(masternode) => return Ok(masternode),
                        None => return Err(anyhow!("no masternode is online")),
                    },
                    Err(_) => return Err(anyhow!("decode masternode json failed")),
                },
                StatusCode::UNAUTHORIZED => {
                    return Err(anyhow!("invalid access token"));
                }
                _ => return Err(anyhow!("unknown status code")),
            },
            Err(_) => return Err(anyhow!("cannot send request to admin service",)),
        }
    }

    async fn register_masternode(
        self: Arc<Self>,
        x_api_key: String,
        info: MasternodeInfo,
    ) -> Result<(), Error> {
        let client = self.clone().get_xapikey_client(x_api_key);
        match client
            .clone()
            .post(self.register_masternode.clone())
            .json(&info)
            .send()
            .await
        {
            Ok(res) => match res.status() {
                StatusCode::OK => return Ok(()),
                _ => {
                    return Err(anyhow!(
                        "register masternode failed err={:?} ",
                        res.json::<ErrorWrapper>().await
                    ))
                }
            },
            Err(e) => return Err(anyhow!("register master node failed err={}", e)),
        }
    }

    async fn deregister_masternode(self: Arc<Self>, x_api_key: String) -> Result<(), Error> {
        let client = self.clone().get_xapikey_client(x_api_key);
        match client
            .clone()
            .post(self.deregister_masternode.clone())
            .send()
            .await
        {
            Ok(res) => match res.status() {
                StatusCode::OK => return Ok(()),
                _ => {
                    return Err(anyhow!(
                        "deregister master node failed err=unknown status code"
                    ))
                }
            },
            Err(e) => return Err(anyhow!("deregister master node failed err={}", e)),
        }
    }
}
