use crate::service::user_service::UserService;

pub struct ServiceManager {
 pub user: UserService
}

impl ServiceManager {
  pub fn new(user: UserService) -> Self {
      ServiceManager {user}
  }
}