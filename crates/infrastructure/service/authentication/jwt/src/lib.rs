use ca_application::gateway::service::authentication::{
    AuthenticationService, AuthenticationServiceError, Claims,
};

#[allow(dead_code)]
struct JwtAuthenticationService {
    secret: String,
}

impl JwtAuthenticationService {
    #[allow(dead_code)]
    pub fn new(secret: String) -> Self {
        JwtAuthenticationService { secret }
    }
}

impl AuthenticationService for JwtAuthenticationService {
    fn authenticate(&self, token: String) -> Result<Claims, AuthenticationServiceError> {
        let secret = self.secret.as_bytes();
        let token_data = jsonwebtoken::decode::<Claims>(
            &token,
            &jsonwebtoken::DecodingKey::from_secret(secret),
            &jsonwebtoken::Validation::default(),
        )
        .map_err(|_| AuthenticationServiceError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ca_application::gateway::service::authentication::AuthenticationServiceError;
    use jsonwebtoken::{encode, Header};

    #[test]
    fn test_authentication_service() {
        let secret = "my_secret_key".to_string();
        let service = JwtAuthenticationService::new(secret.clone());

        let claims = Claims {
            sub: "user_id".to_string(),
            exp: 10000000000,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
        )
        .unwrap();

        let result = service.authenticate(token);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().sub, claims.sub);
    }
}
