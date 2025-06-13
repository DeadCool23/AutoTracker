use crate::error::ServiceError;
use crate::services_traits::Authorizer;
use async_trait::async_trait;
use data_access::repositories_traits::UserRepository;
use models::{Document, Role, User};

use super::validator::Validator;

pub struct AuthService {
    user_repo: Box<dyn UserRepository>,
}

impl AuthService {
    pub fn from(user_repo: Box<dyn UserRepository>) -> Self {
        AuthService { user_repo }
    }
}

unsafe impl Send for AuthService {}
unsafe impl Sync for AuthService {}

#[async_trait]
impl Authorizer for AuthService {
    async fn auth(&self, email: &String, pswd: &String) -> Result<User, ServiceError> {
        log::info!("Attempting authentication for email: {}", email);

        if !Validator::is_valid_email(email) {
            log::warn!("Invalid email format: {}", email);
            return Err(ServiceError::InvalidDataError(
                "email or password".to_string(),
            ));
        }

        if !Validator::is_valid_password(pswd) {
            log::warn!("Invalid password format for email: {}", email);
            return Err(ServiceError::InvalidDataError(
                "email or password".to_string(),
            ));
        }

        let res = self.user_repo.get_user_by_auth_info(email, pswd).await?;

        match res {
            Some(user) => {
                log::info!("Successful authentication for email: {}", email);
                Ok(user)
            }
            None => {
                log::warn!("Failed authentication attempt for email: {}", email);
                Err(ServiceError::InvalidDataError(
                    "email or password".to_string(),
                ))
            }
        }
    }

    async fn register(
        &self,
        firstname: &String,
        surname: &String,
        lastname: Option<String>,
        email: &String,
        pswd: &String,
        rep_pswd: &String,
    ) -> Result<(), ServiceError> {
        log::info!("Starting registration process for email: {}", email);

        if !Validator::is_valid_email(email) {
            log::warn!("Invalid email format during registration: {}", email);
            return Err(ServiceError::InvalidDataError("email".to_string()));
        }

        if !Validator::is_valid_password(pswd) {
            log::warn!(
                "Invalid password format during registration for email: {}",
                email
            );
            return Err(ServiceError::InvalidDataError("password".to_string()));
        }

        if pswd != rep_pswd {
            log::warn!("Password mismatch during registration for email: {}", email);
            return Err(ServiceError::InvalidDataError(
                "passwords unmatch".to_string(),
            ));
        }

        match self.user_repo.get_user_by_email(email).await? {
            Some(_) => {
                log::warn!("Registration attempt with existing email: {}", email);
                Err(ServiceError::IsExistError("email".to_string()))
            }
            None => {
                let user = User {
                    name: firstname.to_string(),
                    surname: surname.to_string(),
                    lastname: lastname.clone(),
                    email: email.to_string(),
                    role: Role::user,
                    is_verified: false,
                    passport: None,
                };

                self.user_repo.insert_user(&user, pswd).await?;
                log::info!("Successfully registered new user: {}", email);
                Ok(())
            }
        }
    }

    async fn passport_confirm(
        &self,
        email: &String,
        passport: &Document,
    ) -> Result<(), ServiceError> {
        log::info!("Starting passport confirmation for email: {}", email);

        if !Validator::is_valid_email(email) {
            log::warn!(
                "Invalid email format during passport confirmation: {}",
                email
            );
            return Err(ServiceError::InvalidDataError("email".to_string()));
        }

        if !Validator::is_valid_passport(passport) {
            log::warn!("Invalid passport data for email: {}", email);
            return Err(ServiceError::InvalidDataError("passport".to_string()));
        }

        match self.user_repo.get_user_by_email(email).await? {
            Some(_) => {}
            None => {
                log::warn!(
                    "Passport confirmation attempt for non-existent user: {}",
                    email
                );
                return Err(ServiceError::NotFoundError("email".to_string()));
            }
        }

        match self.user_repo.get_user_by_passport(passport).await? {
            Some(_) => {
                log::warn!(
                    "Passport confirm attempt whith exiting passport: {:#?}",
                    passport
                );
                return Err(ServiceError::IsExistError("passport".to_string()));
            }
            None => {}
        }

        self.user_repo.update_user_passport(email, passport).await?;
        log::info!("Successfully updated passport for user: {}", email);
        Ok(())
    }
}
