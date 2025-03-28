use rocket::serde::json::Json;

use crate::model::notification::Notification;
use crate::model::subscriber::SubcriberRequest;
use crate::service::notification::NotificationService;
use bambangshop_receiver::Result;
