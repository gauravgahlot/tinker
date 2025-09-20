use anyhow::Result;
use kube;

pub struct TinkerbellClient {
    k8s: kube::Client,
}

impl TinkerbellClient {
    pub async fn connect() -> Result<TinkerbellClient> {
        let client = kube::Client::try_default().await?;

        Ok(Self { k8s: client })
    }

    pub fn namespace(&self) -> String {
        self.k8s.default_namespace().to_string()
    }
}
