use crate::errors::AppError;
use crate::ConnType;

use crate::models::schema::{
    cpustats::dsl::*,
    cputimes::dsl::*,
    disks::dsl::*,
    hosts::{
        self,
        dsl::{hosts as dsl_host, uuid, *},
    },
    ioblocks::dsl::*,
    ionets::dsl::*,
    loadavg::dsl::*,
    memory::dsl::*,
    swap::dsl::*,
};
use crate::models::{
    CpuStatsDTO, CpuTimesDTO, DiskDTOList, HttpPostHost, IoBlockDTOList, IoNetDTOList, LoadAvgDTO,
    MemoryDTO, SwapDTO,
};

use diesel::*;
use serde::{Deserialize, Serialize};

/// DB Specific struct for hosts table
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name = "hosts"]
#[primary_key(uuid)]
pub struct Host {
    pub system: String,
    pub os_version: String,
    pub hostname: String,
    pub uptime: i64,
    pub uuid: String,
    pub created_at: chrono::NaiveDateTime,
}

impl Host {
    /// Insert the host data (update or create) (multiple value at once (Vec<HttpPostHost>))
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `items` - The Vec<HttpPostHost> we just got from the Post request (contains all our info)
    pub fn insert(conn: &ConnType, items: &[HttpPostHost]) -> Result<(), AppError> {
        let len = items.len();
        // If there is only one item, it's faster to only insert one to avoid allocation of vector
        if len == 1 {
            return Self::insert_one(conn, &items[0]);
        } else if len == 0 {
            return Ok(());
        }
        // Even if this method (using Vec) use more memory, it prefer speed over low RAM usage.
        let mut v_ncpustats: Vec<CpuStatsDTO> = Vec::with_capacity(len);
        let mut v_ncputimes: Vec<CpuTimesDTO> = Vec::with_capacity(len);
        let mut v_nloadavg: Vec<LoadAvgDTO> = Vec::with_capacity(len);
        let mut v_nmemory: Vec<MemoryDTO> = Vec::with_capacity(len);
        let mut v_nswap: Vec<SwapDTO> = Vec::with_capacity(len);
        // For these vector we can't predict the lenght of them, as a server/computer can have
        // a new net interfaces or disks at any time. So we create regular Vec that will grow if needed.
        let mut v_ndisks: DiskDTOList = Vec::new();
        let mut v_nioblocks: IoBlockDTOList = Vec::new();
        let mut v_nionets: IoNetDTOList = Vec::new();

        for item in items {
            // Only insert Option if they are present to their Vector
            if let Some(value_cpustats) = Option::<CpuStatsDTO>::from(item) {
                v_ncpustats.push(value_cpustats);
            }
            if let Some(value_cputimes) = Option::<CpuTimesDTO>::from(item) {
                v_ncputimes.push(value_cputimes);
            }
            if let Some(value_loadavg) = Option::<LoadAvgDTO>::from(item) {
                v_nloadavg.push(value_loadavg);
            }
            if let Some(value_memory) = Option::<MemoryDTO>::from(item) {
                v_nmemory.push(value_memory);
            }
            if let Some(value_swap) = Option::<SwapDTO>::from(item) {
                v_nswap.push(value_swap);
            }
            if let Some(value_disks) = Option::<DiskDTOList>::from(item).as_mut() {
                v_ndisks.append(value_disks);
            }
            if let Some(value_iostats) = Option::<IoBlockDTOList>::from(item).as_mut() {
                v_nioblocks.append(value_iostats);
            }
            if let Some(value_iocounters) = Option::<IoNetDTOList>::from(item).as_mut() {
                v_nionets.append(value_iocounters);
            }

            // Insert Host data, if conflict, only update uptime
            insert_into(hosts)
                .values(&Host::from(item))
                .on_conflict(uuid)
                .do_update()
                .set(uptime.eq(item.uptime))
                .execute(conn)?;
        }
        // Insert Vec of Table from the for loop in one call (66% faster)
        insert_into(cpustats).values(&v_ncpustats).execute(conn)?;
        insert_into(cputimes).values(&v_ncputimes).execute(conn)?;
        insert_into(loadavg).values(&v_nloadavg).execute(conn)?;
        insert_into(memory).values(&v_nmemory).execute(conn)?;
        insert_into(swap).values(&v_nswap).execute(conn)?;
        insert_into(disks).values(&v_ndisks).execute(conn)?;
        insert_into(ioblocks).values(&v_nioblocks).execute(conn)?;
        insert_into(ionets).values(&v_nionets).execute(conn)?;
        // If we reached this point, everything went well so return an empty Closure
        Ok(())
    }

    /// Insert the host data (update or create) (one value of HttpPostHost at a time)
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `item` - The Vec<HttpPostHost>[0] we just got from the Post request (contains all our info)
    pub fn insert_one(conn: &ConnType, item: &HttpPostHost) -> Result<(), AppError> {
        // Insert Host data, if conflict, only update uptime
        insert_into(hosts)
            .values(&Host::from(item))
            .on_conflict(uuid)
            .do_update()
            .set(uptime.eq(item.uptime))
            .execute(conn)?;
        // Only insert Option if they are present
        if let Some(value) = Option::<CpuStatsDTO>::from(item) {
            insert_into(cpustats).values(&value).execute(conn)?;
        }
        if let Some(value) = Option::<CpuTimesDTO>::from(item) {
            insert_into(cputimes).values(&value).execute(conn)?;
        }
        if let Some(value) = Option::<LoadAvgDTO>::from(item) {
            insert_into(loadavg).values(&value).execute(conn)?;
        }
        if let Some(value) = Option::<MemoryDTO>::from(item) {
            insert_into(memory).values(&value).execute(conn)?;
        }
        if let Some(value) = Option::<SwapDTO>::from(item) {
            insert_into(swap).values(&value).execute(conn)?;
        }
        if let Some(value) = Option::<DiskDTOList>::from(item) {
            insert_into(disks).values(&value).execute(conn)?;
        }
        if let Some(value) = Option::<IoBlockDTOList>::from(item) {
            insert_into(ioblocks).values(&value).execute(conn)?;
        }
        if let Some(value) = Option::<IoNetDTOList>::from(item) {
            insert_into(ionets).values(&value).execute(conn)?;
        }
        // If we reached this point, everything went well so return an empty Closure
        Ok(())
    }

    /// Return a Vector of Host
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `size` - The number of elements to fetch
    /// * `page` - How many items you want to skip (page * size)
    pub fn list_hosts(conn: &ConnType, size: i64, page: i64) -> Result<Vec<Self>, AppError> {
        Ok(dsl_host
            .limit(size)
            .offset(page * size)
            .order_by(hostname.asc())
            .load(conn)?)
    }
}

impl From<&HttpPostHost> for Host {
    fn from(item: &HttpPostHost) -> Host {
        Host {
            system: item.system.to_owned(),
            os_version: item.os_version.to_owned(),
            hostname: item.hostname.to_string(),
            uptime: item.uptime,
            uuid: item.uuid.to_string(),
            created_at: item.created_at,
        }
    }
}
