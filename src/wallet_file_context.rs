use std::path::Path;
use std::path::PathBuf;

use anyhow::ensure;
use anyhow::Result;

use super::wallet_entropy::WalletEntropy;
use super::wallet_file;

/// Wrapper around [`WalletFile`] with extra context.
#[derive(Debug, Clone)]
pub struct WalletFileContext {
    pub(crate) wallet_file: wallet_file::WalletFile,

    pub wallet_secret_path: PathBuf,
    pub incoming_randomness_file: PathBuf,
    pub outgoing_randomness_file: PathBuf,

    pub wallet_is_new: bool,
}

impl WalletFileContext {
    pub fn wallet_secret_path(wallet_directory_path: &Path) -> PathBuf {
        wallet_directory_path.join(wallet_file::WALLET_SECRET_FILE_NAME)
    }

    fn wallet_outgoing_secrets_path(wallet_directory_path: &Path) -> PathBuf {
        wallet_directory_path.join(wallet_file::WALLET_OUTGOING_SECRETS_FILE_NAME)
    }

    fn wallet_incoming_secrets_path(wallet_directory_path: &Path) -> PathBuf {
        wallet_directory_path.join(wallet_file::WALLET_INCOMING_SECRETS_FILE_NAME)
    }

    /// Read a wallet from disk or create it.
    ///
    /// Read a wallet file from the `wallet.dat` file in the given directory if
    /// it exists, or otherwise create new wallet secret and save it there.
    /// Also, create files for incoming and outgoing randomness which should be
    /// appended to with each incoming and outgoing transaction.
    pub fn read_from_file_or_create(wallet_directory_path: &Path) -> Result<Self> {
        let wallet_secret_path = Self::wallet_secret_path(wallet_directory_path);
        let wallet_is_new;
        let wallet_secret = if wallet_secret_path.exists() {
//            info!(
//                "***** Reading wallet from {} *****\n\n\n",
//                wallet_secret_path.display()
//            );
            wallet_is_new = false;
            wallet_file::WalletFile::read_from_file(&wallet_secret_path)?
        } else {
//            info!(
//                "***** Creating new wallet in {} *****\n\n\n",
//                wallet_secret_path.display()
//            );
            let new_wallet = wallet_file::WalletFile::new_random();
            new_wallet.save_to_disk(&wallet_secret_path)?;
            wallet_is_new = true;
            new_wallet
        };

        // Generate files for outgoing and ingoing randomness if those files
        // do not already exist
        let outgoing_randomness_file = Self::wallet_outgoing_secrets_path(wallet_directory_path);
        if !outgoing_randomness_file.exists() {
            wallet_file::WalletFile::create_empty_wallet_randomness_file(&outgoing_randomness_file)
                .unwrap_or_else(|_| {
                    panic!(
                        "Create file for outgoing randomness must succeed. \
                        Attempted to create file: {}",
                        outgoing_randomness_file.to_string_lossy()
                    )
                });
        }

        let incoming_randomness_file = Self::wallet_incoming_secrets_path(wallet_directory_path);
        if !incoming_randomness_file.exists() {
            wallet_file::WalletFile::create_empty_wallet_randomness_file(&incoming_randomness_file)
                .unwrap_or_else(|_| {
                    panic!(
                        "Create file for outgoing randomness must succeed. \
                        Attempted to create file: {}",
                        incoming_randomness_file.to_string_lossy()
                    )
                });
        }

        // Sanity checks that files were actually created
        ensure!(
            wallet_secret_path.exists(),
            "Wallet secret file '{}' must exist on disk after reading/creating it.",
            wallet_secret_path.display(),
        );
        ensure!(
            outgoing_randomness_file.exists(),
            "file containing outgoing randomness '{}' must exist on disk.",
            outgoing_randomness_file.display(),
        );
        ensure!(
            incoming_randomness_file.exists(),
            "file containing ingoing randomness '{}' must exist on disk.",
            incoming_randomness_file.display(),
        );

        Ok(Self {
            wallet_file: wallet_secret,
            wallet_secret_path,
            incoming_randomness_file,
            outgoing_randomness_file,
            wallet_is_new,
        })
    }

    /// Extract the entropy
    pub fn entropy(&self) -> WalletEntropy {
        self.wallet_file.entropy()
    }
}

