fn main() {
    // If this is the actix-server handling all alerts related things, here's a pseudo code:
    //
    //  - Do all the basics check, initialize logger and all

    // Load env variable from .env
    dotenv::dotenv().ok();
    // Init the logger and set the debug level correctly
    sproot::configure_logger();

    //  - Connect to the database (using Diesel ?)
    //  - Setup a pool of connection using r2d2
    //  - Start an async task which will loop forever
    //      - In that loop we'll get all alerts
    //          - For all those alerts we'll perform the check needed
    //          - If an incident is already ongoing for that alert
    //              - We're analyzing the result and determine if the incident is finished
    //                  - If the incident is finished we update it with Diesel setting end date
    //                  - We'll also in (in the future) send an email containing all the information during that range time (cpu, memory, disks, net, ...)
    //          - Else analyze the result and determine if we need to trigger an incident
    //              - If new incident we create it with Diesel and email/... to recipients
    //          - After all the works we just "sleep" for the next interval (which can be configured ?)[a lower interval will be more resource intensive but will give better reaction]
    //  - Start the actix server with different routes
    //      - A route to create new alerts
    //      - A route to update existing alerts (modify, pause, ...)
    //      - A route to get all alerts
    //      - A route to delete alerts
    println!("zpour");
}
