// Created By CSDC-K
// I'm not crpytographer i just pressed all keys :D

// CRYPTIT HARDWARE FINGERPRINT
// version : 0.1


use hardware_query::{GPUVendor, HardwareInfo};
use sha2::{Sha256, Digest};
use crate::libs::errors;
use crate::libs::argon2_lib;
use crate::libs::salt_gen;
use crate::libs::salt_gen::salt_gen;

pub fn _func_get_cpu_info(password : &str, salt_leng : usize) -> Result<Vec<u8>, errors::cRyptoError>{
    let hw = HardwareInfo::query().unwrap();

    let salt_len = salt_gen::salt_gen(salt_leng)?;

    let cpu = hw.cpu();
    let ram = hw.memory();
 
    let mut gpu_vendor: Option<GPUVendor> = None;
    let mut gpu_modelname: Option<String> = None;
    let mut gpu_gb: Option<f64> = None;

    for gpu in hw.gpus() {
        gpu_vendor = Some(gpu.vendor.clone());
        gpu_modelname = Some(gpu.model_name.clone());
        gpu_gb = Some(gpu.memory_gb().clone());

        break;
    }


    let raw_fingerprint = format!(
        "cpu_name:{}|
        cpu_logicores:{}|
        cpu_basefreq:{}|
        cpu_pyhscore:{}|
        ram_channels:{}|
        ram_eccsupport:{}|
        ram_banwith:{}|
        ram_speed:{}|
        ram_usage:{}|
        gpu_vendor:{:?}|
        gpu_modelname:{:?}|
        gpu_gb:{:?}|
        raw_password:{}|
        salt:{}",


        cpu.model_name,
        cpu.logical_cores,
        cpu.base_frequency,
        cpu.physical_cores,
        ram.channels,
        ram.ecc_support,
        ram.total_mb,
        ram.speed_mhz,
        ram.used_mb,
        gpu_vendor,
        gpu_modelname,
        gpu_gb,
        password,
        salt_len
        
    );
    let result = argon2_lib::create_argon2(&raw_fingerprint, salt_leng)?;


    Ok(result.to_vec())


}