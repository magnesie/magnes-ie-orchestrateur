mod services;
mod jobs_buffer;
mod orchestrator;
mod clusters;
mod ssh_client;
mod meteo_service;

use meteo_service::MeteoClient;

use std::sync::{Arc, RwLock};

use services::{PhotogrammetryService, ImageStorageService, ServicesKeeper, ServiceAccessInformation};
use jobs_buffer::{JobsBuffer};
use orchestrator::*;

fn main() -> Result<(), String>{
    let services_keeper = Arc::new(RwLock::new(ServicesKeeper::new()));
    let jobs_buffer = Arc::new(RwLock::new(JobsBuffer::new()));

    let image_storage_service = ImageStorageService::new(services_keeper.clone())?;
    let input_access_info = ServiceAccessInformation::new(
        "localhost",
        7880,
        "",
        "",
    );
    services_keeper.write().unwrap().register_service("image storage", input_access_info);

    let photogrammetry_service = PhotogrammetryService::new(services_keeper.clone())?;

    let orchestrator = Orchestrator::new(
        10,
        86400,
        jobs_buffer.clone(),
        image_storage_service.clone(),
        photogrammetry_service.clone()
    );
    orchestrator.start();

    // println!("{}",&cluster.has_green_energy_available());
    Ok(())
}
