// Copyright (c) Facebook, Inc. and its affiliates.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use anyhow::{bail, Error, Result};
use regex::Regex;
use std::str::FromStr;
use structopt::StructOpt;

// make_option macro will build a enum of tags that map to string values by
// implementing the FromStr trait.
// This is useful when are trying to processing or display fields base on
// a user's input. Here's a use case:
// We display fields in the order of user's input. After we got
// the input array, dfill trait will automatically generate a vec of fns base
// on that array. For example, user input `--fields cpu_usage cpu_user`,
// enum generated by make_option will auto translate string to enum tags. After
// that dfill trait will gerenate `vec![print_cpu_usage, print_cpu_user]`. And
// the dprint trait will just iterate over the fns and call it with current model.
//
// Another user case is for the select feature, we don't want a giant match
// of string patterns once user select some field to do some operations. Instead,
// we can use a match of enum tags, that will be much faster.
macro_rules! make_option {
    ($name:ident {$($str_field:tt: $enum_field:ident,)*}) => {
        #[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
        pub enum $name {
            $($enum_field,)*
        }

        impl FromStr for $name {
            type Err = Error;

            fn from_str(opt: &str) -> Result<Self> {
                match opt.to_lowercase().as_str() {
                    $($str_field => Ok($name::$enum_field),)*
                    _ => bail!("Fail to parse {}", opt)
                }
            }
        }
    }
}

make_option! (SysField {
    "timestamp": Timestamp,
    "datetime": Datetime,
    "cpu": Cpu,
    "mem": Mem,
    "io": Io,
    "hostname": Hostname,
    "cpu_usage": CpuUsagePct,
    "cpu_user": CpuUserPct,
    "cpu_system": CpuSystemPct,
    "mem_total": MemTotal,
    "mem_free": MemFree,
    "mem_anon": MemAnon,
    "mem_file": MemFile,
    "huge_page_total": HpTotal,
    "huge_page_free": HpFree,
    "io_read": IoRead,
    "io_write": IoWrite,
});

make_option! (ProcField {
    "timestamp": Timestamp,
    "datetime": Datetime,
    "io": Io,
    "mem": Mem,
    "cpu": Cpu,
    "pid": Pid,
    "ppid": Ppid,
    "comm": Comm,
    "state": State,
    "uptime": Uptime,
    "cgroup": Cgroup,
    "cpu_user": CpuUserPct,
    "cpu_sys": CpuSysPct,
    "cpu_threads": CpuNumThreads,
    "cpu_total": CpuTotalPct,
    "mem_rss": MemRssBytes,
    "mem_minorfaults": MemMinor,
    "mem_majorfaults": MemMajor,
    "io_read": IoRead,
    "io_write": IoWrite,
    "io_total": IoTotal,
});

make_option! (CgroupField {
    "timestamp": Timestamp,
    "datetime": Datetime,
    "cpu": Cpu,
    "mem": Mem,
    "io": Io,
    "pressure": Pressure,
    "name": Name,
    "full_path": FullPath,
    "cpu_usage": CpuUsage,
    "cpu_user": CpuUser,
    "cpu_system": CpuSystem,
    "cpu_nr_periods": CpuNrPeriods,
    "cpu_nr_throttled": CpuNrThrottled,
    "cpu_throttled": CpuThrottled,
    "mem_total": MemTotal,
    "mem_swap": MemSwap,
    "mem_anon": MemAnon,
    "mem_file": MemFile,
    "mem_kernel": MemKernel,
    "mem_slab": MemSlab,
    "mem_sock": MemSock,
    "mem_shmem": MemShem,
    "mem_file_mapped": MemFileMapped,
    "mem_file_dirty": MemFileDirty,
    "mem_file_writeback": MemFileWriteBack,
    "mem_anon_thp": MemAnonThp,
    "mem_inactive_anon": MemInactiveAnon,
    "mem_active_anon": MemActiveAnon,
    "mem_inactive_file": MemInactiveFile,
    "mem_active_file": MemActiveFile,
    "mem_unevictable": MemUnevictable,
    "mem_slab_reclaimable": MemSlabReclaimable,
    "mem_slab_unreclaimable": MemSlabUnreclaimable,
    "mem_pgfault": Pgfault,
    "mem_pgmajfault": MemPgmajfault,
    "mem_workingset_refault": MemWorkingsetRefault,
    "mem_workingset_activate": MemWorkingsetActivate,
    "mem_workingset_nodereclaim": MemWorkingsetNodereclaim,
    "mem_pgrefill": MemPgrefill,
    "mem_pgscan": MemPgscan,
    "mem_pgsteal": MemPgsteal,
    "mem_pgactivate": MemPgactivate,
    "mem_pgdeactivate": MemPgdeactivate,
    "mem_pglazyfree": MemPglazyfree,
    "mem_pglazyfreed": MemPglazyfreed,
    "mem_thp_fault_alloc": MemTHPFaultAlloc,
    "mem_thp_collapse_alloc": MemTHPCollapseAlloc,
    "io_read": IoRead,
    "io_write": IoWrite,
    "io_rios": IoRiops,
    "io_wios": IoWiops,
    "io_dbps": IoDbps,
    "io_diops": IoDiops,
    "io_total": IoTotal,
    "pressure_cpu_some": CpuSome,
    "pressure_io_some": IoSome,
    "pressure_io_full": IoFull,
    "pressure_mem_full": MemFull,
    "pressure_mem_some": MemSome,
});

make_option! (OutputFormat {
    "raw": Raw,
    "csv": Csv,
    "json": Json,
    "kv": KeyVal,
});

#[derive(Debug, StructOpt, Default, Clone)]
pub struct GeneralOpt {
    /// Show all top layer fields. If --default is specified, it overrides any specified fields via --fields.
    #[structopt(long)]
    pub default: bool,
    /// Show all fields. If --everything is specified, --fields and --default are overridden.
    #[structopt(long)]
    pub everything: bool,
    /// Show more infomation other than default.
    #[structopt(short, long)]
    pub detail: bool,
    /// Begin time, same format as replay
    #[structopt(long, short)]
    pub begin: String,
    /// End time, same format as replay
    #[structopt(long, short)]
    pub end: Option<String>,
    /// Take a regex and apply to --select selected field. See command level doc for example.
    #[structopt(long, short = "F")]
    pub filter: Option<Regex>,
    /// Sort (lower to higher) by --select selected field. See command level doc for example.
    #[structopt(long)]
    pub sort: bool,
    /// Sort (higher to lower) by --select selected field. See command level doc for example.
    #[structopt(long)]
    pub rsort: bool,
    // display top N field. See command level doc for example.
    #[structopt(long, default_value = "0")]
    pub top: u32,
    /// Repeat title, for each N line, it will render a line of title. Only for raw output format.
    #[structopt(long = "repeat-title")]
    pub repeat_title: Option<usize>,
    /// Output format. Choose from raw, csv, kv, json. Default to raw
    #[structopt(long, short = "O")]
    pub output_format: Option<OutputFormat>,
    /// Output destination, default to stdout.
    #[structopt(long, short)]
    pub output: Option<String>,
}

#[derive(Debug, StructOpt, Clone)]
pub enum DumpCommand {
    /// Dump system stats
    ///
    /// ********************** Available fields **********************
    ///
    /// timestamp, datetime, hostname
    ///
    /// cpu_usage, cpu_user, cpu_system
    ///
    /// mem_total, mem_free, mem_anon, mem_file, huge_page_total, huge_page_free
    ///
    /// io_read, io_write
    ///
    /// ********************** Aggregated fields **********************
    ///
    /// * cpu: includes [cpu_usage, cpu_user, cpu_system]
    ///
    /// * mem: includes [mem_total, mem_free]. Additionally includes [mem_anon, mem_file,
    ///   huge_page_total, huge_page_free] if --detail is specified.
    ///
    /// * io: includes [io_read, io_write]
    ///
    /// --default will have all of [cpu, mem, io]. To display everything, use --everything.
    ///
    /// ********************** Example Commands **********************
    ///
    /// $ below dump system -b "08:30:00" -e "08:30:30" -f datetime io hostname -O csv
    ///
    /// $ below dump system -b "08:30:00" -e "08:30:30" -f datetime -O csv -f hostname -f io
    System {
        /// Select which fields to display and in what order.
        #[structopt(short, long)]
        fields: Option<Vec<SysField>>,
        #[structopt(flatten)]
        opts: GeneralOpt,
    },
    /// Dump process stats
    ///
    /// ********************** Available fields **********************
    ///
    /// timestamp, datetime, pid, ppid, comm, state, uptime, cgroup
    ///
    /// cpu_user, cpu_sys, cpu_threads, cpu_total
    ///
    /// mem_rss, mem_minorfaults, mem_majorfaults
    ///
    /// io_read, io_write, io_total
    ///
    /// ********************** Aggregated fields **********************
    ///
    /// * cpu: includes [cpu_total]. Additionally includes [cpu_user, cpu_sys, cpu_threads] if --detail specified
    ///
    /// * mem: includes [mem_rss]. Addtionally includes [mem_minorfaults, mem_majorfaults] if --detail specified
    ///
    /// * io: includes [io_read, io_write]. Addtionally includes[io_total] -if --detail specified
    ///
    /// --default will have all of [pid, comm, cpu, mem, io]. To display everything, use --everything.
    ///
    /// ********************** Example Commands **********************
    ///
    /// Simple example:
    ///
    /// $ below dump process -b "08:30:00" -e "08:30:30" -f comm cpu io_total -O csv
    ///
    /// Output stats for all "below*" matched processes from 08:30:00 to 08:30:30:
    ///
    /// $ below dump process -b "08:30:00" -e "08:30:30" -s comm -F below* -O json
    ///
    /// Output stats for top 5 CPU intense processes for each time slice from 08:30:00 to 08:30:30:
    ///
    /// $ below dump process -b "08:30:00" -e "08:30:30" -s cpu_total --rsort --top 5
    Process {
        /// Select which fields to display and in what order.
        #[structopt(short, long)]
        fields: Option<Vec<ProcField>>,
        #[structopt(flatten)]
        opts: GeneralOpt,
        /// Select field for operation, use with --sort, --rsort, --filter, --top
        #[structopt(long, short)]
        select: Option<ProcField>,
    },
    /// Dump cgroup stats
    ///
    /// ********************** Available fields **********************
    ///
    /// timestamp, datetime, name, full_path
    ///
    /// cpu_usage, cpu_user, cpu_system, cpu_nr_periods, cpu_nr_throttled
    ///
    /// mem_total, mem_anon, mem_file, mem_kernel, mem_slab, mem_sock, mem_shem,
    /// mem_file_mapped, mem_file_dirty, mem_file_writeback, mem_anon_thp, mem_inactive_anon,
    /// mem_active_anon, mem_inactive_file, mem_active_file, mem_unevictable, mem_slab_reclaimable,
    /// mem_slab_unreclaimable
    ///
    /// io_read, io_write, io_wios, io_rios, io_dbps, io_diops, io_total
    ///
    /// pressure_cpu_some, pressure_io_some, pressure_io_full, pressure_mem_some, pressure_mem_full
    ///
    /// ********************** Aggregated fields **********************
    ///
    /// * cpu: includes [cpu_usage]. Addtionally includes [cpu_*] if --detail specified.
    ///
    /// * mem: includes [mem_total]. Addtionally includes [mem_*] if --detail specified.
    ///
    /// * io: incldues [io_read, io_write]. Addtionally includes [io_*] if --detail specified.
    ///
    /// * pressure: includes [pressure_cpu_some, pressure_mem_full, pressure_io_full],
    /// Addtionally includes [pressure_*] if --detail specified
    ///
    /// --default will have all of [name, cpu, mem, io, pressure]. To display everything, use --everything.
    ///
    /// ********************** Example Commands **********************
    ///
    /// Simple example:
    ///
    /// $ below dump cgroup -b "08:30:00" -e "08:30:30" -f name cpu -O csv
    ///
    /// Output stats for all cgroups matching pattern "below*" for time slices
    /// from 08:30:00 to 08:30:30:
    ///
    /// $ below dump cgroup -b "08:30:00" -e "08:30:30" -s comm -F below* -O json
    ///
    /// Output stats for top 5 CPU intense cgroups for each time slice
    /// from 08:30:00 to 08:30:30 recursively:
    ///
    /// $ below dump cgroup -b "08:30:00" -e "08:30:30" -s cpu_usage --rsort --top 5
    Cgroup {
        /// Select which fields to display and in what order.
        #[structopt(short, long)]
        fields: Option<Vec<CgroupField>>,
        #[structopt(flatten)]
        opts: GeneralOpt,
        /// Select field for operation, use with --sort, --rsort, --filter, --top
        #[structopt(long, short)]
        select: Option<CgroupField>,
    },
}
