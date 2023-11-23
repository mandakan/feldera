fn main() {
    dbsp_adapters::server::server_main(|workers| {
        circuit(workers)
            .map(|(dbsp, catalog)| {
                (
                    Box::new(dbsp) as Box<dyn dbsp_adapters::DbspCircuitHandle>,
                    Box::new(catalog) as Box<dyn dbsp_adapters::CircuitCatalog>,
                )
            })
            .map_err(|e| dbsp_adapters::ControllerError::dbsp_error(e))
    })
        .unwrap_or_else(|e| {
            eprintln!("{e}");
            std::process::exit(1);
        });
}
