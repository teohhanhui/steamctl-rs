use steamguard::protobufs::steammessages_auth_steamclient::EAuthSessionGuardType;

#[derive(Clone, Debug)]
pub struct AllowedConfirmation {
    pub confirmation_type: EAuthSessionGuardType,
    pub associated_message: String,
}
