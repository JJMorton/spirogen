pub mod maths;
pub mod shapes;
pub mod wheels;

use axum::{
    extract::Query, response::Json, routing::get, Router
};
use maths::Coordinate;
use serde::{Deserialize, Serialize};
use shapes::{Circle, ParametricShape, Rod};
use std::f64::consts::PI;
use wheels::{transform_for_pen, transform_for_wheel};


/// A response indicating that there was an error
#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

/// A response containing a complete pattern
#[derive(Serialize)]
struct PatternResponse {
    points: Vec<Coordinate>,
}

/// The query parameters required to create a pattern
#[derive(Serialize, Deserialize, Debug)]
struct PatternQuery {
    guide: ShapeType,
    wheel: ShapeType,
    guide_radius: f64,
    wheel_radius: f64,
    pen_radius: f64,
    pen_theta: f64,
    guide_param: Option<f64>,
    wheel_param: Option<f64>,
    inside: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
enum ShapeType {
    Circle,
    Rod,
}

impl ShapeType {
    pub fn needs_param(&self) -> bool {
        match self {
            Self::Circle => false,
            Self::Rod => true,
        }
    }
    pub fn to_shape(&self, radius: f64, param: f64) -> Box<dyn ParametricShape> {
        return match self {
            ShapeType::Circle => Box::new(Circle::new(radius)),
            ShapeType::Rod => Box::new(Rod::new(radius, param)),
        }
    }
}


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(route_help))
        .route("/pattern", get(route_pattern));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn route_help() -> String {
    concat!(
        "SPIROGEN API\n",
        "\n",
        "GET / This help page\n",
        "\n",
        "GET /pattern Get the points resulting from a pair of wheels\n",
        "\t       ?guide=<Shape>\n",
        "\t       &wheel=<Shape>\n",
        "\t&guide_radius=<radius>\n",
        "\t&wheel_radius=<radius>\n",
        "\t  &pen_radius=<radius in 0-1>\n",
        "\t   &pen_theta=<angle in radians>\n",
        "\t &guide_param=[additional parameter]\n",
        "\t &wheel_param=[addditional parameter]\n",
        "\t      &inside=[true/false default false]\n",
    ).to_owned()
}

async fn route_pattern(
    Query(params): Query<PatternQuery>
) -> Result<Json<PatternResponse>, Json<ErrorResponse>> {

    // Check for shapes which require a parameter
    if params.guide_param.is_none() && params.guide.needs_param() {
        return Err(Json(ErrorResponse{
            message: format!("guide type {:?} requires guide_param", params.guide).to_owned()
        }))
    }
    if params.wheel_param.is_none() && params.wheel.needs_param() {
        return Err(Json(ErrorResponse{
            message: format!("wheel type {:?} requires wheel_param", params.wheel).to_owned()
        }))
    }

    // Check for negative lengths
    if params.guide_radius <= 0.0 || params.wheel_radius <= 0.0 {
        return Err(Json(ErrorResponse{
            message: "non-positive radius supplied".to_owned()
        }))
    }
    if params.guide_param.unwrap_or(1.0) <= 0.0 || params.wheel_param.unwrap_or(1.0) <= 0.0 {
        return Err(Json(ErrorResponse{
            message: "non-positive shape parameter supplied".to_owned()
        }))
    }

    // Check the pen's parameters
    if params.pen_radius < 0.0 || params.pen_radius > 1.0 {
        return Err(Json(ErrorResponse{
            message: "pen_radius is outside the range [0, 1]".to_owned()
        }))
    }
    if params.pen_theta < 0.0 || params.pen_theta > 2.0 * PI {
        return Err(Json(ErrorResponse{
            message: "pen_theta is outside the range [0, 2PI]".to_owned()
        }))
    }

    // Construct the guide and wheel shapes
    let guide = params.guide.to_shape(
        params.guide_radius,
        params.guide_param.unwrap_or(1.0)
    );
    let wheel = params.wheel.to_shape(
        params.wheel_radius,
        params.wheel_param.unwrap_or(1.0)
    );

    // Check that the wheel is compatible with the guide
    let inside = params.inside.unwrap_or(false);
    if inside && wheel.max_radius() > guide.min_radius() {
        return Err(Json(ErrorResponse{
            message: "wheel does not fit inside guide".to_owned()
        }))
    }

    // Ok, construct the pattern!
    let mut points: Vec<Coordinate> = Vec::new();
    for i in 0..300 {
        let s = guide.perimeter() * 0.01 * (i as f64);
        let trans_wheel = transform_for_wheel(&*wheel, &*guide, inside, s);
        let trans_pen = transform_for_pen(&*wheel, params.pen_theta, params.pen_radius);
        points.push(trans_wheel * trans_pen * Coordinate::null());
    }

    Ok(Json(PatternResponse{
        points,
    }))

}
