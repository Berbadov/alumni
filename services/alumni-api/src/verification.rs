//! Verification scaffolding for government papers, email, and SMS checks.
//! Email and SMS checks stay inactive by default until `REQUIRE_EMAIL_VERIFICATION`
//! or `REQUIRE_SMS_VERIFICATION` is enabled in `Config`.

use crate::models::{ModelError, VerificationStatus};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationChannel {
    GovernmentPapers,
    Email,
    Sms,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerificationPolicy {
    pub require_government_papers: bool,
    pub email_check_active: bool,
    pub sms_check_active: bool,
}

impl Default for VerificationPolicy {
    fn default() -> Self {
        Self {
            require_government_papers: true,
            email_check_active: false,
            sms_check_active: false,
        }
    }
}

impl VerificationPolicy {
    #[must_use]
    pub const fn from_flags(email_check_active: bool, sms_check_active: bool) -> Self {
        Self {
            require_government_papers: true,
            email_check_active,
            sms_check_active,
        }
    }

    /// Returns verification channels that still block promotion for a user.
    #[must_use]
    pub fn pending_channels(
        self,
        status: VerificationStatus,
        email_verified: bool,
        phone_verified: bool,
    ) -> Vec<VerificationChannel> {
        let mut pending = Vec::new();
        if self.require_government_papers && status != VerificationStatus::Verified {
            pending.push(VerificationChannel::GovernmentPapers);
        }
        if self.email_check_active && !email_verified {
            pending.push(VerificationChannel::Email);
        }
        if self.sms_check_active && !phone_verified {
            pending.push(VerificationChannel::Sms);
        }
        pending
    }
}

pub fn validate_phone_number(value: &str) -> Result<String, ModelError> {
    let trimmed = value.trim();
    let digits = trimmed.strip_prefix('+').unwrap_or(trimmed);
    if !(7..=15).contains(&digits.len()) || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return Err(ModelError::Invalid(
            "phone_number must contain 7 to 15 digits",
        ));
    }
    Ok(trimmed.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_policy_keeps_email_and_sms_inactive() {
        let policy = VerificationPolicy::default();
        assert!(!policy.email_check_active);
        assert!(!policy.sms_check_active);
        assert_eq!(
            policy.pending_channels(VerificationStatus::Pending, false, false),
            vec![VerificationChannel::GovernmentPapers]
        );
        assert!(
            policy
                .pending_channels(VerificationStatus::Verified, false, false)
                .is_empty()
        );
    }

    #[test]
    fn enabled_flags_require_email_and_sms_channels() {
        let policy = VerificationPolicy::from_flags(true, true);
        assert_eq!(
            policy.pending_channels(VerificationStatus::Pending, false, false),
            vec![
                VerificationChannel::GovernmentPapers,
                VerificationChannel::Email,
                VerificationChannel::Sms,
            ]
        );
    }

    #[test]
    fn phone_number_validates_digits() {
        assert_eq!(validate_phone_number(" +905551112233 ").unwrap(), "+905551112233");
        for bad in ["", "abc", "+12", "1234567890123456"] {
            assert!(matches!(
                validate_phone_number(bad),
                Err(ModelError::Invalid(_))
            ));
        }
    }
}
