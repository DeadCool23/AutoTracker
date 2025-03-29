use crate::error::ServiceError;
use crate::services_traits::Authorizer;
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

impl Authorizer for AuthService {
    fn auth(&self, email: &String, pswd: &String) -> Result<User, ServiceError> {
        if !Validator::is_valid_email(email) || !Validator::is_valid_password(pswd) {
            return Err(ServiceError::InvalidDataError(
                "email or password".to_string(),
            ));
        }

        let res = self.user_repo.get_user_by_auth_info(email, pswd)?;
        if let Some(user) = res {
            Ok(user)
        } else {
            Err(ServiceError::InvalidDataError(
                "email or password".to_string(),
            ))
        }
    }

    fn register(
        &self,
        firstname: &String,
        surname: &String,
        lastname: &Option<String>,
        email: &String,
        pswd: &String,
        rep_pswd: &String,
    ) -> Result<(), ServiceError> {
        if !Validator::is_valid_email(email) {
            return Err(ServiceError::InvalidDataError("email".to_string()));
        }
        if !Validator::is_valid_password(pswd) {
            return Err(ServiceError::InvalidDataError("password".to_string()));
        }
        if pswd != rep_pswd {
            return Err(ServiceError::InvalidDataError(
                "passwords unmatch".to_string(),
            ));
        }

        match self.user_repo.get_user_by_email(email)? {
            Some(_) => return Err(ServiceError::IsExistError("email".to_string())),
            None => {
                self.user_repo.add_user(&User {
                    id: 0,
                    name: firstname.to_string(),
                    surname: surname.to_string(),
                    lastname: lastname.clone(),
                    email: email.to_string(),
                    password: pswd.to_string(),
                    role: Role::user,
                    is_verified: false,
                    passport: None,
                    cars: None,
                })?;
            }
        }
        Ok(())
    }

    fn passport_confirm(&self, email: &String, passport: &Document) -> Result<(), ServiceError> {
        if !Validator::is_valid_email(email) {
            return Err(ServiceError::InvalidDataError("email".to_string()));
        }
        if !Validator::is_valid_passport(passport) {
            return Err(ServiceError::InvalidDataError("passport".to_string()));
        }

        match self.user_repo.get_user_by_email(email)? {
            Some(_) => {
                self.user_repo.update_user_passport(email, passport)?;
            }
            None => return Err(ServiceError::NotFoundError("email".to_string())),
        }
        Ok(())
    }
}
