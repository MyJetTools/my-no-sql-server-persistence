use std::sync::Arc;

use my_http_server_controllers::controllers::ControllersMiddleware;

use crate::app::AppContext;

pub fn build(app: &Arc<AppContext>) -> ControllersMiddleware {
    let mut result = ControllersMiddleware::new();

    let api_controller = super::api::ApiController::new();
    result.register_get_action(Arc::new(api_controller));

    result.register_get_action(Arc::new(super::logs_controller::GetFatalErrorsAction::new(
        app.clone(),
    )));

    result.register_get_action(Arc::new(super::status_controller::StatusController::new(
        app.clone(),
    )));

    result.register_get_action(Arc::new(super::logs_controller::LogsByTableAction::new(
        app.clone(),
    )));

    result.register_get_action(Arc::new(super::logs_controller::LogsByProcessAction::new(
        app.clone(),
    )));

    result.register_get_action(Arc::new(super::logs_controller::HomeAction::new(
        app.clone(),
    )));

    result.register_get_action(Arc::new(super::home_controller::IndexAction::new(
        app.clone(),
    )));

    result.register_get_action(Arc::new(super::prometheus_controller::MetricsAction::new(
        app.clone(),
    )));

    result
}
