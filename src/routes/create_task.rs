use actix_web::{web, HttpResponse};
use serde::Deserialize;
#[derive(Deserialize, Clone, Copy, Debug)]
pub enum Lang {
    Rust,
    Csharp,
}

#[derive(Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    Check,
    Test,
    Build,
    Deploy,
}

#[derive(Deserialize, Clone, Copy, Debug)]
pub enum Output {
    StorageAccount,
    AzureContainerRegistry,
}

#[derive(Deserialize, Debug)]
pub struct Task {
    lang: Lang,
    steps: Vec<Step>,
    source: String,
    output: Option<Output>,
}

fn is_building(steps: &Vec<Step>) -> bool {
    for step in steps {
        if *step == Step::Build {
            return true;
        }
    }
    false
}

pub async fn create_task(task: web::Json<Task>) -> HttpResponse {
    // validate edge_cases
    let lang = task.lang;
    let steps = task.steps.clone();
    let repository = task.source.clone();
    let output = task.output;

    if is_building(&steps) && output.is_none() {
        HttpResponse::BadRequest()
            .append_header(("Error", "Building with no output"))
            .finish()
    } else if !is_building(&steps) && output.is_some() {
        HttpResponse::BadRequest()
        .append_header(("Error", "Output will not be used because you are not building, remove the build step and add test and/or check if you want to validate the code without deploying it"))
        .finish()
    } else {
        HttpResponse::Ok().finish()
    }
}
