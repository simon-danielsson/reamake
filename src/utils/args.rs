use std::{io, path::PathBuf};

// use crate::IssueStatus;

#[derive(PartialEq, Clone)]
pub struct Arguments {
    pub help: bool,
    pub summary: bool,
    pub no_rec: bool,
    pub filter_tag: String,
    pub filter_desc: String,
    // pub filter_status: Option<IssueStatus>,
    pub opt_dir: PathBuf,
    /// (id<u32>, status to switch to<Option<IssueStatus>>)
    // pub change_status: (u32, Option<IssueStatus>),
    pub copy_id: u32,
}

pub fn parse() -> io::Result<Arguments> {
    let mut it = std::env::args().skip(1); // skip program name
    let mut filter_tag = String::new();
    let mut filter_status = String::new();
    let mut filter_desc = String::new();
    let mut help = false;
    let mut summary = false;
    let mut no_rec = false;
    let mut opt_dir = PathBuf::new();

    let mut copy_id: u32 = 0;

    // status change subc vars
    // let mut status_ch_status: Option<IssueStatus> = None;
    let mut status_ch_id: u32 = 0;

    while let Some(arg) = it.next() {
        match arg.as_str() {
            "-t" => {
                filter_tag =
                    it.next().expect("No tag was given after the \"-t\" flag.");
            }

            "-s" => {
                filter_status = it
                    .next()
                    .expect("No status was given after the \"-s\" flag.");
            }

            "-d" => {
                filter_desc = it
                    .next()
                    .expect("No status was given after the \"-s\" flag.");
            }

            "-r" => {
                no_rec = true;
            }

            "help" => {
                help = true;
            }

            // change status of issue acc. to id
            // "open" | "op" => {
            //     status_ch_status = Some(IssueStatus::Open);
            //     status_ch_id = it
            //         .next()
            //         .as_deref()
            //         .unwrap_or(format!("{}", 0).as_str())
            //         .parse::<u32>()
            //         .unwrap_or(0);
            // }
            // "close" | "cl" | "closed" => {
            //     status_ch_status = Some(IssueStatus::Closed);
            //     status_ch_id = it
            //         .next()
            //         .as_deref()
            //         .unwrap_or(format!("{}", 0).as_str())
            //         .parse::<u32>()
            //         .unwrap_or(0);
            // }
            // "progress" | "pr" | "prog" => {
            //     status_ch_status = Some(IssueStatus::InProgress);
            //     status_ch_id = it
            //         .next()
            //         .as_deref()
            //         .unwrap_or(format!("{}", 0).as_str())
            //         .parse::<u32>()
            //         .unwrap_or(0);
            // }
            "copy" | "cp" => {
                copy_id = it
                    .next()
                    .as_deref()
                    .unwrap_or(format!("{}", 0).as_str())
                    .parse::<u32>()
                    .unwrap_or(0);
            }

            "summary" => {
                summary = true;
            }

            other => {
                opt_dir = PathBuf::from(other); // optional target dir
                break;
            }
        }
    }

    // let s_lower = filter_status.to_lowercase();
    // let status: Option<IssueStatus> = match () {
    //     _ if s_lower.contains("op") || s_lower.contains("en") => Some(IssueStatus::Open),
    //     _ if s_lower.contains("pr") || s_lower.contains("og") => {
    //         Some(IssueStatus::InProgress)
    //     }
    //     _ if s_lower.contains("cl") || s_lower.contains("os") => Some(IssueStatus::Closed),
    //     _ => None,
    // };

    let filter_tag = filter_tag.trim();
    let filter_desc = filter_desc.trim();

    Ok(Arguments {
        help,
        summary,
        no_rec,
        filter_tag: filter_tag.to_string(),
        filter_desc: filter_desc.to_string(),
        // filter_status: status,
        opt_dir,
        // change_status: (status_ch_id, status_ch_status),
        copy_id,
    })
}
