use access_control::act::Action;
use actix_web::{
  Scope,
  web::{self, Data, Json, Path, Query},
};
use chrono::Utc;
use database::{
  resource_usage::get_workspace_usage_size,
  workspace::{
    select_all_user_non_guest_workspaces, select_workspace_member_count_from_workspace_id,
  },
};
use shared_entity::{
  dto::billing_dto::{
    Currency, RecurringInterval, SetSubscriptionRecurringInterval, SubscriptionCancelRequest,
    SubscriptionLinkRequest, SubscriptionPlan, SubscriptionPlanDetail, SubscriptionStatus,
    WorkspaceSubscriptionStatus, WorkspaceUsageAndLimit,
  },
  response::{AppResponse, JsonAppResponse},
};
use uuid::Uuid;

use crate::{biz::authentication::jwt::Authorization, state::AppState};

pub fn billing_scope() -> Scope {
  web::scope("/billing/api/v1")
    .service(web::resource("/customer-id").route(web::get().to(customer_id_handler)))
    .service(web::resource("/subscription-link").route(web::get().to(subscription_link_handler)))
    .service(
      web::resource("/cancel-subscription").route(web::post().to(cancel_subscription_handler)),
    )
    .service(
      web::resource("/subscription-status").route(web::get().to(list_subscription_status_handler)),
    )
    .service(
      web::resource("/portal-session-link").route(web::get().to(portal_session_link_handler)),
    )
    .service(
      web::resource("/subscription-status/{workspace_id}")
        .route(web::get().to(workspace_subscription_status_handler)),
    )
    .service(
      web::resource("/active-subscription/{workspace_id}")
        .route(web::get().to(active_workspace_subscriptions_handler)),
    )
    .service(
      web::resource("/subscription-recurring-interval")
        .route(web::post().to(set_subscription_recurring_interval_handler)),
    )
    .service(
      web::resource("/subscriptions").route(web::get().to(subscription_plan_details_handler)),
    )
}

async fn customer_id_handler(auth: Authorization) -> actix_web::Result<JsonAppResponse<String>> {
  let user_uuid = auth.uuid()?;
  let customer_id = format!("freeflowy-{}", user_uuid);
  Ok(AppResponse::Ok().with_data(customer_id).into())
}

async fn subscription_link_handler(
  auth: Authorization,
  query: Query<SubscriptionLinkRequest>,
  state: Data<AppState>,
) -> actix_web::Result<JsonAppResponse<String>> {
  let user_uuid = auth.uuid()?;
  let uid = state.user_cache.get_user_uid(&user_uuid).await?;
  let workspace_id = Uuid::parse_str(&query.workspace_id)
    .map_err(|e| app_error::AppError::InvalidRequest(e.to_string()))?;
  state
    .workspace_access_control
    .enforce_action(&uid, &workspace_id, Action::Read)
    .await?;

  let redirect_url = if query.success_url.is_empty() {
    state.config.appflowy_web_url.clone()
  } else {
    query.success_url.clone()
  };
  Ok(AppResponse::Ok().with_data(redirect_url).into())
}

async fn cancel_subscription_handler(
  auth: Authorization,
  payload: Json<SubscriptionCancelRequest>,
  state: Data<AppState>,
) -> actix_web::Result<JsonAppResponse<()>> {
  let user_uuid = auth.uuid()?;
  let uid = state.user_cache.get_user_uid(&user_uuid).await?;
  let workspace_id = Uuid::parse_str(&payload.workspace_id)
    .map_err(|e| app_error::AppError::InvalidRequest(e.to_string()))?;
  state
    .workspace_access_control
    .enforce_action(&uid, &workspace_id, Action::Read)
    .await?;

  Ok(AppResponse::Ok().into())
}

async fn list_subscription_status_handler(
  auth: Authorization,
  state: Data<AppState>,
) -> actix_web::Result<JsonAppResponse<Vec<WorkspaceSubscriptionStatus>>> {
  let user_uuid = auth.uuid()?;
  let workspaces = select_all_user_non_guest_workspaces(&state.pg_pool, &user_uuid).await?;
  let subscription_statuses = workspaces
    .into_iter()
    .map(|workspace| make_workspace_subscription_status(workspace.workspace_id))
    .collect();
  Ok(AppResponse::Ok().with_data(subscription_statuses).into())
}

async fn portal_session_link_handler(
  _auth: Authorization,
  state: Data<AppState>,
) -> actix_web::Result<JsonAppResponse<String>> {
  Ok(
    AppResponse::Ok()
      .with_data(state.config.appflowy_web_url.clone())
      .into(),
  )
}

async fn workspace_subscription_status_handler(
  auth: Authorization,
  workspace_id: Path<Uuid>,
  state: Data<AppState>,
) -> actix_web::Result<JsonAppResponse<Vec<WorkspaceSubscriptionStatus>>> {
  let user_uuid = auth.uuid()?;
  let uid = state.user_cache.get_user_uid(&user_uuid).await?;
  let workspace_id = workspace_id.into_inner();
  state
    .workspace_access_control
    .enforce_action(&uid, &workspace_id, Action::Read)
    .await?;

  Ok(
    AppResponse::Ok()
      .with_data(vec![make_workspace_subscription_status(workspace_id)])
      .into(),
  )
}

async fn active_workspace_subscriptions_handler(
  auth: Authorization,
  workspace_id: Path<Uuid>,
  state: Data<AppState>,
) -> actix_web::Result<JsonAppResponse<Vec<SubscriptionPlan>>> {
  let user_uuid = auth.uuid()?;
  let uid = state.user_cache.get_user_uid(&user_uuid).await?;
  let workspace_id = workspace_id.into_inner();
  state
    .workspace_access_control
    .enforce_action(&uid, &workspace_id, Action::Read)
    .await?;

  Ok(
    AppResponse::Ok()
      .with_data(vec![
        SubscriptionPlan::Team,
        SubscriptionPlan::AiMax,
        SubscriptionPlan::AiLocal,
      ])
      .into(),
  )
}

async fn set_subscription_recurring_interval_handler(
  auth: Authorization,
  payload: Json<SetSubscriptionRecurringInterval>,
  state: Data<AppState>,
) -> actix_web::Result<JsonAppResponse<()>> {
  let user_uuid = auth.uuid()?;
  let uid = state.user_cache.get_user_uid(&user_uuid).await?;
  let workspace_id = Uuid::parse_str(&payload.workspace_id)
    .map_err(|e| app_error::AppError::InvalidRequest(e.to_string()))?;
  state
    .workspace_access_control
    .enforce_action(&uid, &workspace_id, Action::Read)
    .await?;

  Ok(AppResponse::Ok().into())
}

async fn subscription_plan_details_handler(
  _auth: Authorization,
) -> actix_web::Result<JsonAppResponse<Vec<SubscriptionPlanDetail>>> {
  let plans = vec![
    SubscriptionPlanDetail {
      currency: Currency::USD,
      price_cents: 0,
      recurring_interval: RecurringInterval::Month,
      plan: SubscriptionPlan::Pro,
    },
    SubscriptionPlanDetail {
      currency: Currency::USD,
      price_cents: 0,
      recurring_interval: RecurringInterval::Year,
      plan: SubscriptionPlan::Pro,
    },
    SubscriptionPlanDetail {
      currency: Currency::USD,
      price_cents: 0,
      recurring_interval: RecurringInterval::Month,
      plan: SubscriptionPlan::Team,
    },
    SubscriptionPlanDetail {
      currency: Currency::USD,
      price_cents: 0,
      recurring_interval: RecurringInterval::Year,
      plan: SubscriptionPlan::Team,
    },
    SubscriptionPlanDetail {
      currency: Currency::USD,
      price_cents: 0,
      recurring_interval: RecurringInterval::Month,
      plan: SubscriptionPlan::AiMax,
    },
    SubscriptionPlanDetail {
      currency: Currency::USD,
      price_cents: 0,
      recurring_interval: RecurringInterval::Month,
      plan: SubscriptionPlan::AiLocal,
    },
  ];
  Ok(AppResponse::Ok().with_data(plans).into())
}

pub(crate) async fn workspace_usage_and_limit_handler(
  auth: Authorization,
  workspace_id: Path<Uuid>,
  state: Data<AppState>,
) -> actix_web::Result<JsonAppResponse<WorkspaceUsageAndLimit>> {
  let user_uuid = auth.uuid()?;
  let uid = state.user_cache.get_user_uid(&user_uuid).await?;
  let workspace_id = workspace_id.into_inner();
  state
    .workspace_access_control
    .enforce_action(&uid, &workspace_id, Action::Read)
    .await?;

  let member_count = select_workspace_member_count_from_workspace_id(&state.pg_pool, &workspace_id)
    .await?
    .unwrap_or_default();
  let storage_bytes = get_workspace_usage_size(&state.pg_pool, &workspace_id).await?;
  let usage = WorkspaceUsageAndLimit {
    member_count,
    member_count_limit: i64::MAX,
    storage_bytes: i64::try_from(storage_bytes).unwrap_or(i64::MAX),
    storage_bytes_limit: i64::MAX,
    storage_bytes_unlimited: true,
    single_upload_limit: i64::MAX,
    single_upload_unlimited: true,
    ai_responses_count: 0,
    ai_responses_count_limit: i64::MAX,
    ai_image_responses_count: 0,
    ai_image_responses_count_limit: i64::MAX,
    local_ai: true,
    ai_responses_unlimited: true,
  };
  Ok(AppResponse::Ok().with_data(usage).into())
}

fn make_workspace_subscription_status(workspace_id: Uuid) -> WorkspaceSubscriptionStatus {
  WorkspaceSubscriptionStatus {
    workspace_id: workspace_id.to_string(),
    workspace_plan: SubscriptionPlan::Team,
    recurring_interval: RecurringInterval::Month,
    subscription_status: SubscriptionStatus::Active,
    subscription_quantity: 9999,
    cancel_at: None,
    current_period_end: Utc::now().timestamp() + 315_360_000,
  }
}
