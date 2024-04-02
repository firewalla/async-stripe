use serde::Serialize;

use crate::client::{Client, Response};
use crate::ids::SubscriptionId;
use crate::resources::{CreateSubscriptionItems, Subscription};

#[derive(Clone, Debug, Default, Serialize)]
pub struct CancelSubscription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_period_end: Option<bool>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct CancelSubscriptionNew {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prorate: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_now: Option<bool>,
}

impl CancelSubscription {
    pub fn new() -> CancelSubscription {
        CancelSubscription { at_period_end: None }
    }
}

impl Subscription {
    /// Cancels a subscription.
    ///
    /// For more details see <https://stripe.com/docs/api#cancel_subscription>.
    pub fn cancel(
        client: &Client,
        subscription_id: &SubscriptionId,
        params: CancelSubscription,
    ) -> Response<Subscription> {
        client.delete_query(&format!("/subscriptions/{}", subscription_id), params)
    }

    pub fn cancel_new(
        client: &Client,
        subscription_id: &SubscriptionId,
        params: CancelSubscriptionNew,
    ) -> Response<Subscription> {
        client.delete_query(&format!("/subscriptions/{}", subscription_id), params)
    }

}

impl CreateSubscriptionItems {
    pub fn new() -> Self {
        Default::default()
    }
}
