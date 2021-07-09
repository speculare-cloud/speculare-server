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

{
	// TEST
	let alert = Alerts {
		name: "cpu_usage".into(),
		table: "cputimes".into(),
		lookup: "average 10m of cuser,nice,system,irq,softirq,steal over idle,iowait".into(),
		timing: 60,
		warn: "$this > 50".into(),
		crit: "$this > 80".into(),
		info: "average cpu utilization over the last 10 minutes".into(),
		host_uuid: "07c2ff2b28f01e93ef1ea6e311664a97facefba7".into(),
		where_clause: None,
	};

	let mut lookup_parts = alert.lookup.split(' ');

	let req_type = lookup_parts.next();
	let req_time = lookup_parts.next();
	let req_numerator = lookup_parts.nth(1);
	let req_divisor = lookup_parts.nth(1);
	dbg!(&req_type);
	dbg!(&req_time);
	dbg!(&req_numerator);
	dbg!(&req_divisor);

	let pg_agregate = match req_type {
		Some("average") => "avg",
		Some(&_) => {
			panic!("Unhandled aggregation function");
		}
		None => {
			panic!("Can't determine the aggreation function, no req_type");
		}
	};
	dbg!(&pg_agregate);

	let pg_time = match req_time {
		Some(v) => v,
		None => {
			panic!("Can't determine the time range, no req_time");
		}
	};
	dbg!(&pg_time);

	let mut pg_select = String::new();
	let select_cols = req_numerator.unwrap().split(',');
	for col in select_cols {
		pg_select.push_str(&format!("{}({})::int8 + ", pg_agregate, col));
	}
	pg_select.drain(pg_select.len() - 3..pg_select.len());
	pg_select.push_str(" as numerator, ");

	let select_cols = req_divisor.unwrap().split(',');
	for col in select_cols {
		pg_select.push_str(&format!("{}({})::int8 + ", pg_agregate, col));
	}
	pg_select.drain(pg_select.len() - 3..pg_select.len());
	pg_select.push_str(" as divisor");
	dbg!(&pg_select);

	let mut pg_where = String::new();
	if alert.where_clause.is_some() {
		pg_where.push_str(" AND ");
		pg_where.push_str(&alert.where_clause.unwrap());
	};

	let query = format!("SELECT time_bucket('{0}', created_at) as time, {1} FROM {2} WHERE host_uuid=$1 AND created_at > now() at time zone 'utc' - INTERVAL '{0}' {3} GROUP BY time ORDER BY time DESC", pg_time, pg_select, alert.table, pg_where);
	dbg!(&query);

	let tmp_query = query.to_lowercase();
	for statement in DISALLOWED_STATEMENT {
		assert!(!tmp_query.contains(statement));
	}

	let results = sql_query(query)
		.bind::<Text, _>(alert.host_uuid)
		.load::<PctDTORaw>(&pool.get().unwrap());
	dbg!(&results);

	let percentage = compute_percentage(&results.unwrap());
	dbg!(&percentage);

	let shound_warn = eval_boolean(&alert.warn.replace("$this", &percentage.to_string()));
	dbg!(&shound_warn);

	let shound_crit = eval_boolean(&alert.crit.replace("$this", &percentage.to_string()));
	dbg!(&shound_crit);
}