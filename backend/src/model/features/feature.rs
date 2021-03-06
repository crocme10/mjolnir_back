use super::{background, scenario};
use crate::{error, gql};
use chrono::prelude::*;
use futures::stream::{self, TryStreamExt};
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};
use slog::debug;
use snafu::ResultExt;
use sqlx::{
    postgres::{PgQueryAs, PgRow},
    row::{FromRow, Row},
};
use uuid::Uuid;

#[derive(Debug, PartialEq, Serialize, Deserialize, GraphQLObject)]
pub struct Feature {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl<'c> FromRow<'c, PgRow<'c>> for Feature {
    fn from_row(row: &PgRow<'c>) -> Result<Self, sqlx::Error> {
        Ok(Feature {
            id: row.get(0),
            name: row.get(1),
            description: row.get(2),
            tags: row.get(3),
            created_at: row.get(4),
            updated_at: row.get(5),
        })
    }
}

pub async fn fetch_all_features(context: &gql::Context) -> Result<Vec<Feature>, error::Error> {
    debug!(context.logger, "Retrieving all features");
    // We select everything except search which is a created field.
    sqlx::query_as("SELECT id, name, description, tags, created_at, updated_at FROM main.features")
        .fetch_all(&context.pool)
        .await
        .map(Into::<Vec<Feature>>::into)
        .context(error::DBError {
            details: "Could not retrieve features",
        })
}

// I'm not sure this function is useful, i've implemented it for testing loading -> fetching.
pub async fn fetch_feature_by_id(
    id: Uuid,
    context: &gql::Context,
) -> Result<Feature, error::Error> {
    debug!(context.logger, "Fetching feature with id '{}'", id);
    // We select everything except search which is a created field.
    sqlx::query_as(
        "SELECT id, name, description, tags, created_at, updated_at FROM main.features WHERE id=$1",
    )
    .bind(id)
    .fetch_one(&context.pool)
    .await
    .context(error::DBError {
        details: "Could not retrieve features",
    })
}

pub async fn create_or_replace_feature(
    name: String,
    description: String,
    tags: Vec<String>,
    context: &gql::Context,
) -> Result<Feature, error::Error> {
    debug!(context.logger, "Creating or Replacing Feature '{}'", name);
    sqlx::query_as("SELECT * FROM main.create_or_replace_feature($1, $2, $3)")
        .bind(name.clone())
        .bind(description)
        .bind(tags)
        .fetch_one(&context.pool)
        .await
        .context(error::DBError {
            details: format!("Could not create or replace feature '{}'", name),
        })
}

pub async fn create_or_replace_feature_from_string(
    feature: String,
    context: &gql::Context,
) -> Result<Feature, error::Error> {
    debug!(context.logger, "Creating or Replacing Feature from string");

    let feature = gherkin_rust::Feature::parse(feature).context(error::GherkinError {
        details: String::from("Could not parse feature"),
    })?;

    create_or_replace_feature_from_gherkin(feature, context).await
}

pub async fn create_or_replace_feature_from_gherkin(
    feature: gherkin_rust::Feature,
    context: &gql::Context,
) -> Result<Feature, error::Error> {
    debug!(context.logger, "Creating or Replacing Feature from gherkin");

    let res: Feature = sqlx::query_as("SELECT * FROM main.create_or_replace_feature($1, $2, $3)")
        .bind(feature.name)
        .bind(feature.description.unwrap_or(String::from("")))
        .bind(feature.tags)
        .fetch_one(&context.pool)
        .await
        .context(error::DBError {
            details: format!("Could not create or replace feature"),
        })?;

    let id = res.id;

    if let Some(background) = feature.background {
        let _background =
            background::create_or_replace_background_from_gherkin(background, &id, context).await?;
    }
    // Here we're turning the feature's scenarios into a stream of Result<Scenario, _>, on
    // which we can use try_for_each and insert them in the database
    stream::iter(feature.scenarios.into_iter().map(|scenario| Ok(scenario)))
        .try_for_each(|scenario| async {
            let _scenario =
                scenario::create_or_replace_scenario_from_gherkin(scenario, &id, context).await?;
            Ok(())
        })
        .await?;

    Ok(res)
}

pub async fn delete_feature_by_id(
    id: Uuid,
    context: &gql::Context,
) -> Result<Feature, error::Error> {
    debug!(context.logger, "Deleting feature with id '{}'", id);
    // We select everything except search which is a created field.
    sqlx::query_as("SELECT * FROM main.delete_feature($1)")
        .bind(id)
        .fetch_one(&context.pool)
        .await
        .context(error::DBError {
            details: "Could not delete feature",
        })
}
