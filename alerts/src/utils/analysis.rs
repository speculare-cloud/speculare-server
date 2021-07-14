use super::{query::execute_query, QueryType};

use evalexpr::*;
use sproot::{models::Alerts, ConnType};

/// This function is the core of the monitoring, this is where we:
/// - Execute the query and get the result
/// - Evaluate if we need to trigger an incidents or not
pub fn execute_analysis(query: &str, alert: &Alerts, qtype: &QueryType, conn: &ConnType) {
    // Execute the query passed as arguement (this query was build previously)
    let result = execute_query(query, &alert.host_uuid, qtype, conn);
    trace!("{}", &result);

    // Determine if we are in a Warn or Crit level of incidents
    let should_warn = eval_boolean(&alert.warn.replace("$this", &result));
    let should_crit = eval_boolean(&alert.crit.replace("$this", &result));
    trace!("{:?}, {:?}", should_warn, should_crit);
}
