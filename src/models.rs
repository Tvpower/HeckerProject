use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Flower {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub name: String,
    pub hours: f64,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct NewFlower{
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub name: String,
    pub hours: f64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFlower{
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub name: Option<String>,
    pub hours: Option<f64>,
}