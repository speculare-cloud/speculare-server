use crate::errors::AppError;
use crate::ConnType;

use super::schema::{
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
use super::{
    CpuStatsDTO, CpuTimesDTO, DisksDTOList, HttpPostHost, IoBlockDTOList, IoNetDTOList, LoadAvgDTO,
    MemoryDTO, SwapDTO,
};

use diesel::*;
use serde::{Deserialize, Serialize};

// ========================
// DATABASE Specific struct
// ========================
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
        // If there is only one item, it's faster to only insert one
        // == avoid allocation of vector
        if items.len() == 1 {
            return Self::insert_one(conn, &items[0]);
        }
        // Even if this method (using Vec) use more memory, it prefer speed over low RAM usage.
        // For the first three (v_ncpuinfo, v_nloadavg, v_nmemory) we init them with a capacity
        // because in the best case, there will only be items.len() elements for each.
        // For v_ndisks and v_nioblocks we cannot predict the numbers of elements.
        //      On this last point, we can in fact. We can simply check the len of
        //        - items[0].disks.len()
        //        - items[0].iostats.len()
        //      But as they are Optional, we also have to check if they are .is_some()
        //      Doing so would be optimal to avoid allocations, and if they are .is_none()
        //      we could skip the construction of new_disks and new_ioblock, as well as their
        //      respective insert.
        let mut v_ncpustats: Vec<CpuStatsDTO> = Vec::with_capacity(items.len());
        let mut v_ncputimes: Vec<CpuTimesDTO> = Vec::with_capacity(items.len());
        let mut v_nloadavg: Vec<LoadAvgDTO> = Vec::with_capacity(items.len());
        let mut v_nmemory: Vec<MemoryDTO> = Vec::with_capacity(items.len());
        let mut v_nswap: Vec<SwapDTO> = Vec::with_capacity(items.len());
        let mut v_ndisks: DisksDTOList = Vec::new();
        let mut v_nioblocks: IoBlockDTOList = Vec::new();
        let mut v_nionets: IoNetDTOList = Vec::new();

        for item in items {
            // Construct the new Struct from item
            let new_data = Host::from(item);
            let new_cpustats = Option::<CpuStatsDTO>::from(item);
            let new_cputimes = Option::<CpuTimesDTO>::from(item);
            let new_loadavg = Option::<LoadAvgDTO>::from(item);
            let new_memory = Option::<MemoryDTO>::from(item);
            let new_swap = Option::<SwapDTO>::from(item);
            let mut new_disks = Option::<DisksDTOList>::from(item);
            let mut new_ioblock = Option::<IoBlockDTOList>::from(item);
            let mut new_ionet = Option::<IoNetDTOList>::from(item);

            // Add some result in their vec for BatchInsert
            if let Some(value_cpustats) = new_cpustats {
                v_ncpustats.push(value_cpustats);
            }
            if let Some(value_cputimes) = new_cputimes {
                v_ncputimes.push(value_cputimes);
            }
            if let Some(value_loadavg) = new_loadavg {
                v_nloadavg.push(value_loadavg);
            }
            if let Some(value_memory) = new_memory {
                v_nmemory.push(value_memory);
            }
            if let Some(value_swap) = new_swap {
                v_nswap.push(value_swap);
            }
            if let Some(value_disks) = new_disks.as_mut() {
                v_ndisks.append(value_disks);
            }
            if let Some(value_iostats) = new_ioblock.as_mut() {
                v_nioblocks.append(value_iostats);
            }
            if let Some(value_iocounters) = new_ionet.as_mut() {
                v_nionets.append(value_iocounters);
            }

            // Insert Host data, if conflict, only update uptime
            insert_into(hosts)
                .values(&new_data)
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
        // Construct the new Struct from item
        let new_data = Host::from(item);
        let new_cpustats = Option::<CpuStatsDTO>::from(item);
        let new_cputimes = Option::<CpuTimesDTO>::from(item);
        let new_loadavg = Option::<LoadAvgDTO>::from(item);
        let new_memory = Option::<MemoryDTO>::from(item);
        let new_swap = Option::<SwapDTO>::from(item);
        let new_disks = Option::<DisksDTOList>::from(item);
        let new_ioblock = Option::<IoBlockDTOList>::from(item);
        let new_ionet = Option::<IoNetDTOList>::from(item);

        // Insert Host data, if conflict, only update uptime
        insert_into(hosts)
            .values(&new_data)
            .on_conflict(uuid)
            .do_update()
            .set(uptime.eq(item.uptime))
            .execute(conn)?;
        // Only insert Option if they are present
        if let Some(value) = new_cpustats {
            insert_into(cpustats).values(&value).execute(conn)?;
        }
        if let Some(value) = new_cputimes {
            insert_into(cputimes).values(&value).execute(conn)?;
        }
        if let Some(value) = new_loadavg {
            insert_into(loadavg).values(&value).execute(conn)?;
        }
        if let Some(value) = new_memory {
            insert_into(memory).values(&value).execute(conn)?;
        }
        if let Some(value) = new_swap {
            insert_into(swap).values(&value).execute(conn)?;
        }
        if let Some(value) = new_disks {
            insert_into(disks).values(&value).execute(conn)?;
        }
        if let Some(value) = new_ioblock {
            insert_into(ioblocks).values(&value).execute(conn)?;
        }
        if let Some(value) = new_ionet {
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
